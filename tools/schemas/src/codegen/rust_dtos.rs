use crate::model;
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

    use crate::identity::*;
    use crate::formats::Multihash;
    use chrono::{DateTime, Utc};
    use std::path::PathBuf;
    use enum_variants::*;
    use bitflags::bitflags;
    "#
);

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn render(model: model::Model, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    writeln!(w, "{}", PREAMBLE)?;

    for typ in model.types.values() {
        if typ.id().name == "Manifest" {
            continue;
        }

        writeln!(w, "////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////")?;
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
        }
        writeln!(w)?;
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_struct(typ: &model::Struct, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    writeln!(w, "#[derive(Clone, PartialEq, Eq, Debug)]")?;
    writeln!(w, "pub struct {} {{", typ.id.join(""))?;

    for field in typ.fields.values() {
        render_description(
            &field.description,
            field.default.as_ref(),
            field.examples.as_ref(),
            w,
        )?;
        if !field.optional {
            writeln!(w, "pub {}: {},", field.name, format_type(&field.typ))?;
        } else {
            writeln!(
                w,
                "pub {}: Option<{}>,",
                field.name,
                format_type(&field.typ)
            )?;
        }
    }

    writeln!(w, "}}")?;
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

fn format_type(typ: &model::Type) -> String {
    match typ {
        model::Type::Boolean => format!("bool"),
        model::Type::Int16 => format!("i16"),
        model::Type::Int32 => format!("i32"),
        model::Type::Int64 => format!("i64"),
        model::Type::UInt16 => format!("u16"),
        model::Type::UInt32 => format!("u32"),
        model::Type::UInt64 => format!("u64"),
        model::Type::String => format!("String"),
        model::Type::DatasetAlias => format!("DatasetAlias"),
        model::Type::DatasetId => format!("DatasetID"),
        model::Type::DatasetRef => format!("DatasetRef"),
        model::Type::DateTime => format!("DateTime<Utc>"),
        model::Type::Flatbuffers => format!("Vec<u8>"),
        model::Type::Multicodec => format!("Multicodec"),
        model::Type::Multihash => format!("Multihash"),
        model::Type::Path => format!("PathBuf"),
        model::Type::Regex => format!("String"),
        model::Type::Url => format!("String"),
        model::Type::Array(t) => format!("Vec<{}>", format_type(&t.item_type)),
        model::Type::Custom(name) => name.join("").to_string(),
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
        "    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]"
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
