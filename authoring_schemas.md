# Authoring Schemas <!-- omit in toc -->

- [Overview](#overview)
- [Core Properties](#core-properties)
  - [Human authoring](#human-authoring)
  - [Roundtrip stability](#roundtrip-stability)
- [Resource Contexts](#resource-contexts)
- [Metaschemas](#metaschemas)
- [Schema Patterns \& Extensions](#schema-patterns--extensions)
  - [Extended Formats](#extended-formats)
  - [Default Values](#default-values)
  - [Unions](#unions)
  - [Short-form Structs](#short-form-structs)
  - [Short-form Unions](#short-form-unions)
- [Maps](#maps)
- [Strict Validation \& Composability](#strict-validation--composability)
- [Generic Fragments](#generic-fragments)
- [Input vs. Canonical Types](#input-vs-canonical-types)
- [Future Ideas](#future-ideas)


## Overview
ODF promotes schema-first design:
- Schemas are defined in a narrow subset of JSON Schema
- Schemas for manifests (top-level objects) may reference schemas of reusable fragments
- A lot of documentation, code for serialization, APIs are generated from schemas


## Core Properties

### Human authoring
Manifests are often written and read by humans and should be optimized for conciseness and clarity.


### Roundtrip stability
Deserializing and serializing a manifest should result in identical data.

Exceptions:
- Short-form unions
- YAML whitespaces and formatting alternatives
- Comments


## Resource Contexts
`Resource` is a top-level envelope that describes a desired state of a resource instance in ODF. See the [Resource Framework](specs/automation/resource-framework.md) spec for the full resource model.

The `$schema` property identifies the type and version of the resource. It doubles as a resolvable URL where the schema file is hosted:

```yaml
$schema: https://opendatafabric.org/schemas/config/v1alpha1/SecretSet
headers: {}
spec: {}
```

Schema URL must follow the pattern `{base-url}/{context}/{version}/{Name}`. Schema IDs (`$id`) never include the `.json` extension, while the schema files are stored in the repository as `schemas/{context}/{version}/{Name}.json`.

In case of the core ODF spec the `base-url` is `https://opendatafabric.org/schemas` and contexts are:

| Context | Description |
| --- | --- |
| `auth` | Accounts, groups, policies, and authorization actions |
| `config` | Secrets and variables |
| `data` | Data types and schema definitions |
| `datasets` | Root and derivative datasets, metadata events, dataset actions |
| `engines` | Engine RPC protocol |
| `events` | Filters for domain events |
| `flows` | Triggers and complex workflows that spawn tasks |
| `legacy` | Deprecated types kept for compatibility, e.g. `DatasetSnapshot` and its `Manifest` wrapper |
| `meta` | Generic metaschemas |
| `resources` | Generic resource envelope, shared headers / status, references, and resource metaschemas |
| `sinks` | Delivery of events to external systems, e.g. webhooks |
| `sources` | Polling and push sources, fetch / read / merge steps |
| `storage` | Persistent volumes and storage credentials |
| `tasks` | Units of work executed by nodes |


## Metaschemas
Some schemas are themselves validated against ODF metaschemas that extend JSON Schema with additional requirements or ODF-specific properties:

| Metaschema | Used by |
| --- | --- |
| [`resources/v1alpha1/meta/Resource`](schemas/resources/v1alpha1/meta/Resource.json) | Canonical resource schemas (require `$schema`, `headers`, `spec`, `status`) |
| [`resources/v1alpha1/meta/ResourceInput`](schemas/resources/v1alpha1/meta/ResourceInput.json) | User-authored resource schemas (require `$schema`, `headers`, `spec`) |
| [`resources/v1alpha1/meta/ResourceRef`](schemas/resources/v1alpha1/meta/ResourceRef.json) | Reference types, e.g. `DatasetRef` |
| [`resources/v1alpha1/meta/ResourceHandle`](schemas/resources/v1alpha1/meta/ResourceHandle.json) | Resolved reference types, e.g. `DatasetHandle` |
| [`resources/v1alpha1/meta/ResourceLabel`](schemas/resources/v1alpha1/meta/ResourceLabel.json) | Label schemas, with `labelProperties` |
| [`resources/v1alpha1/meta/ResourceAnnotation`](schemas/resources/v1alpha1/meta/ResourceAnnotation.json) | Annotation schemas |
| [`resources/v1alpha1/meta/ResourceCondition`](schemas/resources/v1alpha1/meta/ResourceCondition.json) | Status condition schemas, e.g. `ReconciliationError` |
| [`resources/v1alpha1/meta/Action`](schemas/resources/v1alpha1/meta/Action.json) | Authorization action schemas, with `actionProperties` |


## Schema Patterns & Extensions

### Extended Formats
The `format` schema property can be used to specify richer type information on top of basic JSON types.

Example:
```json
{
    "type": "integer",
    "format": "uint64"
}
```

### Default Values
Schema properties can have `default` values.

Example `Timestamp` data type schema with two default properties:
```json
{
    "type": "object",
    "required": [],
    "properties": {
        "unit": {
            "$ref": "https://opendatafabric.org/schemas/data/v1alpha1/TimeUnit",
            "default": "Millisecond"
        },
        "timezone": {
            "type": "string",
            "default": "UTC"
        }
    }
}
```

Default properties must not be `required`.

In code generation:
- Default properties are represented as optional (value is not auto-populated) to preserve **round-trip stability**
- An object that doesn't specify a default value and one that specifies it explicitly should be considered **equal** 


### Unions
We use internally tagged format for representing unions:

```yaml
read:
  kind: Csv
  header: true

read:
  kind: NdJson
  schema: {}
```

The standard union tag name is `kind`.

Each variant is a separate schema file. The union schema references them via `allOf + kind const`:

```json
{
  "oneOf": [
    {
      "allOf": [
        { "properties": { "kind": { "type": "string", "const": "A" } }, "required": ["kind"] },
        { "$ref": "https://opendatafabric.org/schemas/domain/v1alpha1/A" }
      ]
    },
    {
      "allOf": [
        { "properties": { "kind": { "type": "string", "const": "B" } }, "required": ["kind"] },
        { "$ref": "https://opendatafabric.org/schemas/domain/v1alpha1/B" }
      ]
    }
  ]
}
```

The `kind` discriminator lives at the union level — variant schemas (`A`, `B`) do not include `kind`. This keeps variant schemas usable outside of a union context and avoids duplication.


### Short-form Structs
Some frequently used structs may have a well-defined way of initializing from a string.

For example:
```yaml
secret:
  value: swordfish
  # contentEncoding: null
```

Would be a lot more conveniently written as:
```yaml
secret: swordfish
```

To achieve this you can use a special schema pattern:
```json
{
    "format": "struct-or-string",
    "oneOf": [
      { "type": "string" },
      {
        "type": "object",
        "required": [],
        "properties": {}
      }
    ]
}
```

Note: The short form may not be preserved during the serialization round-trip.


### Short-form Unions
Some unions are encountered very frequently and the internally-tagged form can get too verbose.

To improve readability we allow the short-form unions where:

```yaml
field:
  kind: VariantName
```

can be written as:

```yaml
field: VariantName
```

Union data types need to opt-in into the short form using:

```json
{
    "format": "union-or-string",
    "oneOf": [
      { "type": "string" },
      { "$ref": "#/$defs/A" },
      { "$ref": "#/$defs/B" }
    ]
}
```

Full form:
```yaml
schema:
  fields:
  - name: event_time
    type:
      kind: Timestamp
  - name: city
    type:
      kind: String
  - name: population
    type:
      kind: String
```

Short form:
```yaml
schema:
  fields:
  - name: event_time
    type: Timestamp
  - name: city
    type: String
  - name: population
    type: String
```

Note: The short form may not be preserved during the serialization round-trip.


## Maps
To express key-value maps you can use the following schema patterns.

`Map<String, String>`:
```json
{
  "type": "object",
  "patternProperties": {
    ".*": { "type": "string" }
  }
}
```

`Map<String, Fragment>`:
```json
{
  "type": "object",
  "patternProperties": {
    ".*": {
      "$ref": "https://opendatafabric.org/schemas/domain/v1alpha1/Fragment"
    }
  }
}
```

`Map<String, AnyJsonValue>`:
```json
{
  "type": "object",
  "patternProperties": {
    ".*": {}
  }
}
```

Note: maps can only appear as top-level fragment schemas.

Codegen notes:
- Flatbuffers:
  - Flatbuffers does not have native maps, so additional `Entries` table will be created
  - A special codegen hint `"codegen": { "flatbuffers": { "mapFormat": "json-encoded-string" } }` can be used to embed the entire map as one JSON string without generating `Entries` table
- GraphQL also doesn't have a map type, so all maps are returned as JSON scalars


## Strict Validation & Composability

Schemas use `unevaluatedProperties: false` (JSON Schema 2020-12) rather than `additionalProperties: false` to catch unknown fields while remaining composable via `allOf`.

**Rule:** `unevaluatedProperties: false` is placed only on **top-level resource schemas** (e.g. `Dataset`, `VariableSet`). Fragment schemas never carry it at their root — doing so would cause `allOf`-based composition to incorrectly reject `kind` and other sibling properties evaluated by other branches.

To extend strict validation into **nested object properties**, add `unevaluatedProperties: false` at the call site alongside the `$ref`:

```json
{
  "properties": {
    "headers": {
      "$ref": "https://opendatafabric.org/schemas/resources/v1alpha1/ResourceHeaders",
      "unevaluatedProperties": false
    }
  }
}
```

This works because `unevaluatedProperties` governs the object it is declared on. A `$ref` inside `properties` validates a child object in a new evaluation scope, so the parent's `unevaluatedProperties` cannot reach into it — the call-site annotation closes that gap.

The same pattern applies to `items` in arrays:

```json
{
  "items": {
    "$ref": "https://opendatafabric.org/schemas/datasets/v1alpha1/MetadataEvent",
    "unevaluatedProperties": false
  }
}
```


## Generic Fragments
Some schemas like `Resource` may need to embed fragments of any type.

This is represented as:
```json
{
  "$id": "https://opendatafabric.org/schemas/resources/v1alpha1/Resource",
  "properties": {
    "spec": {
      "type": "object",
      "format": "fragment"
    }
  }
}
```

Codegen notes:
- In Rust this will generate a generic `Resource<SpecT>` type
- In Flatbuffers the fragment will be stored in `[ubyte]` as a nested flatbuffer
- In GraphQL the generic field will be returned as JSON scalar


## Input vs. Canonical Types
Often we want to give users more convenience and expressiveness when authoring manifests by creating alternative ways to define something. But at the same time we don't want these alternatives seeping into the core layer where strictness and lack of ambiguity is preferred. A good example is `ResourceRef` type that allows users to reference a resource in variety of ways: by name, ID, DID or all the above - in the core layer we want references to always include stable identifiers.

Similarly, some parts of resources are not meant to be modified by users and presence of those fields in schemas would cause confusion.

For these reasons we allow separating "input" (user-authored) and "canonical" (system) schemas.

A good example are [`ResourceHeaders`](schemas/resources/v1alpha1/ResourceHeaders.json) and [`ResourceHeadersInput`](schemas/resources/v1alpha1/ResourceHeadersInput.json) schemas:

- The input schema contains only fields that user can modify, while system fields like `generation` are only present in the canonical type
- The input schema is using `ResourceOwnerRef` type to allow referencing the owning `account` by name, ID, or DID, while canonical type is using `ResourceOwnerHandle` to represent a fully resolved reference.

Note that input type references its canonical type like so:

```json
{
  "$id": "https://opendatafabric.org/schemas/resources/v1alpha1/ResourceHeadersInput",
  "canonicalType": "https://opendatafabric.org/schemas/resources/v1alpha1/ResourceHeaders",
}
```

Canonical types should be trivially convertible into input types.

Input types may require additional identity and reference resolution information to be canonicalized.


## Future Ideas
- Using RDF ontology and JSON-LD
