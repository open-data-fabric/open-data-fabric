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

    pub r#type: Option<Type>,

    pub required: Option<Vec<String>>,

    pub properties: Option<IndexMap<String, Schema>>,

    pub pattern_properties: Option<IndexMap<String, Schema>>,

    pub additional_properties: Option<bool>,

    pub unevaluated_properties: Option<bool>,

    pub one_of: Option<Vec<Schema>>,

    pub all_of: Option<Vec<Schema>>,

    pub r#enum: Option<Vec<serde_json::Value>>,

    pub items: Option<Box<Schema>>,

    #[serde(rename = "$ref")]
    pub r#ref: Option<String>,

    pub r#const: Option<String>,

    // ODF Extensions
    pub format: Option<Format>,

    /// Specifies the default value that all implementations must fall back onto if the property is not defined.
    /// Codegen will still output the field as optional because we want to round-trip serialization to output same data as inputed.
    pub default: Option<serde_json::Value>,

    pub description: Option<String>,

    /// Serialization tag used to preserve binary compatiblity when evolving the schemas
    pub tag: Option<u32>,

    /// Code generation hints per language
    pub codegen: Option<IndexMap<CodegenLanguage, IndexMap<CodegenHint, String>>>,

    /// Marks schema as deprecated
    pub deprecated: Option<bool>,

    pub examples: Option<Vec<serde_json::Value>>,

    pub src: Option<PathBuf>,
}

impl Schema {
    pub fn display(&self) -> SchemaDisplay<'_> {
        SchemaDisplay(self)
    }

    pub fn to_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
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
            format!("https://opendatafabric.org/{}", path.display()),
            "Schema ID does not correspond to the file name"
        );

        schemas.push(schema);
    }

    schemas
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn check_referential_integrity(top_level_schemas: &[Schema]) {
    let mut schemas = HashMap::new();

    for s in top_level_schemas {
        let id = crate::model::schema_id_to_type_id(s.id.as_deref().unwrap_or_default());

        // Add all defs
        if let Some(defs) = &s.defs {
            for (name, ds) in defs {
                let did = id.subtype(name);
                schemas.insert(did, ds);
            }
        }

        // Add top-level schema
        schemas.insert(id, s);
    }

    let mut to_explore = Vec::new();
    let mut explored = HashSet::new();

    // Seed `to_explore` with known roots
    for (id, sch) in schemas.iter().filter(|(id, sch)| {
        matches!(
            sch.format,
            Some(Format::Resource) | Some(Format::RpcMessage)
        ) || id.name == "Manifest"
            || id.name == "DatasetSnapshot"
            || id.name == "MetadataBlock"
    }) {
        to_explore.push((id.clone(), *sch));
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

    if let Some(all_of) = &schema.all_of {
        for variant_schema in all_of {
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

// Matches: https://opendatafabric.org/schemas/{context}/{version}/{Name}.json
// Also matches with an optional #/$defs/{Def} fragment.
static SCHEMA_URL_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(
        r"^https://opendatafabric\.org/schemas/(?P<context>[^/]+)/(?P<version>[^/]+)/(?P<name>[^/.]+)\.json(?:#/\$defs/(?P<def>[^/]+))?$",
    )
    .unwrap()
});

fn decode_ref(reff: &str) -> Ref {
    if let Some(caps) = SCHEMA_URL_RE.captures(reff) {
        Ref::Global {
            schema_id: caps["name"].to_string(),
            subschema_def: caps.name("def").map(|c| c.as_str().to_string()),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Boolean,
    Integer,
    Number,
    String,
    Array,
    Object,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Format {
    // Core
    Email,
    Uri,

    // Scalars
    Int8,
    Int16,
    Int32,
    Int64,
    #[serde(rename = "uint8")]
    UInt8,
    #[serde(rename = "uint16")]
    UInt16,
    #[serde(rename = "uint32")]
    UInt32,
    #[serde(rename = "uint64")]
    UInt64,
    ByteSize,
    DateTime,
    Duration,
    Multicodec,
    Multihash,
    Path,
    Regex,

    // Identity and references
    AccountId,
    AccountName,

    DatasetId,
    DatasetName,
    DatasetAlias, // TODO: Should this be replaced by DatasetRef everywhere?
    DatasetRef,

    Resource,
    ResourceId,
    ResourceName,
    ResourceTypeName,
    ResourceTypeUri,
    ResourceTypeRef,

    RpcMessage,

    // Embedding
    Flatbuffers,
    Fragment,

    // Markers
    UnionOrString,
    StructOrString,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CodegenLanguage {
    Flatbuffers,
    Rust,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CodegenHint {
    Container,
    DtoType,
    MapFormat,
}
