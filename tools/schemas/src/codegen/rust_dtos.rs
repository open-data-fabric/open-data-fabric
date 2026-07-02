use std::collections::BTreeMap;

use super::rust_common::format_ident;
use crate::{
    json_schema::{CodegenHint, CodegenLanguage},
    model,
};
use convert_case::{Case, Casing};

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
    use multiformats::*;
    use serde::{Deserialize, Serialize};
    use setty::types::{ByteSize, DurationString};

    use crate::auth::*;
    use crate::dataset::*;
    use crate::resource::*;
    "#
);

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn render(model: model::Model, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    writeln!(w, "{}", PREAMBLE)?;

    // Group by `context` and sort by names
    let types_by_context: BTreeMap<&str, BTreeMap<String, &model::TypeDefinition>> =
        model.types.values().fold(BTreeMap::new(), |mut map, t| {
            map.entry(t.id().context())
                .or_insert_with(BTreeMap::new)
                .insert(t.id().join("").into(), t);
            map
        });

    for (context, types) in &types_by_context {
        writeln!(
            w,
            "////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////"
        )?;
        writeln!(w, "// {context}")?;
        writeln!(
            w,
            "////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////\n"
        )?;

        writeln!(w, "pub mod {context} {{")?;
        writeln!(w, "#[allow(unused_imports)]")?;
        writeln!(w, "use super::*;\n")?;

        for typ in types.values() {
            if typ
                .codegen_hints()
                .get(&CodegenLanguage::Rust)
                .and_then(|h| h.get(&CodegenHint::DtoType))
                .is_some()
            {
                // Externally defined types are re-exported from the module
                writeln!(
                    w,
                    "pub use crate::{}::{};\n",
                    typ.id().context(),
                    typ.id().name()
                )?;
                continue;
            }

            render_description(typ.description(), None, None, w)?;
            writeln!(w, "///")?;
            writeln!(w, "/// Schema: {}", typ.id().schema_id())?;

            match &typ {
                model::TypeDefinition::Struct(t) => render_struct(t, w)?,
                model::TypeDefinition::Union(t) => {
                    render_union(t, w)?;

                    if typ.id().name() == "MetadataEvent" {
                        writeln!(w)?;
                        render_union_bitflags(t, w)?;
                    }
                }
                model::TypeDefinition::Enum(t) => render_enum(t, w)?,
                model::TypeDefinition::Map(t) => render_map(t, w)?,
            }
            writeln!(w)?;
        }

        writeln!(w, "}}")?;
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_struct(typ: &model::Struct, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let name = typ.id.join("");
    let generics = format!("<{}>", typ.generics.join(", "));

    let mut derives = vec!["Clone", "Debug", "Eq"];

    if !typ.fields.values().any(|f| f.default.is_some()) {
        derives.push("PartialEq");
    }
    if typ.fields.values().all(|f| f.optional) {
        derives.push("Default");
    }

    writeln!(w, "#[derive({})]", derives.join(", "))?;
    writeln!(w, "pub struct {name}{generics} {{")?;

    for field in typ.fields.values() {
        if field.constant.is_some() {
            continue;
        }

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

    let mut statics = Vec::new();

    if typ
        .fields
        .values()
        .any(|f| f.constant.is_some() || f.default.is_some())
    {
        writeln!(w)?;
        writeln!(w, "impl {name} {{")?;

        // Constants: free static field + accessor method
        for field in typ.fields.values() {
            let Some(constant) = &field.constant else {
                continue;
            };

            let static_name = format!(
                "{}_{}",
                name.to_case(Case::UpperSnake),
                format_ident(&field.name).to_uppercase()
            );
            let accessor_name = format_ident(&field.name);

            let (accessor_typ, static_typ, value) = match &field.typ {
                model::Type::String => ("&'static str", "&str", format!("{constant}")),
                model::Type::TypeUri => (
                    "&'static TypeUri",
                    "std::sync::LazyLock<TypeUri>",
                    format!("std::sync::LazyLock::new(|| TypeUri::new_unchecked({constant}))"),
                ),
                typ => {
                    unimplemented!("Const value formatting for {typ:?} types is not implemented")
                }
            };

            writeln!(
                w,
                "pub fn {accessor_name}() -> {accessor_typ} {{ &{static_name} }}"
            )?;

            // For TypeUri, split into string literal + LazyLock
            if matches!(&field.typ, model::Type::TypeUri) {
                let str_static_name = format!("{}_STR", static_name);
                let str_accessor_name = format!("{}_str", accessor_name);
                writeln!(
                    w,
                    "pub fn {str_accessor_name}() -> &'static str {{ {str_static_name} }}"
                )?;
                statics.push(format!("static {str_static_name}: &str = {constant};"));
                statics.push(format!("static {static_name}: {static_typ} = std::sync::LazyLock::new(|| {{ TypeUri::new_unchecked({str_static_name}) }});"));
            } else {
                statics.push(format!("static {static_name}: {static_typ} = {value};"));
            }
        }

        // Defaults
        for field in typ.fields.values() {
            let Some(default) = &field.default else {
                continue;
            };

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
                typ => {
                    unimplemented!("Default value formatting for {typ:?} types is not implemented")
                }
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
                typ => {
                    unimplemented!("Default value formatting for {typ:?} types is not implemented")
                }
            };

            writeln!(w, "}}")?;
        }

        writeln!(w, "}}\n")?;
    }

    for st in statics {
        writeln!(w, "{st}\n")?;
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

        writeln!(w, "}} }}\n")?;
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_union(typ: &model::Union, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    writeln!(w, "#[derive(Clone, PartialEq, Eq, Debug)]")?;
    writeln!(w, "pub enum {} {{", typ.id.join(""))?;
    for variant in &typ.variants {
        writeln!(
            w,
            "{}({}::{}),",
            variant.name(),
            variant.context(),
            variant.join("")
        )?;
    }
    writeln!(w, "}}")?;
    writeln!(w)?;
    writeln!(w, "impl_enum_with_variants!({});", typ.id.name())?;
    for variant in &typ.variants {
        writeln!(
            w,
            "impl_enum_variant!({}::{}({}::{}));",
            typ.id.name(),
            variant.name(),
            variant.context(),
            variant.join("")
        )?;
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_enum(typ: &model::Enum, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    if typ.id.name() == "DatasetKind" {
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
    // TODO: Quick hack - replace with `parse_type_scalar`
    let key_type = match typ
        .codegen_hints
        .get(&CodegenLanguage::Rust)
        .and_then(|h| h.get(&CodegenHint::MapKeyFormat))
        .map(|s| s.as_str())
    {
        Some("type-ref") => model::Type::TypeRef,
        _ => model::Type::String,
    };
    let key_type = format_type(&key_type);
    let value_type = format_type(&typ.value_type);

    writeln!(w, "#[derive(Clone, PartialEq, Eq, Debug)]")?;
    writeln!(w, "pub struct {} {{", typ.id.join(""))?;
    writeln!(
        w,
        "pub entries: std::collections::BTreeMap<{key_type}, {value_type}>,"
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
        model::Type::ByteSize => format!("ByteSize"),
        model::Type::DateTime => format!("DateTime<Utc>"),
        model::Type::Duration => format!("DurationString"),
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
        model::Type::TypeUri => format!("TypeUri"),
        model::Type::TypeName => format!("TypeName"),
        model::Type::TypeRef => format!("TypeRef"),

        model::Type::Flatbuffers => format!("Vec<u8>"),
        model::Type::Generic(name) => name.clone(),
        model::Type::Array(t) => format!("Vec<{}>", format_type(&t.item_type)),
        model::Type::Custom(t) => format!("{}::{}", t.context(), t.join("")),
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
    writeln!(w, "    pub struct {}TypeFlags: u32 {{", typ.id.name())?;
    for (i, variant) in typ.variants.iter().enumerate() {
        writeln!(
            w,
            "        const {} = 1 << {};",
            variant.name().to_case(Case::UpperSnake),
            i
        )?;
    }
    writeln!(w, "    }}")?;
    writeln!(w, "}}")?;
    writeln!(w)?;

    writeln!(
        w,
        "impl From<&{}> for {}TypeFlags {{",
        typ.id.name(),
        typ.id.name()
    )?;
    writeln!(w, "    fn from(v: &{}) -> Self {{", typ.id.name())?;
    writeln!(w, "        match v {{")?;
    for variant in &typ.variants {
        writeln!(
            w,
            "            {}::{}(_) => Self::{},",
            typ.id.name(),
            variant.name(),
            variant.name().to_case(Case::UpperSnake),
        )?;
    }
    writeln!(w, "        }}")?;
    writeln!(w, "    }}")?;
    writeln!(w, "}}")?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
