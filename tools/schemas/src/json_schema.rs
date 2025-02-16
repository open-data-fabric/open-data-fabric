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

    pub examples: Option<Vec<serde_json::Value>>,

    pub src: Option<PathBuf>,
}

impl Schema {
    pub fn display(&self) -> SchemaDisplay {
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

pub fn check_referential_integrity(schemas: &[Schema], roots: &[String]) {
    let schemas: HashMap<String, &Schema> = schemas
        .iter()
        .map(|s| {
            (
                s.id.as_ref()
                    .unwrap()
                    .rsplit_once('/')
                    .unwrap()
                    .1
                    .to_string(),
                s,
            )
        })
        .collect();

    let mut unexplored = Vec::new();
    let mut explored = HashSet::new();

    for root in roots {
        unexplored.push(
            *schemas
                .get(root)
                .expect(&format!("Could not find root schema {}", root)),
        );
    }

    while !unexplored.is_empty() {
        let schema = unexplored.pop().unwrap();
        explored.insert(
            schema
                .id
                .as_ref()
                .unwrap()
                .rsplit_once('/')
                .unwrap()
                .1
                .to_string(),
        );

        for reff in extract_refs(schema) {
            match reff {
                Ref::Global(name) => {
                    if !explored.contains(&name) {
                        unexplored.push(
                            *schemas
                                .get(&name)
                                .expect(&format!("Refereced schema {name} not found")),
                        );
                    }
                }
                Ref::Def(_name) => {
                    // TODO: check all definitions are used
                }
            }
        }
    }

    for name in schemas.keys() {
        if !explored.contains(name) {
            panic!("Schema {} is never used", name);
        }
    }
}

fn extract_refs(schema: &Schema) -> Vec<Ref> {
    let mut refs = Vec::new();

    if let Some(properties) = &schema.properties {
        for prop in properties.values() {
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
        Ref::Global(global.to_string())
    } else if let Some(local) = reff.strip_prefix("#/$defs/") {
        Ref::Def(local.to_string())
    } else {
        panic!("Invalid reference {reff}");
    }
}

enum Ref {
    Global(String),
    Def(String),
}
