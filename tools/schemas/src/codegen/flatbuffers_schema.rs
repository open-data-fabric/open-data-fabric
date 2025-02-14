use indexmap::IndexMap;

use crate::model::{self, TypeName};
use crate::utils::indent_writer::IndentWriter;
use std::collections::HashSet;
use std::io::Write;

const SPEC_URL: &str =
    "https://github.com/kamu-data/open-data-fabric/blob/master/open-data-fabric.md";

const PREAMBLE: &str = indoc::indoc!(
    r#"
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // WARNING: This file is auto-generated from Open Data Fabric Schemas
    // See: http://opendatafabric.org/
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    struct Timestamp {
      year: int32;
      ordinal: uint16;
      seconds_from_midnight: uint32;
      nanoseconds: uint32;
    }
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
    let (model, wrappers) = wrap_union_arrays(model);
    let (model, roots) = wrap_root_unions_with_tables(model);

    writeln!(w, "{}", PREAMBLE)?;

    for typ in in_dependency_order(&model) {
        if !wrappers.contains(typ.name()) && !roots.contains(typ.name()) {
            writeln!(w, "////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////")?;
            writeln!(w, "// {}", typ.name())?;
            writeln!(w, "// {SPEC_URL}#{}-schema", typ.name().0.to_lowercase())?;
            writeln!(w, "////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////")?;
            writeln!(w, "")?;
        }

        match &typ {
            model::TypeDefinition::Object(t) => render_object(t, &model, w)?,
            model::TypeDefinition::Union(t) => render_union(t, w)?,
            model::TypeDefinition::Enum(t) => render_enum(t, w)?,
        }
        writeln!(w, "")?;
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: Can't find a related issue, is this a general Fb limitation or just of rust codegen?
/// Flatbuffers cannot directly store union types in arrays. To solve this we introduce special wrapper table types.
fn wrap_union_arrays(model: model::Model) -> (model::Model, Vec<model::TypeName>) {
    let mut new_model = model.clone();
    let mut wrappers = Vec::new();

    for typ in model.types.values() {
        let model::TypeDefinition::Object(obj) = typ else {
            continue;
        };

        for field in obj.fields.values() {
            let model::Type::Array(arr) = &field.typ else {
                continue;
            };

            let model::Type::Custom(item_type_name) = &*arr.item_type else {
                continue;
            };

            let item_type = model.types.get(&item_type_name.0).unwrap();
            if let model::TypeDefinition::Union(_) = item_type {
                // Create a wrapper type
                let wrapper_type_name = model::TypeName(format!("{}Wrapper", item_type.name()));
                let wrapper_type = model::TypeDefinition::Object(model::Object {
                    name: wrapper_type_name.clone(),
                    fields: IndexMap::from([(
                        "value".to_string(),
                        model::Field {
                            name: "value".to_string(),
                            typ: model::Type::Custom(item_type_name.clone()),
                            optional: false,
                        },
                    )]),
                    description: String::new(),
                });

                new_model
                    .types
                    .insert(wrapper_type_name.0.clone(), wrapper_type);

                // Patch array type
                let model::TypeDefinition::Object(new_object) =
                    new_model.types.get_mut(&typ.name().0).unwrap()
                else {
                    unreachable!();
                };

                let new_field = new_object.fields.get_mut(&field.name).unwrap();

                let model::Type::Array(new_array) = &mut new_field.typ else {
                    unreachable!();
                };

                new_array.item_type = Box::new(model::Type::Custom(wrapper_type_name.clone()));

                wrappers.push(wrapper_type_name);
            }
        }
    }

    (new_model, wrappers)
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: Can't find a related issue, is this a general Fb limitation or just of rust codegen?
/// Flatbuffer unions are hard to work with as top-level types, so for root union we generate special wrapper table types.
fn wrap_root_unions_with_tables(mut model: model::Model) -> (model::Model, HashSet<TypeName>) {
    let mut root_unions: HashSet<_> = model
        .types
        .values()
        .filter_map(|t| match t {
            model::TypeDefinition::Union(t) => Some(t.name.clone()),
            _ => None,
        })
        .collect();

    for typ in model.types.values() {
        let model::TypeDefinition::Object(obj) = typ else {
            continue;
        };

        for field in obj.fields.values() {
            match &field.typ {
                model::Type::Array(array) => match &*array.item_type {
                    model::Type::Custom(type_name) => {
                        root_unions.remove(type_name);
                    }
                    _ => (),
                },
                model::Type::Custom(type_name) => {
                    root_unions.remove(type_name);
                }
                _ => (),
            };
        }
    }

    for root in &root_unions {
        let wrapper_type = model::TypeDefinition::Object(model::Object {
            name: model::TypeName(format!("{root}Root")),
            fields: IndexMap::from([(
                "value".to_string(),
                model::Field {
                    name: "value".to_string(),
                    typ: model::Type::Custom(root.clone()),
                    optional: false,
                },
            )]),
            description: String::new(),
        });

        model
            .types
            .insert(wrapper_type.name().0.clone(), wrapper_type);
    }

    (model, root_unions)
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Depth-first traversal of types ensures that single-pass flatbuffer compiler can resolve all types as they appear
fn in_dependency_order(model: &model::Model) -> Vec<model::TypeDefinition> {
    let mut res = Vec::new();
    let mut visited = HashSet::new();

    for name in model.types.keys() {
        let typ = model.types.get(name).unwrap();
        in_dependency_order_rec(typ, model, &mut visited, &mut res);
    }

    res
}

fn in_dependency_order_rec(
    typ: &model::TypeDefinition,
    model: &model::Model,
    visited: &mut HashSet<String>,
    res: &mut Vec<model::TypeDefinition>,
) {
    if visited.contains(&typ.name().0) {
        return;
    }

    visited.insert(typ.name().0.clone());

    match typ {
        model::TypeDefinition::Object(t) => {
            for field in t.fields.values() {
                in_dependency_order_rec_t(&field.typ, model, visited, res)
            }
        }
        model::TypeDefinition::Union(type_union) => {
            for variant in &type_union.variants {
                in_dependency_order_rec_t(
                    &model::Type::Custom(variant.clone()),
                    model,
                    visited,
                    res,
                )
            }
        }
        model::TypeDefinition::Enum(_) => (),
    }

    res.push(typ.clone());
}

fn in_dependency_order_rec_t(
    typ: &model::Type,
    model: &model::Model,
    visited: &mut HashSet<String>,
    res: &mut Vec<model::TypeDefinition>,
) {
    match typ {
        model::Type::Custom(name) => {
            let typ = model.types.get(&name.0).unwrap();
            in_dependency_order_rec(&typ, model, visited, res);
        }
        model::Type::Array(t) => {
            in_dependency_order_rec_t(&t.item_type, model, visited, res);
        }
        _ => (),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_object(
    typ: &model::Object,
    model: &model::Model,
    w: &mut IndentWriter<&mut dyn std::io::Write>,
) -> Result<(), std::io::Error> {
    writeln!(w, "table {} {{", typ.name)?;
    {
        let mut i = w.indent();
        for field in typ.fields.values() {
            let optionality_modifier = match (field.optional, &field.typ) {
                (false, _) => "",
                (
                    true,
                    model::Type::Boolean
                    | model::Type::UInt32
                    | model::Type::UInt64
                    | model::Type::Int32,
                ) => " = null",
                (true, model::Type::Custom(name)) => match model.types.get(&name.0).unwrap() {
                    model::TypeDefinition::Enum(_) => " = null",
                    _ => "",
                },
                _ => "",
            };

            writeln!(
                i,
                "{}: {}{};",
                field.name,
                format_type(&field.typ),
                optionality_modifier
            )?;
        }
    }
    writeln!(w, "}}")?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_union(
    typ: &model::Union,
    w: &mut IndentWriter<&mut dyn std::io::Write>,
) -> Result<(), std::io::Error> {
    writeln!(w, "union {} {{", typ.name)?;
    {
        let mut i = w.indent();
        for variant in &typ.variants {
            writeln!(i, "{},", variant.0)?;
        }
    }
    writeln!(w, "}}")?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_enum(
    typ: &model::Enum,
    w: &mut IndentWriter<&mut dyn std::io::Write>,
) -> Result<(), std::io::Error> {
    writeln!(w, "enum {}: int32 {{", typ.name)?;
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
        model::Type::Int32 => format!("int32"),
        model::Type::UInt32 => format!("uint32"),
        model::Type::UInt64 => format!("uint64"),
        model::Type::String => format!("string"),
        model::Type::DatasetAlias => format!("string"),
        model::Type::DatasetId => format!("[ubyte]"),
        model::Type::DatasetRef => format!("string"),
        model::Type::DateTime => format!("Timestamp"),
        model::Type::Flatbuffers => format!("[ubyte]"),
        // TODO: Should be uint64 - change in hte next breaking cycle
        model::Type::Multicodec => format!("int64"),
        model::Type::Multihash => format!("[ubyte]"),
        model::Type::Path => format!("string"),
        model::Type::Regex => format!("string"),
        model::Type::Url => format!("string"),
        model::Type::Array(t) => format!("[{}]", format_type(&t.item_type)),
        model::Type::Custom(name) => name.0.clone(),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
