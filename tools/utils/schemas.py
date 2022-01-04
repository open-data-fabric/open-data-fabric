import os
import json
from dataclasses import dataclass

@dataclass(frozen=True)
class Schema:
    name: str
    schema: dict
    path: str
    kind: str # root | metadata-event | engine-op | fragment


def read_schemas(schemas_dir):
    results = {}

    _read_schemas(
        schemas_dir, 
        False, 
        lambda name, sch, path: results.__setitem__(name, Schema(name=name, schema=sch, path=path, kind="root"))
    )

    _read_schemas(
        os.path.join(schemas_dir, "metadata-events"),
        True, 
        lambda name, sch, path: results.__setitem__(name, Schema(name=name, schema=sch, path=path, kind="metadata-event"))
    )

    _read_schemas(
        os.path.join(schemas_dir, "engine-ops"),
        True, 
        lambda name, sch, path: results.__setitem__(name, Schema(name=name, schema=sch, path=path, kind="engine-op"))
    )

    _read_schemas(
        os.path.join(schemas_dir, "fragments"),
        True, 
        lambda name, sch, path: results.__setitem__(name, Schema(name=name, schema=sch, path=path, kind="fragment"))
    )

    return results


def _read_schemas(schemas_dir, recursive, on_schema):
    for fname in os.listdir(schemas_dir):
        path = os.path.join(schemas_dir, fname)
        
        if os.path.isdir(path):
            if recursive:
                _read_schemas_rec(path, recursive, on_schema)
            continue

        on_schema(*_read_schema(path))


def _read_schema(path):
    with open(path) as f:
        sch = json.load(f)
        fname = os.path.splitext(os.path.split(path)[-1])[0]
        name = os.path.splitext(sch['$id'].split('/')[-1])[0]
        assert fname == name, f"{fname} != {name}"
        return name, sch, path