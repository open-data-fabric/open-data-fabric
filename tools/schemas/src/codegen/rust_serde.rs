use std::collections::BTreeMap;

use crate::{
    codegen::rust_common::format_ident,
    json_schema::{CodegenHint, CodegenLanguage},
    model,
};
use convert_case::{Case, Casing};

const PREAMBLE: &str = indoc::indoc!(
    r#"
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // WARNING: This file is auto-generated from Open Data Fabric Schemas
    // See: http://opendatafabric.org/
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    #![allow(clippy::all)]
    #![allow(clippy::pedantic)]
    #![allow(unused_variables)]

    use std::path::PathBuf;

    use ::serde::{Deserialize, Deserializer, Serialize, Serializer};
    use chrono::{DateTime, Utc};
    use multiformats::*;
    use setty::types::{ByteSize, DurationString};

    use super::formats::*;
    use crate::auth::{AccountID, AccountName};
    use crate::dataset::{DatasetAlias, DatasetID, DatasetRef};
    use crate::dtos;
    use crate::errors::ValidationError;
    use crate::resource::{ResourceID, ResourceName, ResourceTypeRef, ResourceTypeUri};

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    pub trait IntoDto {
        type Dto;
        fn into_dto(self) -> Result<Self::Dto, ValidationError>;
    }

    impl IntoDto for ::serde::de::IgnoredAny {
        type Dto = Self;
        fn into_dto(self) -> Result<Self::Dto, ValidationError> { Ok(self) }
    }

    impl IntoDto for ::serde_json::Value {
        type Dto = Self;
        fn into_dto(self) -> Result<Self::Dto, ValidationError> { Ok(self) }
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    macro_rules! implement_serde_as {
        ($dto:ty, $proxy:ty) => {
            impl ::serde_with::SerializeAs<$dto> for $proxy {
                fn serialize_as<S>(value: &$dto, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    // TODO: PERF: Avoid cloning on serialize
                    let value: $proxy = value.clone().into();
                    value.serialize(serializer)
                }
            }

            impl<'de> serde_with::DeserializeAs<'de, $dto> for $proxy {
                fn deserialize_as<D>(deserializer: D) -> Result<$dto, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    use ::serde::de::Error;
                    let proxy = <$proxy>::deserialize(deserializer)?;
                    proxy.try_into().map_err(D::Error::custom)
                }
            }
        };
    }
    "#
);

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn render(model: model::Model, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    writeln!(w, "{}", PREAMBLE)?;

    // Group by `context` and sort by names
    let types_by_context: BTreeMap<&str, BTreeMap<String, &model::TypeDefinition>> = model
        .types
        .values()
        .filter(|t| !matches!(t.metatype(), model::MetaType::Resource))
        .fold(BTreeMap::new(), |mut map, t| {
            map.entry(t.id().context())
                .or_insert_with(BTreeMap::new)
                .insert(t.id().join("").into(), t);
            map
        });

    for (context, types) in &types_by_context {
        writeln!(
            w,
            "////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////"
        )?;
        writeln!(w, "// {context}")?;
        writeln!(
            w,
            "////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////\n"
        )?;

        writeln!(w, "pub mod {context} {{")?;
        writeln!(w, "#[allow(unused_imports)]")?;
        writeln!(w, "use super::*;\n")?;

        for typ in types.values() {
            writeln!(w, "// Schema: {}", typ.id().schema_id())?;

            match &typ {
                model::TypeDefinition::Struct(t) => render_struct(&model, t, w)?,
                model::TypeDefinition::Union(t) => render_union(t, w)?,
                model::TypeDefinition::Enum(t) => render_enum(t, w)?,
                model::TypeDefinition::Map(t) => render_map(&model, t, w)?,
            }

            writeln!(w)?;

            // TODO: Support #[serde_as(as = "X")] for generic types
            if !matches!(typ, model::TypeDefinition::Struct(t) if !t.generics.is_empty()) {
                let name = typ.id().join("");
                writeln!(w, "implement_serde_as!(dtos::{context}::{name}, {name});\n")?;
            }
        }

        writeln!(w, "}}\n")?;
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_struct(
    model: &model::Model,
    typ: &model::Struct,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    let context = typ.id.context();
    let name = typ.id.join("");
    let generics = format!("<{}>", typ.generics.join(", "));
    let is_external = typ
        .codegen_hints
        .get(&CodegenLanguage::Rust)
        .and_then(|h| h.get(&CodegenHint::DtoType))
        .is_some();

    writeln!(w, "#[derive(Debug, Serialize, Deserialize)]")?;
    writeln!(w, "#[serde(deny_unknown_fields)]")?;
    writeln!(w, "#[serde(rename_all = \"camelCase\")]")?;
    writeln!(w, "pub struct {name}{generics} {{")?;

    for field in typ.fields.values() {
        render_field(model, field, w)?;
    }

    writeln!(w, "}}")?;

    writeln!(w)?;

    let generics_dto = format!(
        "<{}>",
        typ.generics
            .iter()
            .map(|v| format!("{v}::Dto"))
            .collect::<Vec<_>>()
            .join(", ")
    );

    writeln!(w, "impl{generics} IntoDto for {name}{generics}")?;
    if !typ.generics.is_empty() {
        writeln!(w, "where")?;
        for generic in &typ.generics {
            writeln!(w, "{generic}: IntoDto,")?;
            writeln!(
                w,
                "<{generic} as IntoDto>::Dto: TryFrom<{generic}>, ValidationError: From<<<{generic} as IntoDto>::Dto as TryFrom<{generic}>>::Error>,"
            )?;
        }
    }
    writeln!(w, "{{")?;
    writeln!(w, "type Dto = dtos::{context}::{name}{generics_dto};")?;
    writeln!(
        w,
        "fn into_dto(self) -> Result<Self::Dto, ValidationError> {{ self.try_into() }}"
    )?;
    writeln!(w, "}}")?;

    if typ.from_string {
        writeln!(w)?;

        writeln!(
            w,
            "impl From<dtos::{context}::{name}> for StructOrString<{name}> {{ fn from(v: dtos::{context}::{name}) -> Self {{ Self(v.into()) }} }}"
        )?;

        writeln!(
            w,
            "impl TryFrom<StructOrString<{name}>> for dtos::{context}::{name} {{ type Error = ValidationError; fn try_from(v: StructOrString<{name}>) -> Result<Self, ValidationError> {{ v.0.try_into() }} }}"
        )?;
    }

    if is_external {
        // External types should implement serde proxy conversions manually
        return Ok(());
    }

    writeln!(w)?;

    let generics_from: Vec<_> = typ.generics.iter().map(|v| format!("{v}From")).collect();
    let generics_to: Vec<_> = typ.generics.iter().map(|v| format!("{v}To")).collect();
    let generics_from_to: Vec<_> = generics_from
        .iter()
        .chain(generics_to.iter())
        .cloned()
        .collect();
    let whereas: Vec<_> = typ
        .generics
        .iter()
        .map(|v| format!("{v}To: From<{v}From>"))
        .collect();

    let generics_from = format!("<{}>", generics_from.join(", "));
    let generics_to = format!("<{}>", generics_to.join(", "));
    let generics_from_to = format!("<{}>", generics_from_to.join(", "));
    let whereas = if typ.generics.is_empty() {
        String::new()
    } else {
        format!("where {}", whereas.join(", "))
    };

    writeln!(
        w,
        "impl{generics_from_to} From<dtos::{context}::{name}{generics_from}> for {name}{generics_to} {whereas} {{"
    )?;
    writeln!(
        w,
        "fn from(v: dtos::{context}::{name}{generics_from}) -> Self {{"
    )?;
    writeln!(w, "Self {{")?;
    for field in typ.fields.values() {
        render_field_into(field, w)?;
    }
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;

    writeln!(w)?;

    let whereas: Vec<_> = typ
        .generics
        .iter()
        .map(|v| format!("{v}To: TryFrom<{v}From>, ValidationError: From<<{v}To as TryFrom<{v}From>>::Error>,"))
        .collect();

    let whereas = if typ.generics.is_empty() {
        String::new()
    } else {
        format!("where {}", whereas.join(", "))
    };

    writeln!(
        w,
        "impl{generics_from_to} TryFrom<{name}{generics_from}> for dtos::{context}::{name}{generics_to} {whereas} {{"
    )?;
    writeln!(w, "type Error = ValidationError;")?;
    writeln!(
        w,
        "fn try_from(v: {name}{generics_from}) -> Result<Self, ValidationError> {{"
    )?;
    writeln!(w, "Ok(Self {{")?;
    for field in typ.fields.values() {
        render_field_try_into(field, w)?;
    }
    writeln!(w, "}})")?;
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;

    Ok(())
}

fn render_field(
    model: &model::Model,
    field: &model::Field,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    let ident = format_ident(&field.name);

    if ident != field.name && !ident.starts_with("r#") {
        writeln!(w, "#[serde(rename = \"{}\")]", field.name)?;
    }

    let container = field
        .codegen_hints
        .get(&CodegenLanguage::Rust)
        .and_then(|m| m.get(&CodegenHint::Container));

    if field.optional {
        if let model::Type::Generic(_) = &field.typ {
            // Workaround for serde bug
            // See: https://github.com/serde-rs/serde/issues/2759
            writeln!(w, "#[serde(default = \"Default::default\")]")?;
        } else {
            writeln!(w, "#[serde(default)]")?;
        }
        writeln!(w, "#[serde(skip_serializing_if = \"Option::is_none\")]")?;
    }

    match &field.typ {
        model::Type::DateTime => {
            if field.optional {
                writeln!(w, "#[serde(with = \"datetime_rfc3339_opt\")]")?;
            } else {
                writeln!(w, "#[serde(with = \"datetime_rfc3339\")]")?;
            }
        }
        model::Type::Flatbuffers => {
            if field.optional {
                writeln!(w, "#[serde(with = \"base64_opt\")]")?;
            } else {
                writeln!(w, "#[serde(with = \"base64\")]")?;
            }
        }
        _ => (),
    };

    let mut typ = format_type(model, &field.typ);
    if let Some(container) = container {
        typ = format!("{container}<{typ}>");
    }
    if field.optional {
        typ = format!("Option<{typ}>");
    }

    writeln!(w, "pub {ident}: {typ},")?;
    Ok(())
}

fn needs_conversion(typ: &model::Type) -> bool {
    match typ {
        model::Type::Generic(_) | model::Type::Custom(_) => true,
        model::Type::Array(t) => needs_conversion(&t.item_type),
        _ => false,
    }
}

fn render_field_into(
    field: &model::Field,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    let container = field
        .codegen_hints
        .get(&CodegenLanguage::Rust)
        .and_then(|m| m.get(&CodegenHint::Container));

    let fname = format_ident(&field.name);

    let convert = if !needs_conversion(&field.typ) {
        format!("v.{fname}")
    } else if let Some(container) = container {
        format!(
            "{container}::new({})",
            format_into(&field.typ, &format!("(*v.{fname})"))
        )
    } else if !field.optional {
        format_into(&field.typ, &format!("v.{fname}"))
    } else {
        format!("v.{fname}.map(|v| {{ {} }})", format_into(&field.typ, "v"))
    };

    writeln!(w, "{fname}: {convert},",)?;

    Ok(())
}

fn format_into(typ: &model::Type, ident: &str) -> String {
    match typ {
        model::Type::Generic(_) | model::Type::Custom(_) => format!("{ident}.into()"),
        model::Type::Array(_) => format!("{ident}.into_iter().map(Into::into).collect()"),
        _ => unreachable!(),
    }
}

fn render_field_try_into(
    field: &model::Field,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    let container = field
        .codegen_hints
        .get(&CodegenLanguage::Rust)
        .and_then(|m| m.get(&CodegenHint::Container));

    let fname = format_ident(&field.name);

    let convert = if !needs_conversion(&field.typ) {
        format!("v.{fname}")
    } else if let Some(container) = container {
        format!(
            "{container}::new({}?)",
            format_try_into(&field.typ, &format!("*v.{fname}"))
        )
    } else if !field.optional {
        format!("{}?", format_try_into(&field.typ, &format!("v.{fname}")))
    } else {
        format!(
            "v.{fname}.map(|v| {{ {} }}).transpose()?",
            format_try_into(&field.typ, "v")
        )
    };

    writeln!(w, "{fname}: {convert},",)?;

    Ok(())
}

fn format_try_into(typ: &model::Type, ident: &str) -> String {
    match typ {
        model::Type::Generic(t) => format!("{t}To::try_from({ident})"),
        model::Type::Custom(t) => {
            format!("dtos::{}::{}::try_from({ident})", t.context(), t.join(""))
        }
        model::Type::Array(t) => {
            format!(
                "{ident}.into_iter().map(|i| {}).collect::<Result<_,_>>()",
                format_try_into(&t.item_type, "i")
            )
        }
        _ => unreachable!(),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_union(typ: &model::Union, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let context = typ.id.context();
    let name = typ.id.join("");

    writeln!(w, "#[derive(Debug, Serialize, Deserialize)]")?;
    writeln!(w, "#[serde(deny_unknown_fields)]")?;
    writeln!(w, "#[serde(tag = \"kind\")]")?;
    writeln!(w, "pub enum {name} {{")?;

    for variant in &typ.variants {
        let var_ctx = variant.context();
        let varname = variant.name();
        let typename = variant.join("");

        // Allow lowercase and camelCase names
        render_aliases(varname, w)?;
        writeln!(w, "{varname}({var_ctx}::{typename}),")?;
    }

    writeln!(w, "}}")?;

    if typ.from_string {
        writeln!(w)?;

        writeln!(
            w,
            "impl From<dtos::{context}::{name}> for UnionOrString<{name}> {{ fn from(v: dtos::{context}::{name}) -> Self {{ Self(v.into()) }} }}"
        )?;

        writeln!(
            w,
            "impl TryFrom<UnionOrString<{name}>> for dtos::{context}::{name} {{ type Error = ValidationError; fn try_from(v: UnionOrString<{name}>) -> Result<Self, Self::Error> {{ v.0.try_into() }} }}"
        )?;
    }

    writeln!(w)?;

    writeln!(w, "impl IntoDto for {name} {{")?;
    writeln!(w, "type Dto = dtos::{context}::{name};")?;
    writeln!(
        w,
        "fn into_dto(self) -> Result<Self::Dto, ValidationError> {{ self.try_into() }}"
    )?;
    writeln!(w, "}}")?;

    writeln!(w)?;

    writeln!(w, "impl From<dtos::{context}::{name}> for {name} {{")?;
    writeln!(w, "fn from(v: dtos::{context}::{name}) -> Self {{")?;
    writeln!(w, "match v {{")?;
    for variant in &typ.variants {
        let varname = variant.name();
        writeln!(
            w,
            "dtos::{context}::{name}::{varname}(v) => Self::{varname}(v.into()),"
        )?;
    }
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;

    writeln!(w)?;

    writeln!(w, "impl TryFrom<{name}> for dtos::{context}::{name} {{")?;
    writeln!(w, "type Error = ValidationError;")?;
    writeln!(w, "fn try_from(v: {name}) -> Result<Self, Self::Error> {{")?;
    writeln!(w, "match v {{")?;
    for variant in &typ.variants {
        let varname = variant.name();
        writeln!(
            w,
            "{name}::{varname}(v) => Ok(Self::{varname}(v.try_into()?)),"
        )?;
    }
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_enum(typ: &model::Enum, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let context = typ.id.context();
    let name = typ.id.join("");

    writeln!(w, "#[derive(Debug, Serialize, Deserialize)]")?;
    writeln!(w, "#[serde(deny_unknown_fields)]")?;
    writeln!(w, "pub enum {name} {{")?;
    {
        for variant in &typ.variants {
            render_aliases(&variant, w)?;
            writeln!(w, "{variant},")?;
        }
    }
    writeln!(w, "}}")?;

    writeln!(w)?;

    writeln!(w, "impl IntoDto for {name} {{")?;
    writeln!(w, "type Dto = dtos::{context}::{name};")?;
    writeln!(
        w,
        "fn into_dto(self) -> Result<Self::Dto, ValidationError> {{ self.try_into() }}"
    )?;
    writeln!(w, "}}")?;

    writeln!(w)?;

    writeln!(w, "impl From<dtos::{context}::{name}> for {name} {{")?;
    writeln!(w, "fn from(v: dtos::{context}::{name}) -> Self {{")?;
    writeln!(w, "match v {{")?;
    for variant in &typ.variants {
        writeln!(w, "dtos::{context}::{name}::{variant} => Self::{variant},")?;
    }
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;

    writeln!(w)?;

    writeln!(w, "impl TryFrom<{name}> for dtos::{context}::{name} {{")?;
    writeln!(w, "type Error = ValidationError;")?;
    writeln!(w, "fn try_from(v: {name}) -> Result<Self, Self::Error> {{")?;
    writeln!(w, "match v {{")?;
    for variant in &typ.variants {
        writeln!(w, "{name}::{variant} => Ok(Self::{variant}),")?;
    }
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_map(
    model: &model::Model,
    typ: &model::Map,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    let context = typ.id.context();
    let name = typ.id.join("");
    let value_type = format_type(model, &typ.value_type);

    writeln!(w, "#[derive(Debug, Serialize, Deserialize)]")?;
    writeln!(w, "pub struct {name} {{")?;
    writeln!(w, "#[serde(flatten)]")?;
    match typ.value_type {
        model::Type::AnyJson => writeln!(w, "#[serde(with = \"map_value_limited_precision\")]")?,
        _ => (),
    }
    writeln!(
        w,
        "pub entries: std::collections::BTreeMap<String, {value_type}>,"
    )?;
    writeln!(w, "}}")?;

    writeln!(w)?;

    let entries_into = match typ.value_type {
        model::Type::Custom(_) => "v.entries.into_iter().map(|(k, v)| (k, v.into())).collect()",
        _ => "v.entries",
    };

    let entries_try_into = match typ.value_type {
        model::Type::Custom(_) => {
            "v.entries.into_iter().map(|(k, v)| -> Result<_, ValidationError> { Ok((k, v.try_into()?)) }).collect::<Result<_,_>>()?"
        }
        _ => "v.entries",
    };

    writeln!(
        w,
        r#"
        impl IntoDto for {name} {{
            type Dto = dtos::{context}::{name};
            fn into_dto(self) -> Result<Self::Dto, ValidationError> {{
                self.try_into()
            }}
        }}

        impl From<dtos::{context}::{name}> for {name} {{
            fn from(v: dtos::{context}::{name}) -> Self {{
                Self {{
                    entries: {entries_into},
                }}
            }}
        }}

        impl TryFrom<{name}> for dtos::{context}::{name} {{
            type Error = ValidationError;
            fn try_from(v: {name}) -> Result<Self, Self::Error> {{
                Ok(Self {{
                    entries: {entries_try_into},
                }})
            }}
        }}
        "#
    )?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Allow lowercase and camelCase names for enums
fn render_aliases(name: &str, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let aliases: Vec<_> =
        std::collections::BTreeSet::from([name.to_lowercase(), name.to_case(Case::Camel)])
            .into_iter()
            .collect();

    write!(w, "#[serde(")?;
    for (i, alias) in aliases.into_iter().enumerate() {
        if i != 0 {
            write!(w, ", ")?;
        }
        write!(w, "alias = \"{alias}\"")?;
    }
    writeln!(w, ")]")?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn format_type(model: &model::Model, typ: &model::Type) -> String {
    match typ {
        model::Type::Boolean => format!("bool"),
        model::Type::Int8 => format!("i8"),
        model::Type::Int16 => format!("i16"),
        model::Type::Int32 => format!("i32"),
        model::Type::Int64 => format!("i64"),
        model::Type::UInt8 => format!("u8"),
        model::Type::UInt16 => format!("u16"),
        model::Type::UInt32 => format!("u32"),
        model::Type::UInt64 => format!("u64"),
        model::Type::String => format!("String"),
        model::Type::ByteSize => format!("ByteSize"),
        model::Type::DateTime => format!("DateTime<Utc>"),
        model::Type::Duration => format!("DurationString"),
        // model::Type::Multicodec => format!("Multicodec"),
        model::Type::Multicodec => format!("String"),
        model::Type::Multihash => format!("Multihash"),
        model::Type::Path => format!("PathBuf"),
        model::Type::Regex => format!("String"),
        model::Type::Url => format!("String"),

        model::Type::DatasetAlias => format!("DatasetAlias"),
        model::Type::DatasetId => format!("DatasetID"),
        model::Type::DatasetRef => format!("DatasetRef"),
        model::Type::AccountId => format!("AccountID"),
        model::Type::AccountName => format!("AccountName"),
        model::Type::ResourceId => format!("ResourceID"),
        model::Type::ResourceName => format!("ResourceName"),
        model::Type::ResourceTypeUri => format!("ResourceTypeUri"),
        model::Type::ResourceTypeName => format!("ResourceTypeName"),
        model::Type::ResourceTypeRef => format!("ResourceTypeRef"),

        model::Type::Flatbuffers => format!("Vec<u8>"),
        model::Type::Generic(t) => t.clone(),
        model::Type::Array(t) => format!("Vec<{}>", format_type(model, &t.item_type)),
        model::Type::Custom(id) => {
            let ctx = id.context();
            let name = id.join("");

            match &model.types[id] {
                model::TypeDefinition::Struct(t) if t.from_string => {
                    format!("StructOrString<{ctx}::{name}>")
                }
                model::TypeDefinition::Union(t) if t.from_string => {
                    format!("UnionOrString<{ctx}::{name}>")
                }
                _ => format!("{ctx}::{name}"),
            }
        }
        model::Type::AnyJson => format!("serde_json::Value"),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
