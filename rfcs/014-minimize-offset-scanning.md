# RFC-014: Minimizing scanning for last offset and block 

[![Issue](https://img.shields.io/github/issues/detail/state/kamu-data/open-data-fabric/62?label=Issue)](https://github.com/kamu-data/open-data-fabric/issues/69)
[![Spec PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/63?label=PR)](https://github.com/kamu-data/open-data-fabric/pull/63)

**Start Date**: 2023-12-17

**Authors**:
- [Sergii Mikhtoniuk](mailto:sergii.mikhtoniuk@kamu.dev), [Kamu](https://kamu.dev)

**Compatibility**:
- [ ] Backwards-compatible
- [ ] Forwards-compatible

## Summary
Modifies `AddData`, `ExecuteQuery` to make sure that last even carries enough information about the output offset and offsets of all inputs to prepare next transaction without excessive scanning of the metadata.

## Motivation
We currently the `AddData` and `ExecuteQuery` events carry  `[start, end]` intervals for blocks and offsets to describe inputs and outputs of a transaction. To represent empty input/output they are made optional.

This is problematic, because to understand what offsets or blocks need to be used used for inputs / output of the next transaction it's sometimes required to scan metadata chain deeply until a non-empty interval is encountered.

Extending metadata to carry this information regardless of whether any input data was used or any output data was written would allow us to always be able to prepare next transaction based on last `AddData` or `ExecuteQuery` block, avoiding the deep traversing of the chain.

## Guide-level explanation
Imagine a derivative dataset that is aggregating data from an IoT source:
- The source writes 1M batches over the year, resulting in ~1M `ExecuteQuery` events
- The dataset is configured to aggregate data per month, thus only ~12 of `ExecuteQuery` events will have data in them
- Because when transaction does not produce data we don't write any offset information, to find out the last offset to use for the next transaction the system needs to routinely **traverse thousands of empty blocks** to find a non-empty one.

Similar problem exists in multi-input derivative datasets where one input updates significantly more frequently than the other, making us scan many `ExecuteQuery` blocks to understand the last input offset interval that was already processed.

The proposed change avoids this by always carrying enough information in the events to understand which offsets and blocks were already processed.


## Reference-level explanation
The `AddData` and `ExecuteQuery` events will add a new `prevOffset` property to represent last offset of the previous data slice. It Must be equal to the last non-empty `outputData.offsetInterval.end`.

The `InputSlice` will no longer use closed `[start, end]` intervals for `blockInterval` and `offsetInterval`. The new schema will use what are essentially half-open intervals where starting point will always be carried across all transaction, even if the interval itself is empy.

Proposed schema:
```json
{
  "description": "Describes a slice of the input dataset used during a transformation",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "datasetId"
  ],
  "properties": {
    "datasetId": {
      "type": "string",
      "format": "dataset-id",
      "description": "Input dataset identifier."
    },
    "prevBlockHash": {
      "type": "string",
      "format": "multihash",
      "description": "Last block of the input dataset that was previously incorporated into the derivative transformation, if any. Must be equal to the last non-empty `newBlockHash`. Together with `newBlockHash` defines a half-open `(prevBlockHash, newBlockHash]` interval of blocks that will be considered in this transaction."
    },
    "newBlockHash": {
      "type": "string",
      "format": "multihash",
      "description": "Hash of the last block that will be incorporated into the derivative transformation. When present, defines a half-open `(prevBlockHash, newBlockHash]` interval of blocks that will be considered in this transaction."
    },
    "prevOffset": {
      "type": "integer",
      "format": "uint64",
      "description": "Last data record offset in the input dataset that was previously incorporated into the derivative transformation, if any. Must be equal to the last non-empty `newOffset`. Together with `newOffset` defines a half-open `(prevOffset, newOffset]` interval of data records that will be considered in this transaction."
    },
    "newOffset": {
      "type": "integer",
      "format": "uint64",
      "description": "Offset of the last data record that will be incorporated into the derivative transformation, if any. When present, defines a half-open `(prevOffset, newOffset]` interval of data records that will be considered in this transaction."
    }
  }
}
```

## Compatibility
This change will be executed as part of the backwards compatibility breaking changes.

## Drawbacks
- Half-open intervals are slightly harder to understand
- Will require more strict validation in implementations

## Alternatives
- We could rely on skip list data structure to find blocks with non-empty intervals
  - While this may work for output data, it would bring too much complexity in case of input datasets

## Prior art
N/A

## Unresolved questions
N/A

## Future possibilities
N/A
