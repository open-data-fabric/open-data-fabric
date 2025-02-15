use convert_case::{Case, Casing};
use std::collections::BTreeMap;

use indexmap::IndexMap;

use crate::json_schema;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Model {
    pub types: BTreeMap<String, TypeDefinition>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeName(pub String);

impl std::fmt::Display for TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub enum TypeDefinition {
    Object(Object),
    Union(Union),
    Enum(Enum),
}

impl TypeDefinition {
    pub fn name(&self) -> &TypeName {
        match self {
            TypeDefinition::Object(v) => &v.name,
            TypeDefinition::Union(v) => &v.name,
            TypeDefinition::Enum(v) => &v.name,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    pub name: TypeName,
    pub fields: IndexMap<String, Field>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct Union {
    pub name: TypeName,
    pub variants: Vec<TypeName>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: TypeName,
    pub variants: Vec<String>,
    pub description: String,
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
    Custom(TypeName),
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
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn parse_jsonschema(schemas: Vec<json_schema::Schema>) -> Model {
    let mut types = BTreeMap::new();

    for mut schema in schemas {
        let id = schema.id.take().expect("Named type missing an $id");
        schema.schema.take().expect("Named type missing a $schema");

        let root_name = id_to_name(&id);

        // Extract all $defs into top-level types
        for (dname, dsch) in schema.defs.take().unwrap_or_default() {
            let def_name = TypeName(format!("{root_name}{dname}"));
            let typ = parse_type_definition(
                def_name.clone(),
                dsch,
                format!("{root_name}.$defs.{def_name}"),
            );
            types.insert(typ.name().0.clone(), typ);
        }

        let typ = parse_type_definition(root_name.clone(), schema, format!("{root_name}"));
        types.insert(typ.name().0.clone(), typ);
    }

    Model { types }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_type_definition(
    name: TypeName,
    schema: json_schema::Schema,
    ctx: String,
) -> TypeDefinition {
    match &schema {
        json_schema::Schema {
            one_of: Some(_), ..
        } => TypeDefinition::Union(parse_type_union(name, schema, ctx)),
        json_schema::Schema {
            r#enum: Some(_), ..
        } => TypeDefinition::Enum(parse_type_enum(name, schema, ctx)),
        json_schema::Schema {
            r#type: Some(t), ..
        } if t == "object" => TypeDefinition::Object(parse_type_object(name, schema, ctx)),
        _ => panic!("Invalid schema: {ctx}: {}", schema.display()),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_type_object(name: TypeName, schema: json_schema::Schema, ctx: String) -> Object {
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
        format: _,
        default: _,
        description: Some(description),
        examples: _,
    } = schema
    else {
        panic!("Invalid object schema: {ctx}: {}", schema.display())
    };

    let mut fields = IndexMap::new();

    for (pname, mut psch) in schema
        .properties
        .expect(&format!("Object schema without properties: {ctx}"))
    {
        let fdesc = psch
            .description
            .take()
            .expect(&format!("Field missing description: {ctx}.{pname}"));
        let ftype = parse_type(psch, format!("{ctx}.{pname}"));
        let fname = pname.to_case(Case::Snake);

        let field = Field {
            name: fname.clone(),
            typ: ftype,
            optional: !required.contains(&pname),
            description: fdesc,
        };

        fields.insert(fname, field);
    }

    // Sanity check `required`
    for req in &required {
        if !fields.contains_key(&req.to_case(Case::Snake)) {
            panic!("Required property {req} is not defined: {ctx}");
        }
    }

    Object {
        name,
        description,
        fields,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_type_union(name: TypeName, schema: json_schema::Schema, ctx: String) -> Union {
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
        name,
        variants,
        description,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_type_enum(name: TypeName, schema: json_schema::Schema, ctx: String) -> Enum {
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
        name,
        variants,
        description,
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
        examples,
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
        default,
        description: None,
        examples: _,
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

fn parse_ref(schema: json_schema::Schema, ctx: String) -> TypeName {
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
        default,
        description,
        examples: None,
    } = schema
    else {
        panic!("Invalid $ref schema: {ctx}: {}", schema.display())
    };

    TypeName(ref_to_name(&reff, format!("{ctx}.$ref")))
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn id_to_name(id: &str) -> TypeName {
    TypeName(id.rsplit_once('/').unwrap().1.to_string())
}

fn ref_to_name(reff: &str, ctx: String) -> String {
    let parent_name = ctx.split_once('.').unwrap().0;
    let name = reff.rsplit_once('/').unwrap().1;
    if reff.starts_with("/schemas/") {
        name.to_string()
    } else if reff.starts_with("#/$defs") {
        format!("{parent_name}{name}")
    } else {
        panic!("Invalid reference: {ctx}: {reff}")
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
