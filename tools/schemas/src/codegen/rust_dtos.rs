use super::rust_common::format_ident;
use crate::{
    json_schema::{CodegenHint, CodegenLanguage},
    model,
};
use convert_case::{Case, Casing};

const SPEC_URL: &str =
    "https://github.com/kamu-data/open-data-fabric/blob/master/open-data-fabric.md";

const PREAMBLE: &str = indoc::indoc!(
    r#"
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // WARNING: This file is auto-generated from Open Data Fabric Schemas
    // See: http://opendatafabric.org/
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    #![allow(clippy::all)]
    #![allow(clippy::pedantic)]

    use std::path::PathBuf;

    use bitflags::bitflags;
    use chrono::{DateTime, Utc};
    use enum_variants::*;
    use serde::{Deserialize, Serialize};

    use crate::formats::Multihash;
    use crate::identity::*;
    "#
);

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn render(model: model::Model, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    writeln!(w, "{}", PREAMBLE)?;

    for typ in model.types.values() {
        if typ
            .codegen_hints()
            .get(&CodegenLanguage::Rust)
            .and_then(|h| h.get(&CodegenHint::DtoType))
            .is_some()
        {
            // Externally defined type
            continue;
        }

        if typ.is_resource_variant() {
            // Resource variants are covered by Resource<SpecT> type
            continue;
        }

        writeln!(
            w,
            "////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////"
        )?;
        writeln!(w)?;
        render_description(typ.description(), None, None, w)?;
        writeln!(w, "///")?;
        writeln!(
            w,
            "/// See: {SPEC_URL}#{}-schema",
            typ.id().join("").to_lowercase()
        )?;

        match &typ {
            model::TypeDefinition::Struct(t) => render_struct(t, w)?,
            model::TypeDefinition::Union(t) => {
                render_union(t, w)?;

                if typ.id().name == "MetadataEvent" {
                    writeln!(w)?;
                    render_union_bitflags(t, w)?;
                }
            }
            model::TypeDefinition::Enum(t) => render_enum(t, w)?,
            model::TypeDefinition::Map(t) => render_map(t, w)?,
        }
        writeln!(w)?;
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_struct(typ: &model::Struct, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let generics = format!("<{}>", typ.generics.join(", "));

    let mut derives = vec!["Clone", "Debug", "Eq"];

    if !typ.fields.values().any(|f| f.default.is_some()) {
        derives.push("PartialEq");
    }
    if typ.fields.values().all(|f| f.optional) {
        derives.push("Default");
    }

    writeln!(w, "#[derive({})]", derives.join(", "))?;
    writeln!(w, "pub struct {}{} {{", typ.id.join(""), generics)?;

    for field in typ.fields.values() {
        render_description(
            &field.description,
            field.default.as_ref(),
            field.examples.as_ref(),
            w,
        )?;

        let mut typ = format_type(&field.typ);
        if let Some(container) = field
            .codegen_hints
            .get(&CodegenLanguage::Rust)
            .and_then(|m| m.get(&CodegenHint::Container))
        {
            typ = format!("{container}<{typ}>");
        }
        if field.optional {
            typ = format!("Option<{typ}>");
        }
        writeln!(w, "pub {}: {},", format_ident(&field.name), typ)?;
    }

    writeln!(w, "}}")?;

    if typ.fields.values().any(|f| f.default.is_some()) {
        writeln!(w)?;
        writeln!(w, "impl {} {{", typ.id.join(""))?;

        for field in typ.fields.values() {
            if let Some(default) = &field.default {
                let name = format_ident(&field.name);
                let mut default_typ = format_type(&field.typ);
                let mut accessor_typ = default_typ.clone();

                let value = match &field.typ {
                    model::Type::Boolean => format!("{default}"),
                    model::Type::String => {
                        default_typ = "&'static str".to_string();
                        accessor_typ = "&str".to_string();
                        format!("{default}")
                    }
                    model::Type::Custom(_tid) => {
                        format!("{default_typ}::{}", default.as_str().unwrap())
                    }
                    typ => unimplemented!(
                        "Default value formatting for {typ:?} types is not implemented"
                    ),
                };

                writeln!(w, "pub fn default_{name}() -> {default_typ} {{")?;
                writeln!(w, "{value}")?;
                writeln!(w, "}}")?;

                writeln!(w, "pub fn {name}(&self) -> {accessor_typ} {{")?;

                match &field.typ {
                    model::Type::Boolean | model::Type::Custom(_) => {
                        writeln!(w, "self.{name}.unwrap_or(Self::default_{name}())")?
                    }
                    model::Type::String => writeln!(
                        w,
                        "self.{name}.as_deref().unwrap_or(Self::default_{name}())"
                    )?,
                    typ => unimplemented!(
                        "Default value formatting for {typ:?} types is not implemented"
                    ),
                };

                writeln!(w, "}}")?;
            }
        }

        writeln!(w, "}}")?;
    }

    // Impl PartialEq that treats missing default properties equal to default values
    if typ.fields.values().any(|f| f.default.is_some()) {
        writeln!(w)?;
        writeln!(w, "impl PartialEq for {} {{", typ.id.join(""))?;
        writeln!(w, "fn eq(&self, other: &Self) -> bool {{")?;

        for (i, field) in typ.fields.values().enumerate() {
            let fname = format_ident(&field.name);
            if i != 0 {
                writeln!(w, "&&")?;
            }
            if field.default.is_none() {
                writeln!(w, "self.{fname} == other.{fname}")?;
            } else {
                match &field.typ {
                    model::Type::String => writeln!(
                        w,
                        "self.{fname}.as_deref().or_else(|| Some(Self::default_{fname}())) == other.{fname}.as_deref().or_else(|| Some(Self::default_{fname}()))"
                    )?,
                    _ => writeln!(
                        w,
                        "self.{fname}.or_else(|| Some(Self::default_{fname}())) == other.{fname}.or_else(|| Some(Self::default_{fname}()))"
                    )?,
                };
            }
        }

        writeln!(w, "}} }}")?;
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_union(typ: &model::Union, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    writeln!(w, "#[derive(Clone, PartialEq, Eq, Debug)]")?;
    writeln!(w, "pub enum {} {{", typ.id.join(""))?;
    for variant in &typ.variants {
        writeln!(w, "{}({}),", variant.name, variant.join(""))?;
    }
    writeln!(w, "}}")?;
    writeln!(w)?;
    writeln!(w, "impl_enum_with_variants!({});", typ.id.name)?;
    for variant in &typ.variants {
        writeln!(
            w,
            "impl_enum_variant!({}::{}({}));",
            typ.id.name,
            variant.name,
            variant.join("")
        )?;
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_enum(typ: &model::Enum, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    if typ.id.name == "DatasetKind" {
        // TODO: Introduce `extra_derives`
        writeln!(w, "#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]")?;
    } else {
        writeln!(w, "#[derive(Clone, Copy, PartialEq, Eq, Debug)]")?;
    }
    writeln!(w, "pub enum {} {{", typ.id.join(""))?;
    for variant in &typ.variants {
        writeln!(w, "{variant},")?;
    }
    writeln!(w, "}}")?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_map(typ: &model::Map, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let value_type = format_type(&typ.value_type);
    writeln!(w, "#[derive(Clone, PartialEq, Eq, Debug)]")?;
    writeln!(w, "pub struct {} {{", typ.id.join(""))?;
    writeln!(
        w,
        "pub entries: std::collections::BTreeMap<String, {value_type}>,"
    )?;
    writeln!(w, "}}")?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn format_type(typ: &model::Type) -> String {
    match typ {
        model::Type::Boolean => format!("bool"),
        model::Type::Int8 => format!("i8"),
        model::Type::Int16 => format!("i16"),
        model::Type::Int32 => format!("i32"),
        model::Type::Int64 => format!("i64"),
        model::Type::UInt8 => format!("u8"),
        model::Type::UInt16 => format!("u16"),
        model::Type::UInt32 => format!("u32"),
        model::Type::UInt64 => format!("u64"),
        model::Type::String => format!("String"),

        // model::Type::Multicodec => format!("Multicodec"),
        model::Type::DateTime => format!("DateTime<Utc>"),
        model::Type::Multicodec => format!("String"),
        model::Type::Multihash => format!("Multihash"),
        model::Type::Path => format!("PathBuf"),
        model::Type::Regex => format!("String"),
        model::Type::Url => format!("String"),

        model::Type::AccountId => format!("AccountID"),
        model::Type::AccountName => format!("AccountName"),
        model::Type::DatasetAlias => format!("DatasetAlias"),
        model::Type::DatasetRef => format!("DatasetRef"),
        model::Type::DatasetId => format!("DatasetID"),
        model::Type::ResourceId => format!("ResourceID"),
        model::Type::ResourceName => format!("ResourceName"),
        model::Type::ResourceTypeUri => format!("ResourceTypeUri"),
        model::Type::ResourceTypeName => format!("ResourceTypeName"),
        model::Type::ResourceTypeRef => format!("ResourceTypeRef"),

        model::Type::Flatbuffers => format!("Vec<u8>"),
        model::Type::Generic(name) => name.clone(),
        model::Type::Array(t) => format!("Vec<{}>", format_type(&t.item_type)),
        model::Type::Custom(name) => name.join("").to_string(),
        model::Type::AnyJson => format!("serde_json::Value"),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_description(
    desc: &str,
    default: Option<&serde_json::Value>,
    examples: Option<&Vec<serde_json::Value>>,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    for line in desc.split('\n') {
        writeln!(w, "/// {line}")?;
    }
    if let Some(default) = default {
        writeln!(w, "///")?;
        writeln!(w, "/// Defaults to: {default}")?;
    }
    if let Some(examples) = examples {
        writeln!(w, "///")?;
        writeln!(w, "/// Examples:")?;
        for ex in examples {
            writeln!(w, "/// - {ex}")?;
        }
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_union_bitflags(
    typ: &model::Union,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    writeln!(w, "bitflags! {{")?;
    writeln!(
        w,
        "    #[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]"
    )?;
    writeln!(w, "    pub struct {}TypeFlags: u32 {{", typ.id.name)?;
    for (i, variant) in typ.variants.iter().enumerate() {
        writeln!(
            w,
            "        const {} = 1 << {};",
            variant.name.to_case(Case::UpperSnake),
            i
        )?;
    }
    writeln!(w, "    }}")?;
    writeln!(w, "}}")?;
    writeln!(w)?;

    writeln!(
        w,
        "impl From<&{}> for {}TypeFlags {{",
        typ.id.name, typ.id.name
    )?;
    writeln!(w, "    fn from(v: &{}) -> Self {{", typ.id.name)?;
    writeln!(w, "        match v {{")?;
    for variant in &typ.variants {
        writeln!(
            w,
            "            {}::{}(_) => Self::{},",
            typ.id.name,
            variant.name,
            variant.name.to_case(Case::UpperSnake),
        )?;
    }
    writeln!(w, "        }}")?;
    writeln!(w, "    }}")?;
    writeln!(w, "}}")?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
