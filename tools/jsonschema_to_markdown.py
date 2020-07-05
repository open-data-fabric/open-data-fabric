#!/usr/bin/env python
import os
import json


NEST_LEVEL = 3


def render_type(sch):
    typ = sch.get("type")
    if typ == "array":
        items = render_type(sch["items"])
        typ = f"array({items})"
    elif typ is None:
        typ = sch["$ref"].split(".")[0]
        typ = f"[{typ}](#{typ.lower()}-schema)"
    else:
        typ = f"`{typ}`"
    return typ


def render_format(sch):
    fmt = sch.get("format")
    if fmt is None:
        return ""
    elif fmt == "date-time":
        return "[date-time](https://json-schema.org/draft/2019-09/json-schema-validation.html#rfc.section.7.3.1)"
    else:
        return f"`{fmt}`"


def render_schema(f, sch):
    name = (os.path.splitext(os.path.split(sch["$id"])[-1])[0],)

    if sch.get("type") == "object":
        render_object(f, sch, name)
    elif "oneOf" in sch:
        render_union(f, sch, name)


def render_union(f, sch, name):
    desc = sch.get("description", "")
    render_preamble(f, name, desc)

    f.write("\n\nUnion type:\n")
    for dname in sch["definitions"]:
        fdname = "::".join(name + (dname,))
        anchor = "".join(name + (dname,)).lower() + "-schema"
        f.write(f"- [{fdname}](#{anchor})\n")

    f.write('\n')

    for dname, dsch in sch["definitions"].items():
        render_object(f, dsch, name + (dname,))
        f.write('\n')


def render_object(f, sch, name):
    desc = sch.get("description", "")
    render_preamble(f, name, desc)
    f.write("\n\n")
    render_table(
        f,
        header=["Property", "Type", "Required", "Format", "Description"],
        header_fmt=[":---:", ":---:", ":---:", ":---:", "---"],
        rows=[[
            f"`{pname}`",
            render_type(psch),
            "V" if pname in sch["required"] else "",
            render_format(psch),
            psch.get("description", "")
        ]
            for pname, psch in sch["properties"].items()
        ])
    f.write("\n")
    render_footer(f, name)


def render_preamble(f, name, desc):
    fname = "::".join(name) + " Schema"
    f.write("#" * (len(name) - 1 + NEST_LEVEL))
    f.write(f" {fname}\n")
    f.write(desc)


def render_footer(f, name):
    f.write(f"[JSON Schema](schemas/{name[0]}.json)\n")


def render_table(f, header, header_fmt, rows):
    f.write("| " + " | ".join(header) + " |\n")
    f.write("| " + " | ".join(header_fmt) + " |\n")

    for values in rows:
        f.write("| " + " | ".join(values) + " |\n")


def render(source, dest):
    with open(source) as f:
        sch = json.load(f)

    try:
        with open(dest, "w") as f:
            render_schema(f, sch)
    except:
        os.remove(dest)
        raise


if __name__ == "__main__":
    import sys
    render(sys.argv[1], sys.argv[2])
