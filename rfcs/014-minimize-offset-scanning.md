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
Removes `SetWatermark` and modifies `AddData` and `ExecuteQuery` events so that they always carried enough information about the output offset and offsets of the inputs to prepare the next transaction without deep scanning of the metadata.

## Motivation
Currently, `AddData` and `ExecuteQuery` events carry  `[start, end]` intervals for blocks and offsets to describe inputs and outputs of a transaction. To represent empty input/output they are **made optional**.

This is problematic, because to understand what offsets or blocks need to be used for inputs / output of the next transaction it's sometimes required to scan metadata chain until a **non-empty interval** is encountered.

Extending metadata to carry this information regardless of whether any input data was used or any output data was written would allow us to prepare the next transaction using the last `AddData` or `ExecuteQuery` block, avoiding deep scanning of the chain.

## Guide-level explanation
Imagine a derivative dataset that is aggregating data from an IoT source:
- The source writes 1M batches over the year, resulting in ~1M `ExecuteQuery` events
- The dataset is configured to aggregate data per month, thus only ~12 of `ExecuteQuery` events will have data in them
- Because when transaction does not produce data we don't write any offset information, to find out the last offset to use for the next transaction the system needs to routinely **traverse thousands of empty blocks** to find a non-empty one.

Similar problem exists in multi-input derivative datasets where one input updates significantly more frequently than the other, making us scan many `ExecuteQuery` blocks to understand the last input offset interval that was already processed.

The proposed change avoids this by always carrying enough information in the events to understand which offsets and blocks were already processed.

It is also proposed to remove the `SetWatermark` event, treating the advancement of the watermark the same as in the case when `AddData` / `ExecuteQuery` blocks don't contain any state change except for the watermark, reducing the complexity and interplay of events that implementations have to deal with.

## Reference-level explanation
The `AddData` and `ExecuteQuery` events will be extended with the new `prevOffset` property to represent last offset of the previous data slice. It must be equal to the last non-empty `outputData.offsetInterval.end`.

For example, if the first `AddData` event of a Root dataset looks like this (using `null` to represent the missing properties explicitly):

```yaml
prevCheckpoint: null
prevOffset: null
newData:
  logicalHash: <hash>
  physicalHash: <hash>
  offsetInterval:
    start: 0
    end: 9
  size: 100
newCheckpoint: null
newWatermark: "2023-12-31T00:00:00"
newSourceState: null
```

A correctly chained `AddData` event with data will look like so:

```yaml
prevCheckpoint: null
prevOffset: 9
newData:
  logicalHash: <hash>
  physicalHash: <hash>
  offsetInterval:
    start: 10
    end: 19
  size: 100
newCheckpoint: null
newWatermark: "2023-12-31T01:00:00"
newSourceState: null
```

While the next chained `AddData` that only advances the watermark will look like so:

```yaml
prevCheckpoint: null
prevOffset: 19
newData: null
newCheckpoint: null
newWatermark: "2023-12-31T03:00:00"
newSourceState: null
```

The `SetWatermark` event, while advancing the watermark, does not carry over any other things like previous offset, checkpoint, or source state, which unnecessarily complicates the processing. We, therefore, propose removing this event in favor of "no-data" variant of `AddData` and `ExecuteQuery` events, as shown in the example above.

Similarly, the `InputSlice` will no longer use closed `[start, end]` intervals for `blockInterval` and `offsetInterval`. The new schema will use what are essentially half-open intervals where starting point will always be carried across all transaction, even if the interval itself is empty.

Proposed `InputSlice` schema:
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
- Will require stricter validation in implementations to catch propagation errors
- Replacing `SetWatermark` with "no-data" version of `AddData` event makes the event slightly bigger, but with an added bonus of minimizing the chain scanning and simplifying resuming the processing 

## Alternatives
- We could rely on "skip list" data structure to find blocks with non-empty intervals
  - While this may work for output data, it would bring too much complexity in case of input datasets

## Prior art
N/A

## Unresolved questions
N/A

## Future possibilities
We are still considering to implement the "skip list" data structure, but primarily to allow skipping all *"routine"* data processing events like `AddData` and `ExecuteQuery` (high-volume) and let us quickly iterate over the *"out-of-ordinary"* (low-volume) metadata events.
