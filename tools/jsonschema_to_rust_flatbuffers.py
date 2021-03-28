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

use super::odf_generated as fb;
mod odf {
    pub use crate::dataset_id::*;
    pub use crate::dtos::*;
    pub use crate::sha::*;
    pub use crate::time_interval::*;
}
use ::flatbuffers::{FlatBufferBuilder, Table, UnionWIPOffset, WIPOffset};
use chrono::prelude::*;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

pub trait FlatbuffersSerializable<'fb> {
    type OffsetT;
    fn serialize(&self, fb: &mut FlatBufferBuilder<'fb>) -> Self::OffsetT;
}

pub trait FlatbuffersDeserializable<T> {
    fn deserialize(fb: T) -> Self;
}

trait FlatbuffersEnumSerializable<'fb, E> {
    fn serialize(&self, fb: &mut FlatBufferBuilder<'fb>) -> (E, WIPOffset<UnionWIPOffset>);
}

trait FlatbuffersEnumDeserializable<'fb, E> {
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

fn interval_to_fb(iv: &odf::TimeInterval) -> fb::TimeInterval {
    use intervals_general::interval::Interval;
    match iv.0 {
        Interval::Closed { bound_pair: p } => fb::TimeInterval::new(
            fb::TimeIntervalType::Closed,
            &datetime_to_fb(p.left()),
            &datetime_to_fb(p.right()),
        ),
        Interval::Open { bound_pair: p } => fb::TimeInterval::new(
            fb::TimeIntervalType::Open,
            &datetime_to_fb(p.left()),
            &datetime_to_fb(p.right()),
        ),
        Interval::LeftHalfOpen { bound_pair: p } => fb::TimeInterval::new(
            fb::TimeIntervalType::LeftHalfOpen,
            &datetime_to_fb(p.left()),
            &datetime_to_fb(p.right()),
        ),
        Interval::RightHalfOpen { bound_pair: p } => fb::TimeInterval::new(
            fb::TimeIntervalType::RightHalfOpen,
            &datetime_to_fb(p.left()),
            &datetime_to_fb(p.right()),
        ),
        Interval::UnboundedClosedRight { right } => fb::TimeInterval::new(
            fb::TimeIntervalType::UnboundedClosedRight,
            &fb::Timestamp::default(),
            &datetime_to_fb(&right),
        ),
        Interval::UnboundedOpenRight { right } => fb::TimeInterval::new(
            fb::TimeIntervalType::UnboundedOpenRight,
            &fb::Timestamp::default(),
            &datetime_to_fb(&right),
        ),
        Interval::UnboundedClosedLeft { left } => fb::TimeInterval::new(
            fb::TimeIntervalType::UnboundedClosedLeft,
            &datetime_to_fb(&left),
            &fb::Timestamp::default(),
        ),
        Interval::UnboundedOpenLeft { left } => fb::TimeInterval::new(
            fb::TimeIntervalType::UnboundedOpenLeft,
            &datetime_to_fb(&left),
            &fb::Timestamp::default(),
        ),
        Interval::Singleton { at } => fb::TimeInterval::new(
            fb::TimeIntervalType::Singleton,
            &datetime_to_fb(&at),
            &fb::Timestamp::default(),
        ),
        Interval::Unbounded => fb::TimeInterval::new(
            fb::TimeIntervalType::Unbounded,
            &fb::Timestamp::default(),
            &fb::Timestamp::default(),
        ),
        Interval::Empty => fb::TimeInterval::new(
            fb::TimeIntervalType::Empty,
            &fb::Timestamp::default(),
            &fb::Timestamp::default(),
        ),
    }
}

fn fb_to_interval(iv: &fb::TimeInterval) -> odf::TimeInterval {
    match iv.type_() {
        fb::TimeIntervalType::Closed => {
            odf::TimeInterval::closed(fb_to_datetime(iv.left()), fb_to_datetime(iv.right()))
                .unwrap()
        }
        fb::TimeIntervalType::Open => {
            odf::TimeInterval::open(fb_to_datetime(iv.left()), fb_to_datetime(iv.right())).unwrap()
        }
        fb::TimeIntervalType::LeftHalfOpen => {
            odf::TimeInterval::left_half_open(fb_to_datetime(iv.left()), fb_to_datetime(iv.right()))
                .unwrap()
        }
        fb::TimeIntervalType::RightHalfOpen => odf::TimeInterval::right_half_open(
            fb_to_datetime(iv.left()),
            fb_to_datetime(iv.right()),
        )
        .unwrap(),
        fb::TimeIntervalType::UnboundedClosedRight => {
            odf::TimeInterval::unbounded_closed_right(fb_to_datetime(iv.right()))
        }
        fb::TimeIntervalType::UnboundedOpenRight => {
            odf::TimeInterval::unbounded_open_right(fb_to_datetime(iv.right()))
        }
        fb::TimeIntervalType::UnboundedClosedLeft => {
            odf::TimeInterval::unbounded_closed_left(fb_to_datetime(iv.left()))
        }
        fb::TimeIntervalType::UnboundedOpenLeft => {
            odf::TimeInterval::unbounded_open_left(fb_to_datetime(iv.left()))
        }
        fb::TimeIntervalType::Singleton => odf::TimeInterval::singleton(fb_to_datetime(iv.left())),
        fb::TimeIntervalType::Unbounded => odf::TimeInterval::unbounded(),
        fb::TimeIntervalType::Empty => odf::TimeInterval::empty(),
    }
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


def is_struct_type(typ):
    return typ in struct_types


def is_enum(typ_or_sch):
    if isinstance(typ_or_sch, dict):
        typ_or_sch = typ_or_sch.get('$ref', '')[:-5]
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
            yield name
        elif fmt == 'sha3-256':
            assert ptype == 'string'
        elif fmt == 'url':
            assert ptype == 'string'
        elif fmt == 'regex':
            assert ptype == 'string'
        elif fmt == 'date-time':
            yield f'&datetime_to_fb(&{name})'
        elif fmt == 'date-time-interval':
            yield f'&interval_to_fb(&{name})'
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
        yield f'{name}'
    elif ptype == 'integer':
        yield f'{name}'
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
