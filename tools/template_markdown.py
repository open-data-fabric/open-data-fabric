#!/usr/bin/env python3
import re

INLINE_MARKDOWN = re.compile(r"!\[.*\]\((.+.md)\)")


def template(source, dest):
    with open(source) as f:
        tpl = f.read()

    while True:
        m = INLINE_MARKDOWN.search(tpl)
        if not m:
            break

        fpath = m.group(1)
        with open(fpath) as f:
            ftext = f.read()

        tpl = tpl[:m.start()] + ftext + tpl[m.end():]

    with open(dest, "w") as f:
        f.write(tpl)


if __name__ == "__main__":
    import sys

    template(sys.argv[1], sys.argv[2])
