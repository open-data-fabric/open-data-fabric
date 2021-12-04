# RFC-001: Record Offsets

**Start Date**: 2021-06-01

[![RFC Status](https://img.shields.io/github/issues/detail/state/kamu-data/open-data-fabric/10?label=RFC%20Status)](https://github.com/kamu-data/open-data-fabric/issues/10)

[![Spec PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/13?label=Spec%20PR)](https://github.com/kamu-data/open-data-fabric/pull/13)

## Summary
[summary]: #summary

This RFC introducess new system column `offset` that represents the sequential number of the row from the beginning of the dataset.

## Motivation
[motivation]: #motivation

We need a way to **uniquely identify** a record in a dataset.

This should simplify referring to data slices (we currently use `system_time` intervals in metadata).

And in future should help with features like fine-grain provenance, corrections, and retractions.

## Explanation
[guide-level-explanation]: #guide-level-explanation

Offset is a monotonically increasing sequential numeric identifie that is assigned to every record and represents its position relative to the beginning of the dataset.

Data type: `uint64` (Parquet/Arrow), `UNSIGNED BIGINT` (DDL), non-null.

Offsets are assigned by coordinator at the time of persisting data (after ingestion or transformation).

## Drawbacks
[drawbacks]: #drawbacks

- Increases the number of system columns the users are exposed to
- Storage overhead
- Sequential identifiers are bad for data-parallel systems
  - But so far our streams are not going to be processed fully in parallel

## Rationale and alternatives
[rationale-and-alternatives]: #rationale-and-alternatives

- We could rely on individual data engines to materialize offset collumn on the fly, but this
  - Would need to be handled in every engine, query console etc.
  - May interfere with storage formats if rows are reordered during writing or reading

## Prior art
[prior-art]: #prior-art

- Many SQL databases have `ROWID` pseudo-column and `ROW_NUMBER()` function
- Kafka relies heavily on stream offsets

## Unresolved questions
[unresolved-questions]: #unresolved-questions


## Future possibilities
[future-possibilities]: #future-possibilities

- Offsets should make it easier to implement features like
  - Fine-grain provenance
  - Corrections & retractions
- This feature should coexist fine with data purging and decay (if those will be needed for high volume/rate use cases like IoT)