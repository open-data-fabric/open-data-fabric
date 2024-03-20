#!/usr/bin/env python

import os
import re
import sys
import json

from typing import Iterator, cast

DEFAULT_INDENT = 4
DOCS_URL = 'https://github.com/kamu-data/open-data-fabric/blob/master/open-data-fabric.md#{}-schema'
PREAMBLE = """
///////////////////////////////////////////////////////////////////////////////
// WARNING: This file is auto-generated from Open Data Fabric Schemas
// See: http://opendatafabric.org/
///////////////////////////////////////////////////////////////////////////////

use bitflags::bitflags;

use crate::MetadataEvent;

///////////////////////////////////////////////////////////////////////////////
"""

JsonType = dict[str, 'JsonType']
SchemaName = str


def render(schemas_dir: str) -> None:
    print(PREAMBLE)

    name, sch = read_schema(schemas_dir, 'metadata-events/MetadataEvent.json')

    try:
        print('/' * 80)
        print(f'// {name}')
        print('// ' + DOCS_URL.format(name.lower()))
        print('/' * 80)
        print()

        for l in render_schema(name, sch):
            print(l)
        print()


    except Exception as ex:
        raise Exception(
            f'Error while rendering {name} schema:\n{sch}'
        ) from ex


def read_schema(schemas_dir: str, file_path: str) -> (SchemaName, JsonType):
    path = os.path.join(schemas_dir, file_path)

    with open(path) as f:
        schema = json.load(f)
        fname = os.path.splitext(os.path.split(path)[-1])[0]
        name = os.path.splitext(schema['$id'].split('/')[-1])[0]
        assert fname == name, f"{fname} != {name}"
        return name, schema


def render_comment(text: str) -> Iterator[str]:
    if not text:
        return

    for line in text.split('\n'):
        yield f"/// {line.strip()}"


def render_schema(name: str, sch: JsonType) -> Iterator[str]:
    if 'oneOf' in sch:
        yield from render_oneof(name, sch)
    else:
        raise Exception(f'Unsupported schema: {sch}')


def render_oneof(name: str, sch: JsonType) -> Iterator[str]:
    enum_name = f'{name}TypeFlags'

    yield 'bitflags! {'
    yield from indent(['#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]'])
    yield from indent([f'pub struct {enum_name}: u32 {{'])

    enames = []

    for i, isch in enumerate(sch["oneOf"]):
        ref = cast(str, isch["$ref"])
        ename = ref.split('/')[-1]
        yield from indent(indent([f'const {to_screaming_snake_case(ename)} = 1 << {i};']))
        enames.append(ename)

    yield from indent(['}'])
    yield '}'
    yield ''
    yield f'impl From<&{name}> for {enum_name} {{'
    yield from indent([f'fn from(v: &{name}) -> Self {{'])
    yield from indent(indent(['match v {']))

    for ename in enames:
        yield f'{name}::{ename}(_) => Self::{to_screaming_snake_case(ename)},'

    yield from indent(indent(['}']))
    yield from indent(['}'])
    yield '}'


def to_screaming_snake_case(name: str) -> str:
    name = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', name).upper()


def indent(gen: list[str] | Iterator[str], i=DEFAULT_INDENT) -> Iterator[str]:
    for item in gen:
        yield ' ' * i + item


if __name__ == "__main__":
    render(sys.argv[1])
