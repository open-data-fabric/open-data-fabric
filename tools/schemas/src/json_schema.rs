use indexmap::IndexMap;
use serde_with::skip_serializing_none;
use std::{
    borrow::Cow,
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
    pub id: Option<SchemaId>,

    #[serde(rename = "$schema")]
    pub schema: Option<SchemaId>,

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

    pub r#const: Option<serde_json::Value>,

    // ODF Extensions ///////////////////////////////////////////////////////////////////////////////////
    //
    /// Links input type with its canonical form
    pub canonical_type: Option<SchemaId>,

    pub format: Option<Format>,

    /// Specifies the default value that all implementations must fall back onto if the property is not defined.
    /// Codegen will still output the field as optional because we want to round-trip serialization to output same data as inputed.
    pub default: Option<serde_json::Value>,

    pub description: Option<String>,

    /// Serialization tag used to preserve binary compatiblity when evolving the schemas
    pub tag: Option<u32>,

    /// Code generation hints per language
    pub codegen: Option<IndexMap<CodegenLanguage, IndexMap<CodegenHint, serde_json::Value>>>,

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
            format!("{id}.json"),
            format!("https://opendatafabric.org/{}", path.display()),
            "Schema ID does not correspond to the file name"
        );

        schemas.push(schema);
    }

    schemas
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn lint(top_level_schemas: &[Schema]) {
    let mut schemas = HashMap::new();

    // Add JSON Schema meta-schema
    let jsonschema = Schema {
        id: None,
        schema: None,
        defs: None,
        r#type: None,
        required: None,
        properties: None,
        pattern_properties: None,
        additional_properties: None,
        unevaluated_properties: None,
        one_of: None,
        all_of: None,
        r#enum: None,
        items: None,
        r#ref: None,
        r#const: None,
        canonical_type: None,
        format: None,
        default: None,
        description: None,
        tag: None,
        codegen: None,
        deprecated: None,
        examples: None,
        src: None,
    };
    schemas.insert(SchemaId::new(SchemaId::METASCHEMA_JSONSCHEMA), &jsonschema);

    for s in top_level_schemas {
        let id = s.id.clone().expect("Top level schema without $id");

        // Add all defs
        if let Some(defs) = &s.defs {
            for (name, ds) in defs {
                let def_id = id.subtype(name);
                schemas.insert(def_id, ds);
            }
        }

        // Add top-level schema
        schemas.insert(id, s);
    }

    // Check names are unique, as some codegens don't support context-level modularity yet
    let mut seen_names = HashMap::new();
    for id in schemas.keys() {
        if id.as_str() == SchemaId::METASCHEMA_JSONSCHEMA
            || id.as_str().starts_with(SchemaId::METASCHEMA_BASE_URL)
        {
            continue;
        }

        let name = if let Some(p) = id.parent() {
            format!("{}{}", p.name(), id.name())
        } else {
            id.name().to_string()
        };
        if let Some(prev) = seen_names.insert(name.clone(), id) {
            panic!(
                "Name {name} in schema {id} is already used by schema {prev}. We don't allow it as some codegens don't yet support context-level modularity",
            )
        }
    }

    let mut to_explore = Vec::new();
    let mut explored = HashSet::new();

    // Seed `to_explore` with known roots
    for (id, sch) in schemas.iter().filter(|(id, sch)| {
        id.as_str() != SchemaId::METASCHEMA_JSONSCHEMA
            && (id.as_str().starts_with(SchemaId::METASCHEMA_BASE_URL)
                || sch.schema.as_deref() == Some(SchemaId::METASCHEMA_MANIFEST)
                || sch.schema.as_deref() == Some(SchemaId::METASCHEMA_RESOURCE_INPUT)
                || sch.schema.as_deref() == Some(SchemaId::METASCHEMA_RESOURCE_CONDITION)
                || sch.schema.as_deref() == Some(SchemaId::METASCHEMA_ENGINE_MESSAGE)
                || id.name() == "Manifest"
                || id.name() == "DatasetSnapshot"
                || id.name() == "MetadataBlock"
                || id.name() == "OperationType")
    }) {
        to_explore.push((id.clone(), *sch));
    }

    // Start exploring the reference graph
    while !to_explore.is_empty() {
        let (id, schema) = to_explore.pop().unwrap();

        explored.insert(id.clone());

        if let Some(c) = &schema.canonical_type
            && !explored.contains(c)
        {
            let schema = schemas
                .get(&c)
                .expect(&format!("Schema {c} referenced by {id} not found"));

            to_explore.push((c.clone(), schema));
        }

        for reff in extract_refs(schema) {
            let ref_id = match reff {
                Ref::Global(id) => id,
                Ref::Def(name) => id.root().subtype(name),
            };

            if !explored.contains(&ref_id) {
                let schema = schemas
                    .get(&ref_id)
                    .expect(&format!("Schema {ref_id} referenced by {id} not found"));

                to_explore.push((ref_id, schema));
            }
        }
    }

    let mut unused_schemas: Vec<_> = schemas
        .keys()
        .filter(|name| !explored.contains(*name))
        .map(|id| id.to_string())
        .collect();

    if !unused_schemas.is_empty() {
        unused_schemas.sort();

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

fn decode_ref(reff: &str) -> Ref {
    if reff.starts_with("http:") || reff.starts_with("https:") {
        Ref::Global(SchemaId::new(reff))
    } else if let Some(local) = reff.strip_prefix("#/$defs/") {
        Ref::Def(local.to_string())
    } else {
        panic!("Invalid reference {reff}");
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SchemaId(String);

// Matches: https://opendatafabric.org/schemas/{context}/{version}/{Name}
// Also matches with an optional #/$defs/{Def} fragment.
pub static SCHEMA_URL_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(
        r"^https://opendatafabric\.org/schemas/(?P<context>[^/]+)/(?P<version>[^/]+)/(?P<name>[^/.]+)(?:#/\$defs/(?P<def>[^/]+))?$",
    )
    .unwrap()
});

impl SchemaId {
    pub const METASCHEMA_BASE_URL: &str = "https://opendatafabric.org/schemas/metaschemas/";
    pub const METASCHEMA_JSONSCHEMA: &str = "https://json-schema.org/draft/2020-12/schema";
    pub const METASCHEMA_MANIFEST: &str =
        "https://opendatafabric.org/schemas/metaschemas/v1alpha1/Manifest";
    pub const METASCHEMA_RESOURCE: &str =
        "https://opendatafabric.org/schemas/metaschemas/v1alpha1/Resource";
    pub const METASCHEMA_RESOURCE_INPUT: &str =
        "https://opendatafabric.org/schemas/metaschemas/v1alpha1/ResourceInput";
    pub const METASCHEMA_RESOURCE_REF: &str =
        "https://opendatafabric.org/schemas/metaschemas/v1alpha1/ResourceRef";
    pub const METASCHEMA_RESOURCE_HANDLE: &str =
        "https://opendatafabric.org/schemas/metaschemas/v1alpha1/ResourceHandle";
    pub const METASCHEMA_RESOURCE_CONDITION: &str =
        "https://opendatafabric.org/schemas/metaschemas/v1alpha1/ResourceCondition";
    pub const METASCHEMA_ENGINE_MESSAGE: &str =
        "https://opendatafabric.org/schemas/metaschemas/v1alpha1/EngineMessage";

    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn root<'a>(&'a self) -> Cow<'a, SchemaId> {
        if let Some(i) = self.0.find("#/$defs/") {
            Cow::Owned(Self(self.0[..i].into()))
        } else {
            Cow::Borrowed(self)
        }
    }

    pub fn parent(&self) -> Option<SchemaId> {
        if let Some(i) = self.0.find("#/$defs/") {
            Some(Self(self.0[..i].into()))
        } else {
            None
        }
    }

    pub fn subtype(&self, name: impl AsRef<str>) -> Self {
        assert!(
            !self.0.contains('#'),
            "Nesting of subtypes is not supported"
        );
        Self(format!("{self}#/$defs/{}", name.as_ref()))
    }

    pub fn root_name(&self) -> &str {
        let cap = SCHEMA_URL_RE
            .captures(&self.0)
            .expect("Invalid schema $id: {self}");

        cap.name("name").unwrap().as_str()
    }

    pub fn name(&self) -> &str {
        let cap = SCHEMA_URL_RE
            .captures(&self.0)
            .unwrap_or_else(|| panic!("Invalid schema $id: {self}"));

        if let Some(name) = cap.name("def") {
            name.as_str()
        } else {
            cap.name("name").unwrap().as_str()
        }
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl std::ops::Deref for SchemaId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl std::fmt::Display for SchemaId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

enum Ref {
    Global(SchemaId),
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
    Did,

    // Meta-types
    TypeName,
    TypeUri,
    TypeRef,

    // Identity and references
    AccountId,
    AccountName,

    DatasetId,
    DatasetName,
    DatasetAlias, // TODO: Should this be replaced by DatasetRef everywhere?
    DatasetRef,

    ResourceId,
    ResourceName,

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
    MapKeyFormat,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FlatbuffersMapFormat {
    JsonEncodedString,
}

impl std::str::FromStr for FlatbuffersMapFormat {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.into()))
    }
}
