use std::path::Path;

use crate::model;
use convert_case::{Case, Casing};

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn render(model: model::Model, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let lvl = 4;
    render_toc(&model, w)?;

    writeln!(w)?;

    render_section(&model, "Manifests", "root", vec!["Manifest"], lvl, w)?;
    render_section(
        &model,
        "Metadata Events",
        "metadata-events",
        vec!["MetadataEvent"],lvl, 
        w,
    )?;
    render_section(&model, "Engine Protocol", "engine-ops", vec![], lvl, w)?;
    render_section(&model, "Fragments", "fragments", vec![], lvl, w)?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn section_id(name: &str) -> String {
    format!("reference-{}", name.to_lowercase().replace(" ", "-"))
}

fn schema_id(name: &str) -> String {
    format!("{}-schema", name.to_lowercase().replace("::", "-"))
}

fn path_to_kind(path: &Path) -> &str {
    match path
        .parent()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
    {
        "schemas" => "root",
        s => s,
    }
}

fn types_by_kind<'a>(
    model: &'a model::Model,
    kind: &str,
    priority: Vec<&'static str>,
) -> Vec<&'a model::TypeDefinition> {
    let mut types: Vec<_> = model
        .types
        .values()
        .filter(|t| path_to_kind(t.src()) == kind)
        .collect();

    types.sort_by_key(|t| {
        if priority.contains(&t.id().name.as_str()) {
            0
        } else {
            1
        }
    });

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
    render_toc_section("Manifests", "root", vec!["Manifest"], model, w)?;
    render_toc_section(
        "Metadata Events",
        "metadata-events",
        vec!["MetadataEvent"],
        model,
        w,
    )?;
    render_toc_section("Engine Protocol", "engine-ops", vec![], model, w)?;
    render_toc_section("Fragments", "fragments", vec![], model, w)?;
    Ok(())
}

fn render_toc_section(
    name: &str,
    kind: &str,
    priority: Vec<&'static str>,
    model: &model::Model,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    let id = section_id(name);
    writeln!(w, "- [{name}](#{id})")?;
    for typ in types_by_kind(model, kind, priority) {
        if typ.id().namespace.is_some() {
            continue;
        }
        let name = &typ.id().name;
        let id = schema_id(&name);
        writeln!(w, "  - [{name}](#{id})")?;
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_section(
    model: &model::Model,
    name: &str,
    kind: &str,
    priority: Vec<&'static str>,
    lvl: usize,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    render_header(name, None, lvl, w)?;
    for typ in types_by_kind(model, kind, priority) {
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
    }
    Ok(())
}

fn render_schema_links(path: &Path, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    writeln!(w, 
        "[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)]({})", path.display())?;
        writeln!(w, 
        "[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](schemas-generated/flatbuffers/opendatafabric.fbs)")?;
        writeln!(w, 
        "[^](#reference-information)")?;
        Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_struct(
    typ: &model::Struct,
    lvl: usize,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    let name = typ.id.join("::");
    render_header(name.as_str(), Some(schema_id(name.as_str())), lvl, w)?;
    writeln!(w, "{}", typ.description)?;
    writeln!(w)?;
    
    render_table(
        vec!["Property", "Type", "Required", "Format", "Description"],
        vec![":---:", ":---:", ":---:", ":---:", "---"],
        typ.fields
            .values()
            .map(|f| {
                vec![
                    format!("`{}`", f.name.to_case(Case::Camel)),
                    as_json_type(&f.typ),
                    if f.optional {
                        String::new()
                    } else {
                        "V".to_string()
                    },
                    as_format(&f.typ),
                    f.description.clone(),
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

fn render_union(
    typ: &model::Union,
    lvl: usize,
    model: &model::Model,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    let name = typ.id.join("::");
    render_header(name.as_str(), Some(schema_id(name.as_str())), lvl, w)?;
    writeln!(w, "{}", typ.description)?;
    writeln!(w)?;
    
    render_table(
        vec!["Union Type", "Description"],
        vec![":---:", "---"],
        typ.variants.iter()
        .map(|v| model.types.get(v).unwrap())
            .map(|t| {
                vec![
                    format!("[{}](#{})", t.id().join("::"), schema_id(t.id().join("::").as_str())),
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
    render_header(name.as_str(), Some(schema_id(name.as_str())), lvl, w)?;
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
fn as_json_type(typ: &model::Type) -> String {
    match typ {
        model::Type::Boolean => format!("`boolean`"),
        model::Type::Int16 |
        model::Type::Int32 |
        model::Type::Int64 |
        model::Type::UInt16 |
        model::Type::UInt32 |
        model::Type::UInt64 => format!("`integer`"),
        model::Type::DatasetAlias |
        model::Type::DatasetId |
        model::Type::DatasetRef |
        model::Type::DateTime |
        model::Type::Flatbuffers |
        model::Type::Multicodec |
        model::Type::Multihash |
        model::Type::Path |
        model::Type::Regex |
        model::Type::Url |
        model::Type::String => format!("`string`"),
        model::Type::Array(t) => format!("array({})", as_json_type(&*t.item_type)),
        model::Type::Custom(t) => format!("[{}](#{})", t.join("::"), schema_id(t.join("::").as_str())),
    }
}


fn as_format(typ: &model::Type) -> String {
    match typ {
        model::Type::Boolean => String::new(),
        model::Type::Int16 => format!("`int16`"),
        model::Type::Int32 => format!("`int32`"),
        model::Type::Int64 => format!("`int64`"),
        model::Type::UInt16 => format!("`uint16`"),
        model::Type::UInt32 => format!("`uint32`"),
        model::Type::UInt64 => format!("`uint64`"),
        model::Type::String => String::new(),
        model::Type::DatasetAlias => format!("[dataset-alias](#dataset-identity)"),
        model::Type::DatasetId => format!("[dataset-id](#dataset-identity)"),
        model::Type::DatasetRef => format!("[dataset-ref](#dataset-identity)"),
        model::Type::DateTime => "[date-time](https://json-schema.org/draft/2019-09/json-schema-validation.html#rfc.section.7.3.1)".to_string(),
        model::Type::Flatbuffers => format!("`flatbuffers`"),
        model::Type::Multicodec => "[multicodec](https://github.com/multiformats/multicodec)".to_string(),
        model::Type::Multihash => "[multihash](https://github.com/multiformats/multihash)".to_string(),
        model::Type::Path => format!("`path`"),
        model::Type::Regex => format!("`regex`"),
        model::Type::Url => format!("`url`"),
        model::Type::Array(_) => String::new(),
        model::Type::Custom(_) => String::new(),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
