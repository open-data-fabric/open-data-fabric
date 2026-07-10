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
pub struct TypeId(json_schema::SchemaId);

impl TypeId {
    pub fn new(schema_id: json_schema::SchemaId) -> Self {
        Self(schema_id)
    }

    pub fn schema_id(&self) -> &json_schema::SchemaId {
        &self.0
    }

    pub fn context(&self) -> &str {
        let cap = json_schema::SCHEMA_URL_RE
            .captures(&self.0)
            .unwrap_or_else(|| panic!("Invalid schema $id: {}", self.0));

        cap.name("context").unwrap().as_str()
    }

    pub fn name(&self) -> &str {
        self.0.name()
    }

    pub fn subtype(&self, name: impl AsRef<str>) -> Self {
        Self(self.0.subtype(name))
    }

    pub fn parent<'a>(&'a self) -> Option<TypeId> {
        if let Some(p) = self.0.parent() {
            Some(TypeId::new(p))
        } else {
            None
        }
    }

    pub fn root<'a>(&'a self) -> Cow<'a, TypeId> {
        if let Some(p) = self.parent() {
            Cow::Owned(p)
        } else {
            Cow::Borrowed(self)
        }
    }

    pub fn join<'a, 'b>(&'a self, sep: &'b str) -> Cow<'a, str> {
        if let Some(p) = self.0.parent() {
            Cow::Owned(format!("{}{sep}{}", p.name(), self.0.name()))
        } else {
            Cow::Borrowed(self.0.name())
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

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

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

    pub fn metatype(&self) -> MetaType {
        match self {
            TypeDefinition::Struct(v) => v.metatype,
            TypeDefinition::Union(v) => v.metatype,
            TypeDefinition::Enum(v) => v.metatype,
            TypeDefinition::Map(v) => v.metatype,
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

    pub fn codegen_hints(&self) -> &CodegenHints {
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

    pub fn get_hint<V: serde::de::DeserializeOwned>(
        &self,
        lang: CodegenLanguage,
        key: CodegenHint,
    ) -> Option<V> {
        get_hint(self.codegen_hints(), lang, key)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeContext {
    Auth,
    Config,
    Data,
    Dataset,
    Engine,
    Flow,
    Ingest,
    Legacy,
    Resource,
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Struct {
    pub id: TypeId,
    pub metatype: MetaType,
    pub fields: IndexMap<String, Field>,
    pub generics: Vec<String>,
    pub description: String,
    pub from_string: bool,
    pub codegen_hints: CodegenHints,
    pub src: PathBuf,
}

impl Struct {
    pub fn get_hint<V: serde::de::DeserializeOwned>(
        &self,
        lang: CodegenLanguage,
        key: CodegenHint,
    ) -> Option<V> {
        get_hint(&self.codegen_hints, lang, key)
    }
}

#[derive(Debug, Clone)]
pub struct Union {
    pub id: TypeId,
    pub metatype: MetaType,
    pub variants: Vec<TypeId>,
    pub description: String,
    pub from_string: bool,
    pub codegen_hints: CodegenHints,
    pub src: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub id: TypeId,
    pub metatype: MetaType,
    pub variants: Vec<String>,
    pub description: String,
    pub src: PathBuf,
    pub codegen_hints: CodegenHints,
    pub format: Type,
}

#[derive(Debug, Clone)]
pub struct Map {
    pub id: TypeId,
    pub metatype: MetaType,
    pub description: String,
    pub value_type: Type,
    pub codegen_hints: CodegenHints,
    pub src: PathBuf,
}

impl Map {
    pub fn get_hint<V: serde::de::DeserializeOwned>(
        &self,
        lang: CodegenLanguage,
        key: CodegenHint,
    ) -> Option<V> {
        get_hint(&self.codegen_hints, lang, key)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

type CodegenHints = IndexMap<CodegenLanguage, IndexMap<CodegenHint, serde_json::Value>>;

fn get_hint<V: serde::de::DeserializeOwned>(
    hints: &CodegenHints,
    lang: CodegenLanguage,
    key: CodegenHint,
) -> Option<V> {
    let val = hints.get(&lang)?.get(&key)?;
    Some(serde_json::from_value(val.clone()).unwrap())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

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

    ByteSize,
    DateTime,
    Duration,
    Multicodec,
    Multihash,
    Path,
    Regex,
    Url,
    Did,

    // Meta-types
    TypeName,
    TypeUri,
    TypeRef,

    // Identity & references
    AccountId,
    AccountName,

    DatasetAlias,
    DatasetId,
    DatasetRef,

    ResourceId,
    ResourceName,

    // Composite
    Flatbuffers,
    Generic(String),
    Array(Array),
    Custom(TypeId),
    AnyJson,
}

#[derive(Debug, Clone, Copy)]
pub enum MetaType {
    Manifest,
    Resource,
    ResourceRef,
    ResourceHandle,
    ResourceCondition,
    EngineMessage,
    Fragment,
}

impl MetaType {
    pub fn from_metaschema(metaschema: Option<&json_schema::SchemaId>) -> Self {
        let id = metaschema
            .map(|s| s.as_str())
            .unwrap_or(json_schema::SchemaId::METASCHEMA_JSONSCHEMA);
        match id {
            json_schema::SchemaId::METASCHEMA_MANIFEST => Self::Manifest,
            json_schema::SchemaId::METASCHEMA_RESOURCE_INPUT => Self::Resource,
            json_schema::SchemaId::METASCHEMA_RESOURCE_REF => Self::ResourceRef,
            json_schema::SchemaId::METASCHEMA_RESOURCE_HANDLE => Self::ResourceHandle,
            json_schema::SchemaId::METASCHEMA_RESOURCE_CONDITION => Self::ResourceCondition,
            json_schema::SchemaId::METASCHEMA_ENGINE_MESSAGE => Self::EngineMessage,
            json_schema::SchemaId::METASCHEMA_JSONSCHEMA => Self::Fragment,
            _ => panic!("Unrecognized meta-schema: {id}"),
        }
    }
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
    pub constant: Option<serde_json::Value>,
    pub explicit_tag: Option<u32>,
    pub deprecated: bool,
    pub codegen_hints: CodegenHints,
}

impl Field {
    pub fn get_hint<V: serde::de::DeserializeOwned>(
        &self,
        lang: CodegenLanguage,
        key: CodegenHint,
    ) -> Option<V> {
        get_hint(&self.codegen_hints, lang, key)
    }
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
        // Skip metaschemas
        let id = schema.id.as_ref().expect("Named type missing an $id");

        if id.starts_with(json_schema::SchemaId::METASCHEMA_BASE_URL) {
            continue;
        }

        let metatype = MetaType::from_metaschema(schema.schema.as_ref());

        // Validate `unevaluatedProperties: false` is specified only for root schemas
        match (
            metatype,
            schema.unevaluated_properties.take() == Some(false),
        ) {
            (MetaType::Manifest | MetaType::Resource | MetaType::EngineMessage, true) => (),
            (
                MetaType::Fragment
                | MetaType::ResourceRef
                | MetaType::ResourceHandle
                | MetaType::ResourceCondition,
                false,
            ) => (),
            (_, false) => {
                panic!("Top-level schemas should define `unevaluatedProperties: false`: {id}")
            }
            (_, true) => {
                panic!(
                    "The `unevaluatedProperties: false` is only allowed on top-level schemas: {id}"
                )
            }
        }

        // Validate `$schema` on resources
        if matches!(metatype, MetaType::Manifest | MetaType::Resource) {
            if let Some(schema_prop) = schema.properties.as_ref().and_then(|p| p.get("$schema")) {
                assert_eq!(schema_prop.r#type, Some(json_schema::Type::String));
                assert_eq!(schema_prop.format, Some(json_schema::Format::TypeUri));
                if let Some(cid) = &schema_prop.r#const {
                    assert_eq!(id.as_str(), cid, "$schema.const must match $id: {id}");
                }
            } else {
                panic!("Resource schemas should define `$schema` property: {id}");
            }
        }

        let root_id = TypeId::new(id.clone());

        let src = schema.src.take().expect("Schema without source path");

        // Extract all $defs into top-level types
        for (dname, dsch) in schema.defs.take().unwrap_or_default() {
            let def_id = root_id.subtype(dname);

            let typ = parse_type_definition(
                def_id.clone(),
                dsch,
                src.clone(),
                format!("{}.$defs.{}", root_id.name(), def_id.name()),
            );
            types.insert(typ.id().clone(), typ);
        }

        let typ =
            parse_type_definition(root_id.clone(), schema, src, format!("{}", root_id.name()));
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
                unevaluated_properties: obj.unevaluated_properties,
                one_of: obj.one_of,
                all_of: obj.all_of,
                r#enum: obj.r#enum,
                items: obj.items,
                r#ref: obj.r#ref,
                r#const: obj.r#const,
                canonical_type: schema.canonical_type,
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
        id: _,
        schema: metaschema,
        defs: None,
        r#type: Some(_),
        required: Some(required),
        properties: Some(properties),
        pattern_properties: None,
        additional_properties: None,
        unevaluated_properties: None,
        one_of: None,
        all_of: None,
        r#enum: None,
        items: None,
        r#ref: None,
        r#const: None,
        canonical_type: _,
        format: None,
        default: None,
        description: Some(description),
        tag: None,
        codegen,
        deprecated: None,
        examples: _,
        src: None,
    } = schema
    else {
        panic!("Invalid struct schema: {ctx}: {}", schema.display())
    };

    let metatype = MetaType::from_metaschema(metaschema.as_ref());

    let mut fields = IndexMap::new();
    let mut generics = Vec::new();

    for (pname, mut psch) in properties {
        let fconst = psch.r#const.take();

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
            constant: fconst,
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
        metatype,
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
        id: _,
        schema: metaschema,
        defs: None,
        r#type: None,
        required: None,
        properties: None,
        pattern_properties: None,
        additional_properties: None,
        unevaluated_properties: None,
        one_of: Some(one_of),
        all_of: None,
        r#enum: None,
        items: None,
        r#ref: None,
        r#const: None,
        canonical_type: _,
        format,
        default: None,
        description: Some(description),
        tag: None,
        codegen: None,
        deprecated: None,
        examples: _,
        src: None,
    } = schema
    else {
        panic!("Invalid union schema: {ctx}: {}", schema.display())
    };

    let from_string = match format {
        Some(json_schema::Format::UnionOrString) => true,
        _ => false,
    };

    let mut variants = Vec::new();
    for (i, schema) in one_of.into_iter().enumerate() {
        let var = parse_type_union_variant(&id, schema, format!("{ctx}.$oneOf.[{i}]"));
        variants.push(var);
    }

    assert!(!variants.is_empty(), "Union must have at least one variant");

    Union {
        id,
        metatype: MetaType::from_metaschema(metaschema.as_ref()),
        variants,
        description,
        from_string,
        codegen_hints: Default::default(),
        src,
    }
}

fn parse_type_union_variant(parent: &TypeId, schema: json_schema::Schema, ctx: String) -> TypeId {
    let json_schema::Schema {
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
        all_of: Some(mut all_of),
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
    } = schema
    else {
        panic!("Invalid union variant schema: {ctx}: {}", schema.display())
    };

    assert_eq!(
        all_of.len(),
        2,
        "Union variants should use `allOf` with `kind` constant and a `$ref`"
    );

    let type_id = parse_ref(
        all_of.pop().unwrap(),
        parent,
        false,
        format!("{ctx}.$allOf.[1]"),
    );

    assert_eq!(
        all_of[0].to_value(),
        serde_json::json!({
          "properties": {
            "kind": {
              "type": "string",
              "const": type_id.name(),
            }
          },
          "required": [
            "kind"
          ]
        }),
        "Invalid `kind` tag schema on union variant"
    );

    type_id
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_type_enum(id: TypeId, schema: json_schema::Schema, src: PathBuf, ctx: String) -> Enum {
    let json_schema::Schema {
        id: _,
        schema: metaschema,
        defs: None,
        r#type: Some(typ),
        required: None,
        properties: None,
        pattern_properties: None,
        additional_properties: None,
        unevaluated_properties: None,
        one_of: None,
        all_of: None,
        r#enum: Some(enums),
        items: None,
        r#ref: None,
        r#const: None,
        canonical_type: None,
        format,
        default: None,
        description: Some(description),
        tag: None,
        codegen,
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
        metatype: MetaType::from_metaschema(metaschema.as_ref()),
        variants,
        description,
        format,
        codegen_hints: codegen.unwrap_or_default(),
        src,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_type_map(id: TypeId, schema: json_schema::Schema, src: PathBuf, ctx: String) -> Map {
    assert_eq!(schema.r#type, Some(json_schema::Type::Object));

    let json_schema::Schema {
        id: _,
        schema: metaschema,
        defs: None,
        r#type: Some(_),
        required: None,
        properties: None,
        pattern_properties: Some(pattern_properties),
        additional_properties: None,
        unevaluated_properties: None,
        one_of: None,
        all_of: None,
        r#enum: None,
        items: None,
        r#ref: None,
        r#const: None,
        canonical_type: _,
        format: None,
        default: None,
        description: Some(description),
        tag: None,
        codegen,
        deprecated: None,
        examples: _,
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
        metatype: MetaType::from_metaschema(metaschema.as_ref()),
        description: description.clone(),
        value_type,
        codegen_hints: codegen.unwrap_or_default(),
        src,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn parse_type(schema: json_schema::Schema, root: &TypeId, ctx: String) -> Type {
    match &schema {
        json_schema::Schema { r#ref: Some(_), .. } => {
            Type::Custom(parse_ref(schema, root, true, ctx))
        }
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
        unevaluated_properties: None,
        one_of: None,
        all_of: None,
        r#enum: None,
        items: Some(items),
        r#ref: None,
        r#const: None,
        canonical_type: None,
        format: None,
        default: None,
        description: None,
        tag: None,
        codegen: None,
        deprecated: None,
        examples: _,
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
        unevaluated_properties: None,
        one_of: None,
        all_of: None,
        r#enum: None,
        items: None,
        r#ref: None,
        r#const: _,
        canonical_type: None,
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

        (json_schema::Type::String, Some(json_schema::Format::ByteSize)) => Type::ByteSize,
        (json_schema::Type::String, Some(json_schema::Format::DateTime)) => Type::DateTime,
        (json_schema::Type::String, Some(json_schema::Format::Duration)) => Type::Duration,
        (json_schema::Type::String, Some(json_schema::Format::Email)) => Type::String,
        (json_schema::Type::String, Some(json_schema::Format::Multicodec)) => Type::Multicodec,
        (json_schema::Type::String, Some(json_schema::Format::Multihash)) => Type::Multihash,
        (json_schema::Type::String, Some(json_schema::Format::Path)) => Type::Path,
        (json_schema::Type::String, Some(json_schema::Format::Regex)) => Type::Regex,
        (json_schema::Type::String, Some(json_schema::Format::Uri)) => Type::Url,
        (json_schema::Type::String, Some(json_schema::Format::Did)) => Type::Did,

        (json_schema::Type::String, Some(json_schema::Format::AccountId)) => Type::AccountId,
        (json_schema::Type::String, Some(json_schema::Format::AccountName)) => Type::AccountName,

        (json_schema::Type::String, Some(json_schema::Format::DatasetAlias)) => Type::DatasetAlias,
        (json_schema::Type::String, Some(json_schema::Format::DatasetId)) => Type::DatasetId,
        (json_schema::Type::String, Some(json_schema::Format::DatasetRef)) => Type::DatasetRef,

        (json_schema::Type::String, Some(json_schema::Format::ResourceId)) => Type::ResourceId,
        (json_schema::Type::String, Some(json_schema::Format::ResourceName)) => Type::ResourceName,
        (json_schema::Type::String, Some(json_schema::Format::TypeUri)) => Type::TypeUri,
        (json_schema::Type::String, Some(json_schema::Format::TypeName)) => Type::TypeName,
        (json_schema::Type::String, Some(json_schema::Format::TypeRef)) => Type::TypeRef,

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

fn parse_ref(
    schema: json_schema::Schema,
    parent: &TypeId,
    is_new_validation_scope: bool,
    ctx: String,
) -> TypeId {
    let json_schema::Schema {
        id: None,
        schema: None,
        defs: None,
        r#type: None,
        required: None,
        properties: None,
        pattern_properties: None,
        additional_properties: None,
        unevaluated_properties,
        one_of: None,
        all_of: None,
        r#enum: None,
        items: None,
        r#ref: Some(reff),
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
    } = schema
    else {
        panic!("Invalid $ref schema: {ctx}: {}", schema.display())
    };

    if is_new_validation_scope != (unevaluated_properties == Some(false)) {
        panic!(
            "Property and array items references must define `unevaluatedProperties: false` schema: {ctx}"
        )
    }

    ref_to_type_id(&reff, parent, format!("{ctx}.$ref"))
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn ref_to_type_id(reff: &str, parent: &TypeId, ctx: String) -> TypeId {
    if reff.starts_with("http:") || reff.starts_with("https:") {
        TypeId::new(json_schema::SchemaId::new(reff))
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
