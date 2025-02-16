use crate::model;

const SPEC_URL: &str =
    "https://github.com/kamu-data/open-data-fabric/blob/master/open-data-fabric.md";

const PREAMBLE: &str = indoc::indoc!(
    r#"
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // WARNING: This file is auto-generated from Open Data Fabric Schemas
    // See: http://opendatafabric.org/
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    #![allow(unused_variables)]

    #![allow(clippy::all)]
    #![allow(clippy::pedantic)]

    use chrono::{DateTime, Utc};

    use crate::prelude::*;
    use crate::queries::Dataset;
    "#
);

const CUSTOM_TYPES: [(&str, &str); 3] = [
    (
        "TransformInput",
        indoc::indoc!(
            r#"
            #[derive(Interface, Debug, Clone)]
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

            #[derive(SimpleObject, Debug, Clone)]
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

            #[derive(SimpleObject, Debug, Clone)]
            #[graphql(complex)]
            pub struct TransformInputDatasetNotAccessible {
                pub dataset_ref: DatasetRef,
            }

            #[ComplexObject]
            impl TransformInputDatasetNotAccessible {
                async fn message(&self) -> String {
                    "Not Accessible".to_string()
                }
            }

            #[derive(SimpleObject, Debug, Clone, PartialEq, Eq)]
            #[graphql(complex)]
            pub struct TransformInput {
                pub dataset_ref: DatasetRef,
                pub alias: String,
            }

            #[ComplexObject]
            impl TransformInput {
                async fn input_dataset(&self, ctx: &Context<'_>) -> Result<TransformInputDataset> {
                    Dataset::try_from_ref(ctx, &self.dataset_ref).await
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
            #[derive(SimpleObject, Debug, Clone, PartialEq, Eq)]
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
            #[derive(SimpleObject, Debug, Clone, PartialEq, Eq)]
            pub struct SetDataSchema {
                pub schema: DataSchema,
            }

            impl From<odf::metadata::SetDataSchema> for SetDataSchema {
                fn from(v: odf::metadata::SetDataSchema) -> Self {
                    // TODO: Error handling?
                    // TODO: Externalize format decision?
                    let arrow_schema = v.schema_as_arrow().unwrap();
                    let schema = DataSchema::from_arrow_schema(&arrow_schema, DataSchemaFormat::ParquetJson);
                    Self { schema }
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

        writeln!(w, "////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////")?;
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
                model::TypeDefinition::Object(t) => render_object(t, w)?,
                model::TypeDefinition::Union(t) => render_union(t, w)?,
                model::TypeDefinition::Enum(t) => render_enum(t, w)?,
            }
        }
        writeln!(w)?;
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_object(typ: &model::Object, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let name = typ.id.join("");

    writeln!(w, "#[derive(SimpleObject, Debug, Clone, PartialEq, Eq)]")?;
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
        if !field.optional {
            writeln!(w, "pub {}: {},", field.name, format_type(&field.typ))?;
        } else {
            writeln!(
                w,
                "pub {}: Option<{}>,",
                field.name,
                format_type(&field.typ)
            )?;
        }
    }

    writeln!(w, "}}")?;
    writeln!(w)?;
    writeln!(w, "impl From<odf::metadata::{name}> for {name} {{")?;
    writeln!(w, "fn from(v: odf::metadata::{name}) -> Self {{")?;
    writeln!(w, "Self {{")?;

    if typ.fields.is_empty() {
        writeln!(w, "_dummy: None")?;
    }

    for field in typ.fields.values() {
        let fname = &field.name;

        match &field.typ {
            model::Type::Array(_) => {
                if !field.optional {
                    writeln!(
                        w,
                        "{fname}: v.{fname}.into_iter().map(Into::into).collect(),"
                    )?;
                } else {
                    writeln!(
                        w,
                        "{fname}: v.{fname}.map(|v| v.into_iter().map(Into::into).collect()),"
                    )?;
                }
            }
            _ => {
                if !field.optional {
                    writeln!(w, "{fname}: v.{fname}.into(),")?;
                } else {
                    writeln!(w, "{fname}: v.{fname}.map(Into::into),")?;
                }
            }
        }
    }
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn render_union(typ: &model::Union, w: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
    let name = typ.id.join("");

    writeln!(w, "#[derive(Union, Debug, Clone, PartialEq, Eq)]")?;
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
    writeln!(w, "pub enum {} {{", typ.id.join(""))?;
    for variant in &typ.variants {
        writeln!(w, "{variant},")?;
    }
    writeln!(w, "}}")?;
    writeln!(w)?;
    writeln!(w, "impl From<odf::metadata::{name}> for {name} {{")?;
    writeln!(w, "fn from(v: odf::metadata::{name}) -> Self {{")?;
    writeln!(w, "match v {{")?;
    for variant in &typ.variants {
        writeln!(w, "odf::metadata::{name}::{variant} => Self::{variant},")?;
    }
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    writeln!(w)?;
    writeln!(w, "impl Into<odf::metadata::{name}> for {name} {{")?;
    writeln!(w, "fn into(self) -> odf::metadata::{name} {{")?;
    writeln!(w, "match self {{")?;
    for variant in &typ.variants {
        writeln!(w, "Self::{variant} => odf::metadata::{name}::{variant},")?;
    }
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;
    writeln!(w, "}}")?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn format_type(typ: &model::Type) -> String {
    match typ {
        model::Type::Boolean => format!("bool"),
        model::Type::Int16 => format!("i16"),
        model::Type::Int32 => format!("i32"),
        model::Type::Int64 => format!("i64"),
        model::Type::UInt16 => format!("u16"),
        model::Type::UInt32 => format!("u32"),
        model::Type::UInt64 => format!("u64"),
        model::Type::String => format!("String"),
        model::Type::DatasetAlias => format!("DatasetAlias"),
        model::Type::DatasetId => format!("DatasetID"),
        model::Type::DatasetRef => format!("DatasetRef"),
        model::Type::DateTime => format!("DateTime<Utc>"),
        model::Type::Flatbuffers => format!("Vec<u8>"),
        model::Type::Multicodec => format!("Multicodec"),
        model::Type::Multihash => format!("Multihash"),
        model::Type::Path => format!("OSPath"),
        model::Type::Regex => format!("String"),
        model::Type::Url => format!("String"),
        model::Type::Array(t) => format!("Vec<{}>", format_type(&t.item_type)),
        model::Type::Custom(name) => name.join("").to_string(),
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
