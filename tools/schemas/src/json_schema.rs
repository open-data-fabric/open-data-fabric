use indexmap::IndexMap;
use serde_with::skip_serializing_none;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    #[serde(rename = "$id")]
    pub id: Option<String>,

    #[serde(rename = "$schema")]
    pub schema: Option<String>,

    #[serde(rename = "$defs")]
    pub defs: Option<IndexMap<String, Schema>>,

    pub r#type: Option<String>,

    pub required: Option<Vec<String>>,

    pub properties: Option<IndexMap<String, Schema>>,

    pub pattern_properties: Option<IndexMap<String, Schema>>,

    pub additional_properties: Option<bool>,

    pub one_of: Option<Vec<Schema>>,

    pub r#enum: Option<Vec<serde_json::Value>>,

    pub items: Option<Box<Schema>>,

    #[serde(rename = "$ref")]
    pub r#ref: Option<String>,

    // ODF Extensions
    pub format: Option<String>,

    pub default: Option<serde_json::Value>,

    pub description: Option<String>,

    /// Serialization tag used to preserve binary compatiblity when evolving the schemas
    pub tag: Option<u32>,

    /// Code generation hints per language
    pub codegen: Option<IndexMap<String, IndexMap<String, String>>>,

    /// Marks schema as deprecated
    pub deprecated: Option<bool>,

    pub examples: Option<Vec<serde_json::Value>>,

    pub src: Option<PathBuf>,
}

impl Schema {
    pub fn display(&self) -> SchemaDisplay<'_> {
        SchemaDisplay(self)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct SchemaDisplay<'a>(&'a Schema);

impl<'a> std::fmt::Display for SchemaDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_string_pretty(self.0).unwrap();
        write!(f, "{}", s)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn load_schemas(schemas_dir: &Path) -> Vec<Schema> {
    let mut schemas = Vec::new();

    for entry in glob::glob(&format!("{}/**/*", schemas_dir.display())).unwrap() {
        let path = entry.unwrap();

        if path.is_dir() {
            continue;
        }

        let mut schema: Schema = serde_json::from_reader(std::fs::File::open(&path).unwrap())
            .expect(&format!("Error while parsing schema in {}", path.display()));

        // Check that all schemas have IDs that match their file names
        let id = schema.id.as_ref().expect(&format!(
            "Top-level schema in {} does not specify an $id",
            path.display()
        ));

        schema.src = Some(path.clone());

        assert_eq!(
            *id,
            format!(
                "http://open-data-fabric.github.com/schemas/{}",
                path.file_stem().unwrap().to_str().unwrap()
            ),
            "Schema ID does not correspond to the file name"
        );

        schemas.push(schema);
    }

    schemas
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn check_referential_integrity(top_level_schemas: &[Schema], roots: &[String]) {
    let mut schemas = HashMap::new();

    // Add top-level schemas
    for s in top_level_schemas {
        let id = crate::model::schema_id_to_type_id(s.id.as_deref().unwrap_or_default());
        schemas.insert(id, s);
    }

    // Add all defs
    for s in top_level_schemas {
        let id = crate::model::schema_id_to_type_id(s.id.as_deref().unwrap_or_default());

        if let Some(defs) = &s.defs {
            for (name, ds) in defs {
                let did = id.subtype(name);
                schemas.insert(did, ds);
            }
        }
    }

    let mut to_explore = Vec::new();
    let mut explored = HashSet::new();

    // Seed `to_explore` with provided known roots
    for root in roots {
        let id = crate::model::TypeId::new_root(root);
        let schema = schemas
            .get(&id)
            .expect(&format!("Could not find specified root schema: {}", root));

        to_explore.push((id, *schema));
    }

    // Start exploring the reference graph
    while !to_explore.is_empty() {
        let (id, schema) = to_explore.pop().unwrap();

        explored.insert(id.clone());

        for reff in extract_refs(schema) {
            let ref_id = match reff {
                Ref::Global {
                    schema_id,
                    subschema_def: None,
                } => crate::model::TypeId {
                    parent: None,
                    name: schema_id,
                },
                Ref::Global {
                    schema_id,
                    subschema_def: Some(subschema_def),
                } => crate::model::TypeId {
                    parent: Some(Box::new(crate::model::TypeId::new_root(schema_id))),
                    name: subschema_def,
                },
                Ref::Def(name) => id.root().subtype(name),
            };

            if !explored.contains(&ref_id) {
                let schema = schemas.get(&ref_id).expect(&format!(
                    "Schema {} referenced by {} not found",
                    ref_id.join("::"),
                    id.join("::")
                ));

                to_explore.push((ref_id, schema));
            }
        }
    }

    let unused_schemas: Vec<_> = schemas
        .keys()
        .filter(|name| !explored.contains(*name))
        .map(|id| id.join("::").to_string())
        .collect();

    if !unused_schemas.is_empty() {
        panic!(
            "Some schemas are never used:\n  {}",
            unused_schemas.join("\n  ")
        );
    }
}

fn extract_refs(schema: &Schema) -> Vec<Ref> {
    let mut refs = Vec::new();

    if let Some(properties) = &schema.properties {
        for prop in properties.values() {
            refs.extend(extract_refs(prop));
        }
    }

    if let Some(pattern_properties) = &schema.pattern_properties {
        for prop in pattern_properties.values() {
            refs.extend(extract_refs(prop));
        }
    }

    if let Some(reff) = &schema.r#ref {
        refs.push(decode_ref(&reff));
    }

    if let Some(one_of) = &schema.one_of {
        for variant_schema in one_of {
            refs.extend(extract_refs(variant_schema));
        }
    }

    if let Some(item_schema) = &schema.items {
        refs.extend(extract_refs(item_schema));
    }

    if let Some(defs) = &schema.defs {
        for def in defs.values() {
            refs.extend(extract_refs(def));
        }
    }

    refs
}

fn decode_ref(reff: &str) -> Ref {
    if let Some(global) = reff.strip_prefix("/schemas/") {
        if let Some((global, local)) = global.split_once("#/$defs/") {
            Ref::Global {
                schema_id: global.to_string(),
                subschema_def: Some(local.to_string()),
            }
        } else {
            Ref::Global {
                schema_id: global.to_string(),
                subschema_def: None,
            }
        }
    } else if let Some(local) = reff.strip_prefix("#/$defs/") {
        Ref::Def(local.to_string())
    } else {
        panic!("Invalid reference {reff}");
    }
}

enum Ref {
    Global {
        schema_id: String,
        subschema_def: Option<String>,
    },
    Def(String),
}
