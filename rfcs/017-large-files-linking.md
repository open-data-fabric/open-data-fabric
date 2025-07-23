# RFC-017: Large Files Linking

[![Issue](https://img.shields.io/github/issues/detail/state/kamu-data/open-data-fabric/108?label=Issue)](https://github.com/kamu-data/open-data-fabric/issues/108)
[![PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/116?label=PR)](https://github.com/kamu-data/open-data-fabric/pull/116)

**Start Date**: 2025-06-14

**Published Date**: 2025-08-25

**Authors**:
- [Sergii Mikhtoniuk](mailto:smikhtoniuk@kamu.dev), [Kamu](kamu.dev)


**Compatibility**:
- [X] Backwards-compatible
- [X] Forwards-compatible

<!--
Backwards-compatible means:
- whether software updated to this RFC will be able to operate on old data
- in case of protocol updates - whether older clients will be able to communicate with newer servers

Forwards-compatible means:
- whether data written by new software still can be used by older software
- if newer clients will be able to communicate with older servers
-->


## Summary
<!--
One paragraph explanation of the feature.
-->
This RFC introduces a new an experimental way to link large binary objects like images, videos, and documents to the dataset.


## Motivation
<!--
Why are we doing this? What use cases does it support? What is the expected outcome?
-->

While Parquet format supports large binary columns, sometimes it's sub-efficient to keep binary data within the structured records of a dataset. Resons may include:
- Downloading the core dataset much faster to query and filter it before deciding which of the large binaries to download
- Storing large binaries in a separate storage from the dataset while maintaining **tamper-proof** properties *(e.g. fast storage for dataset, glacier/tape for infrequently accessed binaries)*
- **Resumable uploads / downloads** of large binaries
- **Streaming** binary data directly from storage without proxying data through a server *(e.g. for image, video, and document previews)*.

> NOTE: For streaming, while it is technically possible to obtain a byte range of a binary value withing a Parquet file and use `Range: bytes=start-end` header to access it - many preview systems take only a URL and don't support streaming within a specified byte range, or the value in Parquet may be in the middle of a compressed chunk requiring special handling.


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

Considering the above, we decided to provide an ability to attach external binaries to ODF datasets.

In order to preserve tamper-proof properties of the ODF datasets we refer to external binaries only by their hashes. This ensures that file cannot be modified without a corresponding change in the dataset and that dataset's `head` block hash acts as a Merkle root of the dataset and linked objects.

The links (hashes) of the external binaries will be specified in data columns (see [rejected alternatives](#rejected-linking-binaries-via-metadata-chain)). Metadata chain will only contain summary information that allows to count and calculate the size of linked objects.

To mark a column that stores links to external objects we will introduce a new `ObjectLink` logical type that initially will be in the `multishash` format. During this early stage we don't have enough confidence in this approach to make `ObjectLink` and `Multihash` part of the core ODF schema, so we will first introduce them via extra attributes mechanism.

Initially we will only support linked objects that are stored alongside (embedded) in a dataset, but may expand it in future towards sharing objects between multiple datasets and use of separate storage.


## Reference-level explanation
<!--
This is the technical portion of the RFC. Explain the design in sufficient detail that:

- Its interaction with other features is clear.
- It is reasonably clear how the feature would be implemented.
- Corner cases are dissected by example.

The section should return to the examples given in the previous section, and explain more fully how the detailed proposal makes those examples work.
-->

Example of defining a column using extended `ObjectLink` type:

```yaml
- name: content_hash
  type:
    kind: String
  extra:
    opendatafabric.org/type:
      kind: ObjectLink
      linkType:
        kind: Multihash
```

Linked object data will be stored in `/data/<multihash>` section of the dataset, together with the data chunks.

Proposed schema for "versioned file" dataset archetype:
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
- name: version
  type:
    kind: UInt64
- name: content_hash
  type:
    kind: String
  extra:
    opendatafabric.org/type:  # [1]
      kind: ObjectLink  # [2]
      linkType:
        kind: Multihash  # [3]
- name: content_length
  type:
    kind: UInt64
- name: content_type
  type:
    kind: Option
    inner:
      kind: String
extra:
    kamu.dev/archetype: VersionedFile  # [4]
```

`[1]` New experimental attribute `opendatafabric.org/type` that defines an extended set of field `Type`s
`[2]` New extended logical type `ObjectLink` signifies that the value references an external object. The mandatory `linkType` property defines the type of the link.
`[3]` New extended logical type `Multihash` signifies a `String` in a self-describing [multihash](https://github.com/multiformats/multihash) format.
`[4]` The `kamu.dev/archetype` is experimental in `kamu.dev` scope and included for example purposes only.


The `AddData` event will be extended with `linkedObjects` summary:
```yaml
event:
  kind: AddData
  prevOffset: 2165
  newData:
    logicalHash: f9680c001200456651f4be2a8c1a2404c89c6356ce875969d69d93333f5ae97155ff996c4a7
    physicalHash: f162014678f832e823c1429eae7d178e0af1957291372b15171414976c04223026581
    offsetInterval:
      start: 2166
      end: 6644
    size: 554005
  newWatermark: 2022-08-12T03:41:37Z
  extra:
    opendatafabric.org/linkedObjects:  # [5]
      numObjects: 1123
      sizeNaive: 100500  # [6]
```

`[5]` New `opendatafabric.org/linkedObjects` custom attribute contains the summary section to understand how many external objects were associated with a certain `AddData` event as well as their total size from metadata only, without querying the individual Parquet data chunks.
`[6]` The total size of all linked objects that doesn't account for possible duplicates (see [notes](#accounting-for-duplicates-in-size-calculation)).

### Impact on existing functionality
To properly support this extension, implementations will need to:
- Allow upload/download of opaque binary data in the `/data` section of the dataset
- Update ingest procedure to ensure referrential integrity - that hashes that appear in such columns have corresponding data objects uploaded
- Update compaction procedures to combine linked object summaries
- Update smart and simple transfer protocols with an additional stage that download/uploads the external data chunks.
- Update dataset data size summaries to separately mention the number of linked objects and their total size.


## Compatibility
<!--
Details on compatibility of these changes.
-->

The change is backwards and forwards compatible in a sense that introduction of the new logical type and extending the `AddData` event will not break old clients - they will be able to work with updated datasets, although they will not transport the external objects along with them.


## Drawbacks
<!--
Why should we *not* do this?
-->


## Rationale and alternatives
<!--
- Why is this design the best in the space of possible designs?
- What other designs have been considered and what is the rationale for not choosing them?
- What is the impact of not doing this?
-->

### Rejected: Linking binaries via metadata chain
Initial idea was to introduce a special event or to extend `AddData` to allow linking the binaries. This would allow to identify which binaries are linked to the dataset using only the metadata chain.

This however introduces many problems:
- Duplication of information between metadata and data chunks
- Having two "sources of truth" that opens up avenue for dangling references
- Issue of compact storage - metadata was not meant to store a lot of information, but this opens up possibility for it to have millions of linked object entries.

We rejected this path in favor of storing links only in data chunks, even though it also has drawbacks:
- We now need to read the data chunks in order to see all linked objects complicates the process of scanning

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


## Unresolved questions
<!--
- What parts of the design do you expect to resolve through the RFC process before this gets merged?
- What parts of the design do you expect to resolve through the implementation of this feature before stabilization?
- What related issues do you consider out of scope for this RFC that could be addressed in the future independently of the solution that comes out of this RFC?
-->

### Linked objects in derivative datasets
It's not clear how linked objects should be treated in derivative datasets. We may want to at least support plain propagation of `ObjectLink` columns. In this case we likely want to reuse the data objects from the input datasets instead of duplicating it to derivative dataset. This entails cross-dataset content links which may complicate size computation and grabage collection.


## Future possibilities
<!--
Think about what the natural extension and evolution of your proposal would be and how it would affect ODF as a whole. Try to use this section as a tool to more fully consider all possible interactions with the ODF in your proposal.

This is also a good place to "dump ideas", if they are out of scope for the RFC you are writing but otherwise related.

If you have tried and cannot think of any future possibilities, you may simply state that you cannot think of anything.

Note that having something written down in the future-possibilities section is not a reason to accept the current or a future RFC; such notes should be in the section on motivation or rationale in this or subsequent RFCs. The section merely provides additional information.
-->

### External storage support
While the initial implementation aims to store large files as part of the dataset itself, we can extend this mechanism in future to support external storages for large objects.

Ideally the choice of storage could vary and be selected on per-node basis, e.g. one node may keep all data together, while another keep objects in a separate cheaper bucket. This means configuration of where liked objects are stored should be separate from `ObjectLink` column definition.

IPFS case is tricky as CID != hash of data - it can vary due to slicing settings. We likely want to store reference that represent content hash independent of the physical file layout of the object, which will require additional layer of mapping between hash references and CIDs.

### Accounting for duplicates in size calculation
We chose to populate metadata with `sizeNaive` as we don't want to require deduplication of references. Deduplication across the whole dataset would involve complex queries that span multiple data chunks which is impractical during ingest stage.

We cannot rely on checking if object store previously contained referenced object during the upload, as this would require complex accumulation of state during the uploads and passing it to ingest. It still would likely to be incorrect in presence of different merge strategies, retractions, compactions, and purging of history.

Even if we performed full deduplication within a dataset during every ingest, depending on the storage used by a node additional deduplication may happen on the storage level (e.g. across multiple datasets), so the size calculation may still over-estimate the footprint.

We therefore decide to keep "naive" size counting as a very rough estimate and leave it up to individual node implementations to perform exact size calculations, e.g. in a heavy background process, if desired.
