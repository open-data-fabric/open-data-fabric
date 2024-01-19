# RFC-001: Record Offsets

**Start Date**: 2021-06-01

[![RFC Status](https://img.shields.io/github/issues/detail/state/kamu-data/open-data-fabric/10?label=RFC%20Status)](https://github.com/kamu-data/open-data-fabric/issues/10)

[![Spec PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/13?label=Spec%20PR)](https://github.com/kamu-data/open-data-fabric/pull/13)

## Summary

This RFC introduces new system column `offset` that represents the sequential number of the row from the beginning of the dataset.

This is a **backwards incompatible change**.

## Motivation

We need a way to **uniquely identify** a record in a dataset.

This should simplify referring to data slices (we currently use `system_time` intervals in metadata).

And in future should help with features like fine-grain provenance, corrections, and retractions.

## Guide-level explanation

Offset is a monotonically increasing sequential numeric identifier that is assigned to every record and represents its position relative to the beginning of the dataset (first record's offset is `0`).

## Reference-level explanation

Common schema will be extended with `offset` column, with data type: `int64` (Parquet/Arrow), `BIGINT` (DDL), non-null. Offset of the first record is `0`.

`DataSclice` schema will be updated to use offsets instead of system time intervals.

`InputSlice` schema will be introduced that contains data offsets and metadata blocks intervals. Previously we relied on system time interval to define both data and metadata slices, but this proved to be not flexible and complex.

`ExecuteQueryRequest` will be extended with `offset` field telling the engine the starting offset to use for new data.

`DatasetVocab` will be extended with `offsetColumn` field.

All intervals (for offsets and metadata) will be closed/inclusive `[start; end]`. Empty intervals should be expressed by the absence of the interval field.

`date-time-interval` format will be removed.

## Drawbacks

- Backwards incompatible change
- Increases the number of system columns the users are exposed to
- Storage overhead
- Sequential identifiers are bad for data-parallel systems
  - But so far our streams are not going to be processed fully in parallel

## Rationale and alternatives

- We could rely on individual data engines to materialize offset column on the fly, but this
  - Would need to be handled in every engine, query console etc.
  - May interfere with storage formats if rows are reordered during writing or reading

## Prior art

- Many SQL databases have `ROWID` pseudo-column and `ROW_NUMBER()` function
- Kafka relies heavily on stream offsets

## Unresolved questions


## Future possibilities

- Offsets should make it easier to implement features like
  - Fine-grain provenance
  - Corrections & retractions
- This feature should coexist fine with data purging and decay (if those will be needed for high volume/rate use cases like IoT)
