use std::collections::BTreeMap;

use crate::model::{self, Type, TypeDefinition, TypeId};

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

const EXCLUDED_CONTEXTS: &[&str] = &["metaschemas"];

// Unions whose variants are not expanded into separate nodes
const COLLAPSED_UNIONS: &[&str] = &["DataType", "ReadStep"];

// One fill color per context (background, keeping text black)
const CONTEXT_COLORS: &[(&str, &str)] = &[
    ("auth", "#f0e0ff"),
    ("config", "#fff0b3"),
    ("data", "#d0f0d0"),
    ("dataset", "#cce5ff"),
    ("engine", "#ffe0cc"),
    ("event", "#e0f7fa"),
    ("flow", "#ffd6e0"),
    ("legacy", "#e0e0e0"),
    ("resource", "#f5f5f5"),
    ("sink", "#ffe8cc"),
    ("source", "#d6f0e8"),
    ("storage", "#e8d6f0"),
];

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn render(model: model::Model, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let types: Vec<&TypeDefinition> = model
        .types
        .values()
        .filter(|t| !EXCLUDED_CONTEXTS.contains(&t.id().context()))
        .filter(|t| {
            // Exclude variant subtypes of collapsed unions
            t.id()
                .parent()
                .map(|p| !COLLAPSED_UNIONS.contains(&p.name()))
                .unwrap_or(true)
        })
        .collect();

    writeln!(w, "%%{{init: {{\"flowchart\": {{\"defaultRenderer\": \"elk\"}}}} }}%%")?;
    writeln!(w, "graph TD")?;
    writeln!(w)?;

    // classDef per context
    for (context, color) in CONTEXT_COLORS {
        writeln!(w, "  classDef ctx_{context} fill:{color},stroke:#999,color:#000")?;
    }
    writeln!(w)?;

    // Nodes grouped by context in subgraph boxes
    let mut by_context: BTreeMap<&str, Vec<&TypeId>> = BTreeMap::new();
    for t in &types {
        by_context.entry(t.id().context()).or_default().push(t.id());
    }

    for (context, ids) in &by_context {
        writeln!(w, "  subgraph {context}[\"{context}\"]")?;
        let mut node_ids = Vec::new();
        for id in ids {
            let node_id = id.join("_");
            let label = id.join("::");
            if node_id == label {
                writeln!(w, "    {node_id}")?;
            } else {
                writeln!(w, "    {node_id}[\"{label}\"]")?;
            }
            node_ids.push(node_id.into_owned());
        }
        writeln!(w, "  end")?;
        writeln!(w, "  class {} ctx_{context}", node_ids.join(","))?;
        writeln!(w)?;
    }

    // Edges: one per Custom-typed field / array item
    for t in &types {
        let from = t.id().join("_");
        match t {
            TypeDefinition::Struct(s) => {
                for (fname, field) in &s.fields {
                    emit_type_edges(&from, fname, &field.typ, w)?;
                }
            }
            TypeDefinition::Union(u) => {
                if !COLLAPSED_UNIONS.contains(&u.id.name()) {
                    for variant_id in &u.variants {
                        if !EXCLUDED_CONTEXTS.contains(&variant_id.context()) {
                            writeln!(w, "  {from} -->|\"variant\"| {}", variant_id.join("_"))?;
                        }
                    }
                }
            }
            TypeDefinition::Map(m) => {
                if let Type::Custom(ref_id) = &m.value_type {
                    if !EXCLUDED_CONTEXTS.contains(&ref_id.context()) {
                        writeln!(w, "  {from} -->|\"values\"| {}", ref_id.join("_"))?;
                    }
                }
            }
            TypeDefinition::Enum(_) => {}
        }
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn emit_type_edges(
    from: &str,
    field_name: &str,
    typ: &Type,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    match typ {
        Type::Custom(ref_id) => {
            if !EXCLUDED_CONTEXTS.contains(&ref_id.context()) {
                writeln!(w, "  {from} -->|\"{field_name}\"| {}", ref_id.join("_"))?;
            }
        }
        Type::Array(arr) => {
            emit_type_edges(from, field_name, &arr.item_type, w)?;
        }
        _ => {}
    }
    Ok(())
}
