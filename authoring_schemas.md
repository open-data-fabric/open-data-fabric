# Authoring Schemas <!-- omit in toc -->

- [Overview](#overview)
- [Core Properties](#core-properties)
  - [Human authoring](#human-authoring)
  - [Roundtrip stability](#roundtrip-stability)
- [Resource Domains](#resource-domains)
- [Schema Patterns \& Extensions](#schema-patterns--extensions)
  - [Extended Formats](#extended-formats)
  - [Default Values](#default-values)
  - [Unions](#unions)
  - [Short-form Structs](#short-form-structs)
  - [Short-form Unions](#short-form-unions)
- [Maps](#maps)
- [Generic Fragments](#generic-fragments)
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


## Resource Domains
`Resource` is a top-level envelope that describes a desired state of a resource instance in ODF.

The `context` and `kind` properties identify the bounded context and type of the resource contained in the enelope:

```yaml
context: config.opendatafabric.org/v1
kind: SecretSet
header: {}
spec: {}
```

Core ODF domains include:

| Context | Description |
| --- | --- |
| `auth.opendatafabric.org/v1alpha` | Accounts, relationships, permissions |
| `config.opendatafabric.org/v1alpha` | Secrets and variables |
| `datasets.opendatafabric.org/v1alpha` | Root and derivative datasets, ingestion, processing, views |
| `flows.opendatafabric.org/v1alpha` | Triggers and complex workflows that spawn tasks |
| `storage.opendatafabric.org/v1alpha` | Storage classes and volumes |
| `tasks.opendatafabric.org/v1alpha` | Executors, tasks, capacity, prioritization |
| `webhooks.opendatafabric.org/v1alpha` | Events and push notiffications |


## Schema Patterns & Extensions

### Extended Formats
The `format` schema proeprty can be used to specify richer type information on top of basic JSON types.

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
    "additionalProperties": false,
    "required": [],
    "properties": {
        "unit": {
            "$ref": "/schemas/TimeUnit",
            "default": "Millisecond",
        },
        "timezone": {
            "type": "string",
            "default": "UTC",
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

When defining a union use the following form:
```json
{
  "oneOf": [
    { "$ref": "#/$defs/A" },
    { "$ref": "#/$defs/B" }
  ],
  "$defs": {
    "A": {
      "type": "object",
      "additionalProperties": false,
      "required": ["kind"],
      "properties": {
        "kind": {
          "type": "string",
          "format": "enum-tag",
          "const": "A"
        }
      }
    },
    "B": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "kind"
      ],
      "properties": {
        "kind": {
          "type": "string",
          "format": "enum-tag",
          "const": "B"
        }
      }
    }
  }
}
```

Note how every definition of the union variant must include `kind` with the special format `enum-tag` and the name of the variant `const` as its first propety.


### Short-form Structs
Some frequently used structs may have a well-defined way of initializing from a string.

For example:
```yaml
secret:
  value: swordfish
  # contentEncoding: null
```

Would be a lot more conviniently written as:
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
  "additionalProperties": false,
  "patternProperties": {
    ".*": { "type": "string" }
  }
}
```

`Map<String, Fragment>`:
```json
{
  "type": "object",
  "additionalProperties": false,
  "patternProperties": {
    ".*": {
      "$ref": "/schemas/Fragment"
    }
  }
}
```

`Map<String, AnyJsonValue>`:
```json
{
  "type": "object",
  "additionalProperties": false,
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


## Generic Fragments
Some schemas like `Resource` may need to embed fragments of any type.

This is represented as:
```json
{
  "$id": "http://open-data-fabric.github.com/schemas/Resource",
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


## Future Ideas
- Separating input types and normalized data forms that are stored in the system
- Using RDF ontology and JSON-LD

TODO: Link tickets?
