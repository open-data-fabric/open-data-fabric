import os
import json
from dataclasses import dataclass
from typing import Callable, Literal, Tuple

JsonType = dict[str, "JsonType"]
StrPath = os.PathLike | str
SchemaName = str
SchemaRec = Tuple[SchemaName, JsonType, StrPath]


@dataclass(frozen=True)
class Schema:
    name: str
    schema: JsonType
    path: StrPath
    kind: Literal["root", "metadata-event", "engine-op", "fragment"]


def read_schemas(schemas_dir: StrPath) -> dict[SchemaName, Schema]:
    results = {}

    _read_schemas(
        schemas_dir,
        False,
        lambda name, sch, path: results.__setitem__(name, Schema(name=name, schema=sch, path=path, kind="root"))
    )

    _read_schemas(
        os.path.join(schemas_dir, "metadata-events"),
        True,
        lambda name, sch, path: results.__setitem__(name,
                                                    Schema(name=name, schema=sch, path=path, kind="metadata-event"))
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


def _read_schemas(schemas_dir: StrPath, recursive: bool, on_schema: Callable[[*SchemaRec], None]) -> None:
    for fname in os.listdir(schemas_dir):
        path = os.path.join(schemas_dir, fname)

        if os.path.isdir(path):
            if recursive:
                _read_schemas(path, recursive, on_schema)
            continue

        on_schema(*read_schema(path))


def read_schema(path: StrPath) -> SchemaRec:
    with open(path) as f:
        sch = json.load(f)
        fname = os.path.splitext(os.path.split(path)[-1])[0]
        name = os.path.splitext(sch["$id"].split("/")[-1])[0]
        assert fname == name, f"{fname} != {name}"
        return name, sch, path
