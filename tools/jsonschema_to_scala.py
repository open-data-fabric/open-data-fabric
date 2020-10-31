#!/usr/bin/env python
import os
import json


PREAMBLE = [
    '/*',
    ' * Copyright (c) 2018 kamu.dev',
    ' *',
    ' * This Source Code Form is subject to the terms of the Mozilla Public',
    ' * License, v. 2.0. If a copy of the MPL was not distributed with this',
    ' * file, You can obtain one at http://mozilla.org/MPL/2.0/.',
    ' */',
    '',
    'package dev.kamu.core.manifests',
    '',
    'import java.net.URI',
    'import java.time.Instant',
    '',
    'import com.typesafe.config.ConfigObject',
    'import spire.math.Interval',
    '',
    '/' * 80,
    '// WARNING: This file is auto-generated from Open Data Fabric Schemas',
    '// See: http://opendatafabric.org/',
    '/' * 80,
    '',
    'case class DatasetID(s: String) extends AnyVal {',
    '  override def toString: String = s',
    '}',
    '',
]

DEFAULT_INDENT = 2

DOCS_URL = 'https://github.com/kamu-data/open-data-fabric/blob/master/open-data-fabric.md#{}-schema'


extra_types = []


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
    yield f'case class {name} ('
    for pname, psch in sch.get('properties', {}).items():
        required = pname in sch.get('required', ())
        yield from indent(render_field(pname, psch, required))
    yield ')'


def render_field(pname, psch, required):
    typ = get_composite_type(psch)
    if required:
        yield f'{pname}: {typ},'
    else:
        typ = to_optional_type(psch, typ)
        default = get_default(psch)
        yield f'{pname}: {typ} = {default},'


def render_oneof(name, sch):
    yield f'sealed trait {name}'
    yield ''
    yield f'object {name} {{'
    for i, (ename, esch) in enumerate(sch.get('definitions', {}).items()):
        if i != 0:
            yield ''
        yield from indent(render_oneof_element(ename, esch, name))
    yield '}'


def render_oneof_element(ename, esch, parent):
    yield f'case class {ename} ('
    for pname, psch in esch.get('properties', {}).items():
        required = pname in esch.get('required', ())
        yield from indent(render_field(pname, psch, required))
    yield f') extends {parent}'


def render_string_enum(name, sch):
    yield f'sealed trait {name}'
    yield ''
    yield f'object {name} {{'
    for value in sch['enum']:
        capitalized = value[0].upper() + value[1:]
        yield ' ' * DEFAULT_INDENT + f'case object {capitalized} extends {name}'
    yield '}'


def get_composite_type(sch):
    if sch.get('type') == 'array':
        ptyp = get_primitive_type(sch['items'])
        return f'Vector[{ptyp}]'
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
            return 'Long'
        elif fmt == 'url':
            assert ptype == 'string'
            return 'URI'
        elif fmt == 'regex':
            assert ptype == 'string'
            return 'String'
        elif fmt == 'sha3-256':
            assert ptype == 'string'
            return 'String'
        elif fmt == 'date-time':
            return 'Instant'
        elif fmt == 'date-time-interval':
            return 'Interval[Instant]'
        elif fmt == 'dataset-id':
            return 'DatasetID'
        else:
            raise Exception(f'Unsupported format: {sch}')
    if ptype == 'boolean':
        return 'Boolean'
    elif ptype == 'integer':
        return 'Int'
    elif ptype == 'string':
        return 'String'
    elif '$ref' in sch:
        if sch.get('partial') is True:
            return 'ConfigObject'
        else:
            return sch['$ref'].split('.')[0]
    else:
        raise Exception(f'Expected primitive type schema: {sch}')


def to_optional_type(sch, scala_type):
    return f'Option[{scala_type}]'


def get_default(sch):
    return 'None'


def indent(gen, i=DEFAULT_INDENT):
    for l in gen:
        yield ' ' * i + l


if __name__ == "__main__":
    import sys
    render(sys.argv[1])
