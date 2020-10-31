#!/usr/bin/env python
import os
import re
import sys
import json


PREAMBLE = [
    "/" * 80,
    "// WARNING: This file is auto-generated from Open Data Fabric Schemas",
    "// See: http://opendatafabric.org/",
    "/" * 80,
    "",
    "use crate as odf;"
    "use super::odf_generated as fb;"
    "use ::flatbuffers::{FlatBufferBuilder, Table, UnionWIPOffset, WIPOffset};",
    "use std::convert::TryInto;",
    "use chrono::prelude::*;",
    "",
    "pub trait FlatbuffersSerializable<'fb, T> {",
    "    fn serialize(&self, fb: &mut FlatBufferBuilder<'fb>) -> WIPOffset<T>;",
    "}",
    "",
    "pub trait FlatbuffersDeserializable<T> {",
    "    fn deserialize(fb: T) -> Self;",
    "}",
    "",
    "trait FlatbuffersEnumSerializable<'fb, E> {",
    "    fn serialize(&self, fb: &mut FlatBufferBuilder<'fb>) -> (E, WIPOffset<UnionWIPOffset>);",
    "}",
    "",
    "trait FlatbuffersEnumDeserializable<'fb, E> {",
    "    fn deserialize(table: Table<'fb>, t: E) -> Self;",
    "}",
    '',
]

INDENT = 4

DOCS_URL = 'https://github.com/kamu-data/open-data-fabric/blob/master/open-data-fabric.md#{}-schema'


extra_types = []


def is_struct_type(typ):
    return typ in (
        'DatasetVocabulary',
        'DataSlice',
        'SqlQueryStep',
        'TemporalTable',
    )


def is_string_enum(typ):
    return typ in (
        'CompressionFormat',
        'SourceOrdering',
    )


def render(schemas_dir):
    schemas = read_schemas(schemas_dir)

    for l in PREAMBLE:
        print(l)

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
    yield f"impl<'fb> FlatbuffersSerializable<'fb, fb::{name}<'fb>> for odf::{name} {{"
    yield f"    fn serialize(&self, fb: &mut FlatBufferBuilder <'fb>) -> WIPOffset<fb::{name}<'fb>> {{"
    preserialized = set()
    for pname, psch in sch.get('properties', {}).items():
        required = pname in sch.get('required', ())
        lines = list(indent(render_field_pre_ser(
            pname, psch, required), INDENT * 2))
        if lines:
            preserialized.add(pname)
            for l in lines:
                yield l
    yield f"        let mut builder = fb::{name}Builder::new(fb);"
    for pname, psch in sch.get('properties', {}).items():
        required = pname in sch.get('required', ())
        yield from indent(render_field_ser(pname, psch, required, pname in preserialized), INDENT * 2)
    yield f"        builder.finish()"
    yield "    }"
    yield "}"
    yield ""
    yield f"impl<'fb> FlatbuffersDeserializable<fb::{name}<'fb>> for odf::{name} {{"
    yield f"    fn deserialize(proxy: fb::{name}<'fb>) -> Self {{"
    yield f"        odf::{name} {{"
    for pname, psch in sch.get('properties', {}).items():
        required = pname in sch.get('required', ())
        yield from indent(render_field_de(pname, psch, required), INDENT * 3)
    yield "        }"
    yield "    }"
    yield "}"


def render_field_pre_ser(pname, psch, required):
    name = to_snake_case(pname)
    if not required:
        lines = list(indent(pre_ser_composite_type('v', psch)))
        if lines:
            yield f'let {name}_offset = self.{name}.as_ref().map(|v| {{'
            for l in lines:
                yield l
            yield '});'
    else:
        lines = list(indent(pre_ser_composite_type(f'self.{name}', psch)))
        if lines:
            yield f'let {name}_offset = {{'
            for l in lines:
                yield l
            yield '};'


def render_field_ser(pname, psch, required, preserialized):
    name = to_snake_case(pname)
    if preserialized:
        if required:
            yield f"builder.add_{name}({name}_offset);"
        else:
            yield f"{name}_offset.map(|off| builder.add_{name}(off));"
    else:
        if required:
            yield f"builder.add_{name}({{"
            yield from ser_composite_type(f"self.{name}", psch)
            yield "});"
        else:
            yield f"self.{name}.as_ref().map(|v| {{"
            yield from ser_composite_type("v", psch)
            yield f"}}).map(|v| builder.add_{name}(&v));"


def render_field_de(pname, psch, required):
    name = to_snake_case(pname)
    yield f"{name}:"
    yield f"    proxy.{name}().map(|v| {{"
    yield from indent(indent(de_composite_type("v", psch, f"proxy.{name}_type()")))
    yield f"    }})"
    if required:
        yield "    .unwrap()"
    yield ","


def render_oneof(name, sch):
    yield f"impl<'fb> FlatbuffersEnumSerializable<'fb, fb::{name}> for odf::{name} {{"
    yield f"    fn serialize(&self, fb: &mut FlatBufferBuilder<'fb>) -> (fb::{name}, WIPOffset<UnionWIPOffset>) {{"
    yield f"        match self {{"
    for (ename, esch) in sch.get('definitions', {}).items():
        yield from indent(render_oneof_element_ser(name, ename, esch), INDENT * 3)
    yield "        }"
    yield "    }"
    yield "}"
    yield ""
    yield f"impl<'fb> FlatbuffersEnumDeserializable<'fb, fb::{name}> for odf::{name} {{"
    yield f"    fn deserialize(table: flatbuffers::Table<'fb>, t: fb::{name}) -> Self {{"
    yield f"        match t {{"
    yield f"            fb::{name}::NONE => panic!(\"Property is missing\"),"
    for (ename, esch) in sch.get('definitions', {}).items():
        yield from indent(render_oneof_element_de(name, ename, esch), INDENT * 3)
    yield "        }"
    yield "    }"
    yield "}"


def render_oneof_element_ser(name, ename, esch):
    struct_name = f'{name}{ename}'
    if not esch.get('properties', ()):
        yield f"odf::{name}::{ename} => panic!(\"Nothing to serialize\"),"
    else:
        yield f"odf::{name}::{ename}(v) => (fb::{name}::{struct_name}, v.serialize(fb).as_union_value()),"


def render_oneof_element_de(name, ename, esch):
    struct_name = f'{name}{ename}'
    if not esch.get('properties', ()):
        yield f"fb::{name}::{struct_name} => odf::{name}::{ename},"
    else:
        yield f"fb::{name}::{struct_name} => odf::{name}::{ename}("
        yield f"    odf::{struct_name}::deserialize("
        yield f"        fb::{struct_name}::init_from_table(table)"
        yield "    )"
        yield "),"
        extra_types.append(lambda: render_struct(struct_name, esch))


def render_string_enum(name, sch):
    return
    yield '#[derive(Clone, Copy, PartialEq, Eq, Debug)]'
    yield f'pub enum {name} {{'
    for value in sch['enum']:
        capitalized = value[0].upper() + value[1:]
        yield ' ' * INDENT + capitalized + ','
    yield '}'


def pre_ser_composite_type(name, sch):
    if sch.get('type') == 'array':
        yield f"let offsets: Vec<_> = {name}.iter().map(|i| {{"
        yield from pre_ser_primitive_type("i", sch['items'])
        yield "}).collect();"
        yield "fb.create_vector(&offsets)"
    elif 'enum' in sch:
        pass
    else:
        yield from pre_ser_primitive_type(name, sch)


def ser_composite_type(name, sch):
    if sch.get('type') == 'array':
        pass
    elif 'enum' in sch:
        pass
    else:
        yield from ser_primitive_type(name, sch)


def de_composite_type(name, sch, enum_t_accessor):
    if sch.get('type') == 'array':
        yield 'unimplemented!()'
        # yield f'{name}.map(|i| {{'
        # yield from indent(convert_primitive_type("i", sch['items']))
        # yield '}).collect()'
    elif 'enum' in sch:
        yield 'unimplemented!()'
        # assert sch['type'] == 'string'
        # extra_types.append(lambda: render_string_enum(sch['enumName'], sch))
        # return sch['enumName']
    else:
        yield from de_primitive_type(name, sch, enum_t_accessor)


def pre_ser_primitive_type(name, sch):
    ptype = sch.get('type')
    fmt = sch.get('format')
    if fmt is not None:
        if fmt == 'int64':
            assert ptype == 'integer'
        elif fmt == 'sha3-256':
            assert ptype == 'string'
            yield f'fb.create_vector(&{name})'
        elif fmt == 'url':
            assert ptype == 'string'
            yield f'fb.create_string(&{name})'
        elif fmt == 'regex':
            assert ptype == 'string'
            yield f'fb.create_string(&{name})'
        elif fmt == 'date-time':
            pass
        elif fmt == 'date-time-interval':
            pass
        elif fmt == 'dataset-id':
            yield f'fb.create_string(&{name})'
        else:
            raise Exception(f'Unsupported format: {sch}')
    elif ptype == 'boolean':
        pass
    elif ptype == 'integer':
        pass
    elif ptype == 'string':
        yield f'fb.create_string(&{name})'
    elif '$ref' in sch:
        yield f'{name}.serialize(fb)'
    else:
        raise Exception(f'Expected primitive type schema: {sch}')


def ser_primitive_type(name, sch):
    ptype = sch.get('type')
    fmt = sch.get('format')
    if fmt is not None:
        if fmt == 'int64':
            assert ptype == 'integer'
            yield f'&fb::Option_int64 {{ value_: {name} }}'
        elif fmt == 'sha3-256':
            assert ptype == 'string'
        elif fmt == 'url':
            assert ptype == 'string'
        elif fmt == 'regex':
            assert ptype == 'string'
        elif fmt == 'date-time':
            yield f'&fb::Timestamp {{ x_: 0 }}'
        elif fmt == 'date-time-interval':
            yield f'&fb::TimeInterval {{ x_: 0 }}'
        elif fmt == 'dataset-id':
            pass
        else:
            raise Exception(f'Unsupported format: {sch}')
    elif ptype == 'boolean':
        yield f'fb::Option_bool {{ value_: *{name} }}'
    elif ptype == 'integer':
        pass
    elif ptype == 'string':
        pass
    elif '$ref' in sch:
        pass
    else:
        raise Exception(f'Expected primitive type schema: {sch}')
        yield


def de_primitive_type(name, sch, enum_t_accessor):
    ptype = sch.get('type')
    fmt = sch.get('format')
    if fmt is not None:
        if fmt == 'int64':
            assert ptype == 'integer'
            yield f'{name}.value()'
        elif fmt == 'sha3-256':
            assert ptype == 'string'
            yield f'odf::Sha3_256::new({name}.try_into().unwrap())'
        elif fmt == 'url':
            assert ptype == 'string'
            yield f'{name}.to_owned()'
        elif fmt == 'regex':
            assert ptype == 'string'
            yield f'{name}.to_owned()'
        elif fmt == 'date-time':
            yield 'Utc.ymd(2121, 1, 1).and_hms(12, 0, 0)'
        elif fmt == 'date-time-interval':
            yield 'odf::TimeInterval::singleton(Utc.ymd(2121, 1, 1).and_hms(12, 0, 0))'
        elif fmt == 'dataset-id':
            yield 'unimplemented!()'
        else:
            raise Exception(f'Unsupported format: {sch}')
    elif ptype == 'boolean':
        yield f'{name}.value()'
    elif ptype == 'integer':
        yield f'{name}.value()'
    elif ptype == 'string':
        yield f'{name}.to_owned()'
    elif '$ref' in sch:
        t = sch['$ref'].split('.')[0]
        if is_struct_type(t):
            yield f'odf::{t}::deserialize({name})'
        else:
            yield f'odf::{t}::deserialize({name}, {enum_t_accessor})'
    else:
        raise Exception(f'Expected primitive type schema: {sch}')


def to_optional_type(sch, typ):
    return f'Option<{typ}>'


def to_snake_case(name):
    name = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', name).lower()


def indent(gen, i=INDENT):
    for l in gen:
        yield ' ' * i + l


if __name__ == "__main__":
    render(sys.argv[1])
