# Data Schema <!-- omit in toc -->

**Domain**: Datasets
**Depends on**: -


## Abstract

This spec defines the ODF schema format used to describe the logical structure of [data](../glossary.md#data) independently of how it is physically encoded in files (e.g. Parquet, Avro, CSV) or in memory (e.g. Arrow). It defines the set of supported logical types, the mechanism for attaching custom attributes to schemas and fields, and the rules for converting between ODF and Apache Arrow schemas.


## Table of Contents <!-- omit in toc -->

- [Abstract](#abstract)
- [Logical Types vs. Encoding](#logical-types-vs-encoding)
- [Schema Format](#schema-format)
  - [Nullability](#nullability)
- [Logical Types](#logical-types)
  - [Short Form](#short-form)
- [Extra Attributes](#extra-attributes)
  - [Well-known Attributes](#well-known-attributes)
- [Conversion to and from Arrow](#conversion-to-and-from-arrow)
- [Schema in APIs](#schema-in-apis)


## Logical Types vs. Encoding
ODF schema describes **logical types** of data - the semantics and value boundaries of a type and how it can be operated on. It deliberately does not capture the **encoding** (physical layout) of data such as compression, dictionary encoding, offset widths, or view-based representations.

This separation ensures that:
- Logical types remain stable across different file formats, data chunks, and engines
- Encoding strategies (compression, dictionary usage, encoding hints) can evolve independently, both at the file level and in memory
- Producers and consumers can negotiate encoding at runtime while relying on a consistent logical schema contract

While inspired by [Apache Arrow](https://arrow.apache.org/), ODF schema intentionally differs from the [Arrow flatbuffer schema](https://github.com/apache/arrow/blob/main/format/Schema.fbs), which mixes logical types with in-memory layout details and is not suited to be defined manually by humans.


## Schema Format
A [schema](../glossary.md#schema) (`DataSchema`) consists of:
- `fields` - an ordered list of top-level fields (columns)
- `extra` - optional [extra attributes](#extra-attributes) that apply to the schema as a whole

Each field (`DataField`) consists of:
- `name` - name of the field
- `type` - [logical type](#logical-types) of the field
- `extra` - optional [extra attributes](#extra-attributes) that apply to the field

Example:

```yaml
fields:
  - name: offset
    type:
      kind: UInt64
  - name: op
    type:
      kind: UInt8
  - name: system_time
    type:
      kind: Timestamp
      unit: Millisecond
      timezone: UTC
  - name: event_time
    type:
      kind: Timestamp
      unit: Millisecond
      timezone: UTC
  - name: mri_content_hash
    type:
      kind: String
    extra:
      opendatafabric.org/description: References the MRI scan data linked to the dataset by its hash
  - name: subject
    type:
      kind: Struct
      fields:
        - name: id
          type:
            kind: String
          extra:
            opendatafabric.org/description: Subject's unique identity
        - name: gender
          type:
            kind: Option
            inner:
              kind: String
          extra:
            opendatafabric.org/description: Subject's gender
    extra:
      opendatafabric.org/description: Information about the subject
  - name: area_codes
    type:
      kind: List
      itemType:
        kind: String
    extra:
      opendatafabric.org/description: List of body area codes covered by this MRI scan
extra:
  c.com/z: baz
```

See [`DataSchema`](../../schemas/data/v1alpha1/DataSchema.json) reference for the full definition.


### Nullability
Fields are non-nullable by default. Nullability is expressed explicitly via the special `Option` type that wraps the type of the value (unlike Arrow, where nullability is a property of a field). This applies recursively to nested types, e.g. struct fields and list items.


## Logical Types

| Type | Parameters | Description |
| --- | --- | --- |
| `Bool` | | Boolean value representing true or false |
| `Int8`, `Int16`, `Int32`, `Int64` | | Signed integer |
| `UInt8`, `UInt16`, `UInt32`, `UInt64` | | Unsigned integer |
| `Float16`, `Float32`, `Float64` | | Floating-point number |
| `Decimal` | `precision`, `scale` | Fixed-point decimal number with specified total number of digits and number of digits after the decimal point (scale can be negative) |
| `String` | | Unicode string |
| `Binary` | `fixedLength` (optional) | Sequence of bytes, variable-length unless `fixedLength` is specified |
| `Date` | | Calendar date |
| `Time` | `unit` (default `Millisecond`) | Time of day without a date |
| `Timestamp` | `unit` (default `Millisecond`), `timezone` (default `UTC`) | Point in time represented as an offset from the Unix epoch. Timezone is either a name from the Olson timezone database (e.g. `America/New_York`) or an absolute offset (e.g. `+07:30`) |
| `Duration` | `unit` (default `Millisecond`) | Elapsed time interval |
| `List` | `itemType`, `fixedLength` (optional) | List of values of the same type, variable-length unless `fixedLength` is specified |
| `Map` | `keyType`, `valueType`, `keysSorted` (optional) | Map of key-value pairs, represented as a list of entries |
| `Struct` | `fields` | Collection of named [fields](#schema-format), each with its own type |
| `Option` | `inner` | Optional (nullable) value of the inner type |
| `Null` | | Type representing the absence of a value |

Time units are: `Second`, `Millisecond`, `Microsecond`, `Nanosecond`.

See [`DataType`](../../schemas/data/v1alpha1/DataType.json) reference for the full definition.


### Short Form
Types that don't require any parameters can be specified as a plain string instead of an object with a `kind` field. The following two definitions are equivalent:

```yaml
- name: offset
  type:
    kind: UInt64
```

```yaml
- name: offset
  type: UInt64
```

Short form is an input-only convenience - the expanded form is used when schema is stored in binary formats.


## Extra Attributes
Extra attributes is a key-value map that allows attaching custom information to schemas and fields.

Keys must be in the format `<domain>/<attribute-name>` (e.g. `kamu.dev/archetype`). The domain prefix is used to disambiguate attributes and avoid name collisions between different extensions.

Values can be a string or any JSON value, including nested objects.

Extra attributes are represented natively in YAML and JSON, and as a JSON-encoded string in FlatBuffers.


### Well-known Attributes

| Attribute | Applies to | Description |
| --- | --- | --- |
| `opendatafabric.org/description` | Field | Human-readable description of the field |
| `arrow.apache.org/bufferEncoding` | Field | Arrow buffer encoding hint (see [Conversion to and from Arrow](#conversion-to-and-from-arrow)) |
| `arrow.apache.org/dateEncoding` | Field | Arrow date encoding hint (see [Conversion to and from Arrow](#conversion-to-and-from-arrow)) |
| `arrow.apache.org/decimalEncoding` | Field | Arrow decimal encoding hint (see [Conversion to and from Arrow](#conversion-to-and-from-arrow)) |


## Conversion to and from Arrow
Implementations should be able to convert between ODF and Arrow schemas back and forth without information loss. Since ODF schema captures only logical types, Arrow encoding details are preserved using encoding hints in [extra attributes](#extra-attributes):

**Arrow type:** `LargeUtf8`

```yaml
type:
  kind: String
extra:
  arrow.apache.org/bufferEncoding:
    kind: Contiguous
    offsetBitWidth: 64
```

**Arrow type:** `Utf8View` (analogously for `BinaryView`, `ListView`, `LargeListView`)

```yaml
type:
  kind: String
extra:
  arrow.apache.org/bufferEncoding:
    kind: View
    offsetBitWidth: 32
```

**Arrow type:** `Date64` (millisecond-based representation)

```yaml
type:
  kind: Date
extra:
  arrow.apache.org/dateEncoding:
    unit: Millisecond
```

**Arrow type:** `Decimal128` (analogously for `Decimal256`)

```yaml
type:
  kind: Decimal
extra:
  arrow.apache.org/decimalEncoding:
    bitWidth: 128
```


## Schema in APIs
When a schema is returned by APIs it should carry only the logical type information and not the encoding hints, unless the encoding represents the actual encoding of the data that this API transmits.
