# RFC-009: Ingestion Source State

**Start Date**: 2023-04-28

[![RFC](https://img.shields.io/github/issues/detail/state/kamu-data/open-data-fabric/50?label=RFC%20Status)](https://github.com/kamu-data/open-data-fabric/issues/50)

[![Spec PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/51?label=Spec%20PR)](https://github.com/kamu-data/open-data-fabric/pull/51)

**Compatibility**: Backwards-compatible, forwards-incompatible

## Summary
Introduces a `sourceState` field into `AddData` event to store the state of the generic data source from which the data is being added, allowing for incremental ingestion.

## Motivation
ODF implementations often need to store the state of the data source from which data is ingested to avoid expensive recomputations.

For example when using `SetPollingSource` source state can be in the form of:
- ETag of Last-Modified header for HTTP sources
- filename of the last file matched by glob pattern
- or a height of the block of indexed blockchain.

Currently, we don't have anywhere to store such data.

## Guide-level explanation
When ingesting data, an ODF implementation will be able to attach opaque state data to the `AddData` event in order to resume ingestion most efficiently on the next iteration. The source state data will allow differentiating the kind of state that is preserved (similarly to MIME type) and will specify the identity of the source.

## Reference-level explanation
New `SourceState` metadata fragment will be introduced.

The schema of `AddData` metadata event will:
- be extended with an optional `sourceState` field
- have `outputData` field made optional (as it's possible for ingest to produce no new data but update source state, watermark, or checkpoint)

Two predefined source state kinds will be added:
- `odf/etag` - for state identifiers similar to `ETag` HTTP header
- `odf/last-modified` - for RFC3338 timestamps with meaning similar to `Last-Modified` HTTP header

One predefined source ID will be added:
- `odf/polling` - referring to the source specified in the `SetPollingSource` metadata event

Both fields will be plain strings and not enums allowing different implementations to define their own extensions.

It should be **safe to ignore** the source state that an implementation does not understand.

Example `SourceState`:

```yaml
sourceState:
  kind: odf/etag
  source: odf/polling
  value: W/"95c5fde3918cba7c33eaac7ff9d02b22"
```

## Compatibility
This change will be fully backwards-compatible. It will be forwards-incompatible due to making of `AddData::outputData` field optional.

## Drawbacks
- More complexity in metadata

## Alternatives
- Store source state as part of the ingest checkpoint:
  - Would require storing source state alongside engine checkpoints within one file (e.g. by `tar`'ing them together)
  - Would complicate and slows down engine checkpoint extraction
  - Would only work for a single source

## Prior art
N/A

## Unresolved questions
N/A

## Future possibilities
- The source identity (`source` field) should allow us to support multiple concurrent ingestion sources if needed
