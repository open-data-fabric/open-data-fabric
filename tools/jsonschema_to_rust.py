#!/usr/bin/env python
import os
import re
import json


PREAMBLE = """///////////////////////////////////////////////////////////////////////////////
// WARNING: This file is auto-generated from Open Data Fabric Schemas
// See: http://opendatafabric.org/
///////////////////////////////////////////////////////////////////////////////

use super::formats::{datetime_rfc3339, datetime_rfc3339_opt};
use crate::domain::DatasetID;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
"""

DEFAULT_INDENT = 2

DOCS_URL = 'https://github.com/kamu-data/open-data-fabric/blob/master/open-data-fabric.md#{}-schema'


extra_types = []


def render(schemas_dir):
    schemas = read_schemas(schemas_dir)

    print(PREAMBLE)

    for name, sch in schemas.items():
        try:
            if name == 'Manifest':
                continue
            print('/' * 80)
            print(f'// {name}')
            print('// ' + DOCS_URL.format(name.lower()))
            print('/' * 80)
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
    for sch in os.listdir(schemas_dir):
        path = os.path.join(schemas_dir, sch)
        if not os.path.isfile(path):
            continue
        with open(path) as f:
            s = json.load(f)
            name = os.path.splitext(s['$id'].split('/')[-1])[0]
            schemas[name] = s
    return schemas


def render_schema(name, sch):
    if sch.get('type') == 'object':
        yield from render_struct(name, sch)
    elif 'oneOf' in sch:
        yield from render_oneof(name, sch)
    else:
        raise Exception(f'Unsupported schema: {sch}')


def render_struct(name, sch):
    assert sch.get('additionalProperties', False) is False
    yield '#[skip_serializing_none]'
    yield '#[serde(deny_unknown_fields, rename_all = "camelCase")]'
    yield '#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]'
    yield f'pub struct {name} {{'
    for pname, psch in sch.get('properties', {}).items():
        required = pname in sch.get('required', ())
        yield from indent(render_field(pname, psch, required, 'pub'))
    yield '}'


def render_field(pname, psch, required, modifier=None):
    typ = get_composite_type(psch)

    if typ == 'DateTime<Utc>':
        if required:
            yield '#[serde(with = "datetime_rfc3339")]'
        else:
            yield '#[serde(default, with = "datetime_rfc3339_opt")]'

    if not required:
        typ = to_optional_type(psch, typ)

    ret = f'{to_snake_case(pname)}: {typ},'
    if modifier:
        ret = ' '.join((modifier, ret))
    yield ret


def render_oneof(name, sch):
    yield '#[skip_serializing_none]'
    yield '#[serde(deny_unknown_fields, rename_all = "camelCase", tag = "kind")]'
    yield '#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]'
    yield f'pub enum {name} {{'
    for (ename, esch) in sch.get('$defs', {}).items():
        yield from indent(render_oneof_element(name, ename, esch))
    yield '}'


def render_oneof_element(name, ename, esch):
    yield '#[serde(rename_all = "camelCase")]'
    if not esch.get('properties', ()):
        yield f'{ename},'
    else:
        struct_name = f'{name}{ename}'
        yield f'{ename}({struct_name}),'
        # See: https://github.com/rust-lang/rfcs/pull/2593
        extra_types.append(lambda: render_struct(struct_name, esch))


def render_string_enum(name, sch):
    yield '#[skip_serializing_none]'
    yield '#[serde(deny_unknown_fields, rename_all = "camelCase")]'
    yield '#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]'
    yield f'pub enum {name} {{'
    for value in sch['enum']:
        capitalized = value[0].upper() + value[1:]
        yield ' ' * DEFAULT_INDENT + capitalized + ','
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
        elif fmt == 'multihash':
            assert ptype == 'string'
            return 'Multihash'
        elif fmt == 'url':
            assert ptype == 'string'
            return 'String'
        elif fmt == 'regex':
            assert ptype == 'string'
            return 'String'
        elif fmt == 'date-time':
            return 'DateTime<Utc>'
        elif fmt == 'dataset-id':
            return 'DatasetID'
        elif fmt == 'dataset-name':
            assert ptype == 'string'
            return 'DatasetName'
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
    import sys
    render(sys.argv[1])
