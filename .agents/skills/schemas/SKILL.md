---
name: schemas
description: Workflow and conventions for changing JSON schemas under `schemas/` and keeping generated artifacts, specs, and RFCs in sync. Use when adding, modifying, or removing any schema.
---

# Changing Schemas

Also follow the conventions in [`authoring_schemas.md`](../../../authoring_schemas.md) - it describes the subset of JSON Schema used by ODF, unions, short forms, maps, and input vs. canonical types.


## Workflow
After any change in `schemas/`:
1. Run `make lint` - validates schemas and runs tests against the examples.
2. Run `make codegen` - regenerates `specs/reference.md`, `contracts/` (FlatBuffers schema and ERD source), and `tools/schemas/output/`. Commit the regenerated files together with the schema change.
3. Run `make contracts/erd.svg` to refresh the ERD diagram (requires `mmdc`). If it's not available, mention that the SVG is stale.
4. Update the specs that describe the changed schemas, and the RFCs if the change affects their proposals (see the [specs](../specs/SKILL.md) skill).


## Identity
- Schema `$id` follows `https://opendatafabric.org/schemas/{context}/{version}/{Name}` and never includes the `.json` extension. Files are stored as `schemas/{context}/{version}/{Name}.json`.
- Type URLs used in descriptions and `examples` must be real schema IDs.


## Input and Canonical Types
- When a schema has both input and canonical variants (e.g. `TaskSpec` / `TaskSpecInput`, `FlowTrigger` / `FlowTriggerInput`), change both together. A field that exists only in the input type is silently dropped when the resource is stored.
- Input types use references (`*Ref`), canonical types use resolved handles (`*Handle`).


## Validation
- Every `$ref` under `properties` or `items` must carry `unevaluatedProperties: false` at the call site - including references to enums. The linter enforces this.
- Only top-level resource schemas carry `unevaluatedProperties: false` at their root - never add it to fragments.
- The union `kind` discriminator lives at the union level (`allOf` + `kind` const) - don't add `kind` to variant schemas.
- Properties with a `default` must not be `required`.
- Properties common to all variants of a union (e.g. `enabled` on triggers) are repeated in every variant.


## Editing
- Several schemas mix compact (`{ "properties": { "kind": ... } }`) and expanded JSON formatting. Edit them as text rather than parsing and re-serializing the whole file, to keep diffs minimal.
- Preserve whether the file ends with a newline.
- Prefer making an optional property absent over adding a "none" enum variant that means the same thing.
