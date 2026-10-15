---
name: markdown
description: Formatting conventions for all markdown documents in this repository (specs, RFCs, guides). Use when writing or editing any `.md` file.
---

# Markdown Conventions


## Headings
- Leave **two blank lines** before a heading that follows content.
- Leave one blank line between a heading and a sub-heading that immediately follows it.
- Don't apply these rules inside code blocks.

```markdown
## Section
Some content.


## Next Section

### Sub-section
More content.
```


## Table of Contents
- Document title and the `Table of Contents` heading are marked with `<!-- omit in toc -->`.
- TOC lists `##` and `###` headings as nested `-` lists with GitHub-style anchors.
- Keep the TOC in sync whenever headings are added, removed, renamed, or reordered.


## Tables
- Don't pad cells with whitespace to align columns - padding creates noisy diffs when rows are added or edited.
- Use `---` in the separator row.

```markdown
| Column | Description |
| --- | --- |
| `offset` | Sequential identifier of a row |
```


## Links
- Use relative paths for links between files in the repository (e.g. `../glossary.md#dataset`, `../../rfcs/016-odf-schema.md`).
- Link to source files (e.g. JSON schemas in `schemas/{context}/{version}/{Name}.json`) rather than to generated artifacts.
- Run `make lint-markdown` after editing markdown files - it checks that all relative links, anchors, and images resolve (requires `lychee`).
- Prefer reference-style links (`[RFC-016]` with definitions at the bottom of the file) when the same target is linked many times.


## Text
- Use `-` for bullet lists.
- Consecutive single-line header fields (e.g. `**Start Date**:` and `**Published Date**:` in RFCs, or the spec header fields) end with `\` - a hard line break - so they render on separate lines without extra spacing.
- Use ` - ` (spaced hyphen) rather than em dashes.
- Specify a language on fenced code blocks (`yaml`, `json`, `sql`, `fbs`) when there is one.
- End files with a single newline.
