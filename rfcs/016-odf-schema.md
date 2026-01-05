# RFC-016: ODF Schema Format

[![Issue](https://img.shields.io/github/issues/detail/state/kamu-data/open-data-fabric/109?label=Issue)](https://github.com/kamu-data/open-data-fabric/issues/109)
[![PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/116?label=PR)](https://github.com/kamu-data/open-data-fabric/pull/116)

**Start Date**: 2025-06-14

**Published Date**: 2025-08-25

**Authors**:
- [Sergii Mikhtoniuk](mailto:smikhtoniuk@kamu.dev), [Kamu](kamu.dev)


**Compatibility**:
- [X] Backwards-compatible
- [ ] Forwards-compatible


## Summary
This RFC introduces a new human-friendly, extensible schema format for ODF datasets. It replaces the use of Arrow flatbuffer schemas in the `SetDataSchema` event with an explicitly defined logical schema that can include rich metadata, such as column descriptions, logical types, and custom annotations. The new format balances human readability with future extensibility, making it easier to evolve schema definitions and metadata together, and adding support for custom attributes / annotations.


## Motivation
<!--
Why are we doing this? What use cases does it support? What is the expected outcome?
-->

We need an ability to attach extra information to data columns (such as description, ontology, logical types) and dataset as a whole (such as archetypes, visualization option etc.).

We also lack an ability to predefine schema of the dataset in the manifest. We currently only have control over the "read" schema and queries/transformations. The final schema of the dataset always ends up inferred. Ability to define schema is a prerequisite in order to attach any extra information to it.


## Guide-level explanation
<!--
Explain the proposal as if it was already included in the spec, and you were teaching it to another ODF user. That generally means:

- Introducing new named concepts.
- Explaining the feature largely in terms of examples.
- Explaining how one should *think* about the feature, and how it should impact the way they use ODF. It should explain the impact as concretely as possible.
- If applicable, provide sample error messages, deprecation warnings, or migration guidance.
- If applicable, describe the differences between teaching this to existing ODF users and new ODF users.

For implementation-oriented RFCs (e.g. for coordinator internals), this section should focus on how coordinator programmers should think about the change, and give examples of its concrete impact. For policy RFCs, this section should provide an example-driven introduction to the policy, and explain its impact in concrete terms.
-->

Currently in ODF the schema of the actual dataset is never defined in `DatasetSnapshot` manifests - it is **inferred** upon first processing and written in the `SetDataSchema` event using Arrow schema flatbuffer format.

In order for us to enrich the schema with extra information we have two choices:
- Attach extra information to columns via external annotations ([rejected](#rejected-external-annotations))
- Provide ability to explicitly define the schema as part of the manifest and make this definition extensible.

We pick the latter option as it:
- Avoids duplication of column names in several places
- More convenient to see everything related to a column in one place
- Evolution of schema happens along with evolution of annotations - there is no risk of schema and annotations diverging.

To explicitly define the schema we need a format that can be expressed within our manifests.

The first obvious candidate was Apache Arrow which we already use. The only well-defined layout for Arrow schema currently is its [flatbuffers schema](https://github.com/apache/arrow/blob/main/format/Schema.fbs). The [prior art](#prior-art) section links a few tickets that discuss common JSON representation from which we draw inspiration.

We [chose not to use Arrow schema](#rejected-compatibility-with-arrow-flatbuffer-schema) and define our own schema format instead, because:
- The purpose of our schema is different - we want it to describe **logical types** and semantics of column values, rather than **physical layout** of data in files or in memory
- We want schema to be simple, expressive, and easy to define manually in a manifest
- We want to provide extensibility that Arrow schema lacks


## Reference-level explanation

### Flatbuffer schema
The old `SetDataSchema` flatbuffer definition:

```fbs
table SetDataSchema {
  // Apache Arrow schema encoded in its native flatbuffers representation.
  schema: [ubyte];
}
```

Will be modified as follows:

```fbs
table SetDataSchema {
  // DEPRECATED: Apache Arrow schema encoded in its native flatbuffers representation.
  raw_arrow_schema: [ubyte] (id: 0);
  // Defines the logical schema of the data files that follow this event. Will become a required field after migration.
  schema: DataSchema (id: 1);
}
```

This format is compatible with old event format because flatbuffers only care about field IDs and not their names, so we are free to rename the old `schema` field to `raw_arrow_schema` and introduce new `schema` field. Explicit tags are added as a reminder about the ongoing schema migration.


### Manifest Schema Format
```yaml
- kind: SetDataSchema
  schema:  # [1]
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
        extra:  # [2]
          a.com/a: foo
          b.com/x: bar
      - name: mri_content_hash
        type:
          kind: String
        extra:  # [2]
          opendatafabric.org/description: References the MRI scan data linked to the dataset by its hash  # [3]
          opendatafabric.org/type:  # [4]
            kind: Multihash
      - name: subject
        type:
          kind: Struct
          fields:  # [5]
            - name: id
              type:
                kind: String
              extra:
                opendatafabric.org/description: Subject's unique identity
                opendatafabric.org/type:
                  kind: Did
            - name: gender
              type:
                kind: Option  # [6]
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
    extra: # [2]
      c.com/z: baz
```

`[1]` New `schema` field replaces the old `schema` field now containing the ODF schema. Note that schema includes the system columns like `offset`, `op`, and `system_time`.

`[2]` The `extra` field is a container for custom attributes (see [format explanation](#attributes-format) below). Extra attributes will be allowed on the schema level too and work the same as on field (column) level.

`[3]` We introduce `opendatafabric.org/description` attribute to contain field (column) description.

`[4]` The `opendatafabric.org/type` here is used only as an example of possible advanced attribute used to extend logical type beyond core schema - see [RFC-017](./017-large-files-linking.md) for more details.

`[5]` Types recursively work for struct fields.

`[6]` The nullability of fields will be represented by the special `Option` type (unlike on field level in Arrow).


### Extra Attributes Format
Custom attributes is a key-value map.

Keys in the map must be in the format `<domain>/<attribute-name>`. The domain prefix is used to disambiguate and avoid name collisions between different extensions.

Attribute value can be a `string` or any JSON value icluding nested objects.

It will be represented in Flatbuffers as a `string` containing JSON, and natively in YAML.


### Logical Types vs. Encoding
Further we decide that `SetDataSchema` event will be restricted to carry **logical type** information, deprived of encoding details. Currently it is stuck in between - capturing Arrow encoding details, without capturing Parquet encoding. This means we cannot use it as a reliable source of information to enforce similar encoding across all data chunks. At the same time by capturing Arrow encoding details we impose the encoding selection on the read phase.

> Example issue: Old datasets created with earlier versions of Datafusion have `Utf8` and `Binary` types in their schemas. Later versions of Datafusion have switch to use `Utf8View` and `BinaryView` to achieve zero-copy reads. But because when reading the files for querying we use schema from metadata to avoid expensive schema unification step - the encoding details from old schema seep into the query phase and invalidate the view optimization forcing Datafusion to use old types that imply contiguous buffer encoding.

This separation ensures that:
- Logical types remain stable across data chunk formats and engines.
- Encoding strategies (compression, dictionary usage, encoding hints) can evolve independently, both at file level and in-memory level.
- Producers and consumers can negotiate encoding during runtime while relying on consistent logical schema contracts.

### Encoding Hints
The custom attributes mechanism can be used in cases when dataset author wants to provide hints about optimal file and in-memory representation of certain columns. These are left out of scope of this RFC.

### ODF Schema from/to Arrow Conversion
It is desirable for us to be able to convert between ODF and Arrow schema back and forth without information loss. Since ODF focuses on logical types and Arrow on in-memory layout - we will again use custom attriibutes mechanism to preserve encoding information in ODF schema. The following ecoding hints were identified at this initial stage:

**Arrow type:** `LargeUtf8`

**ODF schema:**
```yaml
type:
  kind: String
extra:
  arrow.apache.org/bufferEncoding:
    kind: Contiguous
    offsetBitWidth: 64
```

**Arrow type:** `Utf8View` (analogously for `BinaryView`, `ListView`, `LargeListView`)

**ODF schema:**
```yaml
type:
  kind: String
extra:
  arrow.apache.org/bufferEncoding:
    kind: View
    offsetBitWidth: 32
```

**Arrow type:** `Date64` (millisecond-based representation)

**ODF schema:**
```yaml
type:
  kind: Date
extra:
  arrow.apache.org/dateEncoding:
    unit: Millisecond
```

**Arrow type:** `Decimal128` (analogously for `Decimal256`)

**ODF schema:**
```yaml
type:
  kind: Decimal
extra:
  arrow.apache.org/decimalEncoding:
    bitWidth: 128
```


### Schema in APIs
We decide that when data schema is returned in the APIs it should carry only logical type information and not the encoding, unless the encoding represents the encoding of the data that this API transmits.


## Compatibility
<!--
Details on compatibility of these changes.
-->

Flatbuffer level:
- Flatbuffer schema is made backwards-compatible.
- New datasets will be created **only** with new `schema` field populated, leaving `raw_arrow_schema` field empty - as at the current stage of evolution we can sacrifice forwards-compatibility
- Full removal of `raw_arrow_schema` will be considered in later versions.

Manifest level:
- YAML manifests are backwards-compatible as `SetDataSchema` event never appeared in them before.
- When new `schema` will appear in dataset manifests - only the new tooling will be able to handle it.


### Field optionality during schema upgrades 
Field optionality (nullability) deserves a special note. Arrow currently has very poor control over nullability of fields, and implementations had to ignore nullability when checking for compatibility between schemas. With explicit ODF schemas we would like to pave the path towards enforcing field optionality and making schemas that differ in optionality be considered incompatible. To achieve this we special-case the metadata validation logic: the first time a schema is converted from `raw_arrow_schema` to ODF `schema` the schema equivalence check should ignore differences in field optionality. This will allow dataset authors to define schemas they actually expect, rather than propagate often incorrect nullability inference from Arrow.


## Drawbacks
<!--
Why should we *not* do this?
-->

- It requires changing `SetDataSchema` event to store ODF schemas instead of nested [Arrow Flatbuffers](https://github.com/apache/arrow/blob/main/format/Schema.fbs) (see [RFC-010](./010-data-schema-in-metadata.md))

- At present users would have to define the schema for **system fields** (`offset, op, system_time, event_time`) which is verbose and error-prone. We believe that this extra burden can be resolved by providing better automatic schema inference in the tooling.
  
- This approach is **all-or-nothing**. If user wants to annotate just a few columns - they will need to specify the entire schema. We believe that this extra burden can be resolved by providing better automatic schema inference in the tooling.


## Rationale and alternatives
<!--
- Why is this design the best in the space of possible designs?
- What other designs have been considered and what is the rationale for not choosing them?
- What is the impact of not doing this?
-->

### Rejected: External annotations
**Idea:** We could continue using `SetDataSchema` event to store serialized schema, while using some other event to add annotations to schema and columns.

Example:
```yaml
- kind: SetInfo
  description: Open dataset of MRI scans performed by the X medical lab at Y university
  keywords:
    - MRI
    - Healthcare
  columns:
    - name: event_time
      description: Time when the MRI scan was taken
      a.com/a: foo
    - name: content_hash
      type: Utf8
      description: References the MRI scan data linked to the dataset by its hash
      logicalType:
        kind: ObjectLink
        inner:
          kind: Multihash
    - name: subject
      type: Struct
      fileds:
        - name: id
          logicalType: DID
          description: Subject identifier
        - name: gender
          description: Subject's gender
  a.com/a: foo
  b.com/b:
    $value: baz
    mustUnderstand: true
```

**Reasoning:**
- External annotations allow to only annotate the desired parts of schema without necessary declaring all columns
- This however introduces a chance of them going out of sync and the need for validation
- We already saw the need for the ability to define output schema upfront in datasets and need the human-friendly representation regardless

- Pros:
  - Annotation can be partial
  - Keeps annotation process separate from schema changes
- Cons:
  - Duplication if user has to both define a schema and annotate it
  - Introduces two potentially conflicting sources of truth
    - Will need to validate that annotations refer to columns that exist
    - Need to handle potential desync when schema evolves
  - Does not address inability of user to define schema manually


### Rejected: Compatibility with Arrow Flatbuffer schema
**Idea:**
- Create ODF schemas for Arrow Schema that match the existing flatbuffer-serialized schema in `SetDataSchema` event
- Introduce new extension fields for annotations as part of the Arrow schema object, offsetting their serialization tags to allow for Arrow schema evolution without interfering with our extensions
- New schema will both be compatible with Arrow in flatbuffers and flatbuffer-JSON serialized form

**Reasoning:**
Binary compatibility is difficult to achieve and brittle:
- Due to flatbuffer design `nested: [ubyte]` where data is a separately-serialized `Nested` table is serialized differently from `nested: Nested`, making it not possible to easily match the old binary layout
- Due to Arrow flatbuffers still actively evolving and sometimes breaking compatibility we would have to closely monitoring the upstream changes
- And due to flatbuffers requiring serialization IDs to be sequential, offsetting our extensions would require ugly dummy fields to pad extensions to be sufficiently far enough from base schema

While flatbuffer-JSON is the first candidate for an official "human-friendly" representation, the schema itself makes it frequently overly verbose and cumbersome to use due to multiple design issues. These issues seem severe enough that Rust implementation defined types that significantly differ from the Arrow flatbuffer layout already. 

Few prominent design issues:
- The `children` field appears on the `Field` table, instead of being part of a `Type::Struct` variant
- A single-item `children: [Field]` is used to define the type of elements in a `List` elements, instead of item type being a part of `Type::List` variant
- Because of this multi-purpose use the `name` and `nullable` fields are also made optional
- Integer types in Arrow flatbuffer schema are structured as `Int { bitWidth: 64, is_signed: false }` which is much more verbose than `UInt64`
- There are multiple factors that contribute to combinatorial explosion of types:
  - `Large*` types only differ by using 64-bit offset encoding
  - `View*` types (prefix or ["german-style" strings](https://datafusion.apache.org/blog/2024/09/13/string-view-german-style-strings-part-1/)) are fundamentally an encoding concern, not a logical type
  - `RunEndEncoded` is similarly an encoding concern orthogonal to the type
  - Even the type `Utf8` mixes the fact that it's semantically a "unicode string" and how it is represented

Prior works like Vortex format also [pointed out](https://docs.vortex.dev/concepts/dtypes) the Arrow's lack of distrinction between logical and physical types.

We could introduce some hacks to make schemas easier to define manually, but this would sill mean that the verbose low-level structure is what users would see in our APIs.

We anticipate many more evolutions to the schema:
- Supporting different column-wise compressions schemes
- Supporting file, column, and page-wise encryption

## Prior art
<!--
Discuss prior art, both the good and the bad, in relation to this proposal.
A few examples of what this can include are:

- For core features: Does this feature exist in other technologies and what experience have their community had?
- For community proposals: Is this done by some other community and what were their experiences with it?
- For other teams: What lessons can we learn from what other communities have done here?
- Papers: Are there any published papers or great posts that discuss this? If you have some relevant papers to refer to, this can serve as a more detailed theoretical background.

This section is intended to encourage you as an author to think about the lessons from other projects, provide readers of your RFC with a fuller picture.
If there is no prior art, that is fine - your ideas are interesting to us whether they are brand new or if it is an adaptation from other technologies.
-->

- [Arrow Flatbuffers Schema](https://github.com/apache/arrow/blob/56c0e2f508fdc5137d6734b406634386f9284a52/format/Schema.fbs)
- Arrow issues related to standard JSON representation
  - https://github.com/apache/arrow/issues/13803
  - https://github.com/apache/arrow/issues/25078
  - https://github.com/apache/arrow/issues/34898
- [Logical vs. Physical data types in Vortex](https://docs.vortex.dev/concepts/dtypes)
- [Arrow View types introduction blog post](https://datafusion.apache.org/blog/2024/09/13/string-view-german-style-strings-part-1/)
- [Apache Iceberg Schema](https://iceberg.apache.org/spec/#schemas-and-data-types)
- [JSON-LD](https://www.w3.org/TR/json-ld11/)


## Unresolved questions
<!--
- What parts of the design do you expect to resolve through the RFC process before this gets merged?
- What parts of the design do you expect to resolve through the implementation of this feature before stabilization?
- What related issues do you consider out of scope for this RFC that could be addressed in the future independently of the solution that comes out of this RFC?
-->

### System Columns
The `SetDataSchema` event was meant to represent the full dataset schema, including the system columns.

Requiring system columns when defining schema manually adds a lot of boilerplate to manifest-based schema definitions.

It's unclear yet what is the best way to resolve this issue:
- Should system fields like `offset`, `op`, `system_time` and `event_time` be auto-populated?
- What about `event_time` that often comes from ingested data, can vary in type and renamed?


### Logical Types Evolution
It's not yet fully clear what approach we should take when it comes to expanding the set of logical types:
- Will types like `Multihash` and `Did` ever make it into core schema?
- When should this happen - when every implementation and every engine supports them via custom attributes?
- Can we incorporate new types into the core as long as they provide a fallback? e.g. allowing `Multihash` and `Did` to fall back to `String` type?
- Will semantics of new logical type be governed by ODF schema?


## Future possibilities
<!--
Think about what the natural extension and evolution of your proposal would be and how it would affect ODF as a whole. Try to use this section as a tool to more fully consider all possible interactions with the ODF in your proposal.

This is also a good place to "dump ideas", if they are out of scope for the RFC you are writing but otherwise related.

If you have tried and cannot think of any future possibilities, you may simply state that you cannot think of anything.

Note that having something written down in the future-possibilities section is not a reason to accept the current or a future RFC; such notes should be in the section on motivation or rationale in this or subsequent RFCs. The section merely provides additional information.
-->

### ODF schema in ingest sources
Curretly our push and polling source definitions rely on DDL-style schema. For consistency we should allow defining source schemas in ODF format as well. This is left out of scope of this RFC.


### Compressed enum format
Basic types in new schema will be quite verbose to spell out, e.g.:

```yaml
- name: offset
  type:
    kind: UInt64
```

We could allow enum variants to be shortened to just a string when they don't specify any additional fields, shortening the above to:

```yaml
- name: offset
  type: UInt64
```

This would be similar to `@value` contracted/expanded syntax in JSON-LD.

**NOTE:** Compressed enum format should be **input-only features**, i.e. they would only be applied when reading YAML manifests. The full expanded form would be used in flatbuffers.


### Replacing `SetVocabulary` event
The `SetVocabulary` event can be potentially replaced with annotations on system columns (e.g. `opendatafabric.org/systemColumn: offset`). This could be a way to allow user skip defining system columns in the schema - unless they are specified and tagged explicitly tooling can insert them automatically.


### JSON-LD extensibility
We want the set of schema and column annotations to be fully extensible. Core ODF annotations will be a part of Flatbuffer schema and will be efficient to store, but the reset can be essentially treated as `additionalProperties: true` and store them as arbintrary data. Using JSON-LD's expansion and compation mechanisms we can blur the difference between these for library users.
