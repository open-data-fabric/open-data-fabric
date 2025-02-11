#!/usr/bin/env python
import os
import re
import sys
import json

PREAMBLE = """
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
"""

DEFAULT_INDENT = 4

DOCS_URL = 'https://github.com/kamu-data/open-data-fabric/blob/master/open-data-fabric.md#{}-schema'

CUSTOM_TYPES = {
    "TransformInput": """
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
""",
    # TODO: Move the query/queries ambiguity to YAML layer, so it doesn't affect other layers
    "TransformSql": """
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
""",
    "SetDataSchema": """
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
""",
}

extra_types = []


def render(schemas_dir):
    schemas = read_schemas(schemas_dir)

    print(PREAMBLE)

    for name in sorted(schemas.keys()):
        sch = schemas[name]

        try:
            if name == 'Manifest':
                continue
            print('/' * 120)
            print(f'// {name}')
            print('// ' + DOCS_URL.format(name.lower()))
            print('/' * 120)
            print()

            for l in render_schema(name, sch):
                print(l)
            print()

            # Any extra sibling types that schema needs
            for gen in extra_types:
                for l in gen():
                    print(l)
                print()
            extra_types.clear()

        except Exception as ex:
            raise Exception(
                f'Error while rendering {name} schema:\n{sch}'
            ) from ex


def read_schemas(schemas_dir):
    schemas = {}
    read_schemas_rec(schemas_dir, schemas)
    return schemas


def read_schemas_rec(schemas_dir, schemas):
    for fname in os.listdir(schemas_dir):
        path = os.path.join(schemas_dir, fname)

        if os.path.isdir(path):
            read_schemas_rec(path, schemas)
            continue

        with open(path) as f:
            s = json.load(f)
            fname = os.path.splitext(os.path.split(path)[-1])[0]
            name = os.path.splitext(s['$id'].split('/')[-1])[0]
            assert fname == name, f"{fname} != {name}"
            schemas[name] = s


def render_schema(name, sch):
    if sch.get('type') == 'object':
        yield from render_struct(name, sch)
    elif 'oneOf' in sch:
        yield from render_oneof(name, sch)
    elif "enum" in sch and sch.get("type") == "string":
        yield from render_string_enum(name, sch)
    else:
        raise Exception(f'Unsupported schema: {sch}')


def render_struct(name, sch):
    custom = CUSTOM_TYPES.get(name)
    if custom:
        for line in custom.split('\n'):
            yield line
        return

    props = sch.get("properties", {})
    assert sch.get('additionalProperties', False) is False
    yield '#[derive(SimpleObject, Debug, Clone, PartialEq, Eq)]'
    yield f'pub struct {name} {{'
    if props:
        for pname, psch in props.items():
            required = pname in sch.get('required', ())
            yield from indent(render_field(pname, psch, required, 'pub'))
    else:
        yield from indent(render_field("_dummy", {"type": "string"}, False, 'pub'))
    yield '}'

    yield ''
    yield f'impl From<odf::metadata::{name}> for {name} {{'
    yield f'fn from(v: odf::metadata::{name}) -> Self {{'
    yield 'Self {'
    if not props:
        yield "_dummy: None"
    for pname, psch in props.items():
        required = pname in sch.get('required', ())
        array = psch.get('type') == 'array'
        field = to_snake_case(pname)
        if array:
            if required:
                yield f'{field}: v.{field}.into_iter().map(Into::into).collect(),'
            else:
                yield f'{field}: v.{field}.map(|v| v.into_iter().map(Into::into).collect()),'
        else:
            if required:
                yield f'{field}: v.{field}.into(),'
            else:
                yield f'{field}: v.{field}.map(Into::into),'
    yield '}'
    yield '}'
    yield '}'


def render_field(pname, psch, required, modifier=None):
    typ = get_composite_type(psch)

    if not required:
        typ = to_optional_type(psch, typ)

    ret = f'{to_snake_case(pname)}: {typ},'
    if modifier:
        ret = ' '.join((modifier, ret))
    yield ret


def render_oneof(name, sch):
    yield '#[derive(Union, Debug, Clone, PartialEq, Eq)]'
    yield f'pub enum {name} {{'
    for isch in sch["oneOf"]:
        yield from indent(render_oneof_element(name, sch, isch))
    yield '}'
    yield ''
    yield f'impl From<odf::metadata::{name}> for {name} {{'
    yield f'fn from(v: odf::metadata::{name}) -> Self {{'
    yield 'match v {'
    for isch in sch['oneOf']:
        ref = isch["$ref"]
        ename = ref.split('/')[-1]
        yield f'odf::metadata::{name}::{ename}(v) => Self::{ename}(v.into()),'
    yield '}'
    yield '}'
    yield '}'


def render_oneof_element(name, sch, isch):
    ref = isch["$ref"]
    ename = ref.split('/')[-1]

    if ref.startswith("#/$defs/"):
        esch = sch["$defs"][ename]

        struct_name = f'{name}{ename}'
        yield f'{ename}({struct_name}),'
        # See: https://github.com/rust-lang/rfcs/pull/2593
        extra_types.append(lambda: render_struct(struct_name, esch))
    else:
        yield f'{ename}({ename}),'


def render_string_enum(name, sch):
    yield '#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq)]'
    yield f'pub enum {name} {{'
    for value in sch['enum']:
        capitalized = value[0].upper() + value[1:]
        yield ' ' * DEFAULT_INDENT + capitalized + ','
    yield '}'
    yield ''
    yield f'impl From<odf::metadata::{name}> for {name} {{'
    yield f'fn from(v: odf::metadata::{name}) -> Self {{'
    yield 'match v {'
    for value in sch['enum']:
        capitalized = value[0].upper() + value[1:]
        yield f'odf::metadata::{name}::{capitalized} => Self::{capitalized},'
    yield '}'
    yield '}'
    yield '}'
    yield ''
    yield f'impl Into<odf::metadata::{name}> for {name} {{'
    yield f'fn into(self) -> odf::metadata::{name} {{'
    yield 'match self {'
    for value in sch['enum']:
        capitalized = value[0].upper() + value[1:]
        yield f'Self::{capitalized} => odf::metadata::{name}::{capitalized},'
    yield '}'
    yield '}'
    yield '}'


def get_composite_type(sch):
    if sch.get('type') == 'array':
        ptyp = get_primitive_type(sch['items'])
        return f'Vec<{ptyp}>'
    elif 'enum' in sch:
        assert sch['type'] == 'string'
        extra_types.append(lambda: render_string_enum(sch['enumName'], sch))
        return sch['enumName']
    else:
        return get_primitive_type(sch)


def get_primitive_type(sch):
    ptype = sch.get('type')
    fmt = sch.get('format')
    if fmt is not None:
        if fmt == 'int64':
            assert ptype == 'integer'
            return 'i64'
        if fmt == 'uint64':
            assert ptype == 'integer'
            return 'u64'
        # TODO: Use separate formats (newtype) for data hashes and block hashes
        elif fmt == 'multihash':
            assert ptype == 'string'
            return 'Multihash'
        elif fmt == 'url':
            assert ptype == 'string'
            return 'String'
        elif fmt == 'path':
            assert ptype == 'string'
            return 'OSPath'
        elif fmt == 'regex':
            assert ptype == 'string'
            return 'String'
        elif fmt == 'date-time':
            return 'DateTime<Utc>'
        elif fmt == 'dataset-id':
            return 'DatasetID'
        elif fmt == 'dataset-name':
            return 'DatasetName'
        elif fmt == 'dataset-alias':
            return 'DatasetAlias'
        elif fmt == 'dataset-ref':
            return 'DatasetRef'
        elif fmt == 'dataset-ref-any':
            return 'DatasetRefAny'
        else:
            raise Exception(f'Unsupported format: {sch}')
    if ptype == 'boolean':
        return 'bool'
    elif ptype == 'integer':
        return 'i32'
    elif ptype == 'string':
        return 'String'
    elif '$ref' in sch:
        return sch['$ref'].split('/')[-1]
    else:
        raise Exception(f'Expected primitive type schema: {sch}')


def to_optional_type(sch, typ):
    return f'Option<{typ}>'


def to_snake_case(name):
    name = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', name).lower()


def indent(gen, i=DEFAULT_INDENT):
    for l in gen:
        yield ' ' * i + l


if __name__ == "__main__":
    render(sys.argv[1])
