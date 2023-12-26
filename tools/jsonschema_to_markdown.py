#!/usr/bin/env python
import os
import sys
sys.path.append(os.path.dirname(__file__))

import json
import utils.schemas


class Ctx:
    def __init__(self, out, schemas, header_level, current_schema):
        self.out = out
        self.schemas = schemas
        self.header_level = header_level
        self.current_schema = current_schema

    def nest(self):
        return Ctx(
            out=self.out, 
            schemas=self.schemas,
            header_level=self.header_level + 1,
            current_schema=self.current_schema,
        )
    
    def with_schema(self, schema):
        return Ctx(
            out=self.out, 
            schemas=self.schemas,
            header_level=self.header_level,
            current_schema=schema,
        )

    def section_id(self, name):
        id = name.lower().replace(" ", "-")
        return f"reference-{id}"
    
    def schema_id(self, name):
        id = name.lower().replace("::", "-")
        return f"{id}-schema"


def render_type(ctx, sch):
    typ = sch.get("type")
    if typ == "array":
        items = render_type(ctx, sch["items"])
        typ = f"array({items})"
    elif typ is None:
        typ = sch["$ref"].split("/")[-1]
        typ = f"[{typ}](#{ctx.schema_id(typ)})"
    else:
        typ = f"`{typ}`"
    return typ


def render_format(sch):
    fmt = sch.get("format")
    if fmt is None:
        return ""
    elif fmt == "date-time":
        return "[date-time](https://json-schema.org/draft/2019-09/json-schema-validation.html#rfc.section.7.3.1)"
    elif fmt == "multihash":
        return "[multihash](https://github.com/multiformats/multihash)"
    elif fmt == "multicodec":
        return "[multicodec](https://github.com/multiformats/multicodec)"
    elif fmt in ("dataset-id", "dataset-name", "dataset-alias", "dataset-ref", "dataset-ref-any"):
        return f"[{fmt}](#dataset-identity)"
    elif fmt in ("path", "int64", "regex", "url", "flatbuffers"):
        return f"`{fmt}`"
    else:
        raise Exception(f"Unknown format: {fmt}")


def render_union(ctx, sch, name):
    render_header(ctx, name, ctx.schema_id(name))
    ctx.out.write(sch.get("description", ""))
    ctx.out.write("\n\n")

    rows = []
    for option in sch["oneOf"]:
        option_id = option["$ref"].split("/")[-1]
        
        if option["$ref"].startswith("#"):
            ename = name + "::" + option_id
        else:
            ename = option_id
        
        link = f"[{ename}](#{ctx.schema_id(ename)})"
        
        description = option.get("description", "")
        if not description:
            if option["$ref"].startswith("#"):
                description = sch["$defs"][option_id].get("description", "")
            else:
                description = ctx.schemas[option_id].schema.get("description", "")
        description = description.split("\n")[0]
        rows.append((link, description))

    render_table(
        ctx,
        header=["Union Type", "Description"],
        header_fmt=[":---:", "---"],
        rows=rows
    )
    ctx.out.write('\n')

    render_schema_links(ctx, name)

    ctx.out.write('\n')

    for dname, dsch in sch.get("$defs", {}).items():
        render_object(ctx, dsch, name + "::" + dname)
        ctx.out.write('\n')


def render_enum(ctx, sch, name):
    render_header(ctx, name, ctx.schema_id(name))
    ctx.out.write(sch.get("description", ""))
    ctx.out.write("\n\n")

    render_table(
        ctx,
        header=["Enum Value"],
        header_fmt=[":---:"],
        rows=[(val,) for val in sch["enum"]]
    )
    ctx.out.write('\n')

    render_schema_links(ctx, name)


def render_table(ctx, header, header_fmt, rows):
    ctx.out.write("| " + " | ".join(header) + " |\n")
    ctx.out.write("| " + " | ".join(header_fmt) + " |\n")

    for values in rows:
        ctx.out.write("| " + " | ".join([
            v.replace("\n", "<br/>")
            for v in values
        ]) + " |\n")


def render_schema_links(ctx, name):
    ctx.out.write(
        f"[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)]({ctx.current_schema.path})\n")
    ctx.out.write(
        "[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](schemas-generated/flatbuffers/opendatafabric.fbs)\n")
    ctx.out.write(
        "[^](#reference-information)\n")


def render_object(ctx, sch, name):
    render_header(ctx, name, ctx.schema_id(name))
    ctx.out.write(sch.get("description", ""))
    ctx.out.write("\n\n")

    render_table(
        ctx,
        header=["Property", "Type", "Required", "Format", "Description"],
        header_fmt=[":---:", ":---:", ":---:", ":---:", "---"],
        rows=[[
                f"`{pname}`",
                render_type(ctx, psch),
                "V" if pname in sch["required"] else "",
                render_format(psch),
                psch.get("description", "")
            ]
            for pname, psch in sch["properties"].items()
        ]
    )
    ctx.out.write("\n")
    render_schema_links(ctx, name)


def render_schema(ctx, sch):
    if sch.schema.get("type") == "object":
        render_object(ctx, sch.schema, sch.name)
    elif "oneOf" in sch.schema:
        render_union(ctx, sch.schema, sch.name)
    elif sch.schema.get("type") == "string" and "enum" in sch.schema:
        render_enum(ctx, sch.schema, sch.name)
    else:
        raise Exception(f"Unsupported type: {sch}")


def render_header(ctx, name, id=None):
    if id is None:
        id = ctx.section_id(name)
    ctx.out.write(f"<a name=\"{id}\"></a>\n")
    ctx.out.write("#" * ctx.header_level)
    ctx.out.write(f" {name}\n")


def render_toc(ctx):
    def render_toc_section(ctx, name, kind, priority=()):
        id = ctx.section_id(name)
        ctx.out.write(f"- [{name}](#{id})\n")
        for sch in schemas_by_kind(ctx.schemas, kind, priority):
            id = ctx.schema_id(sch.name)
            ctx.out.write(f"  - [{sch.name}](#{id})\n")

    render_toc_section(ctx, "Manifests", "root", ["Manifest"])
    render_toc_section(ctx, "Metadata Events", "metadata-event", ["MetadataEvent"])
    render_toc_section(ctx, "Engine Protocol", "engine-op")
    render_toc_section(ctx, "Fragments", "fragment")


def schemas_by_kind(schemas, kind, priority=()):
        filtered = [s for s in schemas.values() if s.kind == kind]
        filtered.sort(key=lambda x: x.name)

        for p in priority:
            for i in range(len(filtered)):
                if filtered[i].name == p:
                    s = filtered.pop(i)
                    filtered.insert(0, s)
                    break

        return filtered


def render_all(ctx, schemas_dir):
    def render_section(ctx, name, kind, priority=()):
        render_header(ctx, name)
        for sch in schemas_by_kind(ctx.schemas, kind, priority):
            render_schema(ctx.nest().with_schema(sch), sch)
            ctx.out.write("\n")

    schemas = utils.schemas.read_schemas(schemas_dir)
    ctx.schemas = schemas

    render_toc(ctx)
    ctx.out.write("\n")

    render_section(ctx, "Manifests", "root", ["Manifest"])
    render_section(ctx, "Metadata Events", "metadata-event", ["MetadataEvent"])
    render_section(ctx, "Engine Protocol", "engine-op")
    render_section(ctx, "Fragments", "fragment")


if __name__ == "__main__":
    source = sys.argv[1]
    ctx = Ctx(
        out=sys.stdout,
        schemas=None,
        header_level=4,
        current_schema=None,
    )

    render_all(ctx, source)
