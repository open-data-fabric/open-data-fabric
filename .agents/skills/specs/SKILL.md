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

- Domain and spec file names are kebab-cased titles from RFC-021 (e.g. `specs/auth/node-level-authorization.md`).
- All specs are versioned together - individual specs don't carry a version.


## Spec Template
Every spec file starts with:

```markdown
# {Spec Title} <!-- omit in toc -->

**Domain**: {Domain}
**Depends on**: [{Spec}](../{domain}/{spec}.md), ... (or `-` if none)


## Abstract

One paragraph describing what this spec covers.


## Table of Contents <!-- omit in toc -->

- ...


## ...
Spec-specific sections.
```

- **Depends on** lists other specs this spec builds upon. Keep it accurate - it will be used to generate the dependency map.
- Don't add manual change history - history is provided by GitHub.


## Scope
- Keep each spec focused on its own concern. Content that is about another concept belongs to the spec that owns that concept, e.g. the Data Schema spec describes how schemas of arbitrary data files are represented, while schema evolution rules belong to Data Format and the `SetDataSchema` event belongs to Metadata Chain.
- Don't define general-purpose primitives (hashing, identifiers) in standalone specs. Instead explain how established standards (Multiformats, W3C DID, etc.) are applied in the spec that uses them.
- Specs describe the current state. Motivation, rejected alternatives, unresolved questions, and future possibilities stay in RFCs.


## Glossary
- Core concepts are defined in `specs/glossary.md`, with entries sorted alphabetically and the TOC kept in the same order.
- Link glossary terms where they are used in specs (e.g. `[Dataset](../glossary.md#dataset)`), so readers can always reach the definition.
- Don't redefine glossary terms in specs - link to them and add spec-specific details.


## Cross-linking
- **Specs → schemas**: link to JSON schema files (e.g. `../../schemas/data/v1alpha1/DataSchema.json`) instead of the generated metadata reference.
- **RFCs → specs**: every RFC lists the specs it affects in a `**Specs**:` field under its dates. When a spec is created or changed according to an RFC, update that field.

```markdown
**Specs**:
- [Data Schema](../specs/datasets/data-schema.md)
```


## Extracting From the Monolithic Spec
While content is being moved out of `src/open-data-fabric.md`:
- Move one spec at a time so each step can be reviewed separately.
- Remove moved sections from `src/open-data-fabric.md` in the same step.
- Move text verbatim first, rewrite in a separate step - this keeps the diff of the move reviewable.
- Rewrite links in both directions:
  - In `src/open-data-fabric.md` links are relative to the repository root (e.g. `specs/glossary.md#dataset`), because it's rendered into the root `open-data-fabric.md`.
  - In `specs/` links to not-yet-moved sections point to the generated `open-data-fabric.md` at the repository root (e.g. `../../open-data-fabric.md#metadata-chain`).
  - Update links in the glossary and other already-extracted specs that pointed to moved sections.
- When updating a spec based on an RFC, cross-check the RFC against the actual JSON schemas - they may have evolved since the RFC was accepted.


## RFCs
- When a question in an RFC's "Unresolved questions" is resolved, delete the bullet point - don't strike it through or annotate it.
