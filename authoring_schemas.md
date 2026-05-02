# Authoring Schemas

***This document is WIP***

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

### Short-form Unions
Some unions are encountered very frequently and the internally-tagged for can get too verbose.

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
      {
        "type": "string",
        "enum": ["A", "B"]
      },
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

## Future Ideas
- Separating input types and normalized data forms that are stored in the system
- Using RDF ontology and JSON-LD

TODO: Link tickets?
