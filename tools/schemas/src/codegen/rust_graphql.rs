use crate::{
    codegen::rust_common::format_ident,
    json_schema::{CodegenHint, CodegenLanguage},
    model,
};

const SPEC_URL: &str =
    "https://github.com/kamu-data/open-data-fabric/blob/master/open-data-fabric.md";

const PREAMBLE: &str = indoc::indoc!(
    r#"
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // WARNING: This file is auto-generated from Open Data Fabric Schemas
    // See: http://opendatafabric.org/
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    #![allow(unused_variables)]
    #![allow(dead_code)]
    #![allow(clippy::all)]
    #![allow(clippy::pedantic)]

    use std::sync::Arc;

    use chrono::{DateTime, Utc};

    use crate::prelude::*;
    use crate::queries::Dataset;
    "#
);

const CUSTOM_TYPES: [(&str, &str); 11] = [
    (
        "TransformInput",
        indoc::indoc!(
            r#"
            #[derive(Interface, Debug)]
            #[graphql(field(name = "message", ty = "String"))]
            pub enum TransformInputDataset {
                Accessible(TransformInputDatasetAccessible),
                NotAccessible(TransformInputDatasetNotAccessible),
            }

            impl TransformInputDataset {
                pub fn accessible(dataset: Dataset) -> Self {
                    Self::Accessible(TransformInputDatasetAccessible { dataset })
                }

                pub fn not_accessible(dataset_ref: odf::DatasetRef) -> Self {
                    Self::NotAccessible(TransformInputDatasetNotAccessible {
                        dataset_ref: dataset_ref.into(),
                    })
                }
            }

            #[derive(SimpleObject, Debug)]
            #[graphql(complex)]
            pub struct TransformInputDatasetAccessible {
                pub dataset: Dataset,
            }

            #[ComplexObject]
            impl TransformInputDatasetAccessible {
                async fn message(&self) -> String {
                    "Found".to_string()
                }
            }

            #[derive(SimpleObject, Debug)]
            #[graphql(complex)]
            pub struct TransformInputDatasetNotAccessible {
                pub dataset_ref: DatasetRef<'static>,
            }

            #[ComplexObject]
            impl TransformInputDatasetNotAccessible {
                async fn message(&self) -> String {
                    "Not Accessible".to_string()
                }
            }

            #[derive(SimpleObject, Debug, Clone)]
            #[graphql(complex)]
            pub struct TransformInput {
                pub dataset_ref: DatasetRef<'static>,
                pub alias: String,
            }

            #[ComplexObject]
            impl TransformInput {
                async fn input_dataset(&self, ctx: &Context<'_>) -> Result<TransformInputDataset> {
                    if let Some(dataset) = Dataset::try_from_ref(ctx, &self.dataset_ref).await? {
                        Ok(TransformInputDataset::accessible(dataset))
                    } else {
                        Ok(TransformInputDataset::not_accessible(
                            self.dataset_ref.clone().into(),
                        ))
                    }
                }
            }

            impl From<odf::metadata::TransformInput> for TransformInput {
                fn from(v: odf::metadata::TransformInput) -> Self {
                    Self {
                        dataset_ref: v.dataset_ref.into(),
                        alias: v.alias.unwrap(),
                    }
                }
            }
            "#
        ),
    ),
    // TODO: Move the query/queries ambiguity to YAML layer, so it doesn't affect other layers
    (
        "TransformSql",
        indoc::indoc!(
            r#"
            #[derive(SimpleObject, Debug, Clone)]
            pub struct TransformSql {
                pub engine: String,
                pub version: Option<String>,
                pub queries: Vec<SqlQueryStep>,
                pub temporal_tables: Option<Vec<TemporalTable>>,
            }

            impl From<odf::metadata::TransformSql> for TransformSql {
                fn from(v: odf::metadata::TransformSql) -> Self {
                    let queries = if let Some(query) = v.query {
                        vec![SqlQueryStep { alias: None, query }]
                    } else {
                        v.queries.unwrap().into_iter().map(Into::into).collect()
                    };

                    Self {
                        engine: v.engine.into(),
                        version: v.version.map(Into::into),
                        queries: queries,
                        temporal_tables: v
                            .temporal_tables
                            .map(|v| v.into_iter().map(Into::into).collect()),
                    }
                }
            }
            "#
        ),
    ),
    (
        "SetDataSchema",
        indoc::indoc!(
            r#"
            #[derive(Debug, Clone)]
            pub struct SetDataSchema {
                pub schema: std::sync::Arc<odf::schema::DataSchema>,
            }

            #[Object]
            impl SetDataSchema {
                // TODO: Make `format` required argument
                async fn schema(&self, format: Option<DataSchemaFormat>) -> crate::prelude::DataSchema {
                    crate::prelude::DataSchema::new(
                        self.schema.clone(),
                        format.unwrap_or(DataSchemaFormat::OdfJson),
                    )
                }
            }

            impl From<odf::metadata::SetDataSchema> for SetDataSchema {
                fn from(v: odf::metadata::SetDataSchema) -> Self {
                    Self {
                        schema: std::sync::Arc::new(v.upgrade().schema),
                    }
                }
            }
            "#
        ),
    ),
    (
        "ReadStep",
        indoc::indoc!(
            r#"
            #[derive(Interface, Debug, Clone)]
            #[graphql(field(
                name = "schema",
                ty = "Option<crate::prelude::DataSchema>",
                arg(name = "format", ty = "Option<DataSchemaFormat>"),
            ))]
            pub enum ReadStep {
                Csv(ReadStepCsv),
                GeoJson(ReadStepGeoJson),
                EsriShapefile(ReadStepEsriShapefile),
                Parquet(ReadStepParquet),
                Json(ReadStepJson),
                NdJson(ReadStepNdJson),
                NdGeoJson(ReadStepNdGeoJson),
            }

            impl From<odf::metadata::ReadStep> for ReadStep {
                fn from(v: odf::metadata::ReadStep) -> Self {
                    match v {
                        odf::metadata::ReadStep::Csv(v) => Self::Csv(v.into()),
                        odf::metadata::ReadStep::GeoJson(v) => Self::GeoJson(v.into()),
                        odf::metadata::ReadStep::EsriShapefile(v) => Self::EsriShapefile(v.into()),
                        odf::metadata::ReadStep::Parquet(v) => Self::Parquet(v.into()),
                        odf::metadata::ReadStep::Json(v) => Self::Json(v.into()),
                        odf::metadata::ReadStep::NdJson(v) => Self::NdJson(v.into()),
                        odf::metadata::ReadStep::NdGeoJson(v) => Self::NdGeoJson(v.into()),
                    }
                }
            }
            "#
        ),
    ),
    (
        "ReadStepCsv",
        indoc::indoc!(
            r#"
            #[derive(SimpleObject, Debug, Clone)]
            #[graphql(complex)]
            pub struct ReadStepCsv {
                /// DEPRECATED: A DDL-formatted schema. Schema can be used to coerce values
                /// into more appropriate data types.
                ///
                /// Examples:
                /// - ["date TIMESTAMP","city STRING","population INT"]
                pub ddl_schema: Option<Vec<String>>,
                /// Sets a single character as a separator for each field and value.
                ///
                /// Defaults to: ","
                pub separator: Option<String>,
                /// Decodes the CSV files by the given encoding type.
                ///
                /// Defaults to: "utf8"
                pub encoding: Option<String>,
                /// Sets a single character used for escaping quoted values where the
                /// separator can be part of the value. Set an empty string to turn off
                /// quotations.
                ///
                /// Defaults to: "\""
                pub quote: Option<String>,
                /// Sets a single character used for escaping quotes inside an already
                /// quoted value.
                ///
                /// Defaults to: "\\"
                pub escape: Option<String>,
                /// Use the first line as names of columns.
                ///
                /// Defaults to: false
                pub header: Option<bool>,
                /// Infers the input schema automatically from data. It requires one extra
                /// pass over the data.
                ///
                /// Defaults to: false
                pub infer_schema: Option<bool>,
                /// Sets the string representation of a null value.
                ///
                /// Defaults to: ""
                pub null_value: Option<String>,
                /// Sets the string that indicates a date format. The `rfc3339` is the only
                /// required format, the other format strings are implementation-specific.
                ///
                /// Defaults to: "rfc3339"
                pub date_format: Option<String>,
                /// Sets the string that indicates a timestamp format. The `rfc3339` is the
                /// only required format, the other format strings are
                /// implementation-specific.
                ///
                /// Defaults to: "rfc3339"
                pub timestamp_format: Option<String>,

                #[graphql(skip)]
                pub schema: Option<Arc<odf::schema::DataSchema>>,
            }

            impl From<odf::metadata::ReadStepCsv> for ReadStepCsv {
                fn from(v: odf::metadata::ReadStepCsv) -> Self {
                    let odf::metadata::ReadStepCsv {
                        ddl_schema,
                        separator,
                        encoding,
                        quote,
                        escape,
                        header,
                        infer_schema,
                        null_value,
                        date_format,
                        timestamp_format,
                        schema,
                    } = v;

                    Self {
                        ddl_schema,
                        separator,
                        encoding,
                        quote,
                        escape,
                        header,
                        infer_schema,
                        null_value,
                        date_format,
                        timestamp_format,
                        schema: schema.map(Arc::new),
                    }
                }
            }

            #[ComplexObject]
            impl ReadStepCsv {
                /// Schema used to coerce values into more appropriate data types.
                async fn schema(&self, format: Option<DataSchemaFormat>) -> Option<crate::prelude::DataSchema> {
                    self.schema.clone().or_else(|| {
                        self.ddl_schema.as_ref().and_then(|s| {
                            odf::utils::schema::parse::parse_ddl_to_odf_schema(&s.join(", "))
                                .ok()
                                .map(Arc::new)
                        })
                    }).map(|s| {
                        crate::prelude::DataSchema::new(s, format.unwrap_or(DataSchemaFormat::OdfJson))
                    })
                }
            }
            "#
        ),
    ),
    (
        "ReadStepEsriShapefile",
        indoc::indoc!(
            r#"
            #[derive(SimpleObject, Debug, Clone)]
            #[graphql(complex)]
            pub struct ReadStepEsriShapefile {
                /// DEPRECATED: A DDL-formatted schema. Schema can be used to coerce values
                /// into more appropriate data types.
                pub ddl_schema: Option<Vec<String>>,
                /// If the ZIP archive contains multiple shapefiles use this field to
                /// specify a sub-path to the desired `.shp` file. Can contain glob patterns
                /// to act as a filter.
                pub sub_path: Option<String>,

                #[graphql(skip)]
                pub schema: Option<Arc<odf::schema::DataSchema>>,
            }

            impl From<odf::metadata::ReadStepEsriShapefile> for ReadStepEsriShapefile {
                fn from(v: odf::metadata::ReadStepEsriShapefile) -> Self {
                    let odf::metadata::ReadStepEsriShapefile {
                        ddl_schema,
                        sub_path,
                        schema,
                    } = v;
                    Self {
                        ddl_schema,
                        sub_path,
                        schema: schema.map(Arc::new),
                    }
                }
            }

            #[ComplexObject]
            impl ReadStepEsriShapefile {
                /// Schema used to coerce values into more appropriate data types.
                async fn schema(&self, format: Option<DataSchemaFormat>) -> Option<crate::prelude::DataSchema> {
                    self.schema.clone().or_else(|| {
                        self.ddl_schema.as_ref().and_then(|s| {
                            odf::utils::schema::parse::parse_ddl_to_odf_schema(&s.join(", "))
                                .ok()
                                .map(Arc::new)
                        })
                    }).map(|s| {
                        crate::prelude::DataSchema::new(s, format.unwrap_or(DataSchemaFormat::OdfJson))
                    })
                }
            }
            "#
        ),
    ),
    (
        "ReadStepGeoJson",
        indoc::indoc!(
            r#"
            #[derive(SimpleObject, Debug, Clone)]
            #[graphql(complex)]
            pub struct ReadStepGeoJson {
                /// DEPRECATED: A DDL-formatted schema. Schema can be used to coerce values
                /// into more appropriate data types.
                pub ddl_schema: Option<Vec<String>>,

                #[graphql(skip)]
                pub schema: Option<Arc<odf::schema::DataSchema>>,
            }

            impl From<odf::metadata::ReadStepGeoJson> for ReadStepGeoJson {
                fn from(v: odf::metadata::ReadStepGeoJson) -> Self {
                    let odf::metadata::ReadStepGeoJson {
                        ddl_schema,
                        schema,
                    } = v;
                    Self {
                        ddl_schema,
                        schema: schema.map(Arc::new),
                    }
                }
            }

            #[ComplexObject]
            impl ReadStepGeoJson {
                /// Schema used to coerce values into more appropriate data types.
                async fn schema(&self, format: Option<DataSchemaFormat>) -> Option<crate::prelude::DataSchema> {
                    self.schema.clone().or_else(|| {
                        self.ddl_schema.as_ref().and_then(|s| {
                            odf::utils::schema::parse::parse_ddl_to_odf_schema(&s.join(", "))
                                .ok()
                                .map(Arc::new)
                        })
                    }).map(|s| {
                        crate::prelude::DataSchema::new(s, format.unwrap_or(DataSchemaFormat::OdfJson))
                    })
                }
            }
            "#
        ),
    ),
    (
        "ReadStepJson",
        indoc::indoc!(
            r#"
            #[derive(SimpleObject, Debug, Clone)]
            #[graphql(complex)]
            pub struct ReadStepJson {
                /// Path in the form of `a.b.c` to a sub-element of the root JSON object
                /// that is an array or objects. If not specified it is assumed that the
                /// root element is an array.
                pub sub_path: Option<String>,
                /// DEPRECATED: A DDL-formatted schema. Schema can be used to coerce values
                /// into more appropriate data types.
                pub ddl_schema: Option<Vec<String>>,
                /// Sets the string that indicates a date format. The `rfc3339` is the only
                /// required format, the other format strings are implementation-specific.
                ///
                /// Defaults to: "rfc3339"
                pub date_format: Option<String>,
                /// Allows to forcibly set one of standard basic or extended encodings.
                ///
                /// Defaults to: "utf8"
                pub encoding: Option<String>,
                /// Sets the string that indicates a timestamp format. The `rfc3339` is the
                /// only required format, the other format strings are
                /// implementation-specific.
                ///
                /// Defaults to: "rfc3339"
                pub timestamp_format: Option<String>,

                #[graphql(skip)]
                pub schema: Option<Arc<odf::schema::DataSchema>>,
            }

            impl From<odf::metadata::ReadStepJson> for ReadStepJson {
                fn from(v: odf::metadata::ReadStepJson) -> Self {
                    let odf::metadata::ReadStepJson {
                        sub_path,
                        ddl_schema,
                        date_format,
                        encoding,
                        timestamp_format,
                        schema,
                    } = v;

                    Self {
                        sub_path,
                        ddl_schema,
                        date_format,
                        encoding,
                        timestamp_format,
                        schema: schema.map(Arc::new),
                    }
                }
            }

            #[ComplexObject]
            impl ReadStepJson {
                /// Schema used to coerce values into more appropriate data types.
                async fn schema(&self, format: Option<DataSchemaFormat>) -> Option<crate::prelude::DataSchema> {
                    self.schema.clone().or_else(|| {
                        self.ddl_schema.as_ref().and_then(|s| {
                            odf::utils::schema::parse::parse_ddl_to_odf_schema(&s.join(", "))
                                .ok()
                                .map(Arc::new)
                        })
                    }).map(|s| {
                        crate::prelude::DataSchema::new(s, format.unwrap_or(DataSchemaFormat::OdfJson))
                    })
                }
            }
            "#
        ),
    ),
    (
        "ReadStepNdGeoJson",
        indoc::indoc!(
            r#"
            #[derive(SimpleObject, Debug, Clone)]
            #[graphql(complex)]
            pub struct ReadStepNdGeoJson {
                /// DEPRECATED: A DDL-formatted schema. Schema can be used to coerce values
                /// into more appropriate data types.
                pub ddl_schema: Option<Vec<String>>,

                #[graphql(skip)]
                pub schema: Option<Arc<odf::schema::DataSchema>>,
            }

            impl From<odf::metadata::ReadStepNdGeoJson> for ReadStepNdGeoJson {
                fn from(v: odf::metadata::ReadStepNdGeoJson) -> Self {
                    let odf::metadata::ReadStepNdGeoJson { ddl_schema, schema } = v;
                    Self {
                        ddl_schema,
                        schema: schema.map(Arc::new),
                    }
                }
            }

            #[ComplexObject]
            impl ReadStepNdGeoJson {
                /// Schema used to coerce values into more appropriate data types.
                async fn schema(&self, format: Option<DataSchemaFormat>) -> Option<crate::prelude::DataSchema> {
                    self.schema.clone().or_else(|| {
                        self.ddl_schema.as_ref().and_then(|s| {
                            odf::utils::schema::parse::parse_ddl_to_odf_schema(&s.join(", "))
                                .ok()
                                .map(Arc::new)
                        })
                    }).map(|s| {
                        crate::prelude::DataSchema::new(s, format.unwrap_or(DataSchemaFormat::OdfJson))
                    })
                }
            }
            "#
        ),
    ),
    (
        "ReadStepNdJson",
        indoc::indoc!(
            r#"
            #[derive(SimpleObject, Debug, Clone)]
            #[graphql(complex)]
            pub struct ReadStepNdJson {
                /// DEPRECATED: A DDL-formatted schema. Schema can be used to coerce values
                /// into more appropriate data types.
                pub ddl_schema: Option<Vec<String>>,
                /// Sets the string that indicates a date format. The `rfc3339` is the only
                /// required format, the other format strings are implementation-specific.
                ///
                /// Defaults to: "rfc3339"
                pub date_format: Option<String>,
                /// Allows to forcibly set one of standard basic or extended encodings.
                ///
                /// Defaults to: "utf8"
                pub encoding: Option<String>,
                /// Sets the string that indicates a timestamp format. The `rfc3339` is the
                /// only required format, the other format strings are
                /// implementation-specific.
                ///
                /// Defaults to: "rfc3339"
                pub timestamp_format: Option<String>,

                #[graphql(skip)]
                pub schema: Option<Arc<odf::schema::DataSchema>>,
            }

            impl From<odf::metadata::ReadStepNdJson> for ReadStepNdJson {
                fn from(v: odf::metadata::ReadStepNdJson) -> Self {
                    let odf::metadata::ReadStepNdJson {
                        ddl_schema,
                        date_format,
                        encoding,
                        timestamp_format,
                        schema,
                    } = v;

                    Self {
                        ddl_schema,
                        date_format,
                        encoding,
                        timestamp_format,
                        schema: schema.map(Arc::new),
                    }
                }
            }

            #[ComplexObject]
            impl ReadStepNdJson {
                /// Schema used to coerce values into more appropriate data types.
                async fn schema(&self, format: Option<DataSchemaFormat>) -> Option<crate::prelude::DataSchema> {
                    self.schema.clone().or_else(|| {
                        self.ddl_schema.as_ref().and_then(|s| {
                            odf::utils::schema::parse::parse_ddl_to_odf_schema(&s.join(", "))
                                .ok()
                                .map(Arc::new)
                        })
                    }).map(|s| {
                        crate::prelude::DataSchema::new(s, format.unwrap_or(DataSchemaFormat::OdfJson))
                    })
                }
            }
            "#
        ),
    ),
    (
        "ReadStepParquet",
        indoc::indoc!(
            r#"
            #[derive(SimpleObject, Debug, Clone)]
            #[graphql(complex)]
            pub struct ReadStepParquet {
                /// DEPRECATED: A DDL-formatted schema. Schema can be used to coerce values
                /// into more appropriate data types.
                pub ddl_schema: Option<Vec<String>>,

                #[graphql(skip)]
                pub schema: Option<Arc<odf::schema::DataSchema>>,
            }

            impl From<odf::metadata::ReadStepParquet> for ReadStepParquet {
                fn from(v: odf::metadata::ReadStepParquet) -> Self {
                    let odf::metadata::ReadStepParquet { ddl_schema, schema } = v;
                    Self {
                        ddl_schema,
                        schema: schema.map(Arc::new),
                    }
                }
            }

            #[ComplexObject]
            impl ReadStepParquet {
                /// Schema used to coerce values into more appropriate data types.
                async fn schema(&self, format: Option<DataSchemaFormat>) -> Option<crate::prelude::DataSchema> {
                    self.schema.clone().or_else(|| {
                        self.ddl_schema.as_ref().and_then(|s| {
                            odf::utils::schema::parse::parse_ddl_to_odf_schema(&s.join(", "))
                                .ok()
                                .map(Arc::new)
                        })
                    }).map(|s| {
                        crate::prelude::DataSchema::new(s, format.unwrap_or(DataSchemaFormat::OdfJson))
                    })
                }
            }
            "#
        ),
    ),
];

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn render(model: model::Model, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let custom_types = std::collections::BTreeMap::from(CUSTOM_TYPES);

    writeln!(w, "{}", PREAMBLE)?;

    for typ in model.types.values() {
        if typ.id().name == "Manifest" {
            continue;
        }

        // Schemas are represented in GQL via embedded content, so we don't generate types for those
        if typ.category() == model::TypeCategory::DataSchema {
            continue;
        }

        if typ
            .codegen_hints()
            .get(&CodegenLanguage::Rust)
            .and_then(|h| h.get(&CodegenHint::DtoType))
            .is_some()
        {
            // Externally defined type
            continue;
        }

        writeln!(
            w,
            "////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////"
        )?;
        writeln!(w)?;
        render_description(typ.description(), None, None, w)?;
        writeln!(w, "///")?;
        writeln!(
            w,
            "/// See: {SPEC_URL}#{}-schema",
            typ.id().join("").to_lowercase()
        )?;

        if let Some(custom) = custom_types.get(typ.id().join("").as_str()) {
            writeln!(w, "{custom}")?;
        } else {
            match &typ {
                model::TypeDefinition::Struct(t) => render_struct(t, w)?,
                model::TypeDefinition::Union(t) => render_union(t, w)?,
                model::TypeDefinition::Enum(t) => render_enum(t, w)?,
                model::TypeDefinition::Map(t) => render_map(t, w)?,
            }
        }
        writeln!(w)?;
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_struct(typ: &model::Struct, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let name = typ.id.join("");
    let generics = format!(
        "<{}>",
        typ.generics
            .iter()
            .map(|_| "serde_json::Value".to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );

    writeln!(w, "#[derive(SimpleObject, Debug, Clone)]")?;
    writeln!(w, "pub struct {name} {{")?;

    if typ.fields.is_empty() {
        writeln!(w, "pub _dummy: Option<String>,")?;
    }

    for field in typ.fields.values() {
        render_description(
            &field.description,
            field.default.as_ref(),
            field.examples.as_ref(),
            w,
        )?;
        let mut typ = format_type(&field.typ);
        if let Some(container) = field
            .codegen_hints
            .get(&CodegenLanguage::Rust)
            .and_then(|m| m.get(&CodegenHint::Container))
        {
            typ = format!("{container}<{typ}>");
        }
        if field.optional {
            typ = format!("Option<{typ}>");
        }
        writeln!(w, "pub {}: {},", format_ident(&field.name), typ)?;
    }

    writeln!(w, "}}")?;

    writeln!(w)?;
    writeln!(
        w,
        "impl From<odf::metadata::{name}{generics}> for {name} {{"
    )?;
    writeln!(w, "fn from(v: odf::metadata::{name}{generics}) -> Self {{")?;
    writeln!(w, "Self {{")?;

    if typ.fields.is_empty() {
        writeln!(w, "_dummy: None")?;
    }

    for field in typ.fields.values() {
        let fname = format_ident(&field.name);

        let container = field
            .codegen_hints
            .get(&CodegenLanguage::Rust)
            .and_then(|m| m.get(&CodegenHint::Container));

        let convert = match &field.typ {
            model::Type::Array(_) => {
                if !field.optional {
                    format!("v.{fname}.into_iter().map(Into::into).collect()")
                } else {
                    format!("v.{fname}.map(|v| v.into_iter().map(Into::into).collect())")
                }
            }
            _ => {
                if !field.optional {
                    if let Some(container) = container {
                        format!("{container}::new((*v.{fname}).into())")
                    } else {
                        format!("v.{fname}.into()")
                    }
                } else {
                    if let Some(container) = container {
                        format!("v.{fname}.map(|v| {container}::new((*v).into()))")
                    } else {
                        format!("v.{fname}.map(Into::into)")
                    }
                }
            }
        };

        writeln!(w, "{fname}: {convert},")?;
    }
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_union(typ: &model::Union, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let name = typ.id.join("");

    writeln!(w, "#[derive(Union, Debug, Clone)]")?;
    writeln!(w, "pub enum {name} {{")?;
    for variant in &typ.variants {
        writeln!(w, "{}({}),", variant.name, variant.join(""))?;
    }
    writeln!(w, "}}")?;
    writeln!(w)?;
    writeln!(w, "impl From<odf::metadata::{name}> for {name} {{")?;
    writeln!(w, "fn from(v: odf::metadata::{name}) -> Self {{")?;
    writeln!(w, "match v {{")?;
    for variant in &typ.variants {
        let varname = &variant.name;
        writeln!(
            w,
            "odf::metadata::{name}::{varname}(v) => Self::{varname}(v.into()),"
        )?;
    }
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_enum(typ: &model::Enum, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let name = typ.id.join("");

    writeln!(w, "#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq)]")?;
    writeln!(w, "#[graphql(remote = \"odf::metadata::{name}\")]")?;

    writeln!(w, "pub enum {} {{", typ.id.join(""))?;
    for variant in &typ.variants {
        writeln!(w, "{variant},")?;
    }
    writeln!(w, "}}")?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// We represent all maps as JSON scalars
fn render_map(typ: &model::Map, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let name = typ.id.join("");

    writeln!(
        w,
        r#"
        #[nutype::nutype(derive(AsRef, Clone, Debug, From, Into))]
        pub struct {name}(odf::metadata::{name});
        
        #[async_graphql::Scalar]
        impl async_graphql::ScalarType for {name} {{
            fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {{
                let value: odf::metadata::serde::yaml::{name} = async_graphql::from_value(value)?;
                Ok(Self::new(value.into()))
            }}

            fn to_value(&self) -> async_graphql::Value {{
                let value: odf::metadata::serde::yaml::{name} = self.as_ref().clone().into();
                async_graphql::to_value(&value).unwrap()
            }}
        }}
        "#
    )?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn format_type(typ: &model::Type) -> String {
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
        model::Type::DatasetAlias => format!("DatasetAlias<'static>"),
        model::Type::DatasetId => format!("DatasetID<'static>"),
        model::Type::DatasetRef => format!("DatasetRef<'static>"),
        model::Type::DateTime => format!("DateTime<Utc>"),
        model::Type::Flatbuffers => format!("Vec<u8>"),
        model::Type::Multicodec => format!("Multicodec"),
        model::Type::Multihash => format!("Multihash<'static>"),
        model::Type::Path => format!("OSPath"),
        model::Type::Regex => format!("String"),
        model::Type::Url => format!("String"),
        model::Type::Generic(_) => format!("serde_json::Value"),
        model::Type::Array(t) => format!("Vec<{}>", format_type(&t.item_type)),
        model::Type::Custom(name) => name.join("").to_string(),
        model::Type::AnyJson => format!("serde_json::Value"),
        model::Type::AccountId => format!("AccountID<'static>"),
        model::Type::AccountName => format!("AccountName<'static>"),
        model::Type::ResourceContext => format!("ResourceContext<'static>"),
        model::Type::ResourceKind => format!("ResourceKind<'static>"),
        model::Type::ResourceId => format!("ResourceID<'static>"),
        model::Type::ResourceName => format!("ResourceName<'static>"),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_description(
    desc: &str,
    default: Option<&serde_json::Value>,
    examples: Option<&Vec<serde_json::Value>>,
    w: &mut dyn std::io::Write,
) -> Result<(), std::io::Error> {
    for line in desc.split('\n') {
        writeln!(w, "/// {line}")?;
    }
    if let Some(default) = default {
        writeln!(w, "///")?;
        writeln!(w, "/// Defaults to: {default}")?;
    }
    if let Some(examples) = examples {
        writeln!(w, "///")?;
        writeln!(w, "/// Examples:")?;
        for ex in examples {
            writeln!(w, "/// - {ex}")?;
        }
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
