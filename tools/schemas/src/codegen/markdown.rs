use std::path::Path;

use crate::model;
use convert_case::{Case, Casing};

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn render(model: model::Model, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let lvl = 4;
    render_toc(&model, w)?;

    writeln!(w)?;

    let contexts: std::collections::BTreeSet<&str> =
        model.types.values().map(|t| t.id().context()).collect();

    for context in contexts {
        render_section(&model, context, lvl, w)?;
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn section_id(name: &str) -> String {
    format!("reference-{}", name.to_lowercase().replace(" ", "-"))
}

fn schema_id(name: &str) -> String {
    format!("{}-schema", name.to_lowercase().replace("::", "-"))
}

fn types_by_context<'a>(model: &'a model::Model, context: &str) -> Vec<&'a model::TypeDefinition> {
    let types: Vec<_> = model
        .types
        .values()
        .filter(|t| t.id().context() == context)
        .collect();

    types
}

fn render_table(
    header: Vec<&str>,
    header_fmt: Vec<&str>,
    rows: Vec<Vec<String>>,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    writeln!(w, "| {} |", header.join(" | "))?;
    writeln!(w, "| {} |", header_fmt.join(" | "))?;

    for values in rows {
        let row: Vec<_> = values
            .into_iter()
            .map(|s| s.replace("\n", "<br/>"))
            .collect();
        writeln!(w, "| {} |", row.join(" | "))?;
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_toc(model: &model::Model, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let contexts: std::collections::BTreeSet<&str> =
        model.types.values().map(|t| t.id().context()).collect();

    for context in contexts {
        render_toc_section(context, model, w)?;
    }

    Ok(())
}

fn render_toc_section(
    context: &str,
    model: &model::Model,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    let id = section_id(context);
    writeln!(w, "- [{context}](#{id})")?;
    for typ in types_by_context(model, context) {
        if typ.id().parent().is_some() {
            continue;
        }
        let name = &typ.id().name();
        let id = schema_id(&name);
        writeln!(w, "  - [{name}](#{id})")?;
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_section(
    model: &model::Model,
    context: &str,
    lvl: usize,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    render_header(context, None, lvl, w)?;
    for typ in types_by_context(model, context) {
        render_type(typ, lvl + 1, model, w)?;
        writeln!(w)?;
    }
    Ok(())
}

fn render_header(
    name: &str,
    id: Option<String>,
    lvl: usize,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    let id = id.unwrap_or_else(|| section_id(name));
    writeln!(w, "<a name=\"{id}\"></a>")?;
    for _ in 0..lvl {
        write!(w, "#")?;
    }
    writeln!(w, " {name}")?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_type(
    typ: &model::TypeDefinition,
    lvl: usize,
    model: &model::Model,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    match typ {
        model::TypeDefinition::Struct(t) => render_struct(t, lvl, w)?,
        model::TypeDefinition::Union(t) => render_union(t, lvl, model, w)?,
        model::TypeDefinition::Enum(t) => render_enum(t, lvl, w)?,
        model::TypeDefinition::Map(t) => render_map(t, lvl, w)?,
    }
    Ok(())
}

fn render_schema_links(path: &Path, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    writeln!(
        w,
        "[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)]({})",
        path.display()
    )?;
    writeln!(
        w,
        "[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](schemas-generated/flatbuffers/opendatafabric.fbs)"
    )?;
    writeln!(w, "[^](#reference-information)")?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_struct(
    typ: &model::Struct,
    lvl: usize,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    let name = typ.id.join("::");
    render_header(name.as_ref(), Some(schema_id(name.as_ref())), lvl, w)?;
    writeln!(w, "{}", typ.description)?;
    writeln!(w)?;

    if !typ.fields.is_empty() {
        render_table(
            vec!["Property", "Type", "Required", "Format", "Description"],
            vec![":---:", ":---:", ":---:", ":---:", "---"],
            typ.fields
                .values()
                .map(|f| {
                    let mut description = f.description.clone();

                    if let Some(default) = &f.default {
                        description += format!("\n\nDefault: {default}").as_str();
                    }

                    vec![
                        format!("`{}`", f.name.to_case(Case::Camel)),
                        as_json_type(&f.typ),
                        if f.optional {
                            String::new()
                        } else {
                            "V".to_string()
                        },
                        as_format(&f.typ),
                        description,
                    ]
                })
                .collect(),
            w,
        )?;
        writeln!(w)?;
    }

    render_schema_links(&typ.src, w)?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_union(
    typ: &model::Union,
    lvl: usize,
    model: &model::Model,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    let name = typ.id.join("::");
    render_header(name.as_ref(), Some(schema_id(name.as_ref())), lvl, w)?;
    writeln!(w, "{}", typ.description)?;
    writeln!(w)?;

    render_table(
        vec!["Union Type", "Description"],
        vec![":---:", "---"],
        typ.variants
            .iter()
            .map(|v| {
                model
                    .types
                    .get(v)
                    .expect(&format!("Expected to find type {}", v.join("::")))
            })
            .map(|t| {
                vec![
                    format!(
                        "[{}](#{})",
                        t.id().join("::"),
                        schema_id(t.id().join("::").as_ref())
                    ),
                    t.description().to_string(),
                ]
            })
            .collect(),
        w,
    )?;
    writeln!(w)?;
    render_schema_links(&typ.src, w)?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_enum(
    typ: &model::Enum,
    lvl: usize,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    let name = typ.id.join("::");
    render_header(name.as_ref(), Some(schema_id(name.as_ref())), lvl, w)?;
    writeln!(w, "{}", typ.description)?;
    writeln!(w)?;

    render_table(
        vec!["Enum Value"],
        vec![":---:"],
        typ.variants.iter().map(|v| vec![v.clone()]).collect(),
        w,
    )?;
    writeln!(w)?;
    render_schema_links(&typ.src, w)?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_map(
    typ: &model::Map,
    lvl: usize,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    let name = typ.id.join("::");
    render_header(name.as_ref(), Some(schema_id(name.as_ref())), lvl, w)?;
    writeln!(w, "{}", typ.description)?;
    writeln!(w)?;

    render_schema_links(&typ.src, w)?;

    writeln!(w)?;

    render_header(
        "Known Extensions",
        Some("known-extra-attrs".to_string()),
        lvl + 1,
        w,
    )?;

    writeln!(
        w,
        indoc::indoc!(
            r#"
            | Extension | Description |
            | --- | --- |
            | `opendatafabric.net/description` | Used for human readable schema field descriptions |
            | `opendatafabric.net/type` | An extended set of logical types that ODF recommends but does not require every implementation to support |
            | `opendatafabric.org/linkedObjects` | When attached to `AddData` event contains a summary of how many external objects were associated with a certain transaction as well as their size |
            | `arrow.apache.org/bufferEncoding` | Used to accurately represent buffer encoding type when converting Arrow schema to ODF schema |
            | `arrow.apache.org/dateEncoding` | Used to accurately represent date encoding type when converting Arrow schema to ODF schema |
            | `arrow.apache.org/decimalEncoding` | Used to accurately represent decimal encoding type when converting Arrow schema to ODF schema |
            "#
        )
    )?;

    render_header(
        "Known Extended Types",
        Some("known-extra-types".to_string()),
        lvl + 1,
        w,
    )?;

    writeln!(
        w,
        indoc::indoc!(
            r#"
            | Extended Type | Core Type | Description |
            | --- | --- | --- |
            | `Did` | `String` | Decentralized identifier `did:<method>:<id>` |
            | `Multihash` | `String` | Hash in self-describing [multihash](https://github.com/multiformats/multihash) format |
            | `ObjectLink` | `String` | Signifies that the value references an external object. The mandatory `linkType` property defines the type of the link (e.g. `Multihash`). |
            "#
        )
    )?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn as_json_type(typ: &model::Type) -> String {
    match typ {
        model::Type::Boolean => format!("`boolean`"),
        model::Type::Int8
        | model::Type::Int16
        | model::Type::Int32
        | model::Type::Int64
        | model::Type::UInt8
        | model::Type::UInt16
        | model::Type::UInt32
        | model::Type::UInt64 => format!("`integer`"),
        model::Type::ByteSize
        | model::Type::DatasetAlias
        | model::Type::DatasetId
        | model::Type::DatasetRef
        | model::Type::DateTime
        | model::Type::Duration
        | model::Type::Flatbuffers
        | model::Type::Multicodec
        | model::Type::Multihash
        | model::Type::Path
        | model::Type::Regex
        | model::Type::Url
        | model::Type::AccountId
        | model::Type::AccountName
        | model::Type::ResourceId
        | model::Type::ResourceName
        | model::Type::ResourceTypeUri
        | model::Type::ResourceTypeName
        | model::Type::ResourceTypeRef
        | model::Type::String => format!("`string`"),
        model::Type::Generic(_) => format!("`object`"),
        model::Type::Array(t) => format!("array({})", as_json_type(&*t.item_type)),
        model::Type::Custom(t) => {
            format!("[{}](#{})", t.join("::"), schema_id(t.join("::").as_ref()))
        }
        model::Type::AnyJson => format!("`any`"),
    }
}

fn as_format(typ: &model::Type) -> String {
    match typ {
        model::Type::Boolean => String::new(),
        model::Type::Int8 => format!("`int8`"),
        model::Type::Int16 => format!("`int16`"),
        model::Type::Int32 => format!("`int32`"),
        model::Type::Int64 => format!("`int64`"),
        model::Type::UInt8 => format!("`uint8`"),
        model::Type::UInt16 => format!("`uint16`"),
        model::Type::UInt32 => format!("`uint32`"),
        model::Type::UInt64 => format!("`uint64`"),
        model::Type::String => String::new(),
        model::Type::DatasetAlias => format!("[dataset-alias](#dataset-identity)"),
        model::Type::DatasetId => format!("[dataset-id](#dataset-identity)"),
        model::Type::DatasetRef => format!("[dataset-ref](#dataset-identity)"),
        model::Type::ByteSize => "[byte-size](https://www.thierry-lequeu.fr/data/PELS/Comm/Publications/Newsletter/9704/STORY18.HTML)".to_string(),
        model::Type::DateTime => "[date-time](https://json-schema.org/draft/2019-09/json-schema-validation.html#rfc.section.7.3.1)".to_string(),
        model::Type::Duration => "[duration](https://docs.rs/duration-string/latest/duration_string/)".to_string(),
        model::Type::Flatbuffers => format!("`flatbuffers`"),
        model::Type::Multicodec => "[multicodec](https://github.com/multiformats/multicodec)".to_string(),
        model::Type::Multihash => "[multihash](https://github.com/multiformats/multihash)".to_string(),
        model::Type::Path => format!("`path`"),
        model::Type::Regex => format!("`regex`"),
        model::Type::Url => format!("`url`"),
        model::Type::Generic(_) => format!("`generic`"),
        model::Type::Array(_) => String::new(),
        model::Type::Custom(_) => String::new(),
        model::Type::AnyJson => String::new(),
        // TODO: Link to the spec section
        model::Type::AccountId => String::new(),
        model::Type::AccountName => String::new(),
        model::Type::ResourceId => String::new(),
        model::Type::ResourceName => String::new(),
        model::Type::ResourceTypeUri => String::new(),
        model::Type::ResourceTypeName => String::new(),
        model::Type::ResourceTypeRef => String::new(),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
