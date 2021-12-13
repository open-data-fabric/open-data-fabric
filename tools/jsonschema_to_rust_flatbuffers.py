#!/usr/bin/env python
import os
import re
import sys
import json
import functools


PREAMBLE = """
////////////////////////////////////////////////////////////////////////////////
// WARNING: This file is auto-generated from Open Data Fabric Schemas
// See: http://opendatafabric.org/
////////////////////////////////////////////////////////////////////////////////

#![allow(unused_variables)]
use super::odf_generated as fb;
mod odf {
    pub use crate::dataset_id::*;
    pub use crate::dtos::*;
    pub use crate::multihash::*;
    pub use crate::sha::*;
}
use ::flatbuffers::{FlatBufferBuilder, Table, UnionWIPOffset, WIPOffset};
use chrono::prelude::*;
use std::convert::{TryFrom, TryInto};
use std::path::PathBuf;

pub trait FlatbuffersSerializable<'fb> {
    type OffsetT;
    fn serialize(&self, fb: &mut FlatBufferBuilder<'fb>) -> Self::OffsetT;
}

pub trait FlatbuffersDeserializable<T> {
    fn deserialize(fb: T) -> Self;
}

pub trait FlatbuffersEnumSerializable<'fb, E> {
    fn serialize(&self, fb: &mut FlatBufferBuilder<'fb>) -> (E, WIPOffset<UnionWIPOffset>);
}

pub trait FlatbuffersEnumDeserializable<'fb, E> {
    fn deserialize(table: Table<'fb>, t: E) -> Self
    where
        Self: Sized;
}
"""

FOOTER = """
///////////////////////////////////////////////////////////////////////////////
// Helpers
///////////////////////////////////////////////////////////////////////////////

fn datetime_to_fb(dt: &DateTime<Utc>) -> fb::Timestamp {
    fb::Timestamp::new(
        dt.year(),
        dt.ordinal() as u16,
        dt.naive_utc().num_seconds_from_midnight(),
        dt.naive_utc().nanosecond(),
    )
}

fn fb_to_datetime(dt: &fb::Timestamp) -> DateTime<Utc> {
    Utc.yo(dt.year(), dt.ordinal() as u32)
        .and_time(
            NaiveTime::from_num_seconds_from_midnight_opt(
                dt.seconds_from_midnight(),
                dt.nanoseconds(),
            )
            .unwrap(),
        )
        .unwrap()
}

fn empty_table<'fb>(
    fb: &mut FlatBufferBuilder<'fb>,
) -> WIPOffset<flatbuffers::TableFinishedWIPOffset> {
    let wip = fb.start_table();
    fb.end_table(wip)
}
"""

INDENT = 4

DOCS_URL = 'https://github.com/kamu-data/open-data-fabric/blob/master/open-data-fabric.md#{}-schema'


struct_types = set()
enum_types = set()
string_enum_types = set()

extra_types = []


def is_struct_type(typ_or_sch):
    if isinstance(typ_or_sch, dict):
        typ_or_sch = typ_or_sch.get('$ref', '').split('/')[-1]
    return typ_or_sch in struct_types


def is_enum(typ_or_sch):
    if isinstance(typ_or_sch, dict):
        typ_or_sch = typ_or_sch.get('$ref', '').split('/')[-1]
    return typ_or_sch in enum_types


def is_string_enum(sch):
    return "enumName" in sch and sch.get("type") == "string"


def render(schemas_dir):
    schemas = read_schemas(schemas_dir)

    for name, sch in schemas.items():
        if sch.get("type") == "object":
            struct_types.add(name)
        elif 'oneOf' in sch:
            enum_types.add(name)

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

    print(FOOTER)


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
    yield f"impl<'fb> FlatbuffersSerializable<'fb> for odf::{name} {{"
    yield f"type OffsetT = WIPOffset<fb::{name}<'fb>>;"
    yield ""
    yield f"    fn serialize(&self, fb: &mut FlatBufferBuilder <'fb>) -> Self::OffsetT {{"
    preserialized = set()
    for pname, psch in sch.get('properties', {}).items():
        if is_string_enum(psch):
            extra_types.append(functools.partial(render_string_enum, psch))

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
        yield from indent(
            render_field_ser(pname, psch, required, pname in preserialized),
            INDENT * 2
        )
    yield f"        builder.finish()"
    yield "    }"
    yield "}"
    yield ""
    yield f"impl<'fb> FlatbuffersDeserializable<fb::{name}<'fb>> for odf::{name} {{"
    yield f"    fn deserialize(proxy: fb::{name}<'fb>) -> Self {{"
    yield f"        odf::{name} {{"
    for pname, psch in sch.get('properties', {}).items():
        required = pname in sch.get('required', ())
        yield from indent(
            render_field_de(pname, psch, required),
            INDENT * 3
        )
    yield "        }"
    yield "    }"
    yield "}"


def render_string_enum(sch):
    name = sch["enumName"]
    yield f"impl From<odf::{name}> for fb::{name} {{"
    yield f"    fn from(v: odf::{name}) -> Self {{"
    yield  "        match v {"
    for value in sch['enum']:
        capitalized = value[0].upper() + value[1:]
        yield ' ' * INDENT * 3 + f"odf::{name}::{capitalized} => fb::{name}::{capitalized},"
    yield "        }"
    yield "    }"
    yield "}"
    yield ""
    yield f"impl Into<odf::{name}> for fb::{name} {{"
    yield f"    fn into(self) -> odf::{name} {{"
    yield  "        match self {"
    for value in sch['enum']:
        capitalized = value[0].upper() + value[1:]
        yield ' ' * INDENT * 3 + f"fb::{name}::{capitalized} => odf::{name}::{capitalized},"
    yield "            _ => panic!(\"Invalid enum value: {}\", self.0),"
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
            if is_enum(psch):
                yield f"builder.add_{name}_type({name}_offset.0);"
                yield f"builder.add_{name}({name}_offset.1);"
            else:
                yield f"builder.add_{name}({name}_offset);"
        else:
            if is_enum(psch):
                yield f"{name}_offset.map(|(e, off)| {{ builder.add_{name}_type(e); builder.add_{name}(off) }});"
            else:
                yield f"{name}_offset.map(|off| builder.add_{name}(off));"

    else:
        if required:
            yield f"builder.add_{name}("
            yield from ser_composite_type(f"self.{name}", psch)
            yield ");"
        else:
            yield f"self.{name}.map(|v| builder.add_{name}("
            yield from ser_composite_type("v", psch)
            yield "));"


def render_field_de(pname, psch, required):
    name = to_snake_case(pname)
    yield f"{name}:"
    if required and (is_string_enum(psch) or psch.get("type") in ("integer",)):
        yield from indent(de_composite_type(f"proxy.{name}()", psch, f"proxy.{name}_type()"))
    else:
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
    for (ename, esch) in sch.get('$defs', {}).items():
        yield from indent(render_oneof_element_ser(name, ename, esch), INDENT * 3)
    yield "        }"
    yield "    }"
    yield "}"
    yield ""
    yield f"impl<'fb> FlatbuffersEnumDeserializable<'fb, fb::{name}> for odf::{name} {{"
    yield f"    fn deserialize(table: flatbuffers::Table<'fb>, t: fb::{name}) -> Self {{"
    yield f"        match t {{"
    for (ename, esch) in sch.get('$defs', {}).items():
        yield from indent(render_oneof_element_de(name, ename, esch), INDENT * 3)
    yield f"            _ => panic!(\"Invalid enum value: {{}}\", t.0),"
    yield "        }"
    yield "    }"
    yield "}"


def render_oneof_element_ser(name, ename, esch):
    struct_name = f'{name}{ename}'
    if not esch.get('properties', ()):
        yield f"odf::{name}::{ename} => (fb::{name}::{struct_name}, empty_table(fb).as_union_value()),"
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


def pre_ser_composite_type(name, sch):
    if sch.get('type') == 'array':
        isch = sch['items']
        yield f"let offsets: Vec<_> = {name}.iter().map(|i| {{"
        if is_enum(isch):
            # TODO: This is a dirty hack
            yield "let (value_type, value_offset) = i.serialize(fb);"
            yield "let mut builder = fb::PrepStepWrapperBuilder::new(fb);"
            yield "builder.add_value_type(value_type);"
            yield "builder.add_value(value_offset);"
            yield "builder.finish()"
        else:
            pre_ser = list(pre_ser_composite_type("i", isch))
            if pre_ser:
                for l in pre_ser:
                    yield l
            else:
                # TODO: Another dirty hack ... flatbuffer serialization is very hard to compose
                for l in ser_composite_type("i", isch):
                    yield l[1:]
        yield "}).collect();"
        yield "fb.create_vector(&offsets)"
    elif is_string_enum(sch):
        pass
    elif is_enum(sch) or is_struct_type(sch):
        yield f'{name}.serialize(fb)'
    else:
        yield from pre_ser_primitive_type(name, sch)


def ser_composite_type(name, sch):
    if sch.get('type') == 'array':
        pass
    elif 'enum' in sch:
        yield f"{name}.into()"
    else:
        yield from ser_primitive_type(name, sch)


def de_composite_type(name, sch, enum_t_accessor):
    if sch.get('type') == 'array':
        isch = sch["items"]
        yield f"{name}.iter().map(|i| "
        if is_enum(isch):
            yield from de_composite_type("i.value().unwrap()", isch, "i.value_type()")
        else:
            yield from de_composite_type("i", isch, None)
        yield  ").collect()"
    elif 'enum' in sch:
        yield f"{name}.into()"
    elif '$ref' in sch:
        t = sch['$ref'].split('/')[-1]
        if is_struct_type(t):
            yield f'odf::{t}::deserialize({name})'
        else:
            yield f'odf::{t}::deserialize({name}, {enum_t_accessor})'
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
        elif fmt == 'multihash':
            assert ptype == 'string'
            yield f'fb.create_vector(&{name}.to_bytes())'
        elif fmt == 'url':
            assert ptype == 'string'
            yield f'fb.create_string(&{name})'
        elif fmt == 'path':
            assert ptype == 'string'
            yield f'fb.create_string({name}.to_str().unwrap())'
        elif fmt == 'regex':
            assert ptype == 'string'
            yield f'fb.create_string(&{name})'
        elif fmt == 'date-time':
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
    else:
        raise Exception(f'Expected primitive type schema: {sch}')


def ser_primitive_type(name, sch):
    ptype = sch.get('type')
    fmt = sch.get('format')
    if fmt is not None:
        if fmt == 'int64':
            assert ptype == 'integer'
            yield name
        elif fmt == 'sha3-256':
            assert ptype == 'string'
        elif fmt == 'multihash':
            assert ptype == 'string'
        elif fmt == 'url':
            assert ptype == 'string'
        elif fmt == 'path':
            assert ptype == 'string'
        elif fmt == 'regex':
            assert ptype == 'string'
        elif fmt == 'date-time':
            yield f'&datetime_to_fb(&{name})'
        elif fmt == 'dataset-id':
            pass
        else:
            raise Exception(f'Unsupported format: {sch}')
    elif ptype == 'boolean':
        yield name
    elif ptype == 'integer':
        yield name
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
            yield f'{name}'
        elif fmt == 'sha3-256':
            assert ptype == 'string'
            yield f'odf::Sha3_256::new({name}.try_into().unwrap())'
        elif fmt == 'multihash':
            assert ptype == 'string'
            yield f'odf::Multihash::from_bytes({name}).unwrap()'
        elif fmt == 'url':
            assert ptype == 'string'
            yield f'{name}.to_owned()'
        elif fmt == 'path':
            assert ptype == 'string'
            yield f'PathBuf::from({name})'
        elif fmt == 'regex':
            assert ptype == 'string'
            yield f'{name}.to_owned()'
        elif fmt == 'date-time':
            yield f'fb_to_datetime({name})'
        elif fmt == 'dataset-id':
            yield f'odf::DatasetIDBuf::try_from({name}).unwrap()'
        else:
            raise Exception(f'Unsupported format: {sch}')
    elif ptype == 'boolean':
        yield f'{name}'
    elif ptype == 'integer':
        yield f'{name}'
    elif ptype == 'string':
        yield f'{name}.to_owned()'
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
