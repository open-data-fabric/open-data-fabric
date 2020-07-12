#!/usr/bin/env python
import os
import json


DEFAULT_INDENT = 2


def render(schemas_dir):
    schemas = read_schemas(schemas_dir)

    for name, sch in schemas.items():
        try:
            if name == 'Manifest':
                continue
            for l in render_schema(name, sch):
                print(l)
            print()
        except Exception as ex:
            raise Exception(
                f'Error while rendering {name} schema:\n{sch}'
            ) from ex


def read_schemas(schemas_dir):
    schemas = {}
    for sch in os.listdir(schemas_dir):
        with open(os.path.join(schemas_dir, sch)) as f:
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
    yield '#[serde(rename_all = "camelCase")]'
    yield '#[serde(deny_unknown_fields)]'
    yield '#[derive(Debug, Serialize, Deserialize)]'
    yield f'struct {name} {{'
    for pname, psch in sch.get('properties', {}).items():
        yield from indent(render_field(pname, psch))
    yield '}'


def render_field(pname, psch):
    typ = get_composite_type(psch)
    yield f'{pname}: {typ},'


def render_oneof(name, sch):
    yield '#[serde(rename_all = "camelCase")]'
    yield '#[serde(deny_unknown_fields)]'
    yield '#[derive(Debug, Serialize, Deserialize)]'
    yield f'enum {name} {{'
    for ename, esch in sch.get('definitions', {}).items():
        yield from indent(render_oneof_element(ename, esch))
    yield '}'


def render_oneof_element(ename, esch):
    yield f'{ename} {{'
    for pname, psch in esch.get('properties', {}).items():
        yield from indent(render_field(pname, psch))
    yield '}'


def get_composite_type(sch):
    if sch.get('type') == 'array':
        ptyp = get_primitive_type(sch['items'])
        return f'Vec<{ptyp}>'
    else:
        return get_primitive_type(sch)


def get_primitive_type(sch):
    ptype = sch.get('type')
    fmt = sch.get('format')
    if fmt is not None:
        if fmt == 'int64':
            assert ptype == 'integer'
            return 'i64'
        elif fmt == 'url':
            assert ptype == 'string'
            return 'String'
        elif fmt == 'regex':
            assert ptype == 'string'
            return 'String'
        elif fmt == 'date-time':
            return 'DateTime<Utc>'
        elif fmt == 'date-time-interval':
            return '???'
        else:
            raise Exception(f'Unsupported format: {sch}')
    if ptype == 'boolean':
        return 'bool'
    elif ptype == 'integer':
        return 'i32'
    elif ptype == 'string':
        return 'String'
    elif '$ref' in sch:
        return sch['$ref'].split('.')[0]
    else:
        raise Exception(f'Expected primitive type schema: {sch}')


def indent(gen, i=DEFAULT_INDENT):
    for l in gen:
        yield ' ' * i + l


if __name__ == "__main__":
    import sys
    render(sys.argv[1])
