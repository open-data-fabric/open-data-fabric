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

    use std::path::PathBuf;
    use super::formats::{base64, datetime_rfc3339, datetime_rfc3339_opt};
    use crate::*;
    use ::serde::{Deserialize, Deserializer, Serialize, Serializer};
    use chrono::{DateTime, Utc};
    use serde_with::serde_as;
    use serde_with::skip_serializing_none;

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    macro_rules! implement_serde_as {
        ($dto:ty, $impl:ty, $impl_name:literal) => {
            impl ::serde_with::SerializeAs<$dto> for $impl {
                fn serialize_as<S>(source: &$dto, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    <$impl>::serialize(source, serializer)
                }
            }

            impl<'de> serde_with::DeserializeAs<'de, $dto> for $impl {
                fn deserialize_as<D>(deserializer: D) -> Result<$dto, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    <$impl>::deserialize(deserializer)
                }
            }
        };
    }
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
        writeln!(w, "// {}", typ.id().join(""))?;
        writeln!(
            w,
            "// {SPEC_URL}#{}-schema",
            typ.id().join("").to_lowercase()
        )?;
        writeln!(w, "////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////")?;
        writeln!(w, "")?;

        match &typ {
            model::TypeDefinition::Struct(t) => render_struct(t, w)?,
            model::TypeDefinition::Union(t) => render_union(t, w)?,
            model::TypeDefinition::Enum(t) => render_enum(t, w)?,
        }
        writeln!(w)?;
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_struct(typ: &model::Struct, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let name = typ.id.join("");

    writeln!(w, "#[serde_as]")?;
    writeln!(w, "#[skip_serializing_none]")?;
    writeln!(
        w,
        "#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]"
    )?;
    writeln!(w, "#[serde(remote = \"{name}\")]")?;
    writeln!(
        w,
        "#[serde(deny_unknown_fields, rename_all = \"camelCase\")]"
    )?;
    writeln!(w, "pub struct {name}Def {{")?;

    for field in typ.fields.values() {
        render_field(field, w)?;
    }

    writeln!(w, "}}")?;
    writeln!(w)?;
    writeln!(w, "implement_serde_as!({name}, {name}Def, \"{name}Def\");")?;
    Ok(())
}

fn render_field(field: &model::Field, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    match &field.typ {
        model::Type::DateTime => {
            if field.optional {
                writeln!(w, "#[serde(default, with = \"datetime_rfc3339_opt\")]")?;
            } else {
                writeln!(w, "#[serde(with = \"datetime_rfc3339\")]")?;
            }
        }
        model::Type::Flatbuffers => {
            writeln!(w, "#[serde(with = \"base64\")]")?;
        }
        model::Type::Array(arr) => match &*arr.item_type {
            model::Type::Custom(id) => {
                if field.optional {
                    writeln!(w, "#[serde_as(as = \"Option<Vec<{}Def>>\")]", id.join(""))?;
                    writeln!(w, "#[serde(default)]")?;
                } else {
                    writeln!(w, "#[serde_as(as = \"Vec<{}Def>\")]", id.join(""))?;
                }
            }
            _ => (),
        },
        model::Type::Custom(id) => {
            if field.optional {
                writeln!(w, "#[serde_as(as = \"Option<{}Def>\")]", id.join(""))?;
                writeln!(w, "#[serde(default)]")?;
            } else {
                writeln!(w, "#[serde_as(as = \"{}Def\")]", id.join(""))?;
            }
        }
        _ => (),
    };

    let typ = format_type(&field.typ);
    let typ = if field.optional {
        format!("Option<{typ}>")
    } else {
        typ
    };

    writeln!(w, "pub {}: {typ},", field.name)?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_union(typ: &model::Union, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let name = typ.id.join("");

    writeln!(w, "#[serde_as]")?;
    writeln!(
        w,
        "#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]"
    )?;
    writeln!(w, "#[serde(remote = \"{name}\")]")?;
    writeln!(w, "#[serde(deny_unknown_fields, tag = \"kind\")]")?;
    writeln!(w, "pub enum {name}Def {{")?;

    for variant in &typ.variants {
        render_union_variant(variant, w)?;
    }

    writeln!(w, "}}")?;
    writeln!(w)?;
    writeln!(w, "implement_serde_as!({name}, {name}Def, \"{name}Def\");")?;

    Ok(())
}

fn render_union_variant(
    variant: &model::TypeId,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    let varname = &variant.name;
    let typename = variant.join("");

    // Allow lowercase and camelCase names
    render_aliases(varname, w)?;
    writeln!(
        w,
        "{varname}(#[serde_as(as = \"{typename}Def\")] {typename}),"
    )?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_enum(typ: &model::Enum, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let name = typ.id.join("");

    writeln!(
        w,
        "#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]"
    )?;
    writeln!(w, "#[serde(remote = \"{name}\")]")?;
    writeln!(w, "#[serde(deny_unknown_fields)]")?;
    writeln!(w, "pub enum {name}Def {{")?;
    {
        for variant in &typ.variants {
            render_aliases(&variant, w)?;
            writeln!(w, "{variant},")?;
        }
    }
    writeln!(w, "}}")?;
    writeln!(w)?;
    writeln!(w, "implement_serde_as!({name}, {name}Def, \"{name}Def\");")?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Allow lowercase and camelCase names for enums
fn render_aliases(name: &str, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let aliases: Vec<_> =
        std::collections::BTreeSet::from([name.to_lowercase(), name.to_case(Case::Camel)])
            .into_iter()
            .collect();

    write!(w, "#[serde(")?;
    for (i, alias) in aliases.into_iter().enumerate() {
        if i != 0 {
            write!(w, ", ")?;
        }
        write!(w, "alias = \"{alias}\"")?;
    }
    writeln!(w, ")]")?;

    //writeln!(w, "#[serde(alias = \"{}\")]", varname.to_lowercase())?;
    //writeln!(
    //    w,
    //    "{varname}(#[serde_as(as = \"{typename}Def\")] {typename}),"
    //)?;
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
