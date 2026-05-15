use convert_case::{Case, Casing};
use std::{
    borrow::Cow,
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use indexmap::IndexMap;

use crate::json_schema::{self, CodegenHint, CodegenLanguage};

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
    Map(Map),
}

impl TypeDefinition {
    pub fn id(&self) -> &TypeId {
        match self {
            TypeDefinition::Struct(v) => &v.id,
            TypeDefinition::Union(v) => &v.id,
            TypeDefinition::Enum(v) => &v.id,
            TypeDefinition::Map(v) => &v.id,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            TypeDefinition::Struct(v) => &v.description,
            TypeDefinition::Union(v) => &v.description,
            TypeDefinition::Enum(v) => &v.description,
            TypeDefinition::Map(v) => &v.description,
        }
    }

    pub fn codegen_hints(&self) -> &IndexMap<CodegenLanguage, IndexMap<CodegenHint, String>> {
        match self {
            TypeDefinition::Struct(v) => &v.codegen_hints,
            TypeDefinition::Union(v) => &v.codegen_hints,
            TypeDefinition::Enum(v) => &v.codegen_hints,
            TypeDefinition::Map(v) => &v.codegen_hints,
        }
    }

    pub fn src(&self) -> &Path {
        match self {
            TypeDefinition::Struct(v) => &v.src,
            TypeDefinition::Union(v) => &v.src,
            TypeDefinition::Enum(v) => &v.src,
            TypeDefinition::Map(v) => &v.src,
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
            "resources" => TypeCategory::Resource,
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
    Resource,
    Fragment,
    MetadataEvent,
    DataSchema,
    EngineProtocol,
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub id: TypeId,
    pub fields: IndexMap<String, Field>,
    pub generics: Vec<String>,
    pub description: String,
    pub from_string: bool,
    pub codegen_hints: IndexMap<CodegenLanguage, IndexMap<CodegenHint, String>>,
    pub src: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Union {
    pub id: TypeId,
    pub variants: Vec<TypeId>,
    pub description: String,
    pub from_string: bool,
    pub codegen_hints: IndexMap<CodegenLanguage, IndexMap<CodegenHint, String>>,
    pub src: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub id: TypeId,
    pub variants: Vec<String>,
    pub description: String,
    pub src: PathBuf,
    pub codegen_hints: IndexMap<CodegenLanguage, IndexMap<CodegenHint, String>>,
    pub format: Type,
}

#[derive(Debug, Clone)]
pub struct Map {
    pub id: TypeId,
    pub description: String,
    pub value_type: Type,
    pub codegen_hints: IndexMap<CodegenLanguage, IndexMap<CodegenHint, String>>,
    pub src: PathBuf,
}

#[derive(Debug, Clone)]
pub enum Type {
    // Scalars
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

    DateTime,
    Multicodec,
    Multihash,
    Path,
    Regex,
    Url,

    // Identity & references
    AccountId,
    AccountName,

    DatasetAlias,
    DatasetId,
    DatasetRef,

    ResourceContext,
    ResourceKind,
    ResourceId,
    ResourceName,

    // Composite
    Flatbuffers,
    Generic(String),
    Array(Array),
    Custom(TypeId),
    AnyJson,
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
    pub codegen_hints: IndexMap<CodegenLanguage, IndexMap<CodegenHint, String>>,
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
            one_of: Some(_),
            format: None,
            ..
        } => TypeDefinition::Union(parse_type_union(id, schema, src, ctx)),
        json_schema::Schema {
            one_of: Some(_),
            format: Some(json_schema::Format::UnionOrString),
            ..
        } => {
            let mut schema = schema;
            match schema.one_of.as_mut().unwrap().remove(0) {
                json_schema::Schema {
                    r#type: Some(json_schema::Type::String),
                    ..
                } => (),
                _ => panic!("First variant of a `union-or-string` must be a string type: {ctx}"),
            }

            TypeDefinition::Union(parse_type_union(id, schema, src, ctx))
        }
        json_schema::Schema {
            format: Some(json_schema::Format::StructOrString),
            ..
        } => {
            let mut schema = schema;
            let Some(one_of) = &mut schema.one_of else {
                panic!("A `struct-or-string` schema must be a `oneOf` union: {ctx}")
            };

            match one_of.remove(0) {
                json_schema::Schema {
                    r#type: Some(json_schema::Type::String),
                    ..
                } => (),
                _ => panic!("First variant of a `struct-or-string` must be a string type: {ctx}"),
            }

            let obj = schema.one_of.as_mut().unwrap().remove(0);
            assert!(schema.one_of.as_ref().unwrap().is_empty());

            let schema = json_schema::Schema {
                id: schema.id,
                schema: schema.schema,
                defs: schema.defs,
                r#type: obj.r#type,
                required: obj.required,
                properties: obj.properties,
                pattern_properties: obj.pattern_properties,
                additional_properties: obj.additional_properties,
                one_of: obj.one_of,
                r#enum: obj.r#enum,
                items: obj.items,
                r#ref: obj.r#ref,
                format: obj.format,
                default: obj.default,
                description: schema.description,
                tag: obj.tag,
                codegen: obj.codegen,
                deprecated: obj.deprecated,
                examples: obj.examples,
                src: obj.src,
            };
            TypeDefinition::Struct(parse_type_struct(id, schema, src, ctx, true))
        }
        json_schema::Schema {
            r#enum: Some(_), ..
        } => TypeDefinition::Enum(parse_type_enum(id, schema, src, ctx)),
        json_schema::Schema {
            r#type: Some(json_schema::Type::Object),
            pattern_properties: Some(_),
            properties: None,
            ..
        } => TypeDefinition::Map(parse_type_map(id, schema, src, ctx)),
        json_schema::Schema {
            r#type: Some(json_schema::Type::Object),
            ..
        } => TypeDefinition::Struct(parse_type_struct(id, schema, src, ctx, false)),
        _ => panic!("Invalid schema: {ctx}: {}", schema.display()),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_type_struct(
    id: TypeId,
    schema: json_schema::Schema,
    src: PathBuf,
    ctx: String,
    from_string: bool,
) -> Struct {
    assert_eq!(schema.r#type, Some(json_schema::Type::Object));

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
        codegen,
        deprecated: None,
        examples: None,
        src: None,
    } = schema
    else {
        panic!("Invalid struct schema: {ctx}: {}", schema.display())
    };

    let mut fields = IndexMap::new();
    let mut generics = Vec::new();

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

        let ftype = match ftype {
            Type::Generic(_) => {
                let generic_t = format!("{}{}T", fname[0..1].to_uppercase(), &fname[1..]);
                generics.push(generic_t.clone());
                Type::Generic(generic_t)
            }
            _ => ftype,
        };

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
        generics,
        from_string,
        codegen_hints: codegen.unwrap_or_default(),
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
        panic!("Invalid union schema: {ctx}: {}", schema.display())
    };

    let from_string = match format {
        None => false,
        Some(json_schema::Format::UnionOrString) => true,
        Some(fmt) => panic!("Invalid union format: {ctx}: {fmt:?}"),
    };

    let mut variants = Vec::new();
    for (i, schema) in one_of.into_iter().enumerate() {
        let reff = parse_ref(schema, &id, format!("{ctx}.$oneOf.[{i}]"));
        variants.push(reff);
    }

    assert!(!variants.is_empty(), "Union must have at least one variant");

    Union {
        id,
        variants,
        description,
        from_string,
        codegen_hints: Default::default(),
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

    assert_eq!(
        typ,
        json_schema::Type::String,
        "Only string type enums are supported: {ctx}"
    );

    let mut variants = Vec::new();
    for variant in enums {
        match variant {
            serde_json::Value::String(s) => variants.push(s),
            _ => panic!("Only string type enums are supported: {ctx}: {variant:?}"),
        }
    }

    let format = match format.unwrap_or(json_schema::Format::Int32) {
        json_schema::Format::Int8 => Type::Int8,
        json_schema::Format::Int16 => Type::Int16,
        json_schema::Format::Int32 => Type::Int32,
        json_schema::Format::Int64 => Type::Int64,
        json_schema::Format::UInt8 => Type::UInt8,
        json_schema::Format::UInt16 => Type::UInt16,
        json_schema::Format::UInt32 => Type::UInt32,
        json_schema::Format::UInt64 => Type::UInt64,
        fmt => panic!("Invalid enum format: {ctx}: {fmt:?}"),
    };

    assert!(!variants.is_empty(), "Enum must have at least one variant");

    Enum {
        id,
        variants,
        description,
        format,
        codegen_hints: Default::default(),
        src,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_type_map(id: TypeId, schema: json_schema::Schema, src: PathBuf, ctx: String) -> Map {
    assert_eq!(schema.r#type, Some(json_schema::Type::Object));

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
        codegen,
        deprecated: None,
        examples: None,
        src: None,
    } = schema
    else {
        panic!("Invalid map schema: {ctx}: {}", schema.display())
    };

    if pattern_properties.len() != 1 {
        panic!("Only one pattern is supported in map schema: {ctx}",)
    }

    let value_schema = pattern_properties.into_iter().next().unwrap().1;

    let value_type = parse_type(value_schema, &id, ctx.clone());

    match &value_type {
        Type::String | Type::Custom(_) | Type::AnyJson => (),
        _ => panic!("Map values can only be strings, any, or $ref: {ctx}"),
    }

    Map {
        id,
        description: description.clone(),
        value_type,
        codegen_hints: codegen.unwrap_or_default(),
        src,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_type(schema: json_schema::Schema, root: &TypeId, ctx: String) -> Type {
    match &schema {
        json_schema::Schema { r#ref: Some(_), .. } => Type::Custom(parse_ref(schema, root, ctx)),
        json_schema::Schema {
            r#type: Some(json_schema::Type::Array),
            ..
        } => Type::Array(parse_type_array(schema, root, ctx)),
        json_schema::Schema {
            r#type:
                Some(
                    json_schema::Type::Boolean
                    | json_schema::Type::Integer
                    | json_schema::Type::String,
                ),
            ..
        } => parse_type_scalar(schema, ctx),
        json_schema::Schema {
            r#type: Some(json_schema::Type::Object),
            ..
        } => parse_type_scalar(schema, ctx),
        json_schema::Schema {
            r#type: None,
            r#ref: None,
            one_of: None,
            r#enum: None,
            ..
        } => Type::AnyJson,
        _ => panic!("Invalid schema: {ctx}: {}", schema.display()),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_type_array(schema: json_schema::Schema, root: &TypeId, ctx: String) -> Array {
    assert_eq!(schema.r#type, Some(json_schema::Type::Array));

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

    match (typ, format) {
        (json_schema::Type::Boolean, None) => Type::Boolean,
        (json_schema::Type::Integer, Some(format)) => match format {
            json_schema::Format::Int8 => Type::Int8,
            json_schema::Format::Int16 => Type::Int16,
            json_schema::Format::Int32 => Type::Int32,
            json_schema::Format::Int64 => Type::Int64,
            json_schema::Format::UInt8 => Type::UInt8,
            json_schema::Format::UInt16 => Type::UInt16,
            json_schema::Format::UInt32 => Type::UInt32,
            json_schema::Format::UInt64 => Type::UInt64,
            _ => panic!("Invalid integer format: {ctx}: {}", schema.display()),
        },
        (json_schema::Type::String, None) => Type::String,

        (json_schema::Type::String, Some(json_schema::Format::DateTime)) => Type::DateTime,
        (json_schema::Type::String, Some(json_schema::Format::Multicodec)) => Type::Multicodec,
        (json_schema::Type::String, Some(json_schema::Format::Multihash)) => Type::Multihash,
        (json_schema::Type::String, Some(json_schema::Format::Path)) => Type::Path,
        (json_schema::Type::String, Some(json_schema::Format::Regex)) => Type::Regex,
        (json_schema::Type::String, Some(json_schema::Format::Url)) => Type::Url,

        (json_schema::Type::String, Some(json_schema::Format::AccountId)) => Type::AccountId,
        (json_schema::Type::String, Some(json_schema::Format::AccountName)) => Type::AccountName,

        (json_schema::Type::String, Some(json_schema::Format::DatasetAlias)) => Type::DatasetAlias,
        (json_schema::Type::String, Some(json_schema::Format::DatasetId)) => Type::DatasetId,
        (json_schema::Type::String, Some(json_schema::Format::DatasetRef)) => Type::DatasetRef,

        (json_schema::Type::String, Some(json_schema::Format::ResourceContext)) => {
            Type::ResourceContext
        }
        (json_schema::Type::String, Some(json_schema::Format::ResourceKind)) => Type::ResourceKind,
        (json_schema::Type::String, Some(json_schema::Format::ResourceId)) => Type::ResourceId,
        (json_schema::Type::String, Some(json_schema::Format::ResourceName)) => Type::ResourceName,

        (json_schema::Type::String, Some(json_schema::Format::Flatbuffers)) => Type::Flatbuffers,
        (json_schema::Type::Object, Some(json_schema::Format::Fragment)) => {
            Type::Generic(String::new())
        }
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
