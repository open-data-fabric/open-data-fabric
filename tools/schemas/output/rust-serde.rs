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
use crate::resource::{ResourceID, ResourceName, ResourceTypeRef, ResourceTypeUri};

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait IntoDto {
    type Dto;
    fn into_dto(self) -> Self::Dto;
}

impl IntoDto for ::serde::de::IgnoredAny {
    type Dto = Self;
    fn into_dto(self) -> Self::Dto {
        self
    }
}

impl IntoDto for ::serde_json::Value {
    type Dto = Self;
    fn into_dto(self) -> Self::Dto {
        self
    }
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
                <$proxy>::deserialize(deserializer).map(Into::into)
            }
        }
    };
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// auth
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod auth {
    #[allow(unused_imports)]
    use super::*;

    // Schema: https://opendatafabric.org/schemas/auth/v1alpha1/AccountRef
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct AccountRef {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<AccountID>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<AccountName>,
    }

    impl IntoDto for AccountRef {
        type Dto = dtos::auth::AccountRef;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::auth::AccountRef> for StructOrString<AccountRef> {
        fn from(v: dtos::auth::AccountRef) -> Self {
            Self(v.into())
        }
    }
    impl From<StructOrString<AccountRef>> for dtos::auth::AccountRef {
        fn from(v: StructOrString<AccountRef>) -> Self {
            v.0.into()
        }
    }

    implement_serde_as!(dtos::auth::AccountRef, AccountRef);

    // Schema: https://opendatafabric.org/schemas/auth/v1alpha1/AccountSpec
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct AccountSpec {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account_type: Option<auth::AccountType>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        pub email: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub avatar_url: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub password: Option<StructOrString<config::Secret>>,
    }

    impl IntoDto for AccountSpec {
        type Dto = dtos::auth::AccountSpec;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::auth::AccountSpec> for AccountSpec {
        fn from(v: dtos::auth::AccountSpec) -> Self {
            Self {
                account_type: v.account_type.map(|v| v.into()),
                display_name: v.display_name.map(|v| v),
                email: v.email,
                avatar_url: v.avatar_url.map(|v| v),
                password: v.password.map(|v| v.into()),
            }
        }
    }

    impl From<AccountSpec> for dtos::auth::AccountSpec {
        fn from(v: AccountSpec) -> Self {
            Self {
                account_type: v.account_type.map(|v| v.into()),
                display_name: v.display_name.map(|v| v),
                email: v.email,
                avatar_url: v.avatar_url.map(|v| v),
                password: v.password.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::auth::AccountSpec, AccountSpec);

    // Schema: https://opendatafabric.org/schemas/auth/v1alpha1/AccountType
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub enum AccountType {
        #[serde(alias = "user")]
        User,
        #[serde(alias = "organization")]
        Organization,
    }

    impl IntoDto for AccountType {
        type Dto = dtos::auth::AccountType;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::auth::AccountType> for AccountType {
        fn from(v: dtos::auth::AccountType) -> Self {
            match v {
                dtos::auth::AccountType::User => Self::User,
                dtos::auth::AccountType::Organization => Self::Organization,
            }
        }
    }

    impl From<AccountType> for dtos::auth::AccountType {
        fn from(v: AccountType) -> Self {
            match v {
                AccountType::User => Self::User,
                AccountType::Organization => Self::Organization,
            }
        }
    }

    implement_serde_as!(dtos::auth::AccountType, AccountType);

    // Schema: https://opendatafabric.org/schemas/auth/v1alpha1/Attribute
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct Attribute {
        pub object: StructOrString<resource::ResourceRef>,
        pub name: String,
        pub value: serde_json::Value,
    }

    impl IntoDto for Attribute {
        type Dto = dtos::auth::Attribute;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::auth::Attribute> for Attribute {
        fn from(v: dtos::auth::Attribute) -> Self {
            Self {
                object: v.object.into(),
                name: v.name,
                value: v.value,
            }
        }
    }

    impl From<Attribute> for dtos::auth::Attribute {
        fn from(v: Attribute) -> Self {
            Self {
                object: v.object.into(),
                name: v.name,
                value: v.value,
            }
        }
    }

    implement_serde_as!(dtos::auth::Attribute, Attribute);

    // Schema: https://opendatafabric.org/schemas/auth/v1alpha1/Relation
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct Relation {
        pub subject: StructOrString<resource::ResourceRef>,
        pub relation: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<serde_json::Value>,
        pub object: StructOrString<resource::ResourceRef>,
    }

    impl IntoDto for Relation {
        type Dto = dtos::auth::Relation;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::auth::Relation> for Relation {
        fn from(v: dtos::auth::Relation) -> Self {
            Self {
                subject: v.subject.into(),
                relation: v.relation,
                value: v.value.map(|v| v),
                object: v.object.into(),
            }
        }
    }

    impl From<Relation> for dtos::auth::Relation {
        fn from(v: Relation) -> Self {
            Self {
                subject: v.subject.into(),
                relation: v.relation,
                value: v.value.map(|v| v),
                object: v.object.into(),
            }
        }
    }

    implement_serde_as!(dtos::auth::Relation, Relation);

    // Schema: https://opendatafabric.org/schemas/auth/v1alpha1/RelationsSpec
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct RelationsSpec {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub relations: Option<Vec<auth::Relation>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attributes: Option<Vec<auth::Attribute>>,
    }

    impl IntoDto for RelationsSpec {
        type Dto = dtos::auth::RelationsSpec;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::auth::RelationsSpec> for RelationsSpec {
        fn from(v: dtos::auth::RelationsSpec) -> Self {
            Self {
                relations: v.relations.map(|v| v.into_iter().map(Into::into).collect()),
                attributes: v
                    .attributes
                    .map(|v| v.into_iter().map(Into::into).collect()),
            }
        }
    }

    impl From<RelationsSpec> for dtos::auth::RelationsSpec {
        fn from(v: RelationsSpec) -> Self {
            Self {
                relations: v.relations.map(|v| v.into_iter().map(Into::into).collect()),
                attributes: v
                    .attributes
                    .map(|v| v.into_iter().map(Into::into).collect()),
            }
        }
    }

    implement_serde_as!(dtos::auth::RelationsSpec, RelationsSpec);
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// config
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod config {
    #[allow(unused_imports)]
    use super::*;

    // Schema: https://opendatafabric.org/schemas/config/v1alpha1/Secret
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct Secret {
        pub value: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub content_encoding: Option<String>,
    }

    impl IntoDto for Secret {
        type Dto = dtos::config::Secret;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::config::Secret> for StructOrString<Secret> {
        fn from(v: dtos::config::Secret) -> Self {
            Self(v.into())
        }
    }
    impl From<StructOrString<Secret>> for dtos::config::Secret {
        fn from(v: StructOrString<Secret>) -> Self {
            v.0.into()
        }
    }

    impl From<dtos::config::Secret> for Secret {
        fn from(v: dtos::config::Secret) -> Self {
            Self {
                value: v.value,
                content_encoding: v.content_encoding.map(|v| v),
            }
        }
    }

    impl From<Secret> for dtos::config::Secret {
        fn from(v: Secret) -> Self {
            Self {
                value: v.value,
                content_encoding: v.content_encoding.map(|v| v),
            }
        }
    }

    implement_serde_as!(dtos::config::Secret, Secret);

    // Schema: https://opendatafabric.org/schemas/config/v1alpha1/SecretSetSpec
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct SecretSetSpec {
        pub secrets: config::Secrets,
    }

    impl IntoDto for SecretSetSpec {
        type Dto = dtos::config::SecretSetSpec;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::config::SecretSetSpec> for SecretSetSpec {
        fn from(v: dtos::config::SecretSetSpec) -> Self {
            Self {
                secrets: v.secrets.into(),
            }
        }
    }

    impl From<SecretSetSpec> for dtos::config::SecretSetSpec {
        fn from(v: SecretSetSpec) -> Self {
            Self {
                secrets: v.secrets.into(),
            }
        }
    }

    implement_serde_as!(dtos::config::SecretSetSpec, SecretSetSpec);

    // Schema: https://opendatafabric.org/schemas/config/v1alpha1/Secrets
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Secrets {
        #[serde(flatten)]
        pub entries: std::collections::BTreeMap<String, StructOrString<config::Secret>>,
    }

    impl IntoDto for Secrets {
        type Dto = dtos::config::Secrets;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::config::Secrets> for Secrets {
        fn from(v: dtos::config::Secrets) -> Self {
            Self {
                entries: v.entries.into_iter().map(|(k, v)| (k, v.into())).collect(),
            }
        }
    }

    impl From<Secrets> for dtos::config::Secrets {
        fn from(v: Secrets) -> Self {
            Self {
                entries: v.entries.into_iter().map(|(k, v)| (k, v.into())).collect(),
            }
        }
    }

    implement_serde_as!(dtos::config::Secrets, Secrets);

    // Schema: https://opendatafabric.org/schemas/config/v1alpha1/ValueRef
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct ValueRef {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account: Option<StructOrString<auth::AccountRef>>,
        pub r#type: ResourceTypeRef,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<ResourceID>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<ResourceName>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
    }

    impl IntoDto for ValueRef {
        type Dto = dtos::config::ValueRef;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::config::ValueRef> for StructOrString<ValueRef> {
        fn from(v: dtos::config::ValueRef) -> Self {
            Self(v.into())
        }
    }
    impl From<StructOrString<ValueRef>> for dtos::config::ValueRef {
        fn from(v: StructOrString<ValueRef>) -> Self {
            v.0.into()
        }
    }

    implement_serde_as!(dtos::config::ValueRef, ValueRef);

    // Schema: https://opendatafabric.org/schemas/config/v1alpha1/ValueRefs
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ValueRefs {
        #[serde(flatten)]
        pub entries: std::collections::BTreeMap<String, StructOrString<config::ValueRef>>,
    }

    impl IntoDto for ValueRefs {
        type Dto = dtos::config::ValueRefs;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::config::ValueRefs> for ValueRefs {
        fn from(v: dtos::config::ValueRefs) -> Self {
            Self {
                entries: v.entries.into_iter().map(|(k, v)| (k, v.into())).collect(),
            }
        }
    }

    impl From<ValueRefs> for dtos::config::ValueRefs {
        fn from(v: ValueRefs) -> Self {
            Self {
                entries: v.entries.into_iter().map(|(k, v)| (k, v.into())).collect(),
            }
        }
    }

    implement_serde_as!(dtos::config::ValueRefs, ValueRefs);

    // Schema: https://opendatafabric.org/schemas/config/v1alpha1/Variable
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct Variable {
        pub value: String,
    }

    impl IntoDto for Variable {
        type Dto = dtos::config::Variable;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::config::Variable> for StructOrString<Variable> {
        fn from(v: dtos::config::Variable) -> Self {
            Self(v.into())
        }
    }
    impl From<StructOrString<Variable>> for dtos::config::Variable {
        fn from(v: StructOrString<Variable>) -> Self {
            v.0.into()
        }
    }

    impl From<dtos::config::Variable> for Variable {
        fn from(v: dtos::config::Variable) -> Self {
            Self { value: v.value }
        }
    }

    impl From<Variable> for dtos::config::Variable {
        fn from(v: Variable) -> Self {
            Self { value: v.value }
        }
    }

    implement_serde_as!(dtos::config::Variable, Variable);

    // Schema: https://opendatafabric.org/schemas/config/v1alpha1/VariableSetSpec
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct VariableSetSpec {
        pub variables: config::Variables,
    }

    impl IntoDto for VariableSetSpec {
        type Dto = dtos::config::VariableSetSpec;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::config::VariableSetSpec> for VariableSetSpec {
        fn from(v: dtos::config::VariableSetSpec) -> Self {
            Self {
                variables: v.variables.into(),
            }
        }
    }

    impl From<VariableSetSpec> for dtos::config::VariableSetSpec {
        fn from(v: VariableSetSpec) -> Self {
            Self {
                variables: v.variables.into(),
            }
        }
    }

    implement_serde_as!(dtos::config::VariableSetSpec, VariableSetSpec);

    // Schema: https://opendatafabric.org/schemas/config/v1alpha1/Variables
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Variables {
        #[serde(flatten)]
        pub entries: std::collections::BTreeMap<String, StructOrString<config::Variable>>,
    }

    impl IntoDto for Variables {
        type Dto = dtos::config::Variables;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::config::Variables> for Variables {
        fn from(v: dtos::config::Variables) -> Self {
            Self {
                entries: v.entries.into_iter().map(|(k, v)| (k, v.into())).collect(),
            }
        }
    }

    impl From<Variables> for dtos::config::Variables {
        fn from(v: Variables) -> Self {
            Self {
                entries: v.entries.into_iter().map(|(k, v)| (k, v.into())).collect(),
            }
        }
    }

    implement_serde_as!(dtos::config::Variables, Variables);
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// data
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod data {
    #[allow(unused_imports)]
    use super::*;

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataField
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataField {
        pub name: String,
        pub r#type: UnionOrString<data::DataType>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub extra: Option<data::ExtraAttributes>,
    }

    impl IntoDto for DataField {
        type Dto = dtos::data::DataField;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataField> for DataField {
        fn from(v: dtos::data::DataField) -> Self {
            Self {
                name: v.name,
                r#type: v.r#type.into(),
                extra: v.extra.map(|v| v.into()),
            }
        }
    }

    impl From<DataField> for dtos::data::DataField {
        fn from(v: DataField) -> Self {
            Self {
                name: v.name,
                r#type: v.r#type.into(),
                extra: v.extra.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::data::DataField, DataField);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataSchema
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataSchema {
        pub fields: Vec<data::DataField>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub extra: Option<data::ExtraAttributes>,
    }

    impl IntoDto for DataSchema {
        type Dto = dtos::data::DataSchema;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataSchema> for DataSchema {
        fn from(v: dtos::data::DataSchema) -> Self {
            Self {
                fields: v.fields.into_iter().map(Into::into).collect(),
                extra: v.extra.map(|v| v.into()),
            }
        }
    }

    impl From<DataSchema> for dtos::data::DataSchema {
        fn from(v: DataSchema) -> Self {
            Self {
                fields: v.fields.into_iter().map(Into::into).collect(),
                extra: v.extra.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::data::DataSchema, DataSchema);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(tag = "kind")]
    pub enum DataType {
        #[serde(alias = "binary")]
        Binary(data::DataTypeBinary),
        #[serde(alias = "bool")]
        Bool(data::DataTypeBool),
        #[serde(alias = "date")]
        Date(data::DataTypeDate),
        #[serde(alias = "decimal")]
        Decimal(data::DataTypeDecimal),
        #[serde(alias = "duration")]
        Duration(data::DataTypeDuration),
        #[serde(alias = "float16")]
        Float16(data::DataTypeFloat16),
        #[serde(alias = "float32")]
        Float32(data::DataTypeFloat32),
        #[serde(alias = "float64")]
        Float64(data::DataTypeFloat64),
        #[serde(alias = "int8")]
        Int8(data::DataTypeInt8),
        #[serde(alias = "int16")]
        Int16(data::DataTypeInt16),
        #[serde(alias = "int32")]
        Int32(data::DataTypeInt32),
        #[serde(alias = "int64")]
        Int64(data::DataTypeInt64),
        #[serde(alias = "uInt8", alias = "uint8")]
        UInt8(data::DataTypeUInt8),
        #[serde(alias = "uInt16", alias = "uint16")]
        UInt16(data::DataTypeUInt16),
        #[serde(alias = "uInt32", alias = "uint32")]
        UInt32(data::DataTypeUInt32),
        #[serde(alias = "uInt64", alias = "uint64")]
        UInt64(data::DataTypeUInt64),
        #[serde(alias = "list")]
        List(data::DataTypeList),
        #[serde(alias = "map")]
        Map(data::DataTypeMap),
        #[serde(alias = "null")]
        Null(data::DataTypeNull),
        #[serde(alias = "option")]
        Option(data::DataTypeOption),
        #[serde(alias = "struct")]
        Struct(data::DataTypeStruct),
        #[serde(alias = "time")]
        Time(data::DataTypeTime),
        #[serde(alias = "timestamp")]
        Timestamp(data::DataTypeTimestamp),
        #[serde(alias = "string")]
        String(data::DataTypeString),
    }

    impl From<dtos::data::DataType> for UnionOrString<DataType> {
        fn from(v: dtos::data::DataType) -> Self {
            Self(v.into())
        }
    }
    impl From<UnionOrString<DataType>> for dtos::data::DataType {
        fn from(v: UnionOrString<DataType>) -> Self {
            v.0.into()
        }
    }

    impl IntoDto for DataType {
        type Dto = dtos::data::DataType;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataType> for DataType {
        fn from(v: dtos::data::DataType) -> Self {
            match v {
                dtos::data::DataType::Binary(v) => Self::Binary(v.into()),
                dtos::data::DataType::Bool(v) => Self::Bool(v.into()),
                dtos::data::DataType::Date(v) => Self::Date(v.into()),
                dtos::data::DataType::Decimal(v) => Self::Decimal(v.into()),
                dtos::data::DataType::Duration(v) => Self::Duration(v.into()),
                dtos::data::DataType::Float16(v) => Self::Float16(v.into()),
                dtos::data::DataType::Float32(v) => Self::Float32(v.into()),
                dtos::data::DataType::Float64(v) => Self::Float64(v.into()),
                dtos::data::DataType::Int8(v) => Self::Int8(v.into()),
                dtos::data::DataType::Int16(v) => Self::Int16(v.into()),
                dtos::data::DataType::Int32(v) => Self::Int32(v.into()),
                dtos::data::DataType::Int64(v) => Self::Int64(v.into()),
                dtos::data::DataType::UInt8(v) => Self::UInt8(v.into()),
                dtos::data::DataType::UInt16(v) => Self::UInt16(v.into()),
                dtos::data::DataType::UInt32(v) => Self::UInt32(v.into()),
                dtos::data::DataType::UInt64(v) => Self::UInt64(v.into()),
                dtos::data::DataType::List(v) => Self::List(v.into()),
                dtos::data::DataType::Map(v) => Self::Map(v.into()),
                dtos::data::DataType::Null(v) => Self::Null(v.into()),
                dtos::data::DataType::Option(v) => Self::Option(v.into()),
                dtos::data::DataType::Struct(v) => Self::Struct(v.into()),
                dtos::data::DataType::Time(v) => Self::Time(v.into()),
                dtos::data::DataType::Timestamp(v) => Self::Timestamp(v.into()),
                dtos::data::DataType::String(v) => Self::String(v.into()),
            }
        }
    }

    impl From<DataType> for dtos::data::DataType {
        fn from(v: DataType) -> Self {
            match v {
                DataType::Binary(v) => Self::Binary(v.into()),
                DataType::Bool(v) => Self::Bool(v.into()),
                DataType::Date(v) => Self::Date(v.into()),
                DataType::Decimal(v) => Self::Decimal(v.into()),
                DataType::Duration(v) => Self::Duration(v.into()),
                DataType::Float16(v) => Self::Float16(v.into()),
                DataType::Float32(v) => Self::Float32(v.into()),
                DataType::Float64(v) => Self::Float64(v.into()),
                DataType::Int8(v) => Self::Int8(v.into()),
                DataType::Int16(v) => Self::Int16(v.into()),
                DataType::Int32(v) => Self::Int32(v.into()),
                DataType::Int64(v) => Self::Int64(v.into()),
                DataType::UInt8(v) => Self::UInt8(v.into()),
                DataType::UInt16(v) => Self::UInt16(v.into()),
                DataType::UInt32(v) => Self::UInt32(v.into()),
                DataType::UInt64(v) => Self::UInt64(v.into()),
                DataType::List(v) => Self::List(v.into()),
                DataType::Map(v) => Self::Map(v.into()),
                DataType::Null(v) => Self::Null(v.into()),
                DataType::Option(v) => Self::Option(v.into()),
                DataType::Struct(v) => Self::Struct(v.into()),
                DataType::Time(v) => Self::Time(v.into()),
                DataType::Timestamp(v) => Self::Timestamp(v.into()),
                DataType::String(v) => Self::String(v.into()),
            }
        }
    }

    implement_serde_as!(dtos::data::DataType, DataType);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/Binary
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeBinary {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fixed_length: Option<u64>,
    }

    impl IntoDto for DataTypeBinary {
        type Dto = dtos::data::DataTypeBinary;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeBinary> for DataTypeBinary {
        fn from(v: dtos::data::DataTypeBinary) -> Self {
            Self {
                fixed_length: v.fixed_length.map(|v| v),
            }
        }
    }

    impl From<DataTypeBinary> for dtos::data::DataTypeBinary {
        fn from(v: DataTypeBinary) -> Self {
            Self {
                fixed_length: v.fixed_length.map(|v| v),
            }
        }
    }

    implement_serde_as!(dtos::data::DataTypeBinary, DataTypeBinary);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/Bool
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeBool {}

    impl IntoDto for DataTypeBool {
        type Dto = dtos::data::DataTypeBool;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeBool> for DataTypeBool {
        fn from(v: dtos::data::DataTypeBool) -> Self {
            Self {}
        }
    }

    impl From<DataTypeBool> for dtos::data::DataTypeBool {
        fn from(v: DataTypeBool) -> Self {
            Self {}
        }
    }

    implement_serde_as!(dtos::data::DataTypeBool, DataTypeBool);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/Date
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeDate {}

    impl IntoDto for DataTypeDate {
        type Dto = dtos::data::DataTypeDate;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeDate> for DataTypeDate {
        fn from(v: dtos::data::DataTypeDate) -> Self {
            Self {}
        }
    }

    impl From<DataTypeDate> for dtos::data::DataTypeDate {
        fn from(v: DataTypeDate) -> Self {
            Self {}
        }
    }

    implement_serde_as!(dtos::data::DataTypeDate, DataTypeDate);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/Decimal
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeDecimal {
        pub precision: u32,
        pub scale: i32,
    }

    impl IntoDto for DataTypeDecimal {
        type Dto = dtos::data::DataTypeDecimal;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeDecimal> for DataTypeDecimal {
        fn from(v: dtos::data::DataTypeDecimal) -> Self {
            Self {
                precision: v.precision,
                scale: v.scale,
            }
        }
    }

    impl From<DataTypeDecimal> for dtos::data::DataTypeDecimal {
        fn from(v: DataTypeDecimal) -> Self {
            Self {
                precision: v.precision,
                scale: v.scale,
            }
        }
    }

    implement_serde_as!(dtos::data::DataTypeDecimal, DataTypeDecimal);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/Duration
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeDuration {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub unit: Option<data::TimeUnit>,
    }

    impl IntoDto for DataTypeDuration {
        type Dto = dtos::data::DataTypeDuration;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeDuration> for DataTypeDuration {
        fn from(v: dtos::data::DataTypeDuration) -> Self {
            Self {
                unit: v.unit.map(|v| v.into()),
            }
        }
    }

    impl From<DataTypeDuration> for dtos::data::DataTypeDuration {
        fn from(v: DataTypeDuration) -> Self {
            Self {
                unit: v.unit.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::data::DataTypeDuration, DataTypeDuration);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/Float16
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeFloat16 {}

    impl IntoDto for DataTypeFloat16 {
        type Dto = dtos::data::DataTypeFloat16;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeFloat16> for DataTypeFloat16 {
        fn from(v: dtos::data::DataTypeFloat16) -> Self {
            Self {}
        }
    }

    impl From<DataTypeFloat16> for dtos::data::DataTypeFloat16 {
        fn from(v: DataTypeFloat16) -> Self {
            Self {}
        }
    }

    implement_serde_as!(dtos::data::DataTypeFloat16, DataTypeFloat16);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/Float32
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeFloat32 {}

    impl IntoDto for DataTypeFloat32 {
        type Dto = dtos::data::DataTypeFloat32;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeFloat32> for DataTypeFloat32 {
        fn from(v: dtos::data::DataTypeFloat32) -> Self {
            Self {}
        }
    }

    impl From<DataTypeFloat32> for dtos::data::DataTypeFloat32 {
        fn from(v: DataTypeFloat32) -> Self {
            Self {}
        }
    }

    implement_serde_as!(dtos::data::DataTypeFloat32, DataTypeFloat32);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/Float64
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeFloat64 {}

    impl IntoDto for DataTypeFloat64 {
        type Dto = dtos::data::DataTypeFloat64;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeFloat64> for DataTypeFloat64 {
        fn from(v: dtos::data::DataTypeFloat64) -> Self {
            Self {}
        }
    }

    impl From<DataTypeFloat64> for dtos::data::DataTypeFloat64 {
        fn from(v: DataTypeFloat64) -> Self {
            Self {}
        }
    }

    implement_serde_as!(dtos::data::DataTypeFloat64, DataTypeFloat64);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/Int16
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeInt16 {}

    impl IntoDto for DataTypeInt16 {
        type Dto = dtos::data::DataTypeInt16;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeInt16> for DataTypeInt16 {
        fn from(v: dtos::data::DataTypeInt16) -> Self {
            Self {}
        }
    }

    impl From<DataTypeInt16> for dtos::data::DataTypeInt16 {
        fn from(v: DataTypeInt16) -> Self {
            Self {}
        }
    }

    implement_serde_as!(dtos::data::DataTypeInt16, DataTypeInt16);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/Int32
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeInt32 {}

    impl IntoDto for DataTypeInt32 {
        type Dto = dtos::data::DataTypeInt32;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeInt32> for DataTypeInt32 {
        fn from(v: dtos::data::DataTypeInt32) -> Self {
            Self {}
        }
    }

    impl From<DataTypeInt32> for dtos::data::DataTypeInt32 {
        fn from(v: DataTypeInt32) -> Self {
            Self {}
        }
    }

    implement_serde_as!(dtos::data::DataTypeInt32, DataTypeInt32);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/Int64
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeInt64 {}

    impl IntoDto for DataTypeInt64 {
        type Dto = dtos::data::DataTypeInt64;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeInt64> for DataTypeInt64 {
        fn from(v: dtos::data::DataTypeInt64) -> Self {
            Self {}
        }
    }

    impl From<DataTypeInt64> for dtos::data::DataTypeInt64 {
        fn from(v: DataTypeInt64) -> Self {
            Self {}
        }
    }

    implement_serde_as!(dtos::data::DataTypeInt64, DataTypeInt64);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/Int8
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeInt8 {}

    impl IntoDto for DataTypeInt8 {
        type Dto = dtos::data::DataTypeInt8;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeInt8> for DataTypeInt8 {
        fn from(v: dtos::data::DataTypeInt8) -> Self {
            Self {}
        }
    }

    impl From<DataTypeInt8> for dtos::data::DataTypeInt8 {
        fn from(v: DataTypeInt8) -> Self {
            Self {}
        }
    }

    implement_serde_as!(dtos::data::DataTypeInt8, DataTypeInt8);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/List
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeList {
        pub item_type: Box<UnionOrString<data::DataType>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fixed_length: Option<u64>,
    }

    impl IntoDto for DataTypeList {
        type Dto = dtos::data::DataTypeList;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeList> for DataTypeList {
        fn from(v: dtos::data::DataTypeList) -> Self {
            Self {
                item_type: Box::new((*v.item_type).into()),
                fixed_length: v.fixed_length.map(|v| v),
            }
        }
    }

    impl From<DataTypeList> for dtos::data::DataTypeList {
        fn from(v: DataTypeList) -> Self {
            Self {
                item_type: Box::new((*v.item_type).into()),
                fixed_length: v.fixed_length.map(|v| v),
            }
        }
    }

    implement_serde_as!(dtos::data::DataTypeList, DataTypeList);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/Map
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeMap {
        pub key_type: Box<UnionOrString<data::DataType>>,
        pub value_type: Box<UnionOrString<data::DataType>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub keys_sorted: Option<bool>,
    }

    impl IntoDto for DataTypeMap {
        type Dto = dtos::data::DataTypeMap;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeMap> for DataTypeMap {
        fn from(v: dtos::data::DataTypeMap) -> Self {
            Self {
                key_type: Box::new((*v.key_type).into()),
                value_type: Box::new((*v.value_type).into()),
                keys_sorted: v.keys_sorted.map(|v| v),
            }
        }
    }

    impl From<DataTypeMap> for dtos::data::DataTypeMap {
        fn from(v: DataTypeMap) -> Self {
            Self {
                key_type: Box::new((*v.key_type).into()),
                value_type: Box::new((*v.value_type).into()),
                keys_sorted: v.keys_sorted.map(|v| v),
            }
        }
    }

    implement_serde_as!(dtos::data::DataTypeMap, DataTypeMap);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/Null
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeNull {}

    impl IntoDto for DataTypeNull {
        type Dto = dtos::data::DataTypeNull;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeNull> for DataTypeNull {
        fn from(v: dtos::data::DataTypeNull) -> Self {
            Self {}
        }
    }

    impl From<DataTypeNull> for dtos::data::DataTypeNull {
        fn from(v: DataTypeNull) -> Self {
            Self {}
        }
    }

    implement_serde_as!(dtos::data::DataTypeNull, DataTypeNull);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/Option
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeOption {
        pub inner: Box<UnionOrString<data::DataType>>,
    }

    impl IntoDto for DataTypeOption {
        type Dto = dtos::data::DataTypeOption;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeOption> for DataTypeOption {
        fn from(v: dtos::data::DataTypeOption) -> Self {
            Self {
                inner: Box::new((*v.inner).into()),
            }
        }
    }

    impl From<DataTypeOption> for dtos::data::DataTypeOption {
        fn from(v: DataTypeOption) -> Self {
            Self {
                inner: Box::new((*v.inner).into()),
            }
        }
    }

    implement_serde_as!(dtos::data::DataTypeOption, DataTypeOption);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/String
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeString {}

    impl IntoDto for DataTypeString {
        type Dto = dtos::data::DataTypeString;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeString> for DataTypeString {
        fn from(v: dtos::data::DataTypeString) -> Self {
            Self {}
        }
    }

    impl From<DataTypeString> for dtos::data::DataTypeString {
        fn from(v: DataTypeString) -> Self {
            Self {}
        }
    }

    implement_serde_as!(dtos::data::DataTypeString, DataTypeString);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/Struct
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeStruct {
        pub fields: Vec<data::DataField>,
    }

    impl IntoDto for DataTypeStruct {
        type Dto = dtos::data::DataTypeStruct;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeStruct> for DataTypeStruct {
        fn from(v: dtos::data::DataTypeStruct) -> Self {
            Self {
                fields: v.fields.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl From<DataTypeStruct> for dtos::data::DataTypeStruct {
        fn from(v: DataTypeStruct) -> Self {
            Self {
                fields: v.fields.into_iter().map(Into::into).collect(),
            }
        }
    }

    implement_serde_as!(dtos::data::DataTypeStruct, DataTypeStruct);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/Time
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeTime {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub unit: Option<data::TimeUnit>,
    }

    impl IntoDto for DataTypeTime {
        type Dto = dtos::data::DataTypeTime;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeTime> for DataTypeTime {
        fn from(v: dtos::data::DataTypeTime) -> Self {
            Self {
                unit: v.unit.map(|v| v.into()),
            }
        }
    }

    impl From<DataTypeTime> for dtos::data::DataTypeTime {
        fn from(v: DataTypeTime) -> Self {
            Self {
                unit: v.unit.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::data::DataTypeTime, DataTypeTime);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/Timestamp
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeTimestamp {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub unit: Option<data::TimeUnit>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timezone: Option<String>,
    }

    impl IntoDto for DataTypeTimestamp {
        type Dto = dtos::data::DataTypeTimestamp;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeTimestamp> for DataTypeTimestamp {
        fn from(v: dtos::data::DataTypeTimestamp) -> Self {
            Self {
                unit: v.unit.map(|v| v.into()),
                timezone: v.timezone.map(|v| v),
            }
        }
    }

    impl From<DataTypeTimestamp> for dtos::data::DataTypeTimestamp {
        fn from(v: DataTypeTimestamp) -> Self {
            Self {
                unit: v.unit.map(|v| v.into()),
                timezone: v.timezone.map(|v| v),
            }
        }
    }

    implement_serde_as!(dtos::data::DataTypeTimestamp, DataTypeTimestamp);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/UInt16
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeUInt16 {}

    impl IntoDto for DataTypeUInt16 {
        type Dto = dtos::data::DataTypeUInt16;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeUInt16> for DataTypeUInt16 {
        fn from(v: dtos::data::DataTypeUInt16) -> Self {
            Self {}
        }
    }

    impl From<DataTypeUInt16> for dtos::data::DataTypeUInt16 {
        fn from(v: DataTypeUInt16) -> Self {
            Self {}
        }
    }

    implement_serde_as!(dtos::data::DataTypeUInt16, DataTypeUInt16);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/UInt32
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeUInt32 {}

    impl IntoDto for DataTypeUInt32 {
        type Dto = dtos::data::DataTypeUInt32;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeUInt32> for DataTypeUInt32 {
        fn from(v: dtos::data::DataTypeUInt32) -> Self {
            Self {}
        }
    }

    impl From<DataTypeUInt32> for dtos::data::DataTypeUInt32 {
        fn from(v: DataTypeUInt32) -> Self {
            Self {}
        }
    }

    implement_serde_as!(dtos::data::DataTypeUInt32, DataTypeUInt32);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/UInt64
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeUInt64 {}

    impl IntoDto for DataTypeUInt64 {
        type Dto = dtos::data::DataTypeUInt64;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeUInt64> for DataTypeUInt64 {
        fn from(v: dtos::data::DataTypeUInt64) -> Self {
            Self {}
        }
    }

    impl From<DataTypeUInt64> for dtos::data::DataTypeUInt64 {
        fn from(v: DataTypeUInt64) -> Self {
            Self {}
        }
    }

    implement_serde_as!(dtos::data::DataTypeUInt64, DataTypeUInt64);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/DataType#/$defs/UInt8
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataTypeUInt8 {}

    impl IntoDto for DataTypeUInt8 {
        type Dto = dtos::data::DataTypeUInt8;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::DataTypeUInt8> for DataTypeUInt8 {
        fn from(v: dtos::data::DataTypeUInt8) -> Self {
            Self {}
        }
    }

    impl From<DataTypeUInt8> for dtos::data::DataTypeUInt8 {
        fn from(v: DataTypeUInt8) -> Self {
            Self {}
        }
    }

    implement_serde_as!(dtos::data::DataTypeUInt8, DataTypeUInt8);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/ExtraAttributes
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ExtraAttributes {
        #[serde(flatten)]
        #[serde(with = "map_value_limited_precision")]
        pub entries: std::collections::BTreeMap<String, serde_json::Value>,
    }

    impl IntoDto for ExtraAttributes {
        type Dto = dtos::data::ExtraAttributes;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::ExtraAttributes> for ExtraAttributes {
        fn from(v: dtos::data::ExtraAttributes) -> Self {
            Self { entries: v.entries }
        }
    }

    impl From<ExtraAttributes> for dtos::data::ExtraAttributes {
        fn from(v: ExtraAttributes) -> Self {
            Self { entries: v.entries }
        }
    }

    implement_serde_as!(dtos::data::ExtraAttributes, ExtraAttributes);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/OperationType
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub enum OperationType {
        #[serde(alias = "append")]
        Append,
        #[serde(alias = "retract")]
        Retract,
        #[serde(alias = "correctFrom", alias = "correctfrom")]
        CorrectFrom,
        #[serde(alias = "correctTo", alias = "correctto")]
        CorrectTo,
    }

    impl IntoDto for OperationType {
        type Dto = dtos::data::OperationType;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::OperationType> for OperationType {
        fn from(v: dtos::data::OperationType) -> Self {
            match v {
                dtos::data::OperationType::Append => Self::Append,
                dtos::data::OperationType::Retract => Self::Retract,
                dtos::data::OperationType::CorrectFrom => Self::CorrectFrom,
                dtos::data::OperationType::CorrectTo => Self::CorrectTo,
            }
        }
    }

    impl From<OperationType> for dtos::data::OperationType {
        fn from(v: OperationType) -> Self {
            match v {
                OperationType::Append => Self::Append,
                OperationType::Retract => Self::Retract,
                OperationType::CorrectFrom => Self::CorrectFrom,
                OperationType::CorrectTo => Self::CorrectTo,
            }
        }
    }

    implement_serde_as!(dtos::data::OperationType, OperationType);

    // Schema: https://opendatafabric.org/schemas/data/v1alpha1/TimeUnit
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub enum TimeUnit {
        #[serde(alias = "second")]
        Second,
        #[serde(alias = "millisecond")]
        Millisecond,
        #[serde(alias = "microsecond")]
        Microsecond,
        #[serde(alias = "nanosecond")]
        Nanosecond,
    }

    impl IntoDto for TimeUnit {
        type Dto = dtos::data::TimeUnit;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::data::TimeUnit> for TimeUnit {
        fn from(v: dtos::data::TimeUnit) -> Self {
            match v {
                dtos::data::TimeUnit::Second => Self::Second,
                dtos::data::TimeUnit::Millisecond => Self::Millisecond,
                dtos::data::TimeUnit::Microsecond => Self::Microsecond,
                dtos::data::TimeUnit::Nanosecond => Self::Nanosecond,
            }
        }
    }

    impl From<TimeUnit> for dtos::data::TimeUnit {
        fn from(v: TimeUnit) -> Self {
            match v {
                TimeUnit::Second => Self::Second,
                TimeUnit::Millisecond => Self::Millisecond,
                TimeUnit::Microsecond => Self::Microsecond,
                TimeUnit::Nanosecond => Self::Nanosecond,
            }
        }
    }

    implement_serde_as!(dtos::data::TimeUnit, TimeUnit);
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// dataset
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod dataset {
    #[allow(unused_imports)]
    use super::*;

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/AddData
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct AddData {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prev_checkpoint: Option<Multihash>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prev_offset: Option<u64>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_data: Option<dataset::DataSlice>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_checkpoint: Option<dataset::Checkpoint>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(with = "datetime_rfc3339_opt")]
        pub new_watermark: Option<DateTime<Utc>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_source_state: Option<source::SourceState>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub extra: Option<data::ExtraAttributes>,
    }

    impl IntoDto for AddData {
        type Dto = dtos::dataset::AddData;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::AddData> for AddData {
        fn from(v: dtos::dataset::AddData) -> Self {
            Self {
                prev_checkpoint: v.prev_checkpoint.map(|v| v),
                prev_offset: v.prev_offset.map(|v| v),
                new_data: v.new_data.map(|v| v.into()),
                new_checkpoint: v.new_checkpoint.map(|v| v.into()),
                new_watermark: v.new_watermark.map(|v| v),
                new_source_state: v.new_source_state.map(|v| v.into()),
                extra: v.extra.map(|v| v.into()),
            }
        }
    }

    impl From<AddData> for dtos::dataset::AddData {
        fn from(v: AddData) -> Self {
            Self {
                prev_checkpoint: v.prev_checkpoint.map(|v| v),
                prev_offset: v.prev_offset.map(|v| v),
                new_data: v.new_data.map(|v| v.into()),
                new_checkpoint: v.new_checkpoint.map(|v| v.into()),
                new_watermark: v.new_watermark.map(|v| v),
                new_source_state: v.new_source_state.map(|v| v.into()),
                extra: v.extra.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::dataset::AddData, AddData);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/AttachmentEmbedded
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct AttachmentEmbedded {
        pub path: String,
        pub content: String,
    }

    impl IntoDto for AttachmentEmbedded {
        type Dto = dtos::dataset::AttachmentEmbedded;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::AttachmentEmbedded> for AttachmentEmbedded {
        fn from(v: dtos::dataset::AttachmentEmbedded) -> Self {
            Self {
                path: v.path,
                content: v.content,
            }
        }
    }

    impl From<AttachmentEmbedded> for dtos::dataset::AttachmentEmbedded {
        fn from(v: AttachmentEmbedded) -> Self {
            Self {
                path: v.path,
                content: v.content,
            }
        }
    }

    implement_serde_as!(dtos::dataset::AttachmentEmbedded, AttachmentEmbedded);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/Attachments
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(tag = "kind")]
    pub enum Attachments {
        #[serde(alias = "embedded")]
        Embedded(dataset::AttachmentsEmbedded),
    }

    impl IntoDto for Attachments {
        type Dto = dtos::dataset::Attachments;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::Attachments> for Attachments {
        fn from(v: dtos::dataset::Attachments) -> Self {
            match v {
                dtos::dataset::Attachments::Embedded(v) => Self::Embedded(v.into()),
            }
        }
    }

    impl From<Attachments> for dtos::dataset::Attachments {
        fn from(v: Attachments) -> Self {
            match v {
                Attachments::Embedded(v) => Self::Embedded(v.into()),
            }
        }
    }

    implement_serde_as!(dtos::dataset::Attachments, Attachments);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/Attachments#/$defs/Embedded
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct AttachmentsEmbedded {
        pub items: Vec<dataset::AttachmentEmbedded>,
    }

    impl IntoDto for AttachmentsEmbedded {
        type Dto = dtos::dataset::AttachmentsEmbedded;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::AttachmentsEmbedded> for AttachmentsEmbedded {
        fn from(v: dtos::dataset::AttachmentsEmbedded) -> Self {
            Self {
                items: v.items.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl From<AttachmentsEmbedded> for dtos::dataset::AttachmentsEmbedded {
        fn from(v: AttachmentsEmbedded) -> Self {
            Self {
                items: v.items.into_iter().map(Into::into).collect(),
            }
        }
    }

    implement_serde_as!(dtos::dataset::AttachmentsEmbedded, AttachmentsEmbedded);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/Checkpoint
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct Checkpoint {
        pub physical_hash: Multihash,
        pub size: u64,
    }

    impl IntoDto for Checkpoint {
        type Dto = dtos::dataset::Checkpoint;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::Checkpoint> for Checkpoint {
        fn from(v: dtos::dataset::Checkpoint) -> Self {
            Self {
                physical_hash: v.physical_hash,
                size: v.size,
            }
        }
    }

    impl From<Checkpoint> for dtos::dataset::Checkpoint {
        fn from(v: Checkpoint) -> Self {
            Self {
                physical_hash: v.physical_hash,
                size: v.size,
            }
        }
    }

    implement_serde_as!(dtos::dataset::Checkpoint, Checkpoint);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/CompactionParams
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct CompactionParams {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_slice_size: Option<ByteSize>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_slice_records: Option<u64>,
    }

    impl IntoDto for CompactionParams {
        type Dto = dtos::dataset::CompactionParams;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::CompactionParams> for CompactionParams {
        fn from(v: dtos::dataset::CompactionParams) -> Self {
            Self {
                max_slice_size: v.max_slice_size.map(|v| v),
                max_slice_records: v.max_slice_records.map(|v| v),
            }
        }
    }

    impl From<CompactionParams> for dtos::dataset::CompactionParams {
        fn from(v: CompactionParams) -> Self {
            Self {
                max_slice_size: v.max_slice_size.map(|v| v),
                max_slice_records: v.max_slice_records.map(|v| v),
            }
        }
    }

    implement_serde_as!(dtos::dataset::CompactionParams, CompactionParams);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/DataSlice
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DataSlice {
        pub logical_hash: Multihash,
        pub physical_hash: Multihash,
        pub offset_interval: dataset::OffsetInterval,
        pub size: u64,
    }

    impl IntoDto for DataSlice {
        type Dto = dtos::dataset::DataSlice;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::DataSlice> for DataSlice {
        fn from(v: dtos::dataset::DataSlice) -> Self {
            Self {
                logical_hash: v.logical_hash,
                physical_hash: v.physical_hash,
                offset_interval: v.offset_interval.into(),
                size: v.size,
            }
        }
    }

    impl From<DataSlice> for dtos::dataset::DataSlice {
        fn from(v: DataSlice) -> Self {
            Self {
                logical_hash: v.logical_hash,
                physical_hash: v.physical_hash,
                offset_interval: v.offset_interval.into(),
                size: v.size,
            }
        }
    }

    implement_serde_as!(dtos::dataset::DataSlice, DataSlice);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/DatasetKind
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub enum DatasetKind {
        #[serde(alias = "root")]
        Root,
        #[serde(alias = "derivative")]
        Derivative,
    }

    impl IntoDto for DatasetKind {
        type Dto = dtos::dataset::DatasetKind;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::DatasetKind> for DatasetKind {
        fn from(v: dtos::dataset::DatasetKind) -> Self {
            match v {
                dtos::dataset::DatasetKind::Root => Self::Root,
                dtos::dataset::DatasetKind::Derivative => Self::Derivative,
            }
        }
    }

    impl From<DatasetKind> for dtos::dataset::DatasetKind {
        fn from(v: DatasetKind) -> Self {
            match v {
                DatasetKind::Root => Self::Root,
                DatasetKind::Derivative => Self::Derivative,
            }
        }
    }

    implement_serde_as!(dtos::dataset::DatasetKind, DatasetKind);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/DatasetSelector
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DatasetSelector {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account: Option<StructOrString<auth::AccountRef>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<ResourceID>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub labels: Option<resource::LabelFilter>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub kind: Option<dataset::DatasetKind>,
    }

    impl IntoDto for DatasetSelector {
        type Dto = dtos::dataset::DatasetSelector;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::DatasetSelector> for StructOrString<DatasetSelector> {
        fn from(v: dtos::dataset::DatasetSelector) -> Self {
            Self(v.into())
        }
    }
    impl From<StructOrString<DatasetSelector>> for dtos::dataset::DatasetSelector {
        fn from(v: StructOrString<DatasetSelector>) -> Self {
            v.0.into()
        }
    }

    implement_serde_as!(dtos::dataset::DatasetSelector, DatasetSelector);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/DatasetSpec
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DatasetSpec {
        pub kind: dataset::DatasetKind,
        pub metadata: Vec<dataset::MetadataEvent>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub volume: Option<StructOrString<storage::PersistentVolumeRef>>,
    }

    impl IntoDto for DatasetSpec {
        type Dto = dtos::dataset::DatasetSpec;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::DatasetSpec> for DatasetSpec {
        fn from(v: dtos::dataset::DatasetSpec) -> Self {
            Self {
                kind: v.kind.into(),
                metadata: v.metadata.into_iter().map(Into::into).collect(),
                volume: v.volume.map(|v| v.into()),
            }
        }
    }

    impl From<DatasetSpec> for dtos::dataset::DatasetSpec {
        fn from(v: DatasetSpec) -> Self {
            Self {
                kind: v.kind.into(),
                metadata: v.metadata.into_iter().map(Into::into).collect(),
                volume: v.volume.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::dataset::DatasetSpec, DatasetSpec);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/DatasetVocabulary
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DatasetVocabulary {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub offset_column: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operation_type_column: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub system_time_column: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub event_time_column: Option<String>,
    }

    impl IntoDto for DatasetVocabulary {
        type Dto = dtos::dataset::DatasetVocabulary;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::DatasetVocabulary> for DatasetVocabulary {
        fn from(v: dtos::dataset::DatasetVocabulary) -> Self {
            Self {
                offset_column: v.offset_column.map(|v| v),
                operation_type_column: v.operation_type_column.map(|v| v),
                system_time_column: v.system_time_column.map(|v| v),
                event_time_column: v.event_time_column.map(|v| v),
            }
        }
    }

    impl From<DatasetVocabulary> for dtos::dataset::DatasetVocabulary {
        fn from(v: DatasetVocabulary) -> Self {
            Self {
                offset_column: v.offset_column.map(|v| v),
                operation_type_column: v.operation_type_column.map(|v| v),
                system_time_column: v.system_time_column.map(|v| v),
                event_time_column: v.event_time_column.map(|v| v),
            }
        }
    }

    implement_serde_as!(dtos::dataset::DatasetVocabulary, DatasetVocabulary);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/ExecuteTransform
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct ExecuteTransform {
        pub query_inputs: Vec<dataset::ExecuteTransformInput>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prev_checkpoint: Option<Multihash>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prev_offset: Option<u64>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_data: Option<dataset::DataSlice>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_checkpoint: Option<dataset::Checkpoint>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(with = "datetime_rfc3339_opt")]
        pub new_watermark: Option<DateTime<Utc>>,
    }

    impl IntoDto for ExecuteTransform {
        type Dto = dtos::dataset::ExecuteTransform;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::ExecuteTransform> for ExecuteTransform {
        fn from(v: dtos::dataset::ExecuteTransform) -> Self {
            Self {
                query_inputs: v.query_inputs.into_iter().map(Into::into).collect(),
                prev_checkpoint: v.prev_checkpoint.map(|v| v),
                prev_offset: v.prev_offset.map(|v| v),
                new_data: v.new_data.map(|v| v.into()),
                new_checkpoint: v.new_checkpoint.map(|v| v.into()),
                new_watermark: v.new_watermark.map(|v| v),
            }
        }
    }

    impl From<ExecuteTransform> for dtos::dataset::ExecuteTransform {
        fn from(v: ExecuteTransform) -> Self {
            Self {
                query_inputs: v.query_inputs.into_iter().map(Into::into).collect(),
                prev_checkpoint: v.prev_checkpoint.map(|v| v),
                prev_offset: v.prev_offset.map(|v| v),
                new_data: v.new_data.map(|v| v.into()),
                new_checkpoint: v.new_checkpoint.map(|v| v.into()),
                new_watermark: v.new_watermark.map(|v| v),
            }
        }
    }

    implement_serde_as!(dtos::dataset::ExecuteTransform, ExecuteTransform);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/ExecuteTransformInput
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct ExecuteTransformInput {
        pub dataset_id: DatasetID,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prev_block_hash: Option<Multihash>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_block_hash: Option<Multihash>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prev_offset: Option<u64>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_offset: Option<u64>,
    }

    impl IntoDto for ExecuteTransformInput {
        type Dto = dtos::dataset::ExecuteTransformInput;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::ExecuteTransformInput> for ExecuteTransformInput {
        fn from(v: dtos::dataset::ExecuteTransformInput) -> Self {
            Self {
                dataset_id: v.dataset_id,
                prev_block_hash: v.prev_block_hash.map(|v| v),
                new_block_hash: v.new_block_hash.map(|v| v),
                prev_offset: v.prev_offset.map(|v| v),
                new_offset: v.new_offset.map(|v| v),
            }
        }
    }

    impl From<ExecuteTransformInput> for dtos::dataset::ExecuteTransformInput {
        fn from(v: ExecuteTransformInput) -> Self {
            Self {
                dataset_id: v.dataset_id,
                prev_block_hash: v.prev_block_hash.map(|v| v),
                new_block_hash: v.new_block_hash.map(|v| v),
                prev_offset: v.prev_offset.map(|v| v),
                new_offset: v.new_offset.map(|v| v),
            }
        }
    }

    implement_serde_as!(dtos::dataset::ExecuteTransformInput, ExecuteTransformInput);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/MetadataBlock
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct MetadataBlock {
        #[serde(with = "datetime_rfc3339")]
        pub system_time: DateTime<Utc>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prev_block_hash: Option<Multihash>,
        pub sequence_number: u64,
        pub event: dataset::MetadataEvent,
    }

    impl IntoDto for MetadataBlock {
        type Dto = dtos::dataset::MetadataBlock;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::MetadataBlock> for MetadataBlock {
        fn from(v: dtos::dataset::MetadataBlock) -> Self {
            Self {
                system_time: v.system_time,
                prev_block_hash: v.prev_block_hash.map(|v| v),
                sequence_number: v.sequence_number,
                event: v.event.into(),
            }
        }
    }

    impl From<MetadataBlock> for dtos::dataset::MetadataBlock {
        fn from(v: MetadataBlock) -> Self {
            Self {
                system_time: v.system_time,
                prev_block_hash: v.prev_block_hash.map(|v| v),
                sequence_number: v.sequence_number,
                event: v.event.into(),
            }
        }
    }

    implement_serde_as!(dtos::dataset::MetadataBlock, MetadataBlock);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/MetadataEvent
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(tag = "kind")]
    pub enum MetadataEvent {
        #[serde(alias = "addData", alias = "adddata")]
        AddData(dataset::AddData),
        #[serde(alias = "executeTransform", alias = "executetransform")]
        ExecuteTransform(dataset::ExecuteTransform),
        #[serde(alias = "seed")]
        Seed(dataset::Seed),
        #[serde(alias = "setPollingSource", alias = "setpollingsource")]
        SetPollingSource(legacy::SetPollingSource),
        #[serde(alias = "setTransform", alias = "settransform")]
        SetTransform(dataset::SetTransform),
        #[serde(alias = "setVocab", alias = "setvocab")]
        SetVocab(dataset::SetVocab),
        #[serde(alias = "setAttachments", alias = "setattachments")]
        SetAttachments(dataset::SetAttachments),
        #[serde(alias = "setInfo", alias = "setinfo")]
        SetInfo(dataset::SetInfo),
        #[serde(alias = "setLicense", alias = "setlicense")]
        SetLicense(dataset::SetLicense),
        #[serde(alias = "setDataSchema", alias = "setdataschema")]
        SetDataSchema(dataset::SetDataSchema),
        #[serde(alias = "addPushSource", alias = "addpushsource")]
        AddPushSource(legacy::AddPushSource),
        #[serde(alias = "disablePushSource", alias = "disablepushsource")]
        DisablePushSource(legacy::DisablePushSource),
        #[serde(alias = "disablePollingSource", alias = "disablepollingsource")]
        DisablePollingSource(legacy::DisablePollingSource),
    }

    impl IntoDto for MetadataEvent {
        type Dto = dtos::dataset::MetadataEvent;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::MetadataEvent> for MetadataEvent {
        fn from(v: dtos::dataset::MetadataEvent) -> Self {
            match v {
                dtos::dataset::MetadataEvent::AddData(v) => Self::AddData(v.into()),
                dtos::dataset::MetadataEvent::ExecuteTransform(v) => {
                    Self::ExecuteTransform(v.into())
                }
                dtos::dataset::MetadataEvent::Seed(v) => Self::Seed(v.into()),
                dtos::dataset::MetadataEvent::SetPollingSource(v) => {
                    Self::SetPollingSource(v.into())
                }
                dtos::dataset::MetadataEvent::SetTransform(v) => Self::SetTransform(v.into()),
                dtos::dataset::MetadataEvent::SetVocab(v) => Self::SetVocab(v.into()),
                dtos::dataset::MetadataEvent::SetAttachments(v) => Self::SetAttachments(v.into()),
                dtos::dataset::MetadataEvent::SetInfo(v) => Self::SetInfo(v.into()),
                dtos::dataset::MetadataEvent::SetLicense(v) => Self::SetLicense(v.into()),
                dtos::dataset::MetadataEvent::SetDataSchema(v) => Self::SetDataSchema(v.into()),
                dtos::dataset::MetadataEvent::AddPushSource(v) => Self::AddPushSource(v.into()),
                dtos::dataset::MetadataEvent::DisablePushSource(v) => {
                    Self::DisablePushSource(v.into())
                }
                dtos::dataset::MetadataEvent::DisablePollingSource(v) => {
                    Self::DisablePollingSource(v.into())
                }
            }
        }
    }

    impl From<MetadataEvent> for dtos::dataset::MetadataEvent {
        fn from(v: MetadataEvent) -> Self {
            match v {
                MetadataEvent::AddData(v) => Self::AddData(v.into()),
                MetadataEvent::ExecuteTransform(v) => Self::ExecuteTransform(v.into()),
                MetadataEvent::Seed(v) => Self::Seed(v.into()),
                MetadataEvent::SetPollingSource(v) => Self::SetPollingSource(v.into()),
                MetadataEvent::SetTransform(v) => Self::SetTransform(v.into()),
                MetadataEvent::SetVocab(v) => Self::SetVocab(v.into()),
                MetadataEvent::SetAttachments(v) => Self::SetAttachments(v.into()),
                MetadataEvent::SetInfo(v) => Self::SetInfo(v.into()),
                MetadataEvent::SetLicense(v) => Self::SetLicense(v.into()),
                MetadataEvent::SetDataSchema(v) => Self::SetDataSchema(v.into()),
                MetadataEvent::AddPushSource(v) => Self::AddPushSource(v.into()),
                MetadataEvent::DisablePushSource(v) => Self::DisablePushSource(v.into()),
                MetadataEvent::DisablePollingSource(v) => Self::DisablePollingSource(v.into()),
            }
        }
    }

    implement_serde_as!(dtos::dataset::MetadataEvent, MetadataEvent);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/OffsetInterval
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct OffsetInterval {
        pub start: u64,
        pub end: u64,
    }

    impl IntoDto for OffsetInterval {
        type Dto = dtos::dataset::OffsetInterval;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::OffsetInterval> for OffsetInterval {
        fn from(v: dtos::dataset::OffsetInterval) -> Self {
            Self {
                start: v.start,
                end: v.end,
            }
        }
    }

    impl From<OffsetInterval> for dtos::dataset::OffsetInterval {
        fn from(v: OffsetInterval) -> Self {
            Self {
                start: v.start,
                end: v.end,
            }
        }
    }

    implement_serde_as!(dtos::dataset::OffsetInterval, OffsetInterval);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/ProjectionSpec
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct ProjectionSpec {
        pub inputs: Vec<dataset::TransformInput>,
        pub project: dataset::Transform,
    }

    impl IntoDto for ProjectionSpec {
        type Dto = dtos::dataset::ProjectionSpec;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::ProjectionSpec> for ProjectionSpec {
        fn from(v: dtos::dataset::ProjectionSpec) -> Self {
            Self {
                inputs: v.inputs.into_iter().map(Into::into).collect(),
                project: v.project.into(),
            }
        }
    }

    impl From<ProjectionSpec> for dtos::dataset::ProjectionSpec {
        fn from(v: ProjectionSpec) -> Self {
            Self {
                inputs: v.inputs.into_iter().map(Into::into).collect(),
                project: v.project.into(),
            }
        }
    }

    implement_serde_as!(dtos::dataset::ProjectionSpec, ProjectionSpec);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/Seed
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct Seed {
        pub dataset_id: DatasetID,
        pub dataset_kind: dataset::DatasetKind,
    }

    impl IntoDto for Seed {
        type Dto = dtos::dataset::Seed;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::Seed> for Seed {
        fn from(v: dtos::dataset::Seed) -> Self {
            Self {
                dataset_id: v.dataset_id,
                dataset_kind: v.dataset_kind.into(),
            }
        }
    }

    impl From<Seed> for dtos::dataset::Seed {
        fn from(v: Seed) -> Self {
            Self {
                dataset_id: v.dataset_id,
                dataset_kind: v.dataset_kind.into(),
            }
        }
    }

    implement_serde_as!(dtos::dataset::Seed, Seed);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/SetAttachments
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct SetAttachments {
        pub attachments: dataset::Attachments,
    }

    impl IntoDto for SetAttachments {
        type Dto = dtos::dataset::SetAttachments;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::SetAttachments> for SetAttachments {
        fn from(v: dtos::dataset::SetAttachments) -> Self {
            Self {
                attachments: v.attachments.into(),
            }
        }
    }

    impl From<SetAttachments> for dtos::dataset::SetAttachments {
        fn from(v: SetAttachments) -> Self {
            Self {
                attachments: v.attachments.into(),
            }
        }
    }

    implement_serde_as!(dtos::dataset::SetAttachments, SetAttachments);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/SetDataSchema
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct SetDataSchema {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(with = "base64_opt")]
        pub raw_arrow_schema: Option<Vec<u8>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub schema: Option<data::DataSchema>,
    }

    impl IntoDto for SetDataSchema {
        type Dto = dtos::dataset::SetDataSchema;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::SetDataSchema> for SetDataSchema {
        fn from(v: dtos::dataset::SetDataSchema) -> Self {
            Self {
                raw_arrow_schema: v.raw_arrow_schema.map(|v| v),
                schema: v.schema.map(|v| v.into()),
            }
        }
    }

    impl From<SetDataSchema> for dtos::dataset::SetDataSchema {
        fn from(v: SetDataSchema) -> Self {
            Self {
                raw_arrow_schema: v.raw_arrow_schema.map(|v| v),
                schema: v.schema.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::dataset::SetDataSchema, SetDataSchema);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/SetInfo
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct SetInfo {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub keywords: Option<Vec<String>>,
    }

    impl IntoDto for SetInfo {
        type Dto = dtos::dataset::SetInfo;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::SetInfo> for SetInfo {
        fn from(v: dtos::dataset::SetInfo) -> Self {
            Self {
                description: v.description.map(|v| v),
                keywords: v.keywords.map(|v| v.into_iter().map(Into::into).collect()),
            }
        }
    }

    impl From<SetInfo> for dtos::dataset::SetInfo {
        fn from(v: SetInfo) -> Self {
            Self {
                description: v.description.map(|v| v),
                keywords: v.keywords.map(|v| v.into_iter().map(Into::into).collect()),
            }
        }
    }

    implement_serde_as!(dtos::dataset::SetInfo, SetInfo);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/SetLicense
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct SetLicense {
        pub short_name: String,
        pub name: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub spdx_id: Option<String>,
        pub website_url: String,
    }

    impl IntoDto for SetLicense {
        type Dto = dtos::dataset::SetLicense;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::SetLicense> for SetLicense {
        fn from(v: dtos::dataset::SetLicense) -> Self {
            Self {
                short_name: v.short_name,
                name: v.name,
                spdx_id: v.spdx_id.map(|v| v),
                website_url: v.website_url,
            }
        }
    }

    impl From<SetLicense> for dtos::dataset::SetLicense {
        fn from(v: SetLicense) -> Self {
            Self {
                short_name: v.short_name,
                name: v.name,
                spdx_id: v.spdx_id.map(|v| v),
                website_url: v.website_url,
            }
        }
    }

    implement_serde_as!(dtos::dataset::SetLicense, SetLicense);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/SetTransform
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct SetTransform {
        pub inputs: Vec<dataset::TransformInput>,
        pub transform: dataset::Transform,
    }

    impl IntoDto for SetTransform {
        type Dto = dtos::dataset::SetTransform;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::SetTransform> for SetTransform {
        fn from(v: dtos::dataset::SetTransform) -> Self {
            Self {
                inputs: v.inputs.into_iter().map(Into::into).collect(),
                transform: v.transform.into(),
            }
        }
    }

    impl From<SetTransform> for dtos::dataset::SetTransform {
        fn from(v: SetTransform) -> Self {
            Self {
                inputs: v.inputs.into_iter().map(Into::into).collect(),
                transform: v.transform.into(),
            }
        }
    }

    implement_serde_as!(dtos::dataset::SetTransform, SetTransform);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/SetVocab
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct SetVocab {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub offset_column: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operation_type_column: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub system_time_column: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub event_time_column: Option<String>,
    }

    impl IntoDto for SetVocab {
        type Dto = dtos::dataset::SetVocab;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::SetVocab> for SetVocab {
        fn from(v: dtos::dataset::SetVocab) -> Self {
            Self {
                offset_column: v.offset_column.map(|v| v),
                operation_type_column: v.operation_type_column.map(|v| v),
                system_time_column: v.system_time_column.map(|v| v),
                event_time_column: v.event_time_column.map(|v| v),
            }
        }
    }

    impl From<SetVocab> for dtos::dataset::SetVocab {
        fn from(v: SetVocab) -> Self {
            Self {
                offset_column: v.offset_column.map(|v| v),
                operation_type_column: v.operation_type_column.map(|v| v),
                system_time_column: v.system_time_column.map(|v| v),
                event_time_column: v.event_time_column.map(|v| v),
            }
        }
    }

    implement_serde_as!(dtos::dataset::SetVocab, SetVocab);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/SqlQueryStep
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct SqlQueryStep {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub alias: Option<String>,
        pub query: String,
    }

    impl IntoDto for SqlQueryStep {
        type Dto = dtos::dataset::SqlQueryStep;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::SqlQueryStep> for SqlQueryStep {
        fn from(v: dtos::dataset::SqlQueryStep) -> Self {
            Self {
                alias: v.alias.map(|v| v),
                query: v.query,
            }
        }
    }

    impl From<SqlQueryStep> for dtos::dataset::SqlQueryStep {
        fn from(v: SqlQueryStep) -> Self {
            Self {
                alias: v.alias.map(|v| v),
                query: v.query,
            }
        }
    }

    implement_serde_as!(dtos::dataset::SqlQueryStep, SqlQueryStep);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/TemporalTable
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct TemporalTable {
        pub name: String,
        pub primary_key: Vec<String>,
    }

    impl IntoDto for TemporalTable {
        type Dto = dtos::dataset::TemporalTable;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::TemporalTable> for TemporalTable {
        fn from(v: dtos::dataset::TemporalTable) -> Self {
            Self {
                name: v.name,
                primary_key: v.primary_key.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl From<TemporalTable> for dtos::dataset::TemporalTable {
        fn from(v: TemporalTable) -> Self {
            Self {
                name: v.name,
                primary_key: v.primary_key.into_iter().map(Into::into).collect(),
            }
        }
    }

    implement_serde_as!(dtos::dataset::TemporalTable, TemporalTable);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/Transform
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(tag = "kind")]
    pub enum Transform {
        #[serde(alias = "sql")]
        Sql(dataset::TransformSql),
    }

    impl IntoDto for Transform {
        type Dto = dtos::dataset::Transform;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::Transform> for Transform {
        fn from(v: dtos::dataset::Transform) -> Self {
            match v {
                dtos::dataset::Transform::Sql(v) => Self::Sql(v.into()),
            }
        }
    }

    impl From<Transform> for dtos::dataset::Transform {
        fn from(v: Transform) -> Self {
            match v {
                Transform::Sql(v) => Self::Sql(v.into()),
            }
        }
    }

    implement_serde_as!(dtos::dataset::Transform, Transform);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/TransformInput
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct TransformInput {
        pub dataset_ref: DatasetRef,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub alias: Option<String>,
    }

    impl IntoDto for TransformInput {
        type Dto = dtos::dataset::TransformInput;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::TransformInput> for TransformInput {
        fn from(v: dtos::dataset::TransformInput) -> Self {
            Self {
                dataset_ref: v.dataset_ref,
                alias: v.alias.map(|v| v),
            }
        }
    }

    impl From<TransformInput> for dtos::dataset::TransformInput {
        fn from(v: TransformInput) -> Self {
            Self {
                dataset_ref: v.dataset_ref,
                alias: v.alias.map(|v| v),
            }
        }
    }

    implement_serde_as!(dtos::dataset::TransformInput, TransformInput);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/Transform#/$defs/Sql
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct TransformSql {
        pub engine: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub query: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub queries: Option<Vec<dataset::SqlQueryStep>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub temporal_tables: Option<Vec<dataset::TemporalTable>>,
    }

    impl IntoDto for TransformSql {
        type Dto = dtos::dataset::TransformSql;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::TransformSql> for TransformSql {
        fn from(v: dtos::dataset::TransformSql) -> Self {
            Self {
                engine: v.engine,
                version: v.version.map(|v| v),
                query: v.query.map(|v| v),
                queries: v.queries.map(|v| v.into_iter().map(Into::into).collect()),
                temporal_tables: v
                    .temporal_tables
                    .map(|v| v.into_iter().map(Into::into).collect()),
            }
        }
    }

    impl From<TransformSql> for dtos::dataset::TransformSql {
        fn from(v: TransformSql) -> Self {
            Self {
                engine: v.engine,
                version: v.version.map(|v| v),
                query: v.query.map(|v| v),
                queries: v.queries.map(|v| v.into_iter().map(Into::into).collect()),
                temporal_tables: v
                    .temporal_tables
                    .map(|v| v.into_iter().map(Into::into).collect()),
            }
        }
    }

    implement_serde_as!(dtos::dataset::TransformSql, TransformSql);

    // Schema: https://opendatafabric.org/schemas/dataset/v1alpha1/Watermark
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct Watermark {
        #[serde(with = "datetime_rfc3339")]
        pub system_time: DateTime<Utc>,
        #[serde(with = "datetime_rfc3339")]
        pub event_time: DateTime<Utc>,
    }

    impl IntoDto for Watermark {
        type Dto = dtos::dataset::Watermark;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::dataset::Watermark> for Watermark {
        fn from(v: dtos::dataset::Watermark) -> Self {
            Self {
                system_time: v.system_time,
                event_time: v.event_time,
            }
        }
    }

    impl From<Watermark> for dtos::dataset::Watermark {
        fn from(v: Watermark) -> Self {
            Self {
                system_time: v.system_time,
                event_time: v.event_time,
            }
        }
    }

    implement_serde_as!(dtos::dataset::Watermark, Watermark);
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// engine
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod engine {
    #[allow(unused_imports)]
    use super::*;

    // Schema: https://opendatafabric.org/schemas/engine/v1alpha1/RawQueryRequest
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct RawQueryRequest {
        pub input_data_paths: Vec<PathBuf>,
        pub transform: dataset::Transform,
        pub output_data_path: PathBuf,
    }

    impl IntoDto for RawQueryRequest {
        type Dto = dtos::engine::RawQueryRequest;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::engine::RawQueryRequest> for RawQueryRequest {
        fn from(v: dtos::engine::RawQueryRequest) -> Self {
            Self {
                input_data_paths: v.input_data_paths.into_iter().map(Into::into).collect(),
                transform: v.transform.into(),
                output_data_path: v.output_data_path,
            }
        }
    }

    impl From<RawQueryRequest> for dtos::engine::RawQueryRequest {
        fn from(v: RawQueryRequest) -> Self {
            Self {
                input_data_paths: v.input_data_paths.into_iter().map(Into::into).collect(),
                transform: v.transform.into(),
                output_data_path: v.output_data_path,
            }
        }
    }

    implement_serde_as!(dtos::engine::RawQueryRequest, RawQueryRequest);

    // Schema: https://opendatafabric.org/schemas/engine/v1alpha1/RawQueryResponse
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(tag = "kind")]
    pub enum RawQueryResponse {
        #[serde(alias = "progress")]
        Progress(engine::RawQueryResponseProgress),
        #[serde(alias = "success")]
        Success(engine::RawQueryResponseSuccess),
        #[serde(alias = "invalidQuery", alias = "invalidquery")]
        InvalidQuery(engine::RawQueryResponseInvalidQuery),
        #[serde(alias = "internalError", alias = "internalerror")]
        InternalError(engine::RawQueryResponseInternalError),
    }

    impl IntoDto for RawQueryResponse {
        type Dto = dtos::engine::RawQueryResponse;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::engine::RawQueryResponse> for RawQueryResponse {
        fn from(v: dtos::engine::RawQueryResponse) -> Self {
            match v {
                dtos::engine::RawQueryResponse::Progress(v) => Self::Progress(v.into()),
                dtos::engine::RawQueryResponse::Success(v) => Self::Success(v.into()),
                dtos::engine::RawQueryResponse::InvalidQuery(v) => Self::InvalidQuery(v.into()),
                dtos::engine::RawQueryResponse::InternalError(v) => Self::InternalError(v.into()),
            }
        }
    }

    impl From<RawQueryResponse> for dtos::engine::RawQueryResponse {
        fn from(v: RawQueryResponse) -> Self {
            match v {
                RawQueryResponse::Progress(v) => Self::Progress(v.into()),
                RawQueryResponse::Success(v) => Self::Success(v.into()),
                RawQueryResponse::InvalidQuery(v) => Self::InvalidQuery(v.into()),
                RawQueryResponse::InternalError(v) => Self::InternalError(v.into()),
            }
        }
    }

    implement_serde_as!(dtos::engine::RawQueryResponse, RawQueryResponse);

    // Schema: https://opendatafabric.org/schemas/engine/v1alpha1/RawQueryResponse#/$defs/InternalError
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct RawQueryResponseInternalError {
        pub message: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub backtrace: Option<String>,
    }

    impl IntoDto for RawQueryResponseInternalError {
        type Dto = dtos::engine::RawQueryResponseInternalError;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::engine::RawQueryResponseInternalError> for RawQueryResponseInternalError {
        fn from(v: dtos::engine::RawQueryResponseInternalError) -> Self {
            Self {
                message: v.message,
                backtrace: v.backtrace.map(|v| v),
            }
        }
    }

    impl From<RawQueryResponseInternalError> for dtos::engine::RawQueryResponseInternalError {
        fn from(v: RawQueryResponseInternalError) -> Self {
            Self {
                message: v.message,
                backtrace: v.backtrace.map(|v| v),
            }
        }
    }

    implement_serde_as!(
        dtos::engine::RawQueryResponseInternalError,
        RawQueryResponseInternalError
    );

    // Schema: https://opendatafabric.org/schemas/engine/v1alpha1/RawQueryResponse#/$defs/InvalidQuery
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct RawQueryResponseInvalidQuery {
        pub message: String,
    }

    impl IntoDto for RawQueryResponseInvalidQuery {
        type Dto = dtos::engine::RawQueryResponseInvalidQuery;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::engine::RawQueryResponseInvalidQuery> for RawQueryResponseInvalidQuery {
        fn from(v: dtos::engine::RawQueryResponseInvalidQuery) -> Self {
            Self { message: v.message }
        }
    }

    impl From<RawQueryResponseInvalidQuery> for dtos::engine::RawQueryResponseInvalidQuery {
        fn from(v: RawQueryResponseInvalidQuery) -> Self {
            Self { message: v.message }
        }
    }

    implement_serde_as!(
        dtos::engine::RawQueryResponseInvalidQuery,
        RawQueryResponseInvalidQuery
    );

    // Schema: https://opendatafabric.org/schemas/engine/v1alpha1/RawQueryResponse#/$defs/Progress
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct RawQueryResponseProgress {}

    impl IntoDto for RawQueryResponseProgress {
        type Dto = dtos::engine::RawQueryResponseProgress;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::engine::RawQueryResponseProgress> for RawQueryResponseProgress {
        fn from(v: dtos::engine::RawQueryResponseProgress) -> Self {
            Self {}
        }
    }

    impl From<RawQueryResponseProgress> for dtos::engine::RawQueryResponseProgress {
        fn from(v: RawQueryResponseProgress) -> Self {
            Self {}
        }
    }

    implement_serde_as!(
        dtos::engine::RawQueryResponseProgress,
        RawQueryResponseProgress
    );

    // Schema: https://opendatafabric.org/schemas/engine/v1alpha1/RawQueryResponse#/$defs/Success
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct RawQueryResponseSuccess {
        pub num_records: u64,
    }

    impl IntoDto for RawQueryResponseSuccess {
        type Dto = dtos::engine::RawQueryResponseSuccess;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::engine::RawQueryResponseSuccess> for RawQueryResponseSuccess {
        fn from(v: dtos::engine::RawQueryResponseSuccess) -> Self {
            Self {
                num_records: v.num_records,
            }
        }
    }

    impl From<RawQueryResponseSuccess> for dtos::engine::RawQueryResponseSuccess {
        fn from(v: RawQueryResponseSuccess) -> Self {
            Self {
                num_records: v.num_records,
            }
        }
    }

    implement_serde_as!(
        dtos::engine::RawQueryResponseSuccess,
        RawQueryResponseSuccess
    );

    // Schema: https://opendatafabric.org/schemas/engine/v1alpha1/TransformRequest
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct TransformRequest {
        pub dataset_id: DatasetID,
        pub dataset_alias: DatasetAlias,
        #[serde(with = "datetime_rfc3339")]
        pub system_time: DateTime<Utc>,
        pub vocab: dataset::DatasetVocabulary,
        pub transform: dataset::Transform,
        pub query_inputs: Vec<engine::TransformRequestInput>,
        pub next_offset: u64,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prev_checkpoint_path: Option<PathBuf>,
        pub new_checkpoint_path: PathBuf,
        pub new_data_path: PathBuf,
    }

    impl IntoDto for TransformRequest {
        type Dto = dtos::engine::TransformRequest;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::engine::TransformRequest> for TransformRequest {
        fn from(v: dtos::engine::TransformRequest) -> Self {
            Self {
                dataset_id: v.dataset_id,
                dataset_alias: v.dataset_alias,
                system_time: v.system_time,
                vocab: v.vocab.into(),
                transform: v.transform.into(),
                query_inputs: v.query_inputs.into_iter().map(Into::into).collect(),
                next_offset: v.next_offset,
                prev_checkpoint_path: v.prev_checkpoint_path.map(|v| v),
                new_checkpoint_path: v.new_checkpoint_path,
                new_data_path: v.new_data_path,
            }
        }
    }

    impl From<TransformRequest> for dtos::engine::TransformRequest {
        fn from(v: TransformRequest) -> Self {
            Self {
                dataset_id: v.dataset_id,
                dataset_alias: v.dataset_alias,
                system_time: v.system_time,
                vocab: v.vocab.into(),
                transform: v.transform.into(),
                query_inputs: v.query_inputs.into_iter().map(Into::into).collect(),
                next_offset: v.next_offset,
                prev_checkpoint_path: v.prev_checkpoint_path.map(|v| v),
                new_checkpoint_path: v.new_checkpoint_path,
                new_data_path: v.new_data_path,
            }
        }
    }

    implement_serde_as!(dtos::engine::TransformRequest, TransformRequest);

    // Schema: https://opendatafabric.org/schemas/engine/v1alpha1/TransformRequestInput
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct TransformRequestInput {
        pub dataset_id: DatasetID,
        pub dataset_alias: DatasetAlias,
        pub query_alias: String,
        pub vocab: dataset::DatasetVocabulary,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub offset_interval: Option<dataset::OffsetInterval>,
        pub data_paths: Vec<PathBuf>,
        pub schema_file: PathBuf,
        pub explicit_watermarks: Vec<dataset::Watermark>,
    }

    impl IntoDto for TransformRequestInput {
        type Dto = dtos::engine::TransformRequestInput;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::engine::TransformRequestInput> for TransformRequestInput {
        fn from(v: dtos::engine::TransformRequestInput) -> Self {
            Self {
                dataset_id: v.dataset_id,
                dataset_alias: v.dataset_alias,
                query_alias: v.query_alias,
                vocab: v.vocab.into(),
                offset_interval: v.offset_interval.map(|v| v.into()),
                data_paths: v.data_paths.into_iter().map(Into::into).collect(),
                schema_file: v.schema_file,
                explicit_watermarks: v.explicit_watermarks.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl From<TransformRequestInput> for dtos::engine::TransformRequestInput {
        fn from(v: TransformRequestInput) -> Self {
            Self {
                dataset_id: v.dataset_id,
                dataset_alias: v.dataset_alias,
                query_alias: v.query_alias,
                vocab: v.vocab.into(),
                offset_interval: v.offset_interval.map(|v| v.into()),
                data_paths: v.data_paths.into_iter().map(Into::into).collect(),
                schema_file: v.schema_file,
                explicit_watermarks: v.explicit_watermarks.into_iter().map(Into::into).collect(),
            }
        }
    }

    implement_serde_as!(dtos::engine::TransformRequestInput, TransformRequestInput);

    // Schema: https://opendatafabric.org/schemas/engine/v1alpha1/TransformResponse
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(tag = "kind")]
    pub enum TransformResponse {
        #[serde(alias = "progress")]
        Progress(engine::TransformResponseProgress),
        #[serde(alias = "success")]
        Success(engine::TransformResponseSuccess),
        #[serde(alias = "invalidQuery", alias = "invalidquery")]
        InvalidQuery(engine::TransformResponseInvalidQuery),
        #[serde(alias = "internalError", alias = "internalerror")]
        InternalError(engine::TransformResponseInternalError),
    }

    impl IntoDto for TransformResponse {
        type Dto = dtos::engine::TransformResponse;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::engine::TransformResponse> for TransformResponse {
        fn from(v: dtos::engine::TransformResponse) -> Self {
            match v {
                dtos::engine::TransformResponse::Progress(v) => Self::Progress(v.into()),
                dtos::engine::TransformResponse::Success(v) => Self::Success(v.into()),
                dtos::engine::TransformResponse::InvalidQuery(v) => Self::InvalidQuery(v.into()),
                dtos::engine::TransformResponse::InternalError(v) => Self::InternalError(v.into()),
            }
        }
    }

    impl From<TransformResponse> for dtos::engine::TransformResponse {
        fn from(v: TransformResponse) -> Self {
            match v {
                TransformResponse::Progress(v) => Self::Progress(v.into()),
                TransformResponse::Success(v) => Self::Success(v.into()),
                TransformResponse::InvalidQuery(v) => Self::InvalidQuery(v.into()),
                TransformResponse::InternalError(v) => Self::InternalError(v.into()),
            }
        }
    }

    implement_serde_as!(dtos::engine::TransformResponse, TransformResponse);

    // Schema: https://opendatafabric.org/schemas/engine/v1alpha1/TransformResponse#/$defs/InternalError
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct TransformResponseInternalError {
        pub message: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub backtrace: Option<String>,
    }

    impl IntoDto for TransformResponseInternalError {
        type Dto = dtos::engine::TransformResponseInternalError;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::engine::TransformResponseInternalError> for TransformResponseInternalError {
        fn from(v: dtos::engine::TransformResponseInternalError) -> Self {
            Self {
                message: v.message,
                backtrace: v.backtrace.map(|v| v),
            }
        }
    }

    impl From<TransformResponseInternalError> for dtos::engine::TransformResponseInternalError {
        fn from(v: TransformResponseInternalError) -> Self {
            Self {
                message: v.message,
                backtrace: v.backtrace.map(|v| v),
            }
        }
    }

    implement_serde_as!(
        dtos::engine::TransformResponseInternalError,
        TransformResponseInternalError
    );

    // Schema: https://opendatafabric.org/schemas/engine/v1alpha1/TransformResponse#/$defs/InvalidQuery
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct TransformResponseInvalidQuery {
        pub message: String,
    }

    impl IntoDto for TransformResponseInvalidQuery {
        type Dto = dtos::engine::TransformResponseInvalidQuery;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::engine::TransformResponseInvalidQuery> for TransformResponseInvalidQuery {
        fn from(v: dtos::engine::TransformResponseInvalidQuery) -> Self {
            Self { message: v.message }
        }
    }

    impl From<TransformResponseInvalidQuery> for dtos::engine::TransformResponseInvalidQuery {
        fn from(v: TransformResponseInvalidQuery) -> Self {
            Self { message: v.message }
        }
    }

    implement_serde_as!(
        dtos::engine::TransformResponseInvalidQuery,
        TransformResponseInvalidQuery
    );

    // Schema: https://opendatafabric.org/schemas/engine/v1alpha1/TransformResponse#/$defs/Progress
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct TransformResponseProgress {}

    impl IntoDto for TransformResponseProgress {
        type Dto = dtos::engine::TransformResponseProgress;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::engine::TransformResponseProgress> for TransformResponseProgress {
        fn from(v: dtos::engine::TransformResponseProgress) -> Self {
            Self {}
        }
    }

    impl From<TransformResponseProgress> for dtos::engine::TransformResponseProgress {
        fn from(v: TransformResponseProgress) -> Self {
            Self {}
        }
    }

    implement_serde_as!(
        dtos::engine::TransformResponseProgress,
        TransformResponseProgress
    );

    // Schema: https://opendatafabric.org/schemas/engine/v1alpha1/TransformResponse#/$defs/Success
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct TransformResponseSuccess {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub new_offset_interval: Option<dataset::OffsetInterval>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(with = "datetime_rfc3339_opt")]
        pub new_watermark: Option<DateTime<Utc>>,
    }

    impl IntoDto for TransformResponseSuccess {
        type Dto = dtos::engine::TransformResponseSuccess;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::engine::TransformResponseSuccess> for TransformResponseSuccess {
        fn from(v: dtos::engine::TransformResponseSuccess) -> Self {
            Self {
                new_offset_interval: v.new_offset_interval.map(|v| v.into()),
                new_watermark: v.new_watermark.map(|v| v),
            }
        }
    }

    impl From<TransformResponseSuccess> for dtos::engine::TransformResponseSuccess {
        fn from(v: TransformResponseSuccess) -> Self {
            Self {
                new_offset_interval: v.new_offset_interval.map(|v| v.into()),
                new_watermark: v.new_watermark.map(|v| v),
            }
        }
    }

    implement_serde_as!(
        dtos::engine::TransformResponseSuccess,
        TransformResponseSuccess
    );
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// event
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod event {
    #[allow(unused_imports)]
    use super::*;

    // Schema: https://opendatafabric.org/schemas/event/v1alpha1/EventFilter
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EventFilter {
        #[serde(flatten)]
        #[serde(with = "map_value_limited_precision")]
        pub entries: std::collections::BTreeMap<String, serde_json::Value>,
    }

    impl IntoDto for EventFilter {
        type Dto = dtos::event::EventFilter;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::event::EventFilter> for EventFilter {
        fn from(v: dtos::event::EventFilter) -> Self {
            Self { entries: v.entries }
        }
    }

    impl From<EventFilter> for dtos::event::EventFilter {
        fn from(v: EventFilter) -> Self {
            Self { entries: v.entries }
        }
    }

    implement_serde_as!(dtos::event::EventFilter, EventFilter);
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// flow
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod flow {
    #[allow(unused_imports)]
    use super::*;

    // Schema: https://opendatafabric.org/schemas/flow/v1alpha1/FlowSpec
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct FlowSpec {
        pub target: StructOrString<resource::ResourceSelector>,
        pub triggers: Vec<flow::FlowTrigger>,
        pub tasks: Vec<flow::TaskSpec>,
    }

    impl IntoDto for FlowSpec {
        type Dto = dtos::flow::FlowSpec;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::flow::FlowSpec> for FlowSpec {
        fn from(v: dtos::flow::FlowSpec) -> Self {
            Self {
                target: v.target.into(),
                triggers: v.triggers.into_iter().map(Into::into).collect(),
                tasks: v.tasks.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl From<FlowSpec> for dtos::flow::FlowSpec {
        fn from(v: FlowSpec) -> Self {
            Self {
                target: v.target.into(),
                triggers: v.triggers.into_iter().map(Into::into).collect(),
                tasks: v.tasks.into_iter().map(Into::into).collect(),
            }
        }
    }

    implement_serde_as!(dtos::flow::FlowSpec, FlowSpec);

    // Schema: https://opendatafabric.org/schemas/flow/v1alpha1/FlowTrigger
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(tag = "kind")]
    pub enum FlowTrigger {
        #[serde(alias = "schedule")]
        Schedule(flow::FlowTriggerSchedule),
        #[serde(alias = "event")]
        Event(flow::FlowTriggerEvent),
        #[serde(alias = "source")]
        Source(flow::FlowTriggerSource),
        #[serde(alias = "dataset")]
        Dataset(flow::FlowTriggerDataset),
    }

    impl IntoDto for FlowTrigger {
        type Dto = dtos::flow::FlowTrigger;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::flow::FlowTrigger> for FlowTrigger {
        fn from(v: dtos::flow::FlowTrigger) -> Self {
            match v {
                dtos::flow::FlowTrigger::Schedule(v) => Self::Schedule(v.into()),
                dtos::flow::FlowTrigger::Event(v) => Self::Event(v.into()),
                dtos::flow::FlowTrigger::Source(v) => Self::Source(v.into()),
                dtos::flow::FlowTrigger::Dataset(v) => Self::Dataset(v.into()),
            }
        }
    }

    impl From<FlowTrigger> for dtos::flow::FlowTrigger {
        fn from(v: FlowTrigger) -> Self {
            match v {
                FlowTrigger::Schedule(v) => Self::Schedule(v.into()),
                FlowTrigger::Event(v) => Self::Event(v.into()),
                FlowTrigger::Source(v) => Self::Source(v.into()),
                FlowTrigger::Dataset(v) => Self::Dataset(v.into()),
            }
        }
    }

    implement_serde_as!(dtos::flow::FlowTrigger, FlowTrigger);

    // Schema: https://opendatafabric.org/schemas/flow/v1alpha1/FlowTrigger#/$defs/Dataset
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct FlowTriggerDataset {
        pub dataset: StructOrString<dataset::DatasetSelector>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub events: Option<Vec<String>>,
    }

    impl IntoDto for FlowTriggerDataset {
        type Dto = dtos::flow::FlowTriggerDataset;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::flow::FlowTriggerDataset> for FlowTriggerDataset {
        fn from(v: dtos::flow::FlowTriggerDataset) -> Self {
            Self {
                dataset: v.dataset.into(),
                events: v.events.map(|v| v.into_iter().map(Into::into).collect()),
            }
        }
    }

    impl From<FlowTriggerDataset> for dtos::flow::FlowTriggerDataset {
        fn from(v: FlowTriggerDataset) -> Self {
            Self {
                dataset: v.dataset.into(),
                events: v.events.map(|v| v.into_iter().map(Into::into).collect()),
            }
        }
    }

    implement_serde_as!(dtos::flow::FlowTriggerDataset, FlowTriggerDataset);

    // Schema: https://opendatafabric.org/schemas/flow/v1alpha1/FlowTrigger#/$defs/Event
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct FlowTriggerEvent {
        pub events: event::EventFilter,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cooldown: Option<DurationString>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cooldown_max_batch: Option<u64>,
    }

    impl IntoDto for FlowTriggerEvent {
        type Dto = dtos::flow::FlowTriggerEvent;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::flow::FlowTriggerEvent> for FlowTriggerEvent {
        fn from(v: dtos::flow::FlowTriggerEvent) -> Self {
            Self {
                events: v.events.into(),
                cooldown: v.cooldown.map(|v| v),
                cooldown_max_batch: v.cooldown_max_batch.map(|v| v),
            }
        }
    }

    impl From<FlowTriggerEvent> for dtos::flow::FlowTriggerEvent {
        fn from(v: FlowTriggerEvent) -> Self {
            Self {
                events: v.events.into(),
                cooldown: v.cooldown.map(|v| v),
                cooldown_max_batch: v.cooldown_max_batch.map(|v| v),
            }
        }
    }

    implement_serde_as!(dtos::flow::FlowTriggerEvent, FlowTriggerEvent);

    // Schema: https://opendatafabric.org/schemas/flow/v1alpha1/FlowTrigger#/$defs/Schedule
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct FlowTriggerSchedule {
        pub cron: String,
    }

    impl IntoDto for FlowTriggerSchedule {
        type Dto = dtos::flow::FlowTriggerSchedule;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::flow::FlowTriggerSchedule> for FlowTriggerSchedule {
        fn from(v: dtos::flow::FlowTriggerSchedule) -> Self {
            Self { cron: v.cron }
        }
    }

    impl From<FlowTriggerSchedule> for dtos::flow::FlowTriggerSchedule {
        fn from(v: FlowTriggerSchedule) -> Self {
            Self { cron: v.cron }
        }
    }

    implement_serde_as!(dtos::flow::FlowTriggerSchedule, FlowTriggerSchedule);

    // Schema: https://opendatafabric.org/schemas/flow/v1alpha1/FlowTrigger#/$defs/Source
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct FlowTriggerSource {
        pub source: StructOrString<resource::ResourceRef>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub min_records_to_await: Option<u64>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_await_interval: Option<DurationString>,
    }

    impl IntoDto for FlowTriggerSource {
        type Dto = dtos::flow::FlowTriggerSource;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::flow::FlowTriggerSource> for FlowTriggerSource {
        fn from(v: dtos::flow::FlowTriggerSource) -> Self {
            Self {
                source: v.source.into(),
                min_records_to_await: v.min_records_to_await.map(|v| v),
                max_await_interval: v.max_await_interval.map(|v| v),
            }
        }
    }

    impl From<FlowTriggerSource> for dtos::flow::FlowTriggerSource {
        fn from(v: FlowTriggerSource) -> Self {
            Self {
                source: v.source.into(),
                min_records_to_await: v.min_records_to_await.map(|v| v),
                max_await_interval: v.max_await_interval.map(|v| v),
            }
        }
    }

    implement_serde_as!(dtos::flow::FlowTriggerSource, FlowTriggerSource);

    // Schema: https://opendatafabric.org/schemas/flow/v1alpha1/TaskSpec
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(tag = "kind")]
    pub enum TaskSpec {
        #[serde(alias = "ingest")]
        Ingest(flow::TaskSpecIngest),
        #[serde(alias = "compaction")]
        Compaction(flow::TaskSpecCompaction),
        #[serde(alias = "garbageCollection", alias = "garbagecollection")]
        GarbageCollection(flow::TaskSpecGarbageCollection),
        #[serde(alias = "webhookCall", alias = "webhookcall")]
        WebhookCall(flow::TaskSpecWebhookCall),
    }

    impl IntoDto for TaskSpec {
        type Dto = dtos::flow::TaskSpec;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::flow::TaskSpec> for TaskSpec {
        fn from(v: dtos::flow::TaskSpec) -> Self {
            match v {
                dtos::flow::TaskSpec::Ingest(v) => Self::Ingest(v.into()),
                dtos::flow::TaskSpec::Compaction(v) => Self::Compaction(v.into()),
                dtos::flow::TaskSpec::GarbageCollection(v) => Self::GarbageCollection(v.into()),
                dtos::flow::TaskSpec::WebhookCall(v) => Self::WebhookCall(v.into()),
            }
        }
    }

    impl From<TaskSpec> for dtos::flow::TaskSpec {
        fn from(v: TaskSpec) -> Self {
            match v {
                TaskSpec::Ingest(v) => Self::Ingest(v.into()),
                TaskSpec::Compaction(v) => Self::Compaction(v.into()),
                TaskSpec::GarbageCollection(v) => Self::GarbageCollection(v.into()),
                TaskSpec::WebhookCall(v) => Self::WebhookCall(v.into()),
            }
        }
    }

    implement_serde_as!(dtos::flow::TaskSpec, TaskSpec);

    // Schema: https://opendatafabric.org/schemas/flow/v1alpha1/TaskSpec#/$defs/Compaction
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct TaskSpecCompaction {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub params: Option<dataset::CompactionParams>,
    }

    impl IntoDto for TaskSpecCompaction {
        type Dto = dtos::flow::TaskSpecCompaction;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::flow::TaskSpecCompaction> for TaskSpecCompaction {
        fn from(v: dtos::flow::TaskSpecCompaction) -> Self {
            Self {
                params: v.params.map(|v| v.into()),
            }
        }
    }

    impl From<TaskSpecCompaction> for dtos::flow::TaskSpecCompaction {
        fn from(v: TaskSpecCompaction) -> Self {
            Self {
                params: v.params.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::flow::TaskSpecCompaction, TaskSpecCompaction);

    // Schema: https://opendatafabric.org/schemas/flow/v1alpha1/TaskSpec#/$defs/GarbageCollection
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct TaskSpecGarbageCollection {}

    impl IntoDto for TaskSpecGarbageCollection {
        type Dto = dtos::flow::TaskSpecGarbageCollection;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::flow::TaskSpecGarbageCollection> for TaskSpecGarbageCollection {
        fn from(v: dtos::flow::TaskSpecGarbageCollection) -> Self {
            Self {}
        }
    }

    impl From<TaskSpecGarbageCollection> for dtos::flow::TaskSpecGarbageCollection {
        fn from(v: TaskSpecGarbageCollection) -> Self {
            Self {}
        }
    }

    implement_serde_as!(
        dtos::flow::TaskSpecGarbageCollection,
        TaskSpecGarbageCollection
    );

    // Schema: https://opendatafabric.org/schemas/flow/v1alpha1/TaskSpec#/$defs/Ingest
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct TaskSpecIngest {
        pub source: StructOrString<resource::ResourceRef>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub params: Option<source::IngestParams>,
    }

    impl IntoDto for TaskSpecIngest {
        type Dto = dtos::flow::TaskSpecIngest;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::flow::TaskSpecIngest> for TaskSpecIngest {
        fn from(v: dtos::flow::TaskSpecIngest) -> Self {
            Self {
                source: v.source.into(),
                params: v.params.map(|v| v.into()),
            }
        }
    }

    impl From<TaskSpecIngest> for dtos::flow::TaskSpecIngest {
        fn from(v: TaskSpecIngest) -> Self {
            Self {
                source: v.source.into(),
                params: v.params.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::flow::TaskSpecIngest, TaskSpecIngest);

    // Schema: https://opendatafabric.org/schemas/flow/v1alpha1/TaskSpec#/$defs/WebhookCall
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct TaskSpecWebhookCall {
        pub target: StructOrString<resource::ResourceRef>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub payload: Option<String>,
    }

    impl IntoDto for TaskSpecWebhookCall {
        type Dto = dtos::flow::TaskSpecWebhookCall;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::flow::TaskSpecWebhookCall> for TaskSpecWebhookCall {
        fn from(v: dtos::flow::TaskSpecWebhookCall) -> Self {
            Self {
                target: v.target.into(),
                payload: v.payload.map(|v| v),
            }
        }
    }

    impl From<TaskSpecWebhookCall> for dtos::flow::TaskSpecWebhookCall {
        fn from(v: TaskSpecWebhookCall) -> Self {
            Self {
                target: v.target.into(),
                payload: v.payload.map(|v| v),
            }
        }
    }

    implement_serde_as!(dtos::flow::TaskSpecWebhookCall, TaskSpecWebhookCall);
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// legacy
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod legacy {
    #[allow(unused_imports)]
    use super::*;

    // Schema: https://opendatafabric.org/schemas/legacy/v0/AddPushSource
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct AddPushSource {
        pub source_name: String,
        pub read: source::ReadStep,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub preprocess: Option<dataset::Transform>,
        pub merge: source::MergeStrategy,
    }

    impl IntoDto for AddPushSource {
        type Dto = dtos::legacy::AddPushSource;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::legacy::AddPushSource> for AddPushSource {
        fn from(v: dtos::legacy::AddPushSource) -> Self {
            Self {
                source_name: v.source_name,
                read: v.read.into(),
                preprocess: v.preprocess.map(|v| v.into()),
                merge: v.merge.into(),
            }
        }
    }

    impl From<AddPushSource> for dtos::legacy::AddPushSource {
        fn from(v: AddPushSource) -> Self {
            Self {
                source_name: v.source_name,
                read: v.read.into(),
                preprocess: v.preprocess.map(|v| v.into()),
                merge: v.merge.into(),
            }
        }
    }

    implement_serde_as!(dtos::legacy::AddPushSource, AddPushSource);

    // Schema: https://opendatafabric.org/schemas/legacy/v0/DatasetSnapshot
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DatasetSnapshot {
        pub name: DatasetAlias,
        pub kind: dataset::DatasetKind,
        pub metadata: Vec<dataset::MetadataEvent>,
    }

    impl IntoDto for DatasetSnapshot {
        type Dto = dtos::legacy::DatasetSnapshot;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::legacy::DatasetSnapshot> for DatasetSnapshot {
        fn from(v: dtos::legacy::DatasetSnapshot) -> Self {
            Self {
                name: v.name,
                kind: v.kind.into(),
                metadata: v.metadata.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl From<DatasetSnapshot> for dtos::legacy::DatasetSnapshot {
        fn from(v: DatasetSnapshot) -> Self {
            Self {
                name: v.name,
                kind: v.kind.into(),
                metadata: v.metadata.into_iter().map(Into::into).collect(),
            }
        }
    }

    implement_serde_as!(dtos::legacy::DatasetSnapshot, DatasetSnapshot);

    // Schema: https://opendatafabric.org/schemas/legacy/v0/DisablePollingSource
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DisablePollingSource {}

    impl IntoDto for DisablePollingSource {
        type Dto = dtos::legacy::DisablePollingSource;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::legacy::DisablePollingSource> for DisablePollingSource {
        fn from(v: dtos::legacy::DisablePollingSource) -> Self {
            Self {}
        }
    }

    impl From<DisablePollingSource> for dtos::legacy::DisablePollingSource {
        fn from(v: DisablePollingSource) -> Self {
            Self {}
        }
    }

    implement_serde_as!(dtos::legacy::DisablePollingSource, DisablePollingSource);

    // Schema: https://opendatafabric.org/schemas/legacy/v0/DisablePushSource
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct DisablePushSource {
        pub source_name: String,
    }

    impl IntoDto for DisablePushSource {
        type Dto = dtos::legacy::DisablePushSource;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::legacy::DisablePushSource> for DisablePushSource {
        fn from(v: dtos::legacy::DisablePushSource) -> Self {
            Self {
                source_name: v.source_name,
            }
        }
    }

    impl From<DisablePushSource> for dtos::legacy::DisablePushSource {
        fn from(v: DisablePushSource) -> Self {
            Self {
                source_name: v.source_name,
            }
        }
    }

    implement_serde_as!(dtos::legacy::DisablePushSource, DisablePushSource);

    // Schema: https://opendatafabric.org/schemas/legacy/v0/FetchStep
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(tag = "kind")]
    pub enum FetchStep {
        #[serde(alias = "url")]
        Url(legacy::FetchStepUrl),
        #[serde(alias = "filesGlob", alias = "filesglob")]
        FilesGlob(legacy::FetchStepFilesGlob),
        #[serde(alias = "container")]
        Container(legacy::FetchStepContainer),
        #[serde(alias = "mqtt")]
        Mqtt(legacy::FetchStepMqtt),
        #[serde(alias = "ethereumLogs", alias = "ethereumlogs")]
        EthereumLogs(legacy::FetchStepEthereumLogs),
    }

    impl IntoDto for FetchStep {
        type Dto = dtos::legacy::FetchStep;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::legacy::FetchStep> for FetchStep {
        fn from(v: dtos::legacy::FetchStep) -> Self {
            match v {
                dtos::legacy::FetchStep::Url(v) => Self::Url(v.into()),
                dtos::legacy::FetchStep::FilesGlob(v) => Self::FilesGlob(v.into()),
                dtos::legacy::FetchStep::Container(v) => Self::Container(v.into()),
                dtos::legacy::FetchStep::Mqtt(v) => Self::Mqtt(v.into()),
                dtos::legacy::FetchStep::EthereumLogs(v) => Self::EthereumLogs(v.into()),
            }
        }
    }

    impl From<FetchStep> for dtos::legacy::FetchStep {
        fn from(v: FetchStep) -> Self {
            match v {
                FetchStep::Url(v) => Self::Url(v.into()),
                FetchStep::FilesGlob(v) => Self::FilesGlob(v.into()),
                FetchStep::Container(v) => Self::Container(v.into()),
                FetchStep::Mqtt(v) => Self::Mqtt(v.into()),
                FetchStep::EthereumLogs(v) => Self::EthereumLogs(v.into()),
            }
        }
    }

    implement_serde_as!(dtos::legacy::FetchStep, FetchStep);

    // Schema: https://opendatafabric.org/schemas/legacy/v0/FetchStep#/$defs/Container
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct FetchStepContainer {
        pub image: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub command: Option<Vec<String>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub args: Option<Vec<String>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub env: Option<Vec<source::EnvVar>>,
    }

    impl IntoDto for FetchStepContainer {
        type Dto = dtos::legacy::FetchStepContainer;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::legacy::FetchStepContainer> for FetchStepContainer {
        fn from(v: dtos::legacy::FetchStepContainer) -> Self {
            Self {
                image: v.image,
                command: v.command.map(|v| v.into_iter().map(Into::into).collect()),
                args: v.args.map(|v| v.into_iter().map(Into::into).collect()),
                env: v.env.map(|v| v.into_iter().map(Into::into).collect()),
            }
        }
    }

    impl From<FetchStepContainer> for dtos::legacy::FetchStepContainer {
        fn from(v: FetchStepContainer) -> Self {
            Self {
                image: v.image,
                command: v.command.map(|v| v.into_iter().map(Into::into).collect()),
                args: v.args.map(|v| v.into_iter().map(Into::into).collect()),
                env: v.env.map(|v| v.into_iter().map(Into::into).collect()),
            }
        }
    }

    implement_serde_as!(dtos::legacy::FetchStepContainer, FetchStepContainer);

    // Schema: https://opendatafabric.org/schemas/legacy/v0/FetchStep#/$defs/EthereumLogs
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct FetchStepEthereumLogs {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub chain_id: Option<u64>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub node_url: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub filter: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signature: Option<String>,
    }

    impl IntoDto for FetchStepEthereumLogs {
        type Dto = dtos::legacy::FetchStepEthereumLogs;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::legacy::FetchStepEthereumLogs> for FetchStepEthereumLogs {
        fn from(v: dtos::legacy::FetchStepEthereumLogs) -> Self {
            Self {
                chain_id: v.chain_id.map(|v| v),
                node_url: v.node_url.map(|v| v),
                filter: v.filter.map(|v| v),
                signature: v.signature.map(|v| v),
            }
        }
    }

    impl From<FetchStepEthereumLogs> for dtos::legacy::FetchStepEthereumLogs {
        fn from(v: FetchStepEthereumLogs) -> Self {
            Self {
                chain_id: v.chain_id.map(|v| v),
                node_url: v.node_url.map(|v| v),
                filter: v.filter.map(|v| v),
                signature: v.signature.map(|v| v),
            }
        }
    }

    implement_serde_as!(dtos::legacy::FetchStepEthereumLogs, FetchStepEthereumLogs);

    // Schema: https://opendatafabric.org/schemas/legacy/v0/FetchStep#/$defs/FilesGlob
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct FetchStepFilesGlob {
        pub path: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub event_time: Option<UnionOrString<source::EventTimeSource>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cache: Option<UnionOrString<source::SourceCaching>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order: Option<source::SourceOrdering>,
    }

    impl IntoDto for FetchStepFilesGlob {
        type Dto = dtos::legacy::FetchStepFilesGlob;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::legacy::FetchStepFilesGlob> for FetchStepFilesGlob {
        fn from(v: dtos::legacy::FetchStepFilesGlob) -> Self {
            Self {
                path: v.path,
                event_time: v.event_time.map(|v| v.into()),
                cache: v.cache.map(|v| v.into()),
                order: v.order.map(|v| v.into()),
            }
        }
    }

    impl From<FetchStepFilesGlob> for dtos::legacy::FetchStepFilesGlob {
        fn from(v: FetchStepFilesGlob) -> Self {
            Self {
                path: v.path,
                event_time: v.event_time.map(|v| v.into()),
                cache: v.cache.map(|v| v.into()),
                order: v.order.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::legacy::FetchStepFilesGlob, FetchStepFilesGlob);

    // Schema: https://opendatafabric.org/schemas/legacy/v0/FetchStep#/$defs/Mqtt
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct FetchStepMqtt {
        pub host: String,
        pub port: i32,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub username: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub password: Option<String>,
        pub topics: Vec<source::MqttTopicSubscription>,
    }

    impl IntoDto for FetchStepMqtt {
        type Dto = dtos::legacy::FetchStepMqtt;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::legacy::FetchStepMqtt> for FetchStepMqtt {
        fn from(v: dtos::legacy::FetchStepMqtt) -> Self {
            Self {
                host: v.host,
                port: v.port,
                username: v.username.map(|v| v),
                password: v.password.map(|v| v),
                topics: v.topics.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl From<FetchStepMqtt> for dtos::legacy::FetchStepMqtt {
        fn from(v: FetchStepMqtt) -> Self {
            Self {
                host: v.host,
                port: v.port,
                username: v.username.map(|v| v),
                password: v.password.map(|v| v),
                topics: v.topics.into_iter().map(Into::into).collect(),
            }
        }
    }

    implement_serde_as!(dtos::legacy::FetchStepMqtt, FetchStepMqtt);

    // Schema: https://opendatafabric.org/schemas/legacy/v0/FetchStep#/$defs/Url
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct FetchStepUrl {
        pub url: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub event_time: Option<UnionOrString<source::EventTimeSource>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cache: Option<UnionOrString<source::SourceCaching>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub headers: Option<Vec<source::RequestHeader>>,
    }

    impl IntoDto for FetchStepUrl {
        type Dto = dtos::legacy::FetchStepUrl;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::legacy::FetchStepUrl> for FetchStepUrl {
        fn from(v: dtos::legacy::FetchStepUrl) -> Self {
            Self {
                url: v.url,
                event_time: v.event_time.map(|v| v.into()),
                cache: v.cache.map(|v| v.into()),
                headers: v.headers.map(|v| v.into_iter().map(Into::into).collect()),
            }
        }
    }

    impl From<FetchStepUrl> for dtos::legacy::FetchStepUrl {
        fn from(v: FetchStepUrl) -> Self {
            Self {
                url: v.url,
                event_time: v.event_time.map(|v| v.into()),
                cache: v.cache.map(|v| v.into()),
                headers: v.headers.map(|v| v.into_iter().map(Into::into).collect()),
            }
        }
    }

    implement_serde_as!(dtos::legacy::FetchStepUrl, FetchStepUrl);

    // Schema: https://opendatafabric.org/schemas/legacy/v0/Manifest
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct Manifest<ContentT> {
        pub kind: String,
        pub version: i32,
        pub content: ContentT,
    }

    impl<ContentT> IntoDto for Manifest<ContentT>
    where
        ContentT: IntoDto,
        <ContentT as IntoDto>::Dto: From<ContentT>,
    {
        type Dto = dtos::legacy::Manifest<ContentT::Dto>;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl<ContentTFrom, ContentTTo> From<dtos::legacy::Manifest<ContentTFrom>> for Manifest<ContentTTo>
    where
        ContentTTo: From<ContentTFrom>,
    {
        fn from(v: dtos::legacy::Manifest<ContentTFrom>) -> Self {
            Self {
                kind: v.kind,
                version: v.version,
                content: v.content.into(),
            }
        }
    }

    impl<ContentTFrom, ContentTTo> From<Manifest<ContentTFrom>> for dtos::legacy::Manifest<ContentTTo>
    where
        ContentTTo: From<ContentTFrom>,
    {
        fn from(v: Manifest<ContentTFrom>) -> Self {
            Self {
                kind: v.kind,
                version: v.version,
                content: v.content.into(),
            }
        }
    }

    // Schema: https://opendatafabric.org/schemas/legacy/v0/SetPollingSource
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct SetPollingSource {
        pub fetch: legacy::FetchStep,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prepare: Option<Vec<source::PrepStep>>,
        pub read: source::ReadStep,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub preprocess: Option<dataset::Transform>,
        pub merge: source::MergeStrategy,
    }

    impl IntoDto for SetPollingSource {
        type Dto = dtos::legacy::SetPollingSource;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::legacy::SetPollingSource> for SetPollingSource {
        fn from(v: dtos::legacy::SetPollingSource) -> Self {
            Self {
                fetch: v.fetch.into(),
                prepare: v.prepare.map(|v| v.into_iter().map(Into::into).collect()),
                read: v.read.into(),
                preprocess: v.preprocess.map(|v| v.into()),
                merge: v.merge.into(),
            }
        }
    }

    impl From<SetPollingSource> for dtos::legacy::SetPollingSource {
        fn from(v: SetPollingSource) -> Self {
            Self {
                fetch: v.fetch.into(),
                prepare: v.prepare.map(|v| v.into_iter().map(Into::into).collect()),
                read: v.read.into(),
                preprocess: v.preprocess.map(|v| v.into()),
                merge: v.merge.into(),
            }
        }
    }

    implement_serde_as!(dtos::legacy::SetPollingSource, SetPollingSource);
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// resource
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod resource {
    #[allow(unused_imports)]
    use super::*;

    // Schema: https://opendatafabric.org/schemas/resource/v1alpha1/LabelFilter
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LabelFilter {
        #[serde(flatten)]
        #[serde(with = "map_value_limited_precision")]
        pub entries: std::collections::BTreeMap<String, serde_json::Value>,
    }

    impl IntoDto for LabelFilter {
        type Dto = dtos::resource::LabelFilter;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::resource::LabelFilter> for LabelFilter {
        fn from(v: dtos::resource::LabelFilter) -> Self {
            Self { entries: v.entries }
        }
    }

    impl From<LabelFilter> for dtos::resource::LabelFilter {
        fn from(v: LabelFilter) -> Self {
            Self { entries: v.entries }
        }
    }

    implement_serde_as!(dtos::resource::LabelFilter, LabelFilter);

    // Schema: https://opendatafabric.org/schemas/resource/v1alpha1/Resource
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct Resource<SpecT> {
        #[serde(rename = "$schema")]
        pub schema: ResourceTypeUri,
        pub headers: resource::ResourceHeaders,
        pub spec: SpecT,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<resource::ResourceStatus>,
    }

    impl<SpecT> IntoDto for Resource<SpecT>
    where
        SpecT: IntoDto,
        <SpecT as IntoDto>::Dto: From<SpecT>,
    {
        type Dto = dtos::resource::Resource<SpecT::Dto>;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl<SpecTFrom, SpecTTo> From<dtos::resource::Resource<SpecTFrom>> for Resource<SpecTTo>
    where
        SpecTTo: From<SpecTFrom>,
    {
        fn from(v: dtos::resource::Resource<SpecTFrom>) -> Self {
            Self {
                schema: v.schema,
                headers: v.headers.into(),
                spec: v.spec.into(),
                status: v.status.map(|v| v.into()),
            }
        }
    }

    impl<SpecTFrom, SpecTTo> From<Resource<SpecTFrom>> for dtos::resource::Resource<SpecTTo>
    where
        SpecTTo: From<SpecTFrom>,
    {
        fn from(v: Resource<SpecTFrom>) -> Self {
            Self {
                schema: v.schema,
                headers: v.headers.into(),
                spec: v.spec.into(),
                status: v.status.map(|v| v.into()),
            }
        }
    }

    // Schema: https://opendatafabric.org/schemas/resource/v1alpha1/ResourceAnnotations
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ResourceAnnotations {
        #[serde(flatten)]
        #[serde(with = "map_value_limited_precision")]
        pub entries: std::collections::BTreeMap<String, serde_json::Value>,
    }

    impl IntoDto for ResourceAnnotations {
        type Dto = dtos::resource::ResourceAnnotations;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::resource::ResourceAnnotations> for ResourceAnnotations {
        fn from(v: dtos::resource::ResourceAnnotations) -> Self {
            Self { entries: v.entries }
        }
    }

    impl From<ResourceAnnotations> for dtos::resource::ResourceAnnotations {
        fn from(v: ResourceAnnotations) -> Self {
            Self { entries: v.entries }
        }
    }

    implement_serde_as!(dtos::resource::ResourceAnnotations, ResourceAnnotations);

    // Schema: https://opendatafabric.org/schemas/resource/v1alpha1/ResourceCondition
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct ResourceCondition {
        pub value: serde_json::Value,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reason: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(with = "datetime_rfc3339_opt")]
        pub last_transition_time: Option<DateTime<Utc>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub observed_generation: Option<u64>,
    }

    impl IntoDto for ResourceCondition {
        type Dto = dtos::resource::ResourceCondition;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::resource::ResourceCondition> for ResourceCondition {
        fn from(v: dtos::resource::ResourceCondition) -> Self {
            Self {
                value: v.value,
                reason: v.reason.map(|v| v),
                message: v.message.map(|v| v),
                last_transition_time: v.last_transition_time.map(|v| v),
                observed_generation: v.observed_generation.map(|v| v),
            }
        }
    }

    impl From<ResourceCondition> for dtos::resource::ResourceCondition {
        fn from(v: ResourceCondition) -> Self {
            Self {
                value: v.value,
                reason: v.reason.map(|v| v),
                message: v.message.map(|v| v),
                last_transition_time: v.last_transition_time.map(|v| v),
                observed_generation: v.observed_generation.map(|v| v),
            }
        }
    }

    implement_serde_as!(dtos::resource::ResourceCondition, ResourceCondition);

    // Schema: https://opendatafabric.org/schemas/resource/v1alpha1/ResourceConditions
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ResourceConditions {
        #[serde(flatten)]
        pub entries: std::collections::BTreeMap<String, resource::ResourceCondition>,
    }

    impl IntoDto for ResourceConditions {
        type Dto = dtos::resource::ResourceConditions;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::resource::ResourceConditions> for ResourceConditions {
        fn from(v: dtos::resource::ResourceConditions) -> Self {
            Self {
                entries: v.entries.into_iter().map(|(k, v)| (k, v.into())).collect(),
            }
        }
    }

    impl From<ResourceConditions> for dtos::resource::ResourceConditions {
        fn from(v: ResourceConditions) -> Self {
            Self {
                entries: v.entries.into_iter().map(|(k, v)| (k, v.into())).collect(),
            }
        }
    }

    implement_serde_as!(dtos::resource::ResourceConditions, ResourceConditions);

    // Schema: https://opendatafabric.org/schemas/resource/v1alpha1/ResourceHeaders
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct ResourceHeaders {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<ResourceID>,
        pub name: ResourceName,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account: Option<StructOrString<auth::AccountRef>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub labels: Option<resource::ResourceLabels>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub annotations: Option<resource::ResourceAnnotations>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub generation: Option<u64>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(with = "datetime_rfc3339_opt")]
        pub created_at: Option<DateTime<Utc>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(with = "datetime_rfc3339_opt")]
        pub updated_at: Option<DateTime<Utc>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(with = "datetime_rfc3339_opt")]
        pub deleted_at: Option<DateTime<Utc>>,
    }

    impl IntoDto for ResourceHeaders {
        type Dto = dtos::resource::ResourceHeaders;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::resource::ResourceHeaders> for ResourceHeaders {
        fn from(v: dtos::resource::ResourceHeaders) -> Self {
            Self {
                id: v.id.map(|v| v),
                name: v.name,
                account: v.account.map(|v| v.into()),
                labels: v.labels.map(|v| v.into()),
                annotations: v.annotations.map(|v| v.into()),
                generation: v.generation.map(|v| v),
                created_at: v.created_at.map(|v| v),
                updated_at: v.updated_at.map(|v| v),
                deleted_at: v.deleted_at.map(|v| v),
            }
        }
    }

    impl From<ResourceHeaders> for dtos::resource::ResourceHeaders {
        fn from(v: ResourceHeaders) -> Self {
            Self {
                id: v.id.map(|v| v),
                name: v.name,
                account: v.account.map(|v| v.into()),
                labels: v.labels.map(|v| v.into()),
                annotations: v.annotations.map(|v| v.into()),
                generation: v.generation.map(|v| v),
                created_at: v.created_at.map(|v| v),
                updated_at: v.updated_at.map(|v| v),
                deleted_at: v.deleted_at.map(|v| v),
            }
        }
    }

    implement_serde_as!(dtos::resource::ResourceHeaders, ResourceHeaders);

    // Schema: https://opendatafabric.org/schemas/resource/v1alpha1/ResourceLabels
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ResourceLabels {
        #[serde(flatten)]
        #[serde(with = "map_value_limited_precision")]
        pub entries: std::collections::BTreeMap<String, serde_json::Value>,
    }

    impl IntoDto for ResourceLabels {
        type Dto = dtos::resource::ResourceLabels;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::resource::ResourceLabels> for ResourceLabels {
        fn from(v: dtos::resource::ResourceLabels) -> Self {
            Self { entries: v.entries }
        }
    }

    impl From<ResourceLabels> for dtos::resource::ResourceLabels {
        fn from(v: ResourceLabels) -> Self {
            Self { entries: v.entries }
        }
    }

    implement_serde_as!(dtos::resource::ResourceLabels, ResourceLabels);

    // Schema: https://opendatafabric.org/schemas/resource/v1alpha1/ResourcePhase
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub enum ResourcePhase {
        #[serde(alias = "pending")]
        Pending,
        #[serde(alias = "reconciling")]
        Reconciling,
        #[serde(alias = "ready")]
        Ready,
        #[serde(alias = "failed")]
        Failed,
    }

    impl IntoDto for ResourcePhase {
        type Dto = dtos::resource::ResourcePhase;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::resource::ResourcePhase> for ResourcePhase {
        fn from(v: dtos::resource::ResourcePhase) -> Self {
            match v {
                dtos::resource::ResourcePhase::Pending => Self::Pending,
                dtos::resource::ResourcePhase::Reconciling => Self::Reconciling,
                dtos::resource::ResourcePhase::Ready => Self::Ready,
                dtos::resource::ResourcePhase::Failed => Self::Failed,
            }
        }
    }

    impl From<ResourcePhase> for dtos::resource::ResourcePhase {
        fn from(v: ResourcePhase) -> Self {
            match v {
                ResourcePhase::Pending => Self::Pending,
                ResourcePhase::Reconciling => Self::Reconciling,
                ResourcePhase::Ready => Self::Ready,
                ResourcePhase::Failed => Self::Failed,
            }
        }
    }

    implement_serde_as!(dtos::resource::ResourcePhase, ResourcePhase);

    // Schema: https://opendatafabric.org/schemas/resource/v1alpha1/ResourceRef
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct ResourceRef {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account: Option<StructOrString<auth::AccountRef>>,
        pub r#type: ResourceTypeRef,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<ResourceID>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<ResourceName>,
    }

    impl IntoDto for ResourceRef {
        type Dto = dtos::resource::ResourceRef;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::resource::ResourceRef> for StructOrString<ResourceRef> {
        fn from(v: dtos::resource::ResourceRef) -> Self {
            Self(v.into())
        }
    }
    impl From<StructOrString<ResourceRef>> for dtos::resource::ResourceRef {
        fn from(v: StructOrString<ResourceRef>) -> Self {
            v.0.into()
        }
    }

    implement_serde_as!(dtos::resource::ResourceRef, ResourceRef);

    // Schema: https://opendatafabric.org/schemas/resource/v1alpha1/ResourceSelector
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct ResourceSelector {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account: Option<StructOrString<auth::AccountRef>>,
        pub r#type: ResourceTypeRef,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<ResourceID>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub labels: Option<resource::LabelFilter>,
    }

    impl IntoDto for ResourceSelector {
        type Dto = dtos::resource::ResourceSelector;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::resource::ResourceSelector> for StructOrString<ResourceSelector> {
        fn from(v: dtos::resource::ResourceSelector) -> Self {
            Self(v.into())
        }
    }
    impl From<StructOrString<ResourceSelector>> for dtos::resource::ResourceSelector {
        fn from(v: StructOrString<ResourceSelector>) -> Self {
            v.0.into()
        }
    }

    implement_serde_as!(dtos::resource::ResourceSelector, ResourceSelector);

    // Schema: https://opendatafabric.org/schemas/resource/v1alpha1/ResourceStatus
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct ResourceStatus {
        pub phase: resource::ResourcePhase,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub observed_generation: Option<u64>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(with = "datetime_rfc3339_opt")]
        pub reconciled_at: Option<DateTime<Utc>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub conditions: Option<resource::ResourceConditions>,
    }

    impl IntoDto for ResourceStatus {
        type Dto = dtos::resource::ResourceStatus;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::resource::ResourceStatus> for ResourceStatus {
        fn from(v: dtos::resource::ResourceStatus) -> Self {
            Self {
                phase: v.phase.into(),
                observed_generation: v.observed_generation.map(|v| v),
                reconciled_at: v.reconciled_at.map(|v| v),
                conditions: v.conditions.map(|v| v.into()),
            }
        }
    }

    impl From<ResourceStatus> for dtos::resource::ResourceStatus {
        fn from(v: ResourceStatus) -> Self {
            Self {
                phase: v.phase.into(),
                observed_generation: v.observed_generation.map(|v| v),
                reconciled_at: v.reconciled_at.map(|v| v),
                conditions: v.conditions.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::resource::ResourceStatus, ResourceStatus);
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// sink
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod sink {
    #[allow(unused_imports)]
    use super::*;

    // Schema: https://opendatafabric.org/schemas/sink/v1alpha1/WebhookTargetSpec
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct WebhookTargetSpec {
        pub url: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub secret: Option<StructOrString<config::Secret>>,
    }

    impl IntoDto for WebhookTargetSpec {
        type Dto = dtos::sink::WebhookTargetSpec;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::sink::WebhookTargetSpec> for WebhookTargetSpec {
        fn from(v: dtos::sink::WebhookTargetSpec) -> Self {
            Self {
                url: v.url,
                secret: v.secret.map(|v| v.into()),
            }
        }
    }

    impl From<WebhookTargetSpec> for dtos::sink::WebhookTargetSpec {
        fn from(v: WebhookTargetSpec) -> Self {
            Self {
                url: v.url,
                secret: v.secret.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::sink::WebhookTargetSpec, WebhookTargetSpec);

    // Schema: https://opendatafabric.org/schemas/sink/v1alpha1/WebhookTargetStatus
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct WebhookTargetStatus {
        pub value: sink::WebhookTargetStatusValue,
    }

    impl IntoDto for WebhookTargetStatus {
        type Dto = dtos::sink::WebhookTargetStatus;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::sink::WebhookTargetStatus> for WebhookTargetStatus {
        fn from(v: dtos::sink::WebhookTargetStatus) -> Self {
            Self {
                value: v.value.into(),
            }
        }
    }

    impl From<WebhookTargetStatus> for dtos::sink::WebhookTargetStatus {
        fn from(v: WebhookTargetStatus) -> Self {
            Self {
                value: v.value.into(),
            }
        }
    }

    implement_serde_as!(dtos::sink::WebhookTargetStatus, WebhookTargetStatus);

    // Schema: https://opendatafabric.org/schemas/sink/v1alpha1/WebhookTargetStatus#/$defs/Value
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub enum WebhookTargetStatusValue {
        #[serde(alias = "ready")]
        Ready,
        #[serde(alias = "failed")]
        Failed,
    }

    impl IntoDto for WebhookTargetStatusValue {
        type Dto = dtos::sink::WebhookTargetStatusValue;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::sink::WebhookTargetStatusValue> for WebhookTargetStatusValue {
        fn from(v: dtos::sink::WebhookTargetStatusValue) -> Self {
            match v {
                dtos::sink::WebhookTargetStatusValue::Ready => Self::Ready,
                dtos::sink::WebhookTargetStatusValue::Failed => Self::Failed,
            }
        }
    }

    impl From<WebhookTargetStatusValue> for dtos::sink::WebhookTargetStatusValue {
        fn from(v: WebhookTargetStatusValue) -> Self {
            match v {
                WebhookTargetStatusValue::Ready => Self::Ready,
                WebhookTargetStatusValue::Failed => Self::Failed,
            }
        }
    }

    implement_serde_as!(
        dtos::sink::WebhookTargetStatusValue,
        WebhookTargetStatusValue
    );
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// source
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod source {
    #[allow(unused_imports)]
    use super::*;

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/CompressionFormat
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub enum CompressionFormat {
        #[serde(alias = "gzip")]
        Gzip,
        #[serde(alias = "zip")]
        Zip,
    }

    impl IntoDto for CompressionFormat {
        type Dto = dtos::source::CompressionFormat;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::CompressionFormat> for CompressionFormat {
        fn from(v: dtos::source::CompressionFormat) -> Self {
            match v {
                dtos::source::CompressionFormat::Gzip => Self::Gzip,
                dtos::source::CompressionFormat::Zip => Self::Zip,
            }
        }
    }

    impl From<CompressionFormat> for dtos::source::CompressionFormat {
        fn from(v: CompressionFormat) -> Self {
            match v {
                CompressionFormat::Gzip => Self::Gzip,
                CompressionFormat::Zip => Self::Zip,
            }
        }
    }

    implement_serde_as!(dtos::source::CompressionFormat, CompressionFormat);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/EnvVar
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct EnvVar {
        pub name: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    impl IntoDto for EnvVar {
        type Dto = dtos::source::EnvVar;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::EnvVar> for EnvVar {
        fn from(v: dtos::source::EnvVar) -> Self {
            Self {
                name: v.name,
                value: v.value.map(|v| v),
            }
        }
    }

    impl From<EnvVar> for dtos::source::EnvVar {
        fn from(v: EnvVar) -> Self {
            Self {
                name: v.name,
                value: v.value.map(|v| v),
            }
        }
    }

    implement_serde_as!(dtos::source::EnvVar, EnvVar);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/EventTimeSource
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(tag = "kind")]
    pub enum EventTimeSource {
        #[serde(alias = "fromMetadata", alias = "frommetadata")]
        FromMetadata(source::EventTimeSourceFromMetadata),
        #[serde(alias = "fromPath", alias = "frompath")]
        FromPath(source::EventTimeSourceFromPath),
        #[serde(alias = "fromSystemTime", alias = "fromsystemtime")]
        FromSystemTime(source::EventTimeSourceFromSystemTime),
    }

    impl From<dtos::source::EventTimeSource> for UnionOrString<EventTimeSource> {
        fn from(v: dtos::source::EventTimeSource) -> Self {
            Self(v.into())
        }
    }
    impl From<UnionOrString<EventTimeSource>> for dtos::source::EventTimeSource {
        fn from(v: UnionOrString<EventTimeSource>) -> Self {
            v.0.into()
        }
    }

    impl IntoDto for EventTimeSource {
        type Dto = dtos::source::EventTimeSource;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::EventTimeSource> for EventTimeSource {
        fn from(v: dtos::source::EventTimeSource) -> Self {
            match v {
                dtos::source::EventTimeSource::FromMetadata(v) => Self::FromMetadata(v.into()),
                dtos::source::EventTimeSource::FromPath(v) => Self::FromPath(v.into()),
                dtos::source::EventTimeSource::FromSystemTime(v) => Self::FromSystemTime(v.into()),
            }
        }
    }

    impl From<EventTimeSource> for dtos::source::EventTimeSource {
        fn from(v: EventTimeSource) -> Self {
            match v {
                EventTimeSource::FromMetadata(v) => Self::FromMetadata(v.into()),
                EventTimeSource::FromPath(v) => Self::FromPath(v.into()),
                EventTimeSource::FromSystemTime(v) => Self::FromSystemTime(v.into()),
            }
        }
    }

    implement_serde_as!(dtos::source::EventTimeSource, EventTimeSource);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/EventTimeSource#/$defs/FromMetadata
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct EventTimeSourceFromMetadata {}

    impl IntoDto for EventTimeSourceFromMetadata {
        type Dto = dtos::source::EventTimeSourceFromMetadata;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::EventTimeSourceFromMetadata> for EventTimeSourceFromMetadata {
        fn from(v: dtos::source::EventTimeSourceFromMetadata) -> Self {
            Self {}
        }
    }

    impl From<EventTimeSourceFromMetadata> for dtos::source::EventTimeSourceFromMetadata {
        fn from(v: EventTimeSourceFromMetadata) -> Self {
            Self {}
        }
    }

    implement_serde_as!(
        dtos::source::EventTimeSourceFromMetadata,
        EventTimeSourceFromMetadata
    );

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/EventTimeSource#/$defs/FromPath
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct EventTimeSourceFromPath {
        pub pattern: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timestamp_format: Option<String>,
    }

    impl IntoDto for EventTimeSourceFromPath {
        type Dto = dtos::source::EventTimeSourceFromPath;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::EventTimeSourceFromPath> for EventTimeSourceFromPath {
        fn from(v: dtos::source::EventTimeSourceFromPath) -> Self {
            Self {
                pattern: v.pattern,
                timestamp_format: v.timestamp_format.map(|v| v),
            }
        }
    }

    impl From<EventTimeSourceFromPath> for dtos::source::EventTimeSourceFromPath {
        fn from(v: EventTimeSourceFromPath) -> Self {
            Self {
                pattern: v.pattern,
                timestamp_format: v.timestamp_format.map(|v| v),
            }
        }
    }

    implement_serde_as!(
        dtos::source::EventTimeSourceFromPath,
        EventTimeSourceFromPath
    );

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/EventTimeSource#/$defs/FromSystemTime
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct EventTimeSourceFromSystemTime {}

    impl IntoDto for EventTimeSourceFromSystemTime {
        type Dto = dtos::source::EventTimeSourceFromSystemTime;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::EventTimeSourceFromSystemTime> for EventTimeSourceFromSystemTime {
        fn from(v: dtos::source::EventTimeSourceFromSystemTime) -> Self {
            Self {}
        }
    }

    impl From<EventTimeSourceFromSystemTime> for dtos::source::EventTimeSourceFromSystemTime {
        fn from(v: EventTimeSourceFromSystemTime) -> Self {
            Self {}
        }
    }

    implement_serde_as!(
        dtos::source::EventTimeSourceFromSystemTime,
        EventTimeSourceFromSystemTime
    );

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/IngestParams
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct IngestParams {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target_slice_records: Option<u64>,
    }

    impl IntoDto for IngestParams {
        type Dto = dtos::source::IngestParams;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::IngestParams> for IngestParams {
        fn from(v: dtos::source::IngestParams) -> Self {
            Self {
                target_slice_records: v.target_slice_records.map(|v| v),
            }
        }
    }

    impl From<IngestParams> for dtos::source::IngestParams {
        fn from(v: IngestParams) -> Self {
            Self {
                target_slice_records: v.target_slice_records.map(|v| v),
            }
        }
    }

    implement_serde_as!(dtos::source::IngestParams, IngestParams);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/Ingress
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(tag = "kind")]
    pub enum Ingress {
        #[serde(alias = "url")]
        Url(source::IngressUrl),
        #[serde(alias = "filesGlob", alias = "filesglob")]
        FilesGlob(source::IngressFilesGlob),
        #[serde(alias = "container")]
        Container(source::IngressContainer),
        #[serde(alias = "mqtt")]
        Mqtt(source::IngressMqtt),
        #[serde(alias = "evmLogs", alias = "evmlogs")]
        EvmLogs(source::IngressEvmLogs),
        #[serde(alias = "restEndpoint", alias = "restendpoint")]
        RestEndpoint(source::IngressRestEndpoint),
    }

    impl IntoDto for Ingress {
        type Dto = dtos::source::Ingress;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::Ingress> for Ingress {
        fn from(v: dtos::source::Ingress) -> Self {
            match v {
                dtos::source::Ingress::Url(v) => Self::Url(v.into()),
                dtos::source::Ingress::FilesGlob(v) => Self::FilesGlob(v.into()),
                dtos::source::Ingress::Container(v) => Self::Container(v.into()),
                dtos::source::Ingress::Mqtt(v) => Self::Mqtt(v.into()),
                dtos::source::Ingress::EvmLogs(v) => Self::EvmLogs(v.into()),
                dtos::source::Ingress::RestEndpoint(v) => Self::RestEndpoint(v.into()),
            }
        }
    }

    impl From<Ingress> for dtos::source::Ingress {
        fn from(v: Ingress) -> Self {
            match v {
                Ingress::Url(v) => Self::Url(v.into()),
                Ingress::FilesGlob(v) => Self::FilesGlob(v.into()),
                Ingress::Container(v) => Self::Container(v.into()),
                Ingress::Mqtt(v) => Self::Mqtt(v.into()),
                Ingress::EvmLogs(v) => Self::EvmLogs(v.into()),
                Ingress::RestEndpoint(v) => Self::RestEndpoint(v.into()),
            }
        }
    }

    implement_serde_as!(dtos::source::Ingress, Ingress);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/IngressBuffer
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(tag = "kind")]
    pub enum IngressBuffer {
        #[serde(alias = "memory")]
        Memory(source::IngressBufferMemory),
    }

    impl IntoDto for IngressBuffer {
        type Dto = dtos::source::IngressBuffer;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::IngressBuffer> for IngressBuffer {
        fn from(v: dtos::source::IngressBuffer) -> Self {
            match v {
                dtos::source::IngressBuffer::Memory(v) => Self::Memory(v.into()),
            }
        }
    }

    impl From<IngressBuffer> for dtos::source::IngressBuffer {
        fn from(v: IngressBuffer) -> Self {
            match v {
                IngressBuffer::Memory(v) => Self::Memory(v.into()),
            }
        }
    }

    implement_serde_as!(dtos::source::IngressBuffer, IngressBuffer);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/IngressBuffer#/$defs/Memory
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct IngressBufferMemory {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub buffer_size: Option<u64>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub overflow_policy: Option<String>,
    }

    impl IntoDto for IngressBufferMemory {
        type Dto = dtos::source::IngressBufferMemory;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::IngressBufferMemory> for IngressBufferMemory {
        fn from(v: dtos::source::IngressBufferMemory) -> Self {
            Self {
                buffer_size: v.buffer_size.map(|v| v),
                overflow_policy: v.overflow_policy.map(|v| v),
            }
        }
    }

    impl From<IngressBufferMemory> for dtos::source::IngressBufferMemory {
        fn from(v: IngressBufferMemory) -> Self {
            Self {
                buffer_size: v.buffer_size.map(|v| v),
                overflow_policy: v.overflow_policy.map(|v| v),
            }
        }
    }

    implement_serde_as!(dtos::source::IngressBufferMemory, IngressBufferMemory);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/Ingress#/$defs/Container
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct IngressContainer {
        pub image: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub command: Option<Vec<String>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub args: Option<Vec<String>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub env: Option<Vec<source::EnvVar>>,
    }

    impl IntoDto for IngressContainer {
        type Dto = dtos::source::IngressContainer;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::IngressContainer> for IngressContainer {
        fn from(v: dtos::source::IngressContainer) -> Self {
            Self {
                image: v.image,
                command: v.command.map(|v| v.into_iter().map(Into::into).collect()),
                args: v.args.map(|v| v.into_iter().map(Into::into).collect()),
                env: v.env.map(|v| v.into_iter().map(Into::into).collect()),
            }
        }
    }

    impl From<IngressContainer> for dtos::source::IngressContainer {
        fn from(v: IngressContainer) -> Self {
            Self {
                image: v.image,
                command: v.command.map(|v| v.into_iter().map(Into::into).collect()),
                args: v.args.map(|v| v.into_iter().map(Into::into).collect()),
                env: v.env.map(|v| v.into_iter().map(Into::into).collect()),
            }
        }
    }

    implement_serde_as!(dtos::source::IngressContainer, IngressContainer);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/Ingress#/$defs/EvmLogs
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct IngressEvmLogs {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub chain_id: Option<u64>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub node_url: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub filter: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signature: Option<String>,
    }

    impl IntoDto for IngressEvmLogs {
        type Dto = dtos::source::IngressEvmLogs;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::IngressEvmLogs> for IngressEvmLogs {
        fn from(v: dtos::source::IngressEvmLogs) -> Self {
            Self {
                chain_id: v.chain_id.map(|v| v),
                node_url: v.node_url.map(|v| v),
                filter: v.filter.map(|v| v),
                signature: v.signature.map(|v| v),
            }
        }
    }

    impl From<IngressEvmLogs> for dtos::source::IngressEvmLogs {
        fn from(v: IngressEvmLogs) -> Self {
            Self {
                chain_id: v.chain_id.map(|v| v),
                node_url: v.node_url.map(|v| v),
                filter: v.filter.map(|v| v),
                signature: v.signature.map(|v| v),
            }
        }
    }

    implement_serde_as!(dtos::source::IngressEvmLogs, IngressEvmLogs);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/Ingress#/$defs/FilesGlob
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct IngressFilesGlob {
        pub path: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub event_time: Option<UnionOrString<source::EventTimeSource>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cache: Option<UnionOrString<source::SourceCaching>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order: Option<source::SourceOrdering>,
    }

    impl IntoDto for IngressFilesGlob {
        type Dto = dtos::source::IngressFilesGlob;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::IngressFilesGlob> for IngressFilesGlob {
        fn from(v: dtos::source::IngressFilesGlob) -> Self {
            Self {
                path: v.path,
                event_time: v.event_time.map(|v| v.into()),
                cache: v.cache.map(|v| v.into()),
                order: v.order.map(|v| v.into()),
            }
        }
    }

    impl From<IngressFilesGlob> for dtos::source::IngressFilesGlob {
        fn from(v: IngressFilesGlob) -> Self {
            Self {
                path: v.path,
                event_time: v.event_time.map(|v| v.into()),
                cache: v.cache.map(|v| v.into()),
                order: v.order.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::source::IngressFilesGlob, IngressFilesGlob);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/Ingress#/$defs/Mqtt
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct IngressMqtt {
        pub host: String,
        pub port: i32,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub username: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub password: Option<String>,
        pub topics: Vec<source::MqttTopicSubscription>,
    }

    impl IntoDto for IngressMqtt {
        type Dto = dtos::source::IngressMqtt;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::IngressMqtt> for IngressMqtt {
        fn from(v: dtos::source::IngressMqtt) -> Self {
            Self {
                host: v.host,
                port: v.port,
                username: v.username.map(|v| v),
                password: v.password.map(|v| v),
                topics: v.topics.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl From<IngressMqtt> for dtos::source::IngressMqtt {
        fn from(v: IngressMqtt) -> Self {
            Self {
                host: v.host,
                port: v.port,
                username: v.username.map(|v| v),
                password: v.password.map(|v| v),
                topics: v.topics.into_iter().map(Into::into).collect(),
            }
        }
    }

    implement_serde_as!(dtos::source::IngressMqtt, IngressMqtt);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/Ingress#/$defs/RestEndpoint
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct IngressRestEndpoint {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub buffer: Option<source::IngressBuffer>,
    }

    impl IntoDto for IngressRestEndpoint {
        type Dto = dtos::source::IngressRestEndpoint;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::IngressRestEndpoint> for IngressRestEndpoint {
        fn from(v: dtos::source::IngressRestEndpoint) -> Self {
            Self {
                buffer: v.buffer.map(|v| v.into()),
            }
        }
    }

    impl From<IngressRestEndpoint> for dtos::source::IngressRestEndpoint {
        fn from(v: IngressRestEndpoint) -> Self {
            Self {
                buffer: v.buffer.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::source::IngressRestEndpoint, IngressRestEndpoint);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/Ingress#/$defs/Url
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct IngressUrl {
        pub url: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub event_time: Option<UnionOrString<source::EventTimeSource>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cache: Option<UnionOrString<source::SourceCaching>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub headers: Option<Vec<source::RequestHeader>>,
    }

    impl IntoDto for IngressUrl {
        type Dto = dtos::source::IngressUrl;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::IngressUrl> for IngressUrl {
        fn from(v: dtos::source::IngressUrl) -> Self {
            Self {
                url: v.url,
                event_time: v.event_time.map(|v| v.into()),
                cache: v.cache.map(|v| v.into()),
                headers: v.headers.map(|v| v.into_iter().map(Into::into).collect()),
            }
        }
    }

    impl From<IngressUrl> for dtos::source::IngressUrl {
        fn from(v: IngressUrl) -> Self {
            Self {
                url: v.url,
                event_time: v.event_time.map(|v| v.into()),
                cache: v.cache.map(|v| v.into()),
                headers: v.headers.map(|v| v.into_iter().map(Into::into).collect()),
            }
        }
    }

    implement_serde_as!(dtos::source::IngressUrl, IngressUrl);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/MergeStrategy
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(tag = "kind")]
    pub enum MergeStrategy {
        #[serde(alias = "append")]
        Append(source::MergeStrategyAppend),
        #[serde(alias = "ledger")]
        Ledger(source::MergeStrategyLedger),
        #[serde(alias = "snapshot")]
        Snapshot(source::MergeStrategySnapshot),
        #[serde(alias = "changelogStream", alias = "changelogstream")]
        ChangelogStream(source::MergeStrategyChangelogStream),
        #[serde(alias = "upsertStream", alias = "upsertstream")]
        UpsertStream(source::MergeStrategyUpsertStream),
    }

    impl IntoDto for MergeStrategy {
        type Dto = dtos::source::MergeStrategy;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::MergeStrategy> for MergeStrategy {
        fn from(v: dtos::source::MergeStrategy) -> Self {
            match v {
                dtos::source::MergeStrategy::Append(v) => Self::Append(v.into()),
                dtos::source::MergeStrategy::Ledger(v) => Self::Ledger(v.into()),
                dtos::source::MergeStrategy::Snapshot(v) => Self::Snapshot(v.into()),
                dtos::source::MergeStrategy::ChangelogStream(v) => Self::ChangelogStream(v.into()),
                dtos::source::MergeStrategy::UpsertStream(v) => Self::UpsertStream(v.into()),
            }
        }
    }

    impl From<MergeStrategy> for dtos::source::MergeStrategy {
        fn from(v: MergeStrategy) -> Self {
            match v {
                MergeStrategy::Append(v) => Self::Append(v.into()),
                MergeStrategy::Ledger(v) => Self::Ledger(v.into()),
                MergeStrategy::Snapshot(v) => Self::Snapshot(v.into()),
                MergeStrategy::ChangelogStream(v) => Self::ChangelogStream(v.into()),
                MergeStrategy::UpsertStream(v) => Self::UpsertStream(v.into()),
            }
        }
    }

    implement_serde_as!(dtos::source::MergeStrategy, MergeStrategy);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/MergeStrategy#/$defs/Append
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct MergeStrategyAppend {}

    impl IntoDto for MergeStrategyAppend {
        type Dto = dtos::source::MergeStrategyAppend;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::MergeStrategyAppend> for MergeStrategyAppend {
        fn from(v: dtos::source::MergeStrategyAppend) -> Self {
            Self {}
        }
    }

    impl From<MergeStrategyAppend> for dtos::source::MergeStrategyAppend {
        fn from(v: MergeStrategyAppend) -> Self {
            Self {}
        }
    }

    implement_serde_as!(dtos::source::MergeStrategyAppend, MergeStrategyAppend);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/MergeStrategy#/$defs/ChangelogStream
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct MergeStrategyChangelogStream {
        pub primary_key: Vec<String>,
    }

    impl IntoDto for MergeStrategyChangelogStream {
        type Dto = dtos::source::MergeStrategyChangelogStream;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::MergeStrategyChangelogStream> for MergeStrategyChangelogStream {
        fn from(v: dtos::source::MergeStrategyChangelogStream) -> Self {
            Self {
                primary_key: v.primary_key.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl From<MergeStrategyChangelogStream> for dtos::source::MergeStrategyChangelogStream {
        fn from(v: MergeStrategyChangelogStream) -> Self {
            Self {
                primary_key: v.primary_key.into_iter().map(Into::into).collect(),
            }
        }
    }

    implement_serde_as!(
        dtos::source::MergeStrategyChangelogStream,
        MergeStrategyChangelogStream
    );

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/MergeStrategy#/$defs/Ledger
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct MergeStrategyLedger {
        pub primary_key: Vec<String>,
    }

    impl IntoDto for MergeStrategyLedger {
        type Dto = dtos::source::MergeStrategyLedger;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::MergeStrategyLedger> for MergeStrategyLedger {
        fn from(v: dtos::source::MergeStrategyLedger) -> Self {
            Self {
                primary_key: v.primary_key.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl From<MergeStrategyLedger> for dtos::source::MergeStrategyLedger {
        fn from(v: MergeStrategyLedger) -> Self {
            Self {
                primary_key: v.primary_key.into_iter().map(Into::into).collect(),
            }
        }
    }

    implement_serde_as!(dtos::source::MergeStrategyLedger, MergeStrategyLedger);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/MergeStrategy#/$defs/Snapshot
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct MergeStrategySnapshot {
        pub primary_key: Vec<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub compare_columns: Option<Vec<String>>,
    }

    impl IntoDto for MergeStrategySnapshot {
        type Dto = dtos::source::MergeStrategySnapshot;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::MergeStrategySnapshot> for MergeStrategySnapshot {
        fn from(v: dtos::source::MergeStrategySnapshot) -> Self {
            Self {
                primary_key: v.primary_key.into_iter().map(Into::into).collect(),
                compare_columns: v
                    .compare_columns
                    .map(|v| v.into_iter().map(Into::into).collect()),
            }
        }
    }

    impl From<MergeStrategySnapshot> for dtos::source::MergeStrategySnapshot {
        fn from(v: MergeStrategySnapshot) -> Self {
            Self {
                primary_key: v.primary_key.into_iter().map(Into::into).collect(),
                compare_columns: v
                    .compare_columns
                    .map(|v| v.into_iter().map(Into::into).collect()),
            }
        }
    }

    implement_serde_as!(dtos::source::MergeStrategySnapshot, MergeStrategySnapshot);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/MergeStrategy#/$defs/UpsertStream
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct MergeStrategyUpsertStream {
        pub primary_key: Vec<String>,
    }

    impl IntoDto for MergeStrategyUpsertStream {
        type Dto = dtos::source::MergeStrategyUpsertStream;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::MergeStrategyUpsertStream> for MergeStrategyUpsertStream {
        fn from(v: dtos::source::MergeStrategyUpsertStream) -> Self {
            Self {
                primary_key: v.primary_key.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl From<MergeStrategyUpsertStream> for dtos::source::MergeStrategyUpsertStream {
        fn from(v: MergeStrategyUpsertStream) -> Self {
            Self {
                primary_key: v.primary_key.into_iter().map(Into::into).collect(),
            }
        }
    }

    implement_serde_as!(
        dtos::source::MergeStrategyUpsertStream,
        MergeStrategyUpsertStream
    );

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/MqttQos
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub enum MqttQos {
        #[serde(alias = "atMostOnce", alias = "atmostonce")]
        AtMostOnce,
        #[serde(alias = "atLeastOnce", alias = "atleastonce")]
        AtLeastOnce,
        #[serde(alias = "exactlyOnce", alias = "exactlyonce")]
        ExactlyOnce,
    }

    impl IntoDto for MqttQos {
        type Dto = dtos::source::MqttQos;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::MqttQos> for MqttQos {
        fn from(v: dtos::source::MqttQos) -> Self {
            match v {
                dtos::source::MqttQos::AtMostOnce => Self::AtMostOnce,
                dtos::source::MqttQos::AtLeastOnce => Self::AtLeastOnce,
                dtos::source::MqttQos::ExactlyOnce => Self::ExactlyOnce,
            }
        }
    }

    impl From<MqttQos> for dtos::source::MqttQos {
        fn from(v: MqttQos) -> Self {
            match v {
                MqttQos::AtMostOnce => Self::AtMostOnce,
                MqttQos::AtLeastOnce => Self::AtLeastOnce,
                MqttQos::ExactlyOnce => Self::ExactlyOnce,
            }
        }
    }

    implement_serde_as!(dtos::source::MqttQos, MqttQos);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/MqttTopicSubscription
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct MqttTopicSubscription {
        pub path: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub qos: Option<source::MqttQos>,
    }

    impl IntoDto for MqttTopicSubscription {
        type Dto = dtos::source::MqttTopicSubscription;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::MqttTopicSubscription> for MqttTopicSubscription {
        fn from(v: dtos::source::MqttTopicSubscription) -> Self {
            Self {
                path: v.path,
                qos: v.qos.map(|v| v.into()),
            }
        }
    }

    impl From<MqttTopicSubscription> for dtos::source::MqttTopicSubscription {
        fn from(v: MqttTopicSubscription) -> Self {
            Self {
                path: v.path,
                qos: v.qos.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::source::MqttTopicSubscription, MqttTopicSubscription);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/PrepStep
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(tag = "kind")]
    pub enum PrepStep {
        #[serde(alias = "decompress")]
        Decompress(source::PrepStepDecompress),
        #[serde(alias = "pipe")]
        Pipe(source::PrepStepPipe),
    }

    impl IntoDto for PrepStep {
        type Dto = dtos::source::PrepStep;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::PrepStep> for PrepStep {
        fn from(v: dtos::source::PrepStep) -> Self {
            match v {
                dtos::source::PrepStep::Decompress(v) => Self::Decompress(v.into()),
                dtos::source::PrepStep::Pipe(v) => Self::Pipe(v.into()),
            }
        }
    }

    impl From<PrepStep> for dtos::source::PrepStep {
        fn from(v: PrepStep) -> Self {
            match v {
                PrepStep::Decompress(v) => Self::Decompress(v.into()),
                PrepStep::Pipe(v) => Self::Pipe(v.into()),
            }
        }
    }

    implement_serde_as!(dtos::source::PrepStep, PrepStep);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/PrepStep#/$defs/Decompress
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct PrepStepDecompress {
        pub format: source::CompressionFormat,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sub_path: Option<String>,
    }

    impl IntoDto for PrepStepDecompress {
        type Dto = dtos::source::PrepStepDecompress;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::PrepStepDecompress> for PrepStepDecompress {
        fn from(v: dtos::source::PrepStepDecompress) -> Self {
            Self {
                format: v.format.into(),
                sub_path: v.sub_path.map(|v| v),
            }
        }
    }

    impl From<PrepStepDecompress> for dtos::source::PrepStepDecompress {
        fn from(v: PrepStepDecompress) -> Self {
            Self {
                format: v.format.into(),
                sub_path: v.sub_path.map(|v| v),
            }
        }
    }

    implement_serde_as!(dtos::source::PrepStepDecompress, PrepStepDecompress);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/PrepStep#/$defs/Pipe
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct PrepStepPipe {
        pub command: Vec<String>,
    }

    impl IntoDto for PrepStepPipe {
        type Dto = dtos::source::PrepStepPipe;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::PrepStepPipe> for PrepStepPipe {
        fn from(v: dtos::source::PrepStepPipe) -> Self {
            Self {
                command: v.command.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl From<PrepStepPipe> for dtos::source::PrepStepPipe {
        fn from(v: PrepStepPipe) -> Self {
            Self {
                command: v.command.into_iter().map(Into::into).collect(),
            }
        }
    }

    implement_serde_as!(dtos::source::PrepStepPipe, PrepStepPipe);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/ReadStep
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(tag = "kind")]
    pub enum ReadStep {
        #[serde(alias = "csv")]
        Csv(source::ReadStepCsv),
        #[serde(alias = "geoJson", alias = "geojson")]
        GeoJson(source::ReadStepGeoJson),
        #[serde(alias = "esriShapefile", alias = "esrishapefile")]
        EsriShapefile(source::ReadStepEsriShapefile),
        #[serde(alias = "parquet")]
        Parquet(source::ReadStepParquet),
        #[serde(alias = "json")]
        Json(source::ReadStepJson),
        #[serde(alias = "ndJson", alias = "ndjson")]
        NdJson(source::ReadStepNdJson),
        #[serde(alias = "ndGeoJson", alias = "ndgeojson")]
        NdGeoJson(source::ReadStepNdGeoJson),
    }

    impl IntoDto for ReadStep {
        type Dto = dtos::source::ReadStep;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::ReadStep> for ReadStep {
        fn from(v: dtos::source::ReadStep) -> Self {
            match v {
                dtos::source::ReadStep::Csv(v) => Self::Csv(v.into()),
                dtos::source::ReadStep::GeoJson(v) => Self::GeoJson(v.into()),
                dtos::source::ReadStep::EsriShapefile(v) => Self::EsriShapefile(v.into()),
                dtos::source::ReadStep::Parquet(v) => Self::Parquet(v.into()),
                dtos::source::ReadStep::Json(v) => Self::Json(v.into()),
                dtos::source::ReadStep::NdJson(v) => Self::NdJson(v.into()),
                dtos::source::ReadStep::NdGeoJson(v) => Self::NdGeoJson(v.into()),
            }
        }
    }

    impl From<ReadStep> for dtos::source::ReadStep {
        fn from(v: ReadStep) -> Self {
            match v {
                ReadStep::Csv(v) => Self::Csv(v.into()),
                ReadStep::GeoJson(v) => Self::GeoJson(v.into()),
                ReadStep::EsriShapefile(v) => Self::EsriShapefile(v.into()),
                ReadStep::Parquet(v) => Self::Parquet(v.into()),
                ReadStep::Json(v) => Self::Json(v.into()),
                ReadStep::NdJson(v) => Self::NdJson(v.into()),
                ReadStep::NdGeoJson(v) => Self::NdGeoJson(v.into()),
            }
        }
    }

    implement_serde_as!(dtos::source::ReadStep, ReadStep);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/ReadStep#/$defs/Csv
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct ReadStepCsv {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ddl_schema: Option<Vec<String>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub separator: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub encoding: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quote: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub escape: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub header: Option<bool>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub infer_schema: Option<bool>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub null_value: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub date_format: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timestamp_format: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub schema: Option<data::DataSchema>,
    }

    impl IntoDto for ReadStepCsv {
        type Dto = dtos::source::ReadStepCsv;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::ReadStepCsv> for ReadStepCsv {
        fn from(v: dtos::source::ReadStepCsv) -> Self {
            Self {
                ddl_schema: v
                    .ddl_schema
                    .map(|v| v.into_iter().map(Into::into).collect()),
                separator: v.separator.map(|v| v),
                encoding: v.encoding.map(|v| v),
                quote: v.quote.map(|v| v),
                escape: v.escape.map(|v| v),
                header: v.header.map(|v| v),
                infer_schema: v.infer_schema.map(|v| v),
                null_value: v.null_value.map(|v| v),
                date_format: v.date_format.map(|v| v),
                timestamp_format: v.timestamp_format.map(|v| v),
                schema: v.schema.map(|v| v.into()),
            }
        }
    }

    impl From<ReadStepCsv> for dtos::source::ReadStepCsv {
        fn from(v: ReadStepCsv) -> Self {
            Self {
                ddl_schema: v
                    .ddl_schema
                    .map(|v| v.into_iter().map(Into::into).collect()),
                separator: v.separator.map(|v| v),
                encoding: v.encoding.map(|v| v),
                quote: v.quote.map(|v| v),
                escape: v.escape.map(|v| v),
                header: v.header.map(|v| v),
                infer_schema: v.infer_schema.map(|v| v),
                null_value: v.null_value.map(|v| v),
                date_format: v.date_format.map(|v| v),
                timestamp_format: v.timestamp_format.map(|v| v),
                schema: v.schema.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::source::ReadStepCsv, ReadStepCsv);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/ReadStep#/$defs/EsriShapefile
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct ReadStepEsriShapefile {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ddl_schema: Option<Vec<String>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sub_path: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub schema: Option<data::DataSchema>,
    }

    impl IntoDto for ReadStepEsriShapefile {
        type Dto = dtos::source::ReadStepEsriShapefile;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::ReadStepEsriShapefile> for ReadStepEsriShapefile {
        fn from(v: dtos::source::ReadStepEsriShapefile) -> Self {
            Self {
                ddl_schema: v
                    .ddl_schema
                    .map(|v| v.into_iter().map(Into::into).collect()),
                sub_path: v.sub_path.map(|v| v),
                schema: v.schema.map(|v| v.into()),
            }
        }
    }

    impl From<ReadStepEsriShapefile> for dtos::source::ReadStepEsriShapefile {
        fn from(v: ReadStepEsriShapefile) -> Self {
            Self {
                ddl_schema: v
                    .ddl_schema
                    .map(|v| v.into_iter().map(Into::into).collect()),
                sub_path: v.sub_path.map(|v| v),
                schema: v.schema.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::source::ReadStepEsriShapefile, ReadStepEsriShapefile);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/ReadStep#/$defs/GeoJson
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct ReadStepGeoJson {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ddl_schema: Option<Vec<String>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub schema: Option<data::DataSchema>,
    }

    impl IntoDto for ReadStepGeoJson {
        type Dto = dtos::source::ReadStepGeoJson;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::ReadStepGeoJson> for ReadStepGeoJson {
        fn from(v: dtos::source::ReadStepGeoJson) -> Self {
            Self {
                ddl_schema: v
                    .ddl_schema
                    .map(|v| v.into_iter().map(Into::into).collect()),
                schema: v.schema.map(|v| v.into()),
            }
        }
    }

    impl From<ReadStepGeoJson> for dtos::source::ReadStepGeoJson {
        fn from(v: ReadStepGeoJson) -> Self {
            Self {
                ddl_schema: v
                    .ddl_schema
                    .map(|v| v.into_iter().map(Into::into).collect()),
                schema: v.schema.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::source::ReadStepGeoJson, ReadStepGeoJson);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/ReadStep#/$defs/Json
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct ReadStepJson {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sub_path: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ddl_schema: Option<Vec<String>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub date_format: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub encoding: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timestamp_format: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub schema: Option<data::DataSchema>,
    }

    impl IntoDto for ReadStepJson {
        type Dto = dtos::source::ReadStepJson;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::ReadStepJson> for ReadStepJson {
        fn from(v: dtos::source::ReadStepJson) -> Self {
            Self {
                sub_path: v.sub_path.map(|v| v),
                ddl_schema: v
                    .ddl_schema
                    .map(|v| v.into_iter().map(Into::into).collect()),
                date_format: v.date_format.map(|v| v),
                encoding: v.encoding.map(|v| v),
                timestamp_format: v.timestamp_format.map(|v| v),
                schema: v.schema.map(|v| v.into()),
            }
        }
    }

    impl From<ReadStepJson> for dtos::source::ReadStepJson {
        fn from(v: ReadStepJson) -> Self {
            Self {
                sub_path: v.sub_path.map(|v| v),
                ddl_schema: v
                    .ddl_schema
                    .map(|v| v.into_iter().map(Into::into).collect()),
                date_format: v.date_format.map(|v| v),
                encoding: v.encoding.map(|v| v),
                timestamp_format: v.timestamp_format.map(|v| v),
                schema: v.schema.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::source::ReadStepJson, ReadStepJson);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/ReadStep#/$defs/NdGeoJson
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct ReadStepNdGeoJson {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ddl_schema: Option<Vec<String>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub schema: Option<data::DataSchema>,
    }

    impl IntoDto for ReadStepNdGeoJson {
        type Dto = dtos::source::ReadStepNdGeoJson;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::ReadStepNdGeoJson> for ReadStepNdGeoJson {
        fn from(v: dtos::source::ReadStepNdGeoJson) -> Self {
            Self {
                ddl_schema: v
                    .ddl_schema
                    .map(|v| v.into_iter().map(Into::into).collect()),
                schema: v.schema.map(|v| v.into()),
            }
        }
    }

    impl From<ReadStepNdGeoJson> for dtos::source::ReadStepNdGeoJson {
        fn from(v: ReadStepNdGeoJson) -> Self {
            Self {
                ddl_schema: v
                    .ddl_schema
                    .map(|v| v.into_iter().map(Into::into).collect()),
                schema: v.schema.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::source::ReadStepNdGeoJson, ReadStepNdGeoJson);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/ReadStep#/$defs/NdJson
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct ReadStepNdJson {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ddl_schema: Option<Vec<String>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub date_format: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub encoding: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timestamp_format: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub schema: Option<data::DataSchema>,
    }

    impl IntoDto for ReadStepNdJson {
        type Dto = dtos::source::ReadStepNdJson;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::ReadStepNdJson> for ReadStepNdJson {
        fn from(v: dtos::source::ReadStepNdJson) -> Self {
            Self {
                ddl_schema: v
                    .ddl_schema
                    .map(|v| v.into_iter().map(Into::into).collect()),
                date_format: v.date_format.map(|v| v),
                encoding: v.encoding.map(|v| v),
                timestamp_format: v.timestamp_format.map(|v| v),
                schema: v.schema.map(|v| v.into()),
            }
        }
    }

    impl From<ReadStepNdJson> for dtos::source::ReadStepNdJson {
        fn from(v: ReadStepNdJson) -> Self {
            Self {
                ddl_schema: v
                    .ddl_schema
                    .map(|v| v.into_iter().map(Into::into).collect()),
                date_format: v.date_format.map(|v| v),
                encoding: v.encoding.map(|v| v),
                timestamp_format: v.timestamp_format.map(|v| v),
                schema: v.schema.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::source::ReadStepNdJson, ReadStepNdJson);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/ReadStep#/$defs/Parquet
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct ReadStepParquet {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ddl_schema: Option<Vec<String>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub schema: Option<data::DataSchema>,
    }

    impl IntoDto for ReadStepParquet {
        type Dto = dtos::source::ReadStepParquet;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::ReadStepParquet> for ReadStepParquet {
        fn from(v: dtos::source::ReadStepParquet) -> Self {
            Self {
                ddl_schema: v
                    .ddl_schema
                    .map(|v| v.into_iter().map(Into::into).collect()),
                schema: v.schema.map(|v| v.into()),
            }
        }
    }

    impl From<ReadStepParquet> for dtos::source::ReadStepParquet {
        fn from(v: ReadStepParquet) -> Self {
            Self {
                ddl_schema: v
                    .ddl_schema
                    .map(|v| v.into_iter().map(Into::into).collect()),
                schema: v.schema.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::source::ReadStepParquet, ReadStepParquet);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/RequestHeader
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct RequestHeader {
        pub name: String,
        pub value: String,
    }

    impl IntoDto for RequestHeader {
        type Dto = dtos::source::RequestHeader;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::RequestHeader> for RequestHeader {
        fn from(v: dtos::source::RequestHeader) -> Self {
            Self {
                name: v.name,
                value: v.value,
            }
        }
    }

    impl From<RequestHeader> for dtos::source::RequestHeader {
        fn from(v: RequestHeader) -> Self {
            Self {
                name: v.name,
                value: v.value,
            }
        }
    }

    implement_serde_as!(dtos::source::RequestHeader, RequestHeader);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/SourceCaching
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(tag = "kind")]
    pub enum SourceCaching {
        #[serde(alias = "forever")]
        Forever(source::SourceCachingForever),
    }

    impl From<dtos::source::SourceCaching> for UnionOrString<SourceCaching> {
        fn from(v: dtos::source::SourceCaching) -> Self {
            Self(v.into())
        }
    }
    impl From<UnionOrString<SourceCaching>> for dtos::source::SourceCaching {
        fn from(v: UnionOrString<SourceCaching>) -> Self {
            v.0.into()
        }
    }

    impl IntoDto for SourceCaching {
        type Dto = dtos::source::SourceCaching;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::SourceCaching> for SourceCaching {
        fn from(v: dtos::source::SourceCaching) -> Self {
            match v {
                dtos::source::SourceCaching::Forever(v) => Self::Forever(v.into()),
            }
        }
    }

    impl From<SourceCaching> for dtos::source::SourceCaching {
        fn from(v: SourceCaching) -> Self {
            match v {
                SourceCaching::Forever(v) => Self::Forever(v.into()),
            }
        }
    }

    implement_serde_as!(dtos::source::SourceCaching, SourceCaching);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/SourceCaching#/$defs/Forever
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct SourceCachingForever {}

    impl IntoDto for SourceCachingForever {
        type Dto = dtos::source::SourceCachingForever;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::SourceCachingForever> for SourceCachingForever {
        fn from(v: dtos::source::SourceCachingForever) -> Self {
            Self {}
        }
    }

    impl From<SourceCachingForever> for dtos::source::SourceCachingForever {
        fn from(v: SourceCachingForever) -> Self {
            Self {}
        }
    }

    implement_serde_as!(dtos::source::SourceCachingForever, SourceCachingForever);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/SourceOrdering
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub enum SourceOrdering {
        #[serde(alias = "byEventTime", alias = "byeventtime")]
        ByEventTime,
        #[serde(alias = "byName", alias = "byname")]
        ByName,
    }

    impl IntoDto for SourceOrdering {
        type Dto = dtos::source::SourceOrdering;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::SourceOrdering> for SourceOrdering {
        fn from(v: dtos::source::SourceOrdering) -> Self {
            match v {
                dtos::source::SourceOrdering::ByEventTime => Self::ByEventTime,
                dtos::source::SourceOrdering::ByName => Self::ByName,
            }
        }
    }

    impl From<SourceOrdering> for dtos::source::SourceOrdering {
        fn from(v: SourceOrdering) -> Self {
            match v {
                SourceOrdering::ByEventTime => Self::ByEventTime,
                SourceOrdering::ByName => Self::ByName,
            }
        }
    }

    implement_serde_as!(dtos::source::SourceOrdering, SourceOrdering);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/SourceSpec
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct SourceSpec {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub config: Option<config::ValueRefs>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ingress: Option<source::Ingress>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prepare: Option<Vec<source::PrepStep>>,
        pub read: source::ReadStep,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub preprocess: Option<dataset::Transform>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub merge: Option<source::MergeStrategy>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub vocab: Option<dataset::DatasetVocabulary>,
    }

    impl IntoDto for SourceSpec {
        type Dto = dtos::source::SourceSpec;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::SourceSpec> for SourceSpec {
        fn from(v: dtos::source::SourceSpec) -> Self {
            Self {
                config: v.config.map(|v| v.into()),
                ingress: v.ingress.map(|v| v.into()),
                prepare: v.prepare.map(|v| v.into_iter().map(Into::into).collect()),
                read: v.read.into(),
                preprocess: v.preprocess.map(|v| v.into()),
                merge: v.merge.map(|v| v.into()),
                vocab: v.vocab.map(|v| v.into()),
            }
        }
    }

    impl From<SourceSpec> for dtos::source::SourceSpec {
        fn from(v: SourceSpec) -> Self {
            Self {
                config: v.config.map(|v| v.into()),
                ingress: v.ingress.map(|v| v.into()),
                prepare: v.prepare.map(|v| v.into_iter().map(Into::into).collect()),
                read: v.read.into(),
                preprocess: v.preprocess.map(|v| v.into()),
                merge: v.merge.map(|v| v.into()),
                vocab: v.vocab.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::source::SourceSpec, SourceSpec);

    // Schema: https://opendatafabric.org/schemas/source/v1alpha1/SourceState
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct SourceState {
        pub source_name: String,
        pub kind: String,
        pub value: String,
    }

    impl IntoDto for SourceState {
        type Dto = dtos::source::SourceState;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::source::SourceState> for SourceState {
        fn from(v: dtos::source::SourceState) -> Self {
            Self {
                source_name: v.source_name,
                kind: v.kind,
                value: v.value,
            }
        }
    }

    impl From<SourceState> for dtos::source::SourceState {
        fn from(v: SourceState) -> Self {
            Self {
                source_name: v.source_name,
                kind: v.kind,
                value: v.value,
            }
        }
    }

    implement_serde_as!(dtos::source::SourceState, SourceState);
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// storage
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod storage {
    #[allow(unused_imports)]
    use super::*;

    // Schema: https://opendatafabric.org/schemas/storage/v1alpha1/AwsCredentials
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct AwsCredentials {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub access_key: Option<StructOrString<config::ValueRef>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub secret_key: Option<StructOrString<config::ValueRef>>,
    }

    impl IntoDto for AwsCredentials {
        type Dto = dtos::storage::AwsCredentials;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::storage::AwsCredentials> for AwsCredentials {
        fn from(v: dtos::storage::AwsCredentials) -> Self {
            Self {
                access_key: v.access_key.map(|v| v.into()),
                secret_key: v.secret_key.map(|v| v.into()),
            }
        }
    }

    impl From<AwsCredentials> for dtos::storage::AwsCredentials {
        fn from(v: AwsCredentials) -> Self {
            Self {
                access_key: v.access_key.map(|v| v.into()),
                secret_key: v.secret_key.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(dtos::storage::AwsCredentials, AwsCredentials);

    // Schema: https://opendatafabric.org/schemas/storage/v1alpha1/PersistentVolumeRef
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct PersistentVolumeRef {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub account: Option<StructOrString<auth::AccountRef>>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<ResourceID>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<ResourceName>,
    }

    impl IntoDto for PersistentVolumeRef {
        type Dto = dtos::storage::PersistentVolumeRef;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::storage::PersistentVolumeRef> for StructOrString<PersistentVolumeRef> {
        fn from(v: dtos::storage::PersistentVolumeRef) -> Self {
            Self(v.into())
        }
    }
    impl From<StructOrString<PersistentVolumeRef>> for dtos::storage::PersistentVolumeRef {
        fn from(v: StructOrString<PersistentVolumeRef>) -> Self {
            v.0.into()
        }
    }

    implement_serde_as!(dtos::storage::PersistentVolumeRef, PersistentVolumeRef);

    // Schema: https://opendatafabric.org/schemas/storage/v1alpha1/PersistentVolumeSpec
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(tag = "kind")]
    pub enum PersistentVolumeSpec {
        #[serde(alias = "s3")]
        S3(storage::PersistentVolumeSpecS3),
    }

    impl IntoDto for PersistentVolumeSpec {
        type Dto = dtos::storage::PersistentVolumeSpec;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::storage::PersistentVolumeSpec> for PersistentVolumeSpec {
        fn from(v: dtos::storage::PersistentVolumeSpec) -> Self {
            match v {
                dtos::storage::PersistentVolumeSpec::S3(v) => Self::S3(v.into()),
            }
        }
    }

    impl From<PersistentVolumeSpec> for dtos::storage::PersistentVolumeSpec {
        fn from(v: PersistentVolumeSpec) -> Self {
            match v {
                PersistentVolumeSpec::S3(v) => Self::S3(v.into()),
            }
        }
    }

    implement_serde_as!(dtos::storage::PersistentVolumeSpec, PersistentVolumeSpec);

    // Schema: https://opendatafabric.org/schemas/storage/v1alpha1/PersistentVolumeSpec#/$defs/S3
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct PersistentVolumeSpecS3 {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub endpoint: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        pub bucket: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prefix: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub capacity: Option<storage::VolumeCapacity>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub credentials: Option<storage::AwsCredentials>,
    }

    impl IntoDto for PersistentVolumeSpecS3 {
        type Dto = dtos::storage::PersistentVolumeSpecS3;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::storage::PersistentVolumeSpecS3> for PersistentVolumeSpecS3 {
        fn from(v: dtos::storage::PersistentVolumeSpecS3) -> Self {
            Self {
                endpoint: v.endpoint.map(|v| v),
                region: v.region.map(|v| v),
                bucket: v.bucket,
                prefix: v.prefix.map(|v| v),
                capacity: v.capacity.map(|v| v.into()),
                credentials: v.credentials.map(|v| v.into()),
            }
        }
    }

    impl From<PersistentVolumeSpecS3> for dtos::storage::PersistentVolumeSpecS3 {
        fn from(v: PersistentVolumeSpecS3) -> Self {
            Self {
                endpoint: v.endpoint.map(|v| v),
                region: v.region.map(|v| v),
                bucket: v.bucket,
                prefix: v.prefix.map(|v| v),
                capacity: v.capacity.map(|v| v.into()),
                credentials: v.credentials.map(|v| v.into()),
            }
        }
    }

    implement_serde_as!(
        dtos::storage::PersistentVolumeSpecS3,
        PersistentVolumeSpecS3
    );

    // Schema: https://opendatafabric.org/schemas/storage/v1alpha1/VolumeCapacity
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    #[serde(rename_all = "camelCase")]
    pub struct VolumeCapacity {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub storage: Option<ByteSize>,
    }

    impl IntoDto for VolumeCapacity {
        type Dto = dtos::storage::VolumeCapacity;
        fn into_dto(self) -> Self::Dto {
            self.into()
        }
    }

    impl From<dtos::storage::VolumeCapacity> for VolumeCapacity {
        fn from(v: dtos::storage::VolumeCapacity) -> Self {
            Self {
                storage: v.storage.map(|v| v),
            }
        }
    }

    impl From<VolumeCapacity> for dtos::storage::VolumeCapacity {
        fn from(v: VolumeCapacity) -> Self {
            Self {
                storage: v.storage.map(|v| v),
            }
        }
    }

    implement_serde_as!(dtos::storage::VolumeCapacity, VolumeCapacity);
}
