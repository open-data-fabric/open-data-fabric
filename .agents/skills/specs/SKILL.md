---
name: specs
description: Guidelines for writing and restructuring ODF specification documents under `specs/` and keeping them linked with the glossary, RFCs, and schemas. Use when creating, editing, or extracting a spec, or when an RFC changes a spec.
---

# Writing ODF Specs

Also follow the [markdown](../markdown/SKILL.md) conventions.


## Structure
The spec is a set of interconnected documents grouped by domain, as proposed in [RFC-021](../../../rfcs/021-spec-restructuring.md). The domain tables in that RFC define which specs exist, their scope, and where their content comes from.

```
specs/
├── {domain}/
│   ├── {spec}/          # Images of {spec}.md
│   └── {spec}.md
├── overview.md
├── glossary.md
└── reference.md
```

- Images of a spec live in its `{spec}/` directory next to the spec file. PlantUML diagrams (`.puml`) are built by `make` into an `.svg` with the same name in the same directory - commit both.
- `specs/reference.md` is generated from `schemas/` by `make codegen` - never edit it manually (see the [schemas](../schemas/SKILL.md) skill).
- Domain and spec file names are kebab-cased titles from RFC-021 (e.g. `specs/auth-and-privacy/authorization.md`).
- All specs are versioned together by tagging the repository - spec documents don't carry a version.


## Spec Template
Every spec file starts with:

```markdown
# {Spec Title} <!-- omit in toc -->

**Domain**: {Domain}
**Depends on**: [{Spec}](../{domain}/{spec}.md), ... (or `-` if none)
**RFCs**: [RFC-{NNN}](../../rfcs/{NNN}-{rfc}.md), ... (or `-` if none)


## Abstract

One paragraph describing what this spec covers.


## Table of Contents <!-- omit in toc -->

- ...


## ...
Spec-specific sections.
```

- **Depends on** lists other specs this spec builds upon. Keep it accurate - it will be used to generate the dependency map.
- **RFCs** lists the RFCs that introduced or changed the spec, in ascending order. It must mirror the `**Specs**:` field of those RFCs (see [Cross-linking](#cross-linking)).
- Don't add manual change history - history is provided by GitHub.


## Scope
- Keep each spec focused on its own concern. Content that is about another concept belongs to the spec that owns that concept, e.g. the Data Schema spec describes how schemas of arbitrary data files are represented, while schema evolution rules belong to Data Model and the `SetDataSchema` event belongs to Metadata Chain.
- Don't define general-purpose primitives (hashing, identifiers) in standalone specs. Instead explain how established standards (Multiformats, W3C DID, etc.) are applied in the spec that uses them.
- When moving content between specs, move text verbatim first and rewrite in a separate step - this keeps the diff of the move reviewable. Update all links that pointed to the moved sections.
- Specs describe the current state. Motivation, rejected alternatives, unresolved questions, and future possibilities stay in RFCs.


## Glossary
- Core concepts are defined in `specs/glossary.md`, with entries sorted alphabetically and the TOC kept in the same order.
- Link glossary terms where they are used in specs (e.g. `[Dataset](../glossary.md#dataset)`), so readers can always reach the definition.
- Don't redefine glossary terms in specs - link to them and add spec-specific details.


## Cross-linking
- **Specs → schemas**: link to JSON schema files (e.g. `../../schemas/data/v1alpha1/DataSchema.json`) instead of the generated metadata reference.
- **RFCs ↔ specs**: every RFC lists the specs it affects in a `**Specs**:` field under its dates, and every spec lists the RFCs that affected it in its `**RFCs**:` header field. When a spec is created or changed according to an RFC, update both fields.

```markdown
**Specs**:
- [Data Schema](../specs/datasets/data-schema.md)
```


## Sources of Truth
- When updating a spec based on an RFC, cross-check the RFC against the actual JSON schemas - they may have evolved since the RFC was accepted.
- Don't treat implementations (e.g. `kamu`) as normative - their behavior is likely to change. Use them only to cross-check RFCs, and base spec content on RFCs, schemas, and explicit decisions.


## RFCs
- When a question in an RFC's "Unresolved questions" is resolved, delete the bullet point - don't strike it through or annotate it.
