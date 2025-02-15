use crate::model;
use crate::utils::indent_writer::IndentWriter;
use std::io::Write;

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
    "#
);

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn render(model: model::Model, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let mut w = IndentWriter::new(w, "  ");
    render_impl(model, &mut w)
}

fn render_impl(
    model: model::Model,
    w: &mut IndentWriter<&mut dyn std::io::Write>,
) -> Result<(), std::io::Error> {
    writeln!(w, "{}", PREAMBLE)?;

    for typ in model.types.values() {
        writeln!(w, "////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////")?;
        writeln!(w, "// {}", typ.name())?;
        writeln!(w, "// {SPEC_URL}#{}-schema", typ.name().0.to_lowercase())?;
        writeln!(w, "////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////")?;
        writeln!(w)?;

        match &typ {
            model::TypeDefinition::Object(t) => render_object(t, w)?,
            model::TypeDefinition::Union(t) => render_union(t, w)?,
            model::TypeDefinition::Enum(t) => render_enum(t, w)?,
        }
        writeln!(w, "")?;
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_object(
    typ: &model::Object,
    w: &mut IndentWriter<&mut dyn std::io::Write>,
) -> Result<(), std::io::Error> {
    format_description(&typ.description, w)?;
    writeln!(w, "#[derive(Clone, PartialEq, Eq, Debug)]")?;
    writeln!(w, "pub struct {} {{", typ.name)?;

    let mut i = w.indent();
    for field in typ.fields.values() {
        format_description(&field.description, &mut i)?;
        if !field.optional {
            writeln!(i, "pub {}: {},", field.name, format_type(&field.typ))?;
        } else {
            writeln!(
                i,
                "pub {}: Option<{}>,",
                field.name,
                format_type(&field.typ)
            )?;
        }
    }
    drop(i);

    writeln!(w, "}}")?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_union(
    typ: &model::Union,
    w: &mut IndentWriter<&mut dyn std::io::Write>,
) -> Result<(), std::io::Error> {
    format_description(&typ.description, w)?;
    writeln!(w, "#[derive(Clone, PartialEq, Eq, Debug)]")?;
    writeln!(w, "pub enum {} {{", typ.name)?;
    {
        let mut i = w.indent();
        for variant in &typ.variants {
            writeln!(i, "{}({}),", variant.0, variant.0)?;
        }
    }
    writeln!(w, "}}")?;
    writeln!(w)?;
    writeln!(w, "impl_enum_with_variants!({});", typ.name)?;
    for variant in &typ.variants {
        writeln!(
            w,
            "impl_enum_variant!({}::{}({}));",
            typ.name, variant.0, variant.0
        )?;
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_enum(
    typ: &model::Enum,
    w: &mut IndentWriter<&mut dyn std::io::Write>,
) -> Result<(), std::io::Error> {
    format_description(&typ.description, w)?;
    writeln!(w, "#[derive(Clone, Copy, PartialEq, Eq, Debug)]")?;
    writeln!(w, "pub enum {} {{", typ.name)?;
    {
        let mut i = w.indent();
        for variant in &typ.variants {
            writeln!(i, "{variant},")?;
        }
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
        model::Type::Url => format!("Url"),
        model::Type::Array(t) => format!("Vec<{}>", format_type(&t.item_type)),
        model::Type::Custom(name) => name.0.clone(),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn format_description(desc: &str, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    for line in desc.split('\n') {
        writeln!(w, "/// {line}")?;
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
