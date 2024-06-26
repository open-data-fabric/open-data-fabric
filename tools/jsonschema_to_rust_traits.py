#!/usr/bin/env python
import os
import re
import sys
import json

PREAMBLE = """
////////////////////////////////////////////////////////////////////////////////
// WARNING: This file is auto-generated from Open Data Fabric Schemas
// See: http://opendatafabric.org/
////////////////////////////////////////////////////////////////////////////////

#![allow(clippy::all)]
#![allow(clippy::pedantic)]

use crate::dtos;
use crate::dtos::{CompressionFormat, DatasetKind, SourceOrdering, MqttQos};
use crate::identity::*;
use crate::formats::*;
use chrono::{DateTime, Utc};
use std::path::Path;
"""

DEFAULT_INDENT = 4

DOCS_URL = 'https://github.com/kamu-data/open-data-fabric/blob/master/open-data-fabric.md#{}-schema'

struct_types = set()
extra_types = []


def is_struct_type(typ):
    return typ in struct_types


def is_string_enum(typ):
    return typ in (
        'CompressionFormat',
        'SourceOrdering',
        'DatasetKind',
        'MqttQos',
    )


def render(schemas_dir):
    schemas = read_schemas(schemas_dir)

    for name, sch in schemas.items():
        if sch.get("type") == "object":
            struct_types.add(name)

    print(PREAMBLE)

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
        yield from render_struct_impl(name, sch)
        yield ''
        yield from render_struct_convert(name, sch)
    elif 'oneOf' in sch:
        yield from render_oneof(name, sch)
        yield ''
        yield from render_oneof_impl(name, sch)
        yield ''
        yield from render_oneof_convert(name, sch)
    elif "enum" in sch and sch.get("type") == "string":
        pass
    else:
        raise Exception(f'Unsupported schema: {sch}')


def render_struct(name, sch):
    assert sch.get('additionalProperties', False) is False
    yield f"pub trait {name} {{"
    for pname, psch in sch.get('properties', {}).items():
        required = pname in sch.get('required', ())
        yield from indent(render_field(pname, psch, required, 'pub'))
    yield '}'


def render_struct_impl(name, sch):
    yield f"impl {name} for dtos::{name} {{"
    for pname, psch in sch.get('properties', {}).items():
        required = pname in sch.get('required', ())
        yield from indent(render_field_impl(pname, psch, required, 'pub'))
    yield '}'


def render_struct_convert(name, sch):
    yield f"impl Into<dtos::{name}> for &dyn {name} {{"
    yield ' ' * DEFAULT_INDENT + f"fn into(self) -> dtos::{name} {{"
    yield ' ' * DEFAULT_INDENT * 2 + f"dtos::{name} {{"
    for pname, psch in sch.get('properties', {}).items():
        required = pname in sch.get('required', ())
        yield from indent(indent(indent(render_field_convert(pname, psch, required))))
    yield ' ' * DEFAULT_INDENT * 2 + '}'
    yield ' ' * DEFAULT_INDENT + '}'
    yield '}'


def render_field(pname, psch, required, modifier=None):
    typ = get_composite_type(psch)

    if not required:
        typ = to_optional_type(psch, typ)

    ret = f'fn {to_snake_case(pname)}(&self) -> {typ};'
    yield ret


def render_field_impl(pname, psch, required, modifier=None):
    typ = get_composite_type(psch)

    if not required:
        typ = to_optional_type(psch, typ)

    yield f'fn {to_snake_case(pname)}(&self) -> {typ} {{'
    yield from indent(render_accessor('self.' + to_snake_case(pname), psch, not required))
    yield '}'


def render_field_convert(pname, psch, required):
    name = to_snake_case(pname)
    yield f"{name}:"
    yield from indent(render_clone(f'self.{name}()', psch, not required))
    yield ','


def render_oneof(name, sch):
    yield f"pub enum {name}<'a> {{"
    for isch in sch["oneOf"]:
        yield from indent(render_oneof_element(name, sch, isch))
    yield '}'


def render_oneof_element(name, sch, isch):
    ref = isch["$ref"]
    ename = ref.split('/')[-1]

    if ref.startswith("#/$defs/"):
        esch = sch["$defs"][ename]
        struct_name = f'{name}{ename}'
        yield f"{ename}(&'a dyn {struct_name}),"
        # See: https://github.com/rust-lang/rfcs/pull/2593
        extra_types.append(lambda: render_struct(struct_name, esch))
    else:
        yield f"{ename}(&'a dyn {ename}),"


def render_oneof_impl(name, sch):
    yield f"impl<'a> From<&'a dtos::{name}> for {name}<'a> {{"
    yield ' ' * DEFAULT_INDENT + f"fn from(other: &'a dtos::{name}) -> Self {{"
    yield ' ' * DEFAULT_INDENT * 2 + f"match other {{"
    for isch in sch["oneOf"]:
        yield from indent(indent(indent(render_oneof_element_impl(name, sch, isch))))
    yield ' ' * DEFAULT_INDENT * 2 + '}'
    yield ' ' * DEFAULT_INDENT + '}'
    yield '}'


def render_oneof_element_impl(name, sch, isch):
    ref = isch["$ref"]
    ename = ref.split('/')[-1]

    if ref.startswith("#/$defs/"):
        esch = sch["$defs"][ename]
        struct_name = f'{name}{ename}'
        yield f"dtos::{name}::{ename}(v) => {name}::{ename}(v),"
        extra_types.append(lambda: render_struct_impl(struct_name, esch))
    else:
        yield f"dtos::{name}::{ename}(v) => {name}::{ename}(v),"


def render_oneof_convert(name, sch):
    yield f"impl Into<dtos::{name}> for {name}<'_> {{"
    yield ' ' * DEFAULT_INDENT + f"fn into(self) -> dtos::{name} {{"
    yield ' ' * DEFAULT_INDENT * 2 + f"match self {{"
    for isch in sch["oneOf"]:
        yield from indent(render_oneof_element_convert(name, sch, isch), DEFAULT_INDENT * 3)
    yield ' ' * DEFAULT_INDENT * 2 + '}'
    yield ' ' * DEFAULT_INDENT + '}'
    yield '}'


def render_oneof_element_convert(name, sch, isch):
    ref = isch["$ref"]
    ename = ref.split('/')[-1]

    if ref.startswith("#/$defs/"):
        esch = sch["$defs"][ename]
        struct_name = f'{name}{ename}'
        yield f"{name}::{ename}(v) => dtos::{name}::{ename}(v.into()),"
        extra_types.append(lambda: render_struct_convert(struct_name, esch))
    else:
        yield f"{name}::{ename}(v) => dtos::{name}::{ename}(v.into()),"


def render_string_enum(name, sch):
    yield f'pub enum {name} {{'
    for value in sch['enum']:
        capitalized = value[0].upper() + value[1:]
        yield ' ' * DEFAULT_INDENT + capitalized + ','
    yield '}'


def get_composite_type(sch):
    if sch.get('type') == 'array':
        ptyp = get_primitive_type(sch['items'])
        return f"Box<dyn Iterator<Item = {ptyp}> + '_>"
    elif 'enum' in sch:
        assert sch['type'] == 'string'
        # extra_types.append(lambda: render_string_enum(sch['enumName'], sch))
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
            return "&Multihash"
        elif fmt == 'url':
            assert ptype == 'string'
            return "&str"
        elif fmt == 'path':
            assert ptype == 'string'
            return "&Path"
        elif fmt == 'regex':
            assert ptype == 'string'
            return "&str"
        elif fmt == 'date-time':
            return "DateTime<Utc>"
        elif fmt == 'dataset-id':
            return "&DatasetID"
        elif fmt == 'dataset-name':
            return "&DatasetName"
        elif fmt == 'dataset-alias':
            return "&DatasetAlias"
        elif fmt == 'dataset-ref':
            return "&DatasetRef"
        elif fmt == 'dataset-ref-any':
            return "&DatasetRefAny"
        elif fmt == 'flatbuffers':
            assert ptype == 'string'
            return "&[u8]"
        else:
            raise Exception(f'Unsupported format: {sch}')
    if ptype == 'boolean':
        return 'bool'
    elif ptype == 'integer':
        return 'i32'
    elif ptype == 'string':
        return "&str"
    elif '$ref' in sch:
        t = sch['$ref'].split('/')[-1]
        if is_struct_type(t):
            return f"&dyn {t}"
        else:
            return f"{t}"
    else:
        raise Exception(f'Expected primitive type schema: {sch}')


def to_optional_type(sch, typ):
    return f'Option<{typ}>'


def render_accessor(name, sch, optional, in_ref=False):
    if optional:
        yield f'{name}.as_ref().map(|v| -> {get_composite_type(sch)} {{'
        yield from indent(render_accessor('v', sch, False, True))
        yield '})'
        return

    ptype = sch.get('type')
    fmt = sch.get('format')
    if 'enum' in sch:
        yield f'*{name}' if in_ref else name
    elif '$ref' in sch:
        t = sch['$ref'].split('/')[-1]
        if is_struct_type(t):
            yield name if in_ref else f'&{name}'
        elif not is_string_enum(t):
            yield f'{name}.into()' if in_ref else f'(&{name}).into()'
        else:
            yield f'*{name}' if in_ref else name
    elif fmt:
        if fmt in ('int64', 'uint64'):
            yield name if not in_ref else f'*{name}'
        elif fmt in ('date-time',):
            yield name if not in_ref else f'*{name}'
        elif fmt in ('url', 'path', 'regex'):
            yield f'{name}.as_ref()'
        elif fmt == 'multihash':
            yield f'{name}' if in_ref else f'&{name}'
        elif fmt in ('dataset-id', 'dataset-name', 'dataset-alias', 'dataset-ref', 'dataset-ref-any', 'flatbuffers'):
            yield f'{name}' if in_ref else f'&{name}'
        else:
            raise Exception(f'Unsupported format: {sch}')
    elif ptype == 'boolean':
        yield name if not in_ref else f'*{name}'
    elif ptype == 'integer':
        yield name if not in_ref else f'*{name}'
    elif ptype == 'array':
        yield f"Box::new({name}.iter().map(|i| -> {get_composite_type(sch['items'])} {{"
        yield from indent(render_accessor('i', sch['items'], False, True))
        yield '}))'
    elif ptype == 'string':
        yield f'{name}.as_ref()'
    else:
        raise Exception(f'Unsupported format: {sch}')


def render_clone(name, sch, optional):
    if optional:
        yield f'{name}.map(|v| {{'
        yield from indent(render_clone('v', sch, False))
        yield '})'
        return

    ptype = sch.get('type')
    fmt = sch.get('format')
    if 'enum' in sch:
        yield f'{name}.into()'
    elif '$ref' in sch:
        yield f'{name}.into()'
    elif fmt:
        if fmt in ('int64', 'uint64'):
            yield name
        elif fmt in ('date-time',):
            yield name
        elif fmt in ('dataset-name', 'dataset-alias', 'dataset-ref', 'dataset-ref-any', 'url', 'path', 'regex'):
            yield f'{name}.to_owned()'
        elif fmt == 'multihash':
            yield f'{name}.clone()'
        elif fmt == 'dataset-id':
            yield f'{name}.clone()'
        elif fmt == 'flatbuffers':
            yield f'{name}.to_vec()'
        else:
            raise Exception(f'Unsupported format: {sch}')
    elif ptype == 'boolean':
        yield name
    elif ptype == 'integer':
        yield name
    elif ptype == 'array':
        yield f"{name}.map(|i| {{"
        yield from indent(render_clone('i', sch['items'], False))
        yield '}).collect()'
    elif ptype == 'string':
        yield f'{name}.to_owned()'
    else:
        raise Exception(f'Unsupported format: {sch}')


def to_snake_case(name):
    name = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', name).lower()


def indent(gen, i=DEFAULT_INDENT):
    for l in gen:
        yield ' ' * i + l


if __name__ == "__main__":
    render(sys.argv[1])
