#!/usr/bin/env python
import os
import re
import json


PREAMBLE = """
////////////////////////////////////////////////////////////////////////////////
// WARNING: This file is auto-generated from Open Data Fabric Schemas
// See: http://opendatafabric.org/
////////////////////////////////////////////////////////////////////////////////

use std::path::PathBuf;
use super::formats::{base64, datetime_rfc3339, datetime_rfc3339_opt};
use crate::*;
use ::serde::{Deserialize, Deserializer, Serialize, Serializer};
use chrono::{DateTime, Utc};
use serde_with::serde_as;
use serde_with::skip_serializing_none;

////////////////////////////////////////////////////////////////////////////////

macro_rules! implement_serde_as {
    ($dto:ty, $impl:ty, $impl_name:literal) => {
        impl ::serde_with::SerializeAs<$dto> for $impl {
            fn serialize_as<S>(source: &$dto, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                <$impl>::serialize(source, serializer)
            }
        }

        impl<'de> serde_with::DeserializeAs<'de, $dto> for $impl {
            fn deserialize_as<D>(deserializer: D) -> Result<$dto, D::Error>
            where
                D: Deserializer<'de>,
            {
                <$impl>::deserialize(deserializer)
            }
        }
    };
}

"""

DEFAULT_INDENT = 2

DOCS_URL = 'https://github.com/kamu-data/open-data-fabric/blob/master/open-data-fabric.md#{}-schema'


extra_types = []


def render(schemas_dir):
    schemas = read_schemas(schemas_dir)

    for l in PREAMBLE.split('\n'):
        print(l)

    for name in sorted(schemas.keys()):
        sch = schemas[name]
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
        yield ''
        yield from render_struct_as(name, sch)
    elif 'oneOf' in sch:
        yield from render_oneof(name, sch)
        yield ''
        yield from render_oneof_as(name, sch)
    elif "enum" in sch and sch.get("type") == "string":
        yield from render_string_enum(name, sch)
        yield ''
        yield from render_struct_as(name, sch)
    else:
        raise Exception(f'Unsupported schema: {sch}')


def render_struct(name, sch):
    yield '#[serde_as]'
    yield '#[skip_serializing_none]'
    yield '#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]'
    yield f'#[serde(remote = "{name}")]'
    yield '#[serde(deny_unknown_fields, rename_all = "camelCase")]'
    yield f'pub struct {name}Def {{'
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
    elif psch.get("format") == "flatbuffers":
        yield '#[serde(with = "base64")]'

    if not required:
        typ = to_optional_type(typ)

    ret = f'{to_snake_case(pname)}: {typ},'
    if modifier:
        ret = ' '.join((modifier, ret))

    ext_typ = get_composite_type_external(psch)
    if ext_typ:
        if not required:
            ext_typ = to_optional_type(ext_typ)
        yield f'#[serde_as(as = "{ext_typ}")]'
        if not required:
            yield '#[serde(default)]'
    yield ret


def render_struct_as(name, sch):
    yield f"implement_serde_as!({name}, {name}Def, \"{name}Def\");"


def render_oneof(name, sch):
    yield '#[serde_as]'
    yield '#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]'
    yield f'#[serde(remote = "{name}")]'
    yield '#[serde(deny_unknown_fields, tag = "kind")]'
    yield f'pub enum {name}Def {{'
    for isch in sch["oneOf"]:
        yield from indent(render_oneof_element(name, sch, isch))
    yield '}'


def render_oneof_element(name, sch, isch):
    ref = isch["$ref"]
    ename = ref.split('/')[-1]

    # Allow lowercase and camelCase names
    aliases = {ename.lower(), ename[0].lower() + ename[1:]}
    yield '#[serde({})]'.format(', '.join(f'alias = "{a}"' for a in sorted(aliases)))

    if ref.startswith("#/$defs/"):
        esch = sch["$defs"][ename]
        struct_name = f'{name}{ename}'
        yield f'{ename}(#[serde_as(as = "{struct_name}Def")] {struct_name}),'
        # See: https://github.com/rust-lang/rfcs/pull/2593
        extra_types.append(lambda: render_struct(struct_name, esch))
    else:
        yield f'{ename}(#[serde_as(as = "{ename}Def")] {ename}),'


def render_oneof_as(name, sch):
    yield from render_struct_as(name, sch)
    for (ename, esch) in sch.get('$defs', {}).items():
        struct_name = f'{name}{ename}'
        yield from render_struct_as(struct_name, esch)


def render_string_enum(name, sch):
    yield '#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]'
    yield f'#[serde(remote = "{name}")]'
    yield '#[serde(deny_unknown_fields)]'
    yield f'pub enum {name}Def {{'
    indent = ' ' * DEFAULT_INDENT
    for value in sch['enum']:
        # Allow lowercase and camelCase names
        aliases = {value.lower(), value[0].lower() + value[1:]}
        yield indent + '#[serde({})]'.format(', '.join(f'alias = "{a}"' for a in sorted(aliases)))
        yield indent + value + ','
    yield '}'


def get_composite_type(sch):
    if sch.get('type') == 'array':
        ptyp = get_primitive_type(sch['items'])
        return f'Vec<{ptyp}>'
    elif 'enum' in sch:
        assert sch['type'] == 'string'
        extra_types.append(lambda: render_string_enum(sch['enumName'], sch))
        extra_types.append(lambda: render_struct_as(sch['enumName'], sch))
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
        elif fmt == 'multihash':
            assert ptype == 'string'
            return 'Multihash'
        elif fmt == 'url':
            assert ptype == 'string'
            return 'String'
        elif fmt == 'path':
            assert ptype == 'string'
            return 'PathBuf'
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
        elif fmt == 'flatbuffers':
            assert ptype == 'string'
            return 'Vec<u8>'
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


def to_optional_type(typ):
    return f'Option<{typ}>'


def get_composite_type_external(sch):
    if sch.get('type') == 'array':
        ptyp = get_primitive_type_external(sch['items'])
        return f'Vec<{ptyp}>' if ptyp else None
    elif 'enum' in sch:
        assert sch['type'] == 'string'
        return sch['enumName'] + 'Def'
    else:
        return get_primitive_type_external(sch)


def get_primitive_type_external(sch):
    if '$ref' in sch:
        return sch['$ref'].split('/')[-1] + 'Def'
    return None


def to_snake_case(name):
    name = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', name).lower()


def indent(gen, i=DEFAULT_INDENT):
    for l in gen:
        yield ' ' * i + l


if __name__ == "__main__":
    import sys
    render(sys.argv[1])
