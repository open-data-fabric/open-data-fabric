#!/usr/bin/env python
import os
import re
import json


PREAMBLE = [
    '/' * 80,
    '// WARNING: This file is auto-generated from Open Data Fabric Schemas',
    '// See: http://opendatafabric.org/',
    '/' * 80,
    '',
    'struct Timestamp {',
    '  year: int32;',
    '  ordinal: uint16;',
    '  seconds_from_midnight: uint32;',
    '  nanoseconds: uint32;',
    '}',
    '',
]

DEFAULT_INDENT = 2

DOCS_URL = 'https://github.com/kamu-data/open-data-fabric/blob/master/open-data-fabric.md#{}-schema'


extra_types = []


def is_string_enum(typ):
    return typ in (
        'CompressionFormat',
        'SourceOrdering',
    )


def render(schemas_dir):
    schemas = read_schemas(schemas_dir)

    for l in PREAMBLE:
        print(l)

    for name, sch in in_dependency_order(schemas):
        try:
            if name == 'Manifest':
                continue
            print('/' * 80)
            print(f'// {name}')
            print('// ' + DOCS_URL.format(name.lower()))
            print('/' * 80)
            print()

            lines = list(render_schema(name, sch))

            # Any extra sibling types that schema needs go first
            for extra_lines in extra_types:
                for l in extra_lines:
                    print(l)
                print()
            extra_types.clear()

            for l in lines:
                print(l)
            print()

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


def in_dependency_order(schemas):
    visited = set()

    for name in sorted(schemas.keys()):
        if name in visited:
            continue

        visited.add(name)
        schema = schemas[name]
        yield from _in_dependency_order_rec(schema, visited, schemas)
        yield (name, schema)


def _in_dependency_order_rec(schema, visited, schemas):
    if '$defs' in schema:
        for dschema in schema['$defs'].values():
            yield from _in_dependency_order_rec(dschema, visited, schemas)

    if schema.get('type') == 'object':
        for pschema in schema.get('properties', {}).values():
            yield from _in_dependency_order_rec(pschema, visited, schemas)

    if schema.get('type') == 'array':
        yield from _in_dependency_order_rec(schema['items'], visited, schemas)

    if 'oneOf' in schema:
        for sch in schema['oneOf']:
            yield from _in_dependency_order_rec(sch, visited, schemas)

    if '$ref' in schema:
        ref = schema['$ref']
        if ref.startswith('/schemas/'):
            name = ref.split('/')[-1]
            if name not in visited:
                visited.add(name)
                yield from _in_dependency_order_rec(schemas[name], visited, schemas)
                yield (name, schemas[name])


def render_schema(name, sch):
    if sch.get('type') == 'object':
        yield from render_struct(name, sch)
    elif 'oneOf' in sch:
        yield from render_oneof(name, sch)
    else:
        raise Exception(f'Unsupported schema: {sch}')


def render_struct(name, sch):
    assert sch.get('additionalProperties', False) is False
    yield f'table {name} {{'
    for pname, psch in sch.get('properties', {}).items():
        required = pname in sch.get('required', ())
        yield from indent(render_field(pname, psch, required))
    yield '}'


def render_field(pname, psch, required, modifier=None):
    typ = get_composite_type(psch, required)
    ret = f'{to_snake_case(pname)}: {typ};'
    if modifier:
        ret = ' '.join((modifier, ret))
    yield ret


def render_oneof(name, sch):
    yield f'union {name} {{'
    for option in sch["oneOf"]:
        ename = option["$ref"].split("/")[-1]

        if option["$ref"].startswith("#"):
            struct_name = f"{name}{ename}"
            extra_types.append(list(render_struct(struct_name, sch["$defs"][ename])))
        else:
            struct_name = ename

        yield " " * DEFAULT_INDENT + f'{struct_name},'
    yield '}'

    if sch.get("root"):
        yield ""
        yield f"table {name}Root {{"
        yield  " " * DEFAULT_INDENT + f"value : {name};"
        yield  "}"


def render_string_enum(name, sch):
    yield f'enum {name}: int32 {{'
    for value in sch['enum']:
        capitalized = value[0].upper() + value[1:]
        yield ' ' * DEFAULT_INDENT + capitalized + ','
    yield '}'


def get_composite_type(sch, required):
    if sch.get('type') == 'array':
        ptyp = get_composite_type(sch['items'], required=True)
        if ptyp == 'PrepStep':
            extra_types.append(['table PrepStepWrapper { value: PrepStep; }'])
            return f'[PrepStepWrapper]'
        else:
            return f'[{ptyp}]'
    elif 'enum' in sch:
        assert sch['type'] == 'string'
        ptyp = sch['enumName']
        extra_types.append(list(render_string_enum(ptyp, sch)))
        if not required:
            ptyp += " = null"
        return ptyp
    elif sch.get('type') == 'object' and 'properties' not in sch:
        return '[ubyte]'
    elif '$ref' in sch:
        return sch['$ref'].split('/')[-1]
    else:
        ptyp = get_primitive_type(sch)
        if not required and ptyp not in ('string', '[ubyte]', 'Timestamp'):
            ptyp += " = null"
        return ptyp


def get_primitive_type(sch):
    ptype = sch.get('type')
    fmt = sch.get('format')
    if fmt is not None:
        if fmt == 'int64':
            assert ptype == 'integer'
            return 'int64'
        elif fmt == 'url':
            assert ptype == 'string'
            return 'string'
        elif fmt == 'path':
            assert ptype == 'string'
            return 'string'
        elif fmt == 'regex':
            assert ptype == 'string'
            return 'string'
        elif fmt == 'sha3-256':
            return '[ubyte]'
        elif fmt == 'date-time':
            return 'Timestamp'
        elif fmt == 'dataset-id':
            return 'string'
        else:
            raise Exception(f'Unsupported format: {sch}')
    if ptype == 'boolean':
        return 'bool'
    elif ptype == 'integer':
        return 'int32'
    elif ptype == 'string':
        return 'string'
    else:
        raise Exception(f'Expected primitive type schema: {sch}')


def to_snake_case(name):
    name = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', name).lower()


def indent(gen, i=DEFAULT_INDENT):
    for l in gen:
        yield ' ' * i + l


if __name__ == "__main__":
    import sys
    render(sys.argv[1])
