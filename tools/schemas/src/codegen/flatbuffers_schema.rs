use indexmap::IndexMap;

use crate::model;
use crate::utils::indent_writer::IndentWriter;
use std::collections::HashSet;
use std::io::Write;
use std::path::PathBuf;

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
        if !wrappers.contains(typ.id()) && !roots.contains(typ.id()) {
            writeln!(
                w,
                "////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////"
            )?;
            writeln!(w, "// {}", typ.id().join(""))?;
            render_description(typ.description(), None, None, w)?;
            writeln!(w, "//")?;
            writeln!(
                w,
                "// See: {SPEC_URL}#{}-schema",
                typ.id().join("").to_lowercase()
            )?;
            writeln!(
                w,
                "////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////"
            )?;
            writeln!(w, "")?;
        }

        match &typ {
            model::TypeDefinition::Struct(t) => render_struct(t, &model, w)?,
            model::TypeDefinition::Union(t) => render_union(t, w)?,
            model::TypeDefinition::Enum(t) => render_enum(t, w)?,
            model::TypeDefinition::Extensions(t) => render_extensions(t, w)?,
        }
        writeln!(w, "")?;
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: Can't find a related issue, is this a general Fb limitation or just of rust codegen?
/// Flatbuffers cannot directly store union types in arrays. To solve this we introduce special wrapper table types.
fn wrap_union_arrays(model: model::Model) -> (model::Model, Vec<model::TypeId>) {
    let mut new_model = model.clone();
    let mut wrappers = Vec::new();

    for typ in model.types.values() {
        let model::TypeDefinition::Struct(obj) = typ else {
            continue;
        };

        for field in obj.fields.values() {
            let model::Type::Array(arr) = &field.typ else {
                continue;
            };

            let model::Type::Custom(item_type_name) = &*arr.item_type else {
                continue;
            };

            let item_type = model.types.get(&item_type_name).unwrap();
            if let model::TypeDefinition::Union(_) = item_type {
                // Create a wrapper type
                let wrapper_type_id = model::TypeId {
                    parent: None,
                    name: format!("{}Wrapper", item_type.id().join("")),
                };
                let wrapper_type = model::TypeDefinition::Struct(model::Struct {
                    id: wrapper_type_id.clone(),
                    fields: IndexMap::from([(
                        "value".to_string(),
                        model::Field {
                            name: "value".to_string(),
                            typ: model::Type::Custom(item_type_name.clone()),
                            validations: Vec::new(),
                            optional: false,
                            description: String::new(),
                            explicit_tag: None,
                            default: None,
                            examples: None,
                            deprecated: false,
                            codegen_hints: Default::default(),
                        },
                    )]),
                    description: String::new(),
                    src: PathBuf::new(),
                });

                new_model
                    .types
                    .insert(wrapper_type_id.clone(), wrapper_type);

                // Patch array type
                let model::TypeDefinition::Struct(new_struct) =
                    new_model.types.get_mut(typ.id()).unwrap()
                else {
                    unreachable!();
                };

                let new_field = new_struct.fields.get_mut(&field.name).unwrap();

                let model::Type::Array(new_array) = &mut new_field.typ else {
                    unreachable!();
                };

                new_array.item_type = Box::new(model::Type::Custom(wrapper_type_id.clone()));

                wrappers.push(wrapper_type_id);
            }
        }
    }

    (new_model, wrappers)
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: Can't find a related issue, is this a general Fb limitation or just of rust codegen?
/// Flatbuffer unions are hard to work with as top-level types, so for root union we generate special wrapper table types.
fn wrap_root_unions_with_tables(mut model: model::Model) -> (model::Model, HashSet<model::TypeId>) {
    let mut root_unions: HashSet<_> = model
        .types
        .values()
        .filter_map(|t| match t {
            model::TypeDefinition::Union(t) => Some(t.id.clone()),
            _ => None,
        })
        .collect();

    for typ in model.types.values() {
        let model::TypeDefinition::Struct(obj) = typ else {
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
        let wrapper_type = model::TypeDefinition::Struct(model::Struct {
            id: model::TypeId {
                parent: None,
                name: format!("{}Root", root.name),
            },
            fields: IndexMap::from([(
                "value".to_string(),
                model::Field {
                    name: "value".to_string(),
                    typ: model::Type::Custom(root.clone()),
                    validations: Vec::new(),
                    optional: false,
                    description: String::new(),
                    explicit_tag: None,
                    default: None,
                    examples: None,
                    deprecated: false,
                    codegen_hints: Default::default(),
                },
            )]),
            description: String::new(),
            src: PathBuf::new(),
        });

        model.types.insert(wrapper_type.id().clone(), wrapper_type);
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
    visited: &mut HashSet<model::TypeId>,
    res: &mut Vec<model::TypeDefinition>,
) {
    if visited.contains(typ.id()) {
        return;
    }

    visited.insert(typ.id().clone());

    match typ {
        model::TypeDefinition::Struct(t) => {
            for field in t.fields.values() {
                in_dependency_order_rec_t(&field.typ, model, visited, res)
            }

            // Order struct field types before the struct
            res.push(typ.clone());
        }
        model::TypeDefinition::Union(type_union) => {
            // Order union before its variants, because flatc is a poorly maintained turd
            // See: https://github.com/google/flatbuffers/issues/4725
            res.push(typ.clone());

            for variant in &type_union.variants {
                in_dependency_order_rec_t(
                    &model::Type::Custom(variant.clone()),
                    model,
                    visited,
                    res,
                )
            }
        }
        model::TypeDefinition::Enum(_) | model::TypeDefinition::Extensions(_) => {
            res.push(typ.clone());
        }
    }
}

fn in_dependency_order_rec_t(
    typ: &model::Type,
    model: &model::Model,
    visited: &mut HashSet<model::TypeId>,
    res: &mut Vec<model::TypeDefinition>,
) {
    match typ {
        model::Type::Custom(id) => {
            let typ = model
                .types
                .get(&id)
                .expect(&format!("Expected to find type {id:?}"));

            in_dependency_order_rec(&typ, model, visited, res);
        }
        model::Type::Array(t) => {
            in_dependency_order_rec_t(&t.item_type, model, visited, res);
        }
        _ => (),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_struct(
    typ: &model::Struct,
    model: &model::Model,
    w: &mut IndentWriter<&mut dyn std::io::Write>,
) -> Result<(), std::io::Error> {
    let fields_with_tags = allocate_struct_field_ids(typ, model);

    writeln!(w, "table {} {{", typ.id.join(""))?;
    {
        let mut i = w.indent();
        for field in fields_with_tags {
            let id = field.id;
            let deprecated = field.deprecated;
            let field = field.field;

            render_description(
                &field.description,
                field.default.as_ref(),
                field.examples.as_ref(),
                &mut i,
            )?;

            let optionality_modifier = match (field.optional, &field.typ) {
                (false, _) => "",
                (
                    true,
                    model::Type::Boolean
                    | model::Type::Int8
                    | model::Type::Int16
                    | model::Type::Int32
                    | model::Type::Int64
                    | model::Type::UInt8
                    | model::Type::UInt16
                    | model::Type::UInt32
                    | model::Type::UInt64,
                ) => " = null",
                (true, model::Type::Custom(name)) => match model.types.get(&name).unwrap() {
                    model::TypeDefinition::Enum(_) => " = null",
                    _ => "",
                },
                _ => "",
            };

            let mut attributes = Vec::new();
            if let Some(tag) = id {
                let hint = match &field.typ {
                    model::Type::Custom(fid) => match &model.types[fid] {
                        model::TypeDefinition::Union(_) => " /* union takes 2 slots */",
                        _ => "",
                    },
                    _ => "",
                };
                attributes.push(format!("id: {tag}{hint}"));
            }
            if deprecated {
                attributes.push("deprecated".to_string());
            };
            let attributes = if attributes.is_empty() {
                String::new()
            } else {
                format!(" ({})", attributes.join(", "))
            };

            writeln!(
                i,
                "{}: {}{}{};",
                field.name,
                format_type(&field.typ),
                optionality_modifier,
                attributes,
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
    writeln!(w, "union {} {{", typ.id.join(""))?;
    {
        let mut i = w.indent();
        for variant in &typ.variants {
            writeln!(i, "{},", variant.join(""))?;
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
    writeln!(
        w,
        "enum {}: {} {{",
        typ.id.join(""),
        format_type(&typ.format)
    )?;
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

fn render_extensions(
    typ: &model::Extensions,
    w: &mut IndentWriter<&mut dyn std::io::Write>,
) -> Result<(), std::io::Error> {
    writeln!(w, "table {} {{", typ.id.join(""))?;
    {
        let mut i = w.indent();
        writeln!(i, "// JSON encoded")?;
        writeln!(i, "attributes: string;")?;
    }
    writeln!(w, "}}")?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn format_type(typ: &model::Type) -> String {
    match typ {
        model::Type::Boolean => format!("bool"),
        model::Type::Int8 => format!("byte"),
        model::Type::Int16 => format!("int16"),
        model::Type::Int32 => format!("int32"),
        model::Type::Int64 => format!("int64"),
        model::Type::UInt8 => format!("ubyte"),
        model::Type::UInt16 => format!("uint16"),
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
    if !desc.is_empty() {
        for line in desc.split('\n') {
            writeln!(w, "// {line}")?;
        }
    }
    if let Some(default) = default {
        writeln!(w, "//")?;
        writeln!(w, "// Defaults to: {default}")?;
    }
    if let Some(examples) = examples {
        writeln!(w, "//")?;
        writeln!(w, "// Examples:")?;
        for ex in examples {
            writeln!(w, "// - {ex}")?;
        }
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct FieldWithId {
    field: model::Field,
    id: Option<u32>,

    // Note that this field is different from `Filed::deprecated`.
    // In flatbuffers deprecation means that field accessors will
    // not be generated, while in ODF we continue to generate accessors
    // to allow applications to migrate.
    deprecated: bool,
}

/// This process decides what tag should each schema field have. There are a few caveats:
/// - Union type fields take up two tags, one for type another for value (see https://flatbuffers.dev/schema/#attributes).
/// - All tags must be sequential and start with 0. In light of this we have to produce dummy fields in cases when explicit tag is used with a gap.
fn allocate_struct_field_ids(typ: &model::Struct, model: &model::Model) -> Vec<FieldWithId> {
    // No explicit tags - leave ids empty
    if !typ.fields.values().any(|f| f.explicit_tag.is_some()) {
        return typ
            .fields
            .values()
            .map(|f| FieldWithId {
                field: f.clone(),
                id: None,
                deprecated: false,
            })
            .collect();
    }

    let mut fields_with_ids = Vec::new();
    let mut maybe_prev_id = None;
    let mut maybe_prev_tag = None;

    for f in typ.fields.values() {
        let actual_tag = f.explicit_tag.unwrap();
        let tag_delta = actual_tag - maybe_prev_tag.unwrap_or(0);

        // Unions add implicit field and must specify the tag of the second one
        let slots_per_field = match &f.typ {
            model::Type::Custom(fid) => match &model.types[fid] {
                model::TypeDefinition::Union(_) => 2,
                _ => 1,
            },
            _ => 1,
        };

        let next_id = maybe_prev_id.map(|id| id + slots_per_field).unwrap_or(0);
        let actual_id = maybe_prev_id.unwrap_or(0) + slots_per_field - 1 + tag_delta;

        // If there is a gap - we have to pad it with dummy fields
        for dummy_id in next_id..actual_id {
            let dummy_field = model::Field {
                name: format!("dummy_{dummy_id}"),
                typ: model::Type::UInt8,
                validations: Vec::new(),
                optional: false,
                description: if dummy_id == next_id {
                    format!(
                        "The following field(s) reserve IDs [{}..{}] for schema evolution.",
                        next_id,
                        actual_id - 1
                    )
                } else {
                    String::new()
                },
                default: None,
                examples: None,
                explicit_tag: None,
                // Marking dummy fields as deprecated ensures they cannot be assigned or read by accident
                deprecated: true,
                codegen_hints: Default::default(),
            };

            fields_with_ids.push(FieldWithId {
                field: dummy_field,
                id: Some(dummy_id),
                deprecated: true,
            });
        }

        fields_with_ids.push(FieldWithId {
            field: f.clone(),
            id: Some(actual_id),
            deprecated: false,
        });

        maybe_prev_tag = Some(actual_tag);
        maybe_prev_id = Some(actual_id);
    }

    fields_with_ids
}
