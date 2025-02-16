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
    pub namespace: Option<String>,
    pub name: String,
}

impl TypeId {
    pub fn join<'a, 'b>(&'a self, sep: &'b str) -> Cow<'a, String> {
        if let Some(ns) = &self.namespace {
            Cow::Owned(format!("{ns}{sep}{}", self.name))
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
}

impl TypeDefinition {
    pub fn id(&self) -> &TypeId {
        match self {
            TypeDefinition::Struct(v) => &v.id,
            TypeDefinition::Union(v) => &v.id,
            TypeDefinition::Enum(v) => &v.id,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            TypeDefinition::Struct(v) => &v.description,
            TypeDefinition::Union(v) => &v.description,
            TypeDefinition::Enum(v) => &v.description,
        }
    }

    pub fn src(&self) -> &Path {
        match self {
            TypeDefinition::Struct(v) => &v.src,
            TypeDefinition::Union(v) => &v.src,
            TypeDefinition::Enum(v) => &v.src,
        }
    }
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
}

#[derive(Debug, Clone)]
pub enum Type {
    Boolean,
    Int16,
    Int32,
    Int64,
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
    pub optional: bool,
    pub description: String,
    pub default: Option<serde_json::Value>,
    pub examples: Option<Vec<serde_json::Value>>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn parse_jsonschema(schemas: Vec<json_schema::Schema>) -> Model {
    let mut types = BTreeMap::new();

    for mut schema in schemas {
        let id = schema.id.take().expect("Named type missing an $id");
        schema.schema.take().expect("Named type missing a $schema");

        let src = schema.src.take().expect("Schema without source path");

        let root = TypeId {
            namespace: None,
            name: id.rsplit_once('/').unwrap().1.to_string(),
        };

        // Extract all $defs into top-level types
        for (dname, dsch) in schema.defs.take().unwrap_or_default() {
            let def_name = TypeId {
                namespace: Some(root.name.clone()),
                name: dname,
            };
            let typ = parse_type_definition(
                def_name.clone(),
                dsch,
                src.clone(),
                format!("{}.$defs.{}", root.name, def_name.name),
            );
            types.insert(typ.id().clone(), typ);
        }

        let typ = parse_type_definition(root.clone(), schema, src, format!("{}", root.name));
        types.insert(typ.id().clone(), typ);
    }

    Model { types }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_type_definition(
    name: TypeId,
    schema: json_schema::Schema,
    src: PathBuf,
    ctx: String,
) -> TypeDefinition {
    match &schema {
        json_schema::Schema {
            one_of: Some(_), ..
        } => TypeDefinition::Union(parse_type_union(name, schema, src, ctx)),
        json_schema::Schema {
            r#enum: Some(_), ..
        } => TypeDefinition::Enum(parse_type_enum(name, schema, src, ctx)),
        json_schema::Schema {
            r#type: Some(t), ..
        } if t == "object" => TypeDefinition::Struct(parse_type_struct(name, schema, src, ctx)),
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
        additional_properties: Some(false),
        one_of: None,
        r#enum: None,
        items: None,
        r#ref: None,
        format: None,
        default: None,
        description: Some(description),
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

        let ftype = parse_type(psch, format!("{ctx}.{pname}"));
        let fname = pname.to_case(Case::Snake);

        let field = Field {
            name: fname.clone(),
            typ: ftype,
            optional: !required.contains(&pname),
            description: fdesc,
            default: fdefault,
            examples: fexamples,
        };

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
        additional_properties: None,
        one_of: Some(one_of),
        r#enum: None,
        items: None,
        r#ref: None,
        format: None,
        default: None,
        description: Some(description),
        examples: None,
        src: None,
    } = schema
    else {
        panic!("Invalid union schema: {ctx}: {}", schema.display())
    };

    let mut variants = Vec::new();
    for (i, schema) in one_of.into_iter().enumerate() {
        let reff = parse_ref(schema, format!("{ctx}.$oneOf.[{i}]"));
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
        additional_properties: None,
        one_of: None,
        r#enum: Some(enums),
        items: None,
        r#ref: None,
        format: None,
        default: None,
        description: Some(description),
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

    Enum {
        id,
        variants,
        description,
        src,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_type(schema: json_schema::Schema, ctx: String) -> Type {
    match &schema {
        json_schema::Schema { r#ref: Some(_), .. } => Type::Custom(parse_ref(schema, ctx)),
        json_schema::Schema {
            r#type: Some(t), ..
        } if t == "array" => Type::Array(parse_type_array(schema, ctx)),
        json_schema::Schema {
            r#type: Some(t), ..
        } if ["string", "integer", "boolean"].contains(&t.as_str()) => {
            parse_type_scalar(schema, ctx)
        }
        _ => panic!("Invalid schema: {ctx}: {}", schema.display()),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_type_array(schema: json_schema::Schema, ctx: String) -> Array {
    assert_eq!(schema.r#type.as_deref(), Some("array"));

    let json_schema::Schema {
        id: None,
        schema: None,
        defs: None,
        r#type: Some(_),
        required: None,
        properties: None,
        additional_properties: None,
        one_of: None,
        r#enum: None,
        items: Some(items),
        r#ref: None,
        format: None,
        default: None,
        description: None,
        examples: None,
        src: None,
    } = schema
    else {
        panic!("Invalid array schema: {ctx}: {}", schema.display())
    };

    let item_type = Box::new(parse_type(*items, format!("{ctx}.items")));

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
        additional_properties: None,
        one_of: None,
        r#enum: None,
        items: None,
        r#ref: None,
        format,
        default: None,
        description: None,
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

fn parse_ref(schema: json_schema::Schema, ctx: String) -> TypeId {
    let json_schema::Schema {
        id: None,
        schema: None,
        defs: None,
        r#type: None,
        required: None,
        properties: None,
        additional_properties: None,
        one_of: None,
        r#enum: None,
        items: None,
        r#ref: Some(reff),
        format: None,
        default: None,
        description: None,
        examples: None,
        src: None,
    } = schema
    else {
        panic!("Invalid $ref schema: {ctx}: {}", schema.display())
    };

    ref_to_name(&reff, format!("{ctx}.$ref"))
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn ref_to_name(reff: &str, ctx: String) -> TypeId {
    let parent = ctx.split_once('.').unwrap().0.to_string();
    let name = reff.rsplit_once('/').unwrap().1.to_string();
    if reff.starts_with("/schemas/") {
        TypeId {
            namespace: None,
            name,
        }
    } else if reff.starts_with("#/$defs") {
        TypeId {
            namespace: Some(parent),
            name,
        }
    } else {
        panic!("Invalid reference: {ctx}: {reff}")
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
