use convert_case::{Case, Casing};
use std::{
    borrow::Cow,
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use indexmap::IndexMap;

use crate::json_schema;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Model {
    pub types: BTreeMap<TypeId, TypeDefinition>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeId {
    pub parent: Option<Box<TypeId>>,
    pub name: String,
}

impl TypeId {
    pub fn new_root(name: impl Into<String>) -> Self {
        Self {
            parent: None,
            name: name.into(),
        }
    }

    pub fn subtype(&self, name: impl Into<String>) -> Self {
        Self {
            parent: Some(Box::new(self.clone())),
            name: name.into(),
        }
    }

    pub fn root(&self) -> &TypeId {
        if let Some(p) = &self.parent {
            p.root()
        } else {
            self
        }
    }

    pub fn join<'a, 'b>(&'a self, sep: &'b str) -> Cow<'a, String> {
        if let Some(p) = &self.parent {
            Cow::Owned(format!("{}{sep}{}", p.join(sep), self.name))
        } else {
            Cow::Borrowed(&self.name)
        }
    }
}

impl PartialOrd for TypeId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.join("::").partial_cmp(&other.join("::"))
    }
}

impl Ord for TypeId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.join("::").cmp(&other.join("::"))
    }
}

#[derive(Debug, Clone)]
pub enum TypeDefinition {
    Struct(Struct),
    Union(Union),
    Enum(Enum),
    Extensions(Extensions),
}

impl TypeDefinition {
    pub fn id(&self) -> &TypeId {
        match self {
            TypeDefinition::Struct(v) => &v.id,
            TypeDefinition::Union(v) => &v.id,
            TypeDefinition::Enum(v) => &v.id,
            TypeDefinition::Extensions(v) => &v.id,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            TypeDefinition::Struct(v) => &v.description,
            TypeDefinition::Union(v) => &v.description,
            TypeDefinition::Enum(v) => &v.description,
            TypeDefinition::Extensions(v) => &v.description,
        }
    }

    pub fn src(&self) -> &Path {
        match self {
            TypeDefinition::Struct(v) => &v.src,
            TypeDefinition::Union(v) => &v.src,
            TypeDefinition::Enum(v) => &v.src,
            TypeDefinition::Extensions(v) => &v.src,
        }
    }

    pub fn category(&self) -> TypeCategory {
        match self
            .src()
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
        {
            "schemas" => TypeCategory::Root,
            "fragments" => TypeCategory::Fragment,
            "metadata-events" => TypeCategory::MetadataEvent,
            "schema" => TypeCategory::DataSchema,
            "engine-ops" => TypeCategory::EngineProtocol,
            _ => panic!("Unable to classify path: {}", self.src().display()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeCategory {
    Root,
    Fragment,
    MetadataEvent,
    DataSchema,
    EngineProtocol,
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub id: TypeId,
    pub fields: IndexMap<String, Field>,
    pub description: String,
    pub src: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Union {
    pub id: TypeId,
    pub variants: Vec<TypeId>,
    pub description: String,
    pub src: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub id: TypeId,
    pub variants: Vec<String>,
    pub description: String,
    pub src: PathBuf,
    pub format: Type,
}

#[derive(Debug, Clone)]
pub struct Extensions {
    pub id: TypeId,
    pub description: String,
    pub src: PathBuf,
}

#[derive(Debug, Clone)]
pub enum Type {
    Boolean,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    String,
    DatasetAlias,
    DatasetId,
    DatasetRef,
    DateTime,
    Flatbuffers,
    Multicodec,
    Multihash,
    Path,
    Regex,
    Url,
    Array(Array),
    Custom(TypeId),
}

#[derive(Debug, Clone)]
pub struct Array {
    pub item_type: Box<Type>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub typ: Type,
    pub validations: Vec<Validation>,
    pub optional: bool,
    pub description: String,
    pub default: Option<serde_json::Value>,
    pub examples: Option<Vec<serde_json::Value>>,
    pub explicit_tag: Option<u32>,
    pub deprecated: bool,
    pub codegen_hints: IndexMap<String, IndexMap<String, String>>,
}

#[derive(Debug, Clone)]
pub enum Validation {
    Enum(ValidationEnum),
}

#[derive(Debug, Clone)]
pub struct ValidationEnum {
    pub values: Vec<serde_json::Value>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn parse_jsonschema(schemas: Vec<json_schema::Schema>) -> Model {
    let mut types = BTreeMap::new();

    for mut schema in schemas {
        let root_id = schema_id_to_type_id(&schema.id.take().expect("Named type missing an $id"));
        schema.schema.take().expect("Named type missing a $schema");

        let src = schema.src.take().expect("Schema without source path");

        // Extract all $defs into top-level types
        for (dname, dsch) in schema.defs.take().unwrap_or_default() {
            let def_id = root_id.subtype(dname);

            let typ = parse_type_definition(
                def_id.clone(),
                dsch,
                src.clone(),
                format!("{}.$defs.{}", root_id.name, def_id.name),
            );
            types.insert(typ.id().clone(), typ);
        }

        let typ = parse_type_definition(root_id.clone(), schema, src, format!("{}", root_id.name));
        types.insert(typ.id().clone(), typ);
    }

    Model { types }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_type_definition(
    id: TypeId,
    schema: json_schema::Schema,
    src: PathBuf,
    ctx: String,
) -> TypeDefinition {
    match &schema {
        json_schema::Schema {
            one_of: Some(_), ..
        } => TypeDefinition::Union(parse_type_union(id, schema, src, ctx)),
        json_schema::Schema {
            r#enum: Some(_), ..
        } => TypeDefinition::Enum(parse_type_enum(id, schema, src, ctx)),
        json_schema::Schema {
            r#type: Some(t),
            pattern_properties: Some(_),
            properties: None,
            ..
        } if t == "object" => {
            TypeDefinition::Extensions(parse_type_extensions(id, schema, src, ctx))
        }
        json_schema::Schema {
            r#type: Some(t), ..
        } if t == "object" => TypeDefinition::Struct(parse_type_struct(id, schema, src, ctx)),
        _ => panic!("Invalid schema: {ctx}: {}", schema.display()),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_type_struct(id: TypeId, schema: json_schema::Schema, src: PathBuf, ctx: String) -> Struct {
    assert_eq!(schema.r#type.as_deref(), Some("object"));

    let json_schema::Schema {
        id: None,
        schema: None,
        defs: None,
        r#type: Some(_),
        required: Some(required),
        properties: Some(_),
        pattern_properties: None,
        additional_properties: Some(false),
        one_of: None,
        r#enum: None,
        items: None,
        r#ref: None,
        format: None,
        default: None,
        description: Some(description),
        tag: None,
        codegen: None,
        deprecated: None,
        examples: None,
        src: None,
    } = schema
    else {
        panic!("Invalid struct schema: {ctx}: {}", schema.display())
    };

    let mut fields = IndexMap::new();

    for (pname, mut psch) in schema
        .properties
        .expect(&format!("Struct schema without properties: {ctx}"))
    {
        let fdesc = psch
            .description
            .take()
            .expect(&format!("Field missing description: {ctx}.{pname}"));

        let fdefault = psch.default.take();
        let fexamples = psch.examples.take();

        let ftag = psch.tag.take();
        let codegen_hints = psch.codegen.take().unwrap_or_default();
        let fdeprecated = psch.deprecated.take().unwrap_or(false);

        let validations = parse_validations(&mut psch, format!("{ctx}.{pname}"));

        let ftype = parse_type(psch, &id, format!("{ctx}.{pname}"));
        let fname = pname.to_case(Case::Snake);

        let field = Field {
            name: fname.clone(),
            typ: ftype,
            validations,
            optional: !required.contains(&pname),
            description: fdesc,
            default: fdefault,
            examples: fexamples,
            explicit_tag: ftag,
            deprecated: fdeprecated,
            codegen_hints,
        };

        if field.default.is_some() && !field.optional {
            panic!("Required field cannot have a default value: {ctx}");
        }

        fields.insert(fname, field);
    }

    // Sanity check `required`
    for req in &required {
        if !fields.contains_key(&req.to_case(Case::Snake)) {
            panic!("Required property {req} is not defined: {ctx}");
        }
    }

    Struct {
        id,
        description,
        fields,
        src,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_type_union(id: TypeId, schema: json_schema::Schema, src: PathBuf, ctx: String) -> Union {
    let json_schema::Schema {
        id: None,
        schema: None,
        defs: None,
        r#type: None,
        required: None,
        properties: None,
        pattern_properties: None,
        additional_properties: None,
        one_of: Some(one_of),
        r#enum: None,
        items: None,
        r#ref: None,
        format: None,
        default: None,
        description: Some(description),
        tag: None,
        codegen: None,
        deprecated: None,
        examples: None,
        src: None,
    } = schema
    else {
        panic!("Invalid union schema: {ctx}: {}", schema.display())
    };

    let mut variants = Vec::new();
    for (i, schema) in one_of.into_iter().enumerate() {
        let reff = parse_ref(schema, &id, format!("{ctx}.$oneOf.[{i}]"));
        variants.push(reff);
    }

    Union {
        id,
        variants,
        description,
        src,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_type_enum(id: TypeId, schema: json_schema::Schema, src: PathBuf, ctx: String) -> Enum {
    let json_schema::Schema {
        id: None,
        schema: None,
        defs: None,
        r#type: Some(typ),
        required: None,
        properties: None,
        pattern_properties: None,
        additional_properties: None,
        one_of: None,
        r#enum: Some(enums),
        items: None,
        r#ref: None,
        format,
        default: None,
        description: Some(description),
        tag: None,
        codegen: None,
        deprecated: None,
        examples: None,
        src: None,
    } = schema
    else {
        panic!("Invalid enum schema: {ctx}: {}", schema.display())
    };

    assert_eq!(typ, "string", "Only string type enums are supported: {ctx}");

    let mut variants = Vec::new();
    for variant in enums {
        match variant {
            serde_json::Value::String(s) => variants.push(s),
            _ => panic!("Only string type enums are supported: {ctx}: {variant:?}"),
        }
    }

    let format = match format.unwrap_or("int32".into()).as_str() {
        "int8" => Type::Int8,
        "int16" => Type::Int16,
        "int32" => Type::Int32,
        "int64" => Type::Int64,
        "uint8" => Type::UInt8,
        "uint16" => Type::UInt16,
        "uint32" => Type::UInt32,
        "uint64" => Type::UInt64,
        fmt => panic!("Invalid enum format: {ctx}: {}", fmt),
    };

    Enum {
        id,
        variants,
        description,
        src,
        format,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_type_extensions(
    id: TypeId,
    schema: json_schema::Schema,
    src: PathBuf,
    ctx: String,
) -> Extensions {
    assert_eq!(schema.r#type.as_deref(), Some("object"));

    let json_schema::Schema {
        id: None,
        schema: None,
        defs: None,
        r#type: Some(_),
        required: None,
        properties: None,
        pattern_properties: Some(pattern_properties),
        additional_properties: Some(false),
        one_of: None,
        r#enum: None,
        items: None,
        r#ref: None,
        format: None,
        default: None,
        description: Some(description),
        tag: None,
        codegen: None,
        deprecated: None,
        examples: None,
        src: None,
    } = &schema
    else {
        panic!("Invalid extensions schema: {ctx}: {}", schema.display())
    };

    if pattern_properties.len() != 1 {
        panic!(
            "Only one patterns is supported in extensions schema: {ctx}: {}",
            schema.display()
        )
    }

    let Some(json_schema::Schema {
        id: None,
        schema: None,
        defs: None,
        r#type: None,
        required: None,
        properties: None,
        pattern_properties: None,
        additional_properties: None,
        one_of: None,
        r#enum: None,
        items: None,
        r#ref: None,
        format: None,
        default: None,
        description: None,
        tag: None,
        codegen: None,
        deprecated: None,
        examples: None,
        src: None,
    }) = pattern_properties.values().into_iter().next()
    else {
        panic!(
            "Extensions propety values expected to be any-type: {ctx}: {}",
            schema.display()
        )
    };

    Extensions {
        id,
        description: description.clone(),
        src,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_type(schema: json_schema::Schema, root: &TypeId, ctx: String) -> Type {
    match &schema {
        json_schema::Schema { r#ref: Some(_), .. } => Type::Custom(parse_ref(schema, root, ctx)),
        json_schema::Schema {
            r#type: Some(t), ..
        } if t == "array" => Type::Array(parse_type_array(schema, root, ctx)),
        json_schema::Schema {
            r#type: Some(t), ..
        } if ["string", "integer", "boolean"].contains(&t.as_str()) => {
            parse_type_scalar(schema, ctx)
        }
        _ => panic!("Invalid schema: {ctx}: {}", schema.display()),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_type_array(schema: json_schema::Schema, root: &TypeId, ctx: String) -> Array {
    assert_eq!(schema.r#type.as_deref(), Some("array"));

    let json_schema::Schema {
        id: None,
        schema: None,
        defs: None,
        r#type: Some(_),
        required: None,
        properties: None,
        pattern_properties: None,
        additional_properties: None,
        one_of: None,
        r#enum: None,
        items: Some(items),
        r#ref: None,
        format: None,
        default: None,
        description: None,
        tag: None,
        codegen: None,
        deprecated: None,
        examples: None,
        src: None,
    } = schema
    else {
        panic!("Invalid array schema: {ctx}: {}", schema.display())
    };

    let item_type = Box::new(parse_type(*items, root, format!("{ctx}.items")));

    Array { item_type }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_type_scalar(schema: json_schema::Schema, ctx: String) -> Type {
    let json_schema::Schema {
        id: None,
        schema: None,
        defs: None,
        r#type: Some(typ),
        required: None,
        properties: None,
        pattern_properties: None,
        additional_properties: None,
        one_of: None,
        r#enum: None,
        items: None,
        r#ref: None,
        format,
        default: None,
        description: None,
        tag: None,
        codegen: None,
        deprecated: None,
        examples: _,
        src: None,
    } = &schema
    else {
        panic!("Invalid scalar schema: {ctx}: {}", schema.display())
    };

    match (typ.as_str(), format.as_deref()) {
        ("boolean", None) => Type::Boolean,
        ("integer", Some(format)) => match format {
            "int16" => Type::Int16,
            "int32" => Type::Int32,
            "int64" => Type::Int64,
            "uint16" => Type::UInt16,
            "uint32" => Type::UInt32,
            "uint64" => Type::UInt64,
            _ => panic!("Invalid integer format: {ctx}: {}", schema.display()),
        },
        ("string", None) => Type::String,
        ("string", Some("dataset-alias")) => Type::DatasetAlias,
        ("string", Some("dataset-id")) => Type::DatasetId,
        ("string", Some("dataset-ref")) => Type::DatasetRef,
        ("string", Some("date-time")) => Type::DateTime,
        ("string", Some("flatbuffers")) => Type::Flatbuffers,
        ("string", Some("multicodec")) => Type::Multicodec,
        ("string", Some("multihash")) => Type::Multihash,
        ("string", Some("path")) => Type::Path,
        ("string", Some("regex")) => Type::Regex,
        ("string", Some("url")) => Type::Url,
        _ => panic!("Invalid scalar schema: {ctx}: {}", schema.display()),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_validations(schema: &mut json_schema::Schema, _ctx: String) -> Vec<Validation> {
    let mut validations = Vec::new();

    if let Some(values) = schema.r#enum.take() {
        validations.push(Validation::Enum(ValidationEnum { values }))
    }

    validations
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_ref(schema: json_schema::Schema, parent: &TypeId, ctx: String) -> TypeId {
    let json_schema::Schema {
        id: None,
        schema: None,
        defs: None,
        r#type: None,
        required: None,
        properties: None,
        pattern_properties: None,
        additional_properties: None,
        one_of: None,
        r#enum: None,
        items: None,
        r#ref: Some(reff),
        format: None,
        default: None,
        description: None,
        tag: None,
        codegen: None,
        deprecated: None,
        examples: None,
        src: None,
    } = schema
    else {
        panic!("Invalid $ref schema: {ctx}: {}", schema.display())
    };

    ref_to_type_id(&reff, parent, format!("{ctx}.$ref"))
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn schema_id_to_type_id(id: &str) -> TypeId {
    let Some(suffix) = id.strip_prefix("http://open-data-fabric.github.com/schemas/") else {
        panic!("Invalid schema $id: {id}");
    };
    TypeId {
        parent: None,
        name: suffix.to_string(),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn ref_to_type_id(reff: &str, parent: &TypeId, ctx: String) -> TypeId {
    if let Some(global) = reff.strip_prefix("/schemas/") {
        if let Some((global, local)) = global.split_once("#/$defs/") {
            TypeId {
                parent: Some(Box::new(TypeId {
                    parent: None,
                    name: global.to_string(),
                })),
                name: local.to_string(),
            }
        } else {
            TypeId {
                parent: None,
                name: global.to_string(),
            }
        }
    } else if let Some(local) = reff.strip_prefix("#/$defs/") {
        parent.root().subtype(local)
    } else {
        panic!("Invalid reference: {ctx}: {reff}")
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// In our model we expect:
// - Either no tags to be specified, or every fields has to have a tag
// - Tags should be in increasing order
// - Gaps are allowed (but there's a catch on flatbuffer level)
//
// Note that these rules are different from flatbuffer field IDs and are adapted in the codegen
pub fn check_explicit_tags_sequence(model: &Model) {
    for (id, t) in &model.types {
        let TypeDefinition::Struct(t) = t else {
            continue;
        };

        if !t.fields.values().any(|f| f.explicit_tag.is_some()) {
            continue;
        }

        let mut maybe_prev_tag = None;

        for f in t.fields.values() {
            let tag = f.explicit_tag.unwrap();

            if let Some(prev_tag) = maybe_prev_tag {
                if tag <= prev_tag {
                    panic!(
                        "Invalid tag {}::{} ({tag} less than previous tag {prev_tag})",
                        id.join("::"),
                        f.name,
                    );
                }
            }

            maybe_prev_tag = Some(tag);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
