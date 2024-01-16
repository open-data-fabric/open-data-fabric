# RFC-011: Push Ingest Sources

[![Issue](https://img.shields.io/github/issues/detail/state/kamu-data/open-data-fabric/62?label=Issue)](https://github.com/kamu-data/open-data-fabric/issues/62)
[![Spec PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/63?label=PR)](https://github.com/kamu-data/open-data-fabric/pull/63)

**Start Date**: 2023-10-25

**Authors**:
- [Sergiy Zaychenko](mailto:sergiy.zaychenko@kamu.dev), [Kamu](https://kamu.dev)
- [Sergii Mikhtoniuk](mailto:sergii.mikhtoniuk@kamu.dev), [Kamu](https://kamu.dev)

**Compatibility**:
- [X] Backwards-compatible
- [ ] Forwards-compatible

## Summary
Introduces a set of new metadata events (`AddPushSource`, `DisablePushSource`) for defining "push"-style data ingestion sources for root datasets.

## Motivation
Using `SetPollingSource` event users of ODF can already define "poll"-style data ingestion sources that describe how to read and historize external data to bring it into a root dataset.

We would also like to support "push"-style sources where an external agent periodically adds data into a root dataset. Such sources can range from IoT devices to scripts and all kinds of business process automation.

Similarly to polling sources, push sources also will require defining:
- What format the data is expected to arrive in
- Preprocessing queries (e.g. if the data format of the source like IoT device cannot be changed upstream)
- Merge strategy

## Guide-level explanation
Users can define push sources on root datasets using `AddPushSource`, `DisablePushSource` events.

Multiple push sources can be active simultaneously per one dataset for cases where multiple actors are writing to the same dataset simultaneously in slightly varying formats.

Push and polling sources are mutually exclusive. However, it must be possible to switch dataset from "push" to "pull" ingest and vice versa - thus we also introduce `DisablePollingSource` event that allows to turn off the polling source before switching to push model.

The state of push sources can be stored in existing `sourceState` section of `AddData` event.

The implementations will verify that all push sources result in the same final data schema, as captured in the `SetDataSchema` event.

## Reference-level explanation

`AddPushSource` event schema:

```json
{
  "$id": "http://open-data-fabric.github.com/schemas/AddPushSource",
  "$schema": "http://json-schema.org/draft-07/schema#",
  "description": "Describes how to ingest data into a root dataset from a certain logical source.",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "source",
    "read",
    "merge"
  ],
  "properties": {
    "source": {
      "type": "string",
      "description": "Name that identifies this source within this dataset."
    },
    "read": {
      "$ref": "/schemas/ReadStep",
      "description": "Defines how data is read into structured format."
    },
    "preprocess": {
      "$ref": "/schemas/Transform",
      "description": "Pre-processing query that shapes the data."
    },
    "merge": {
      "$ref": "/schemas/MergeStrategy",
      "description": "Determines how newly-ingested data should be merged with existing history."
    }
  }
}
```

`DisablePushSource` event schema:

```json
{
  "$id": "http://open-data-fabric.github.com/schemas/DisablePushSource",
  "$schema": "http://json-schema.org/draft-07/schema#",
  "description": "Disables the previously defined source.",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "source"
  ],
  "properties": {
    "source": {
      "type": "string",
      "description": "Identifier of the source to be disabled."
    }
  }
}
```

`DisablePollingSource` event schema:

```json
{
  "$id": "http://open-data-fabric.github.com/schemas/DisablePollingSource",
  "$schema": "http://json-schema.org/draft-07/schema#",
  "description": "Disables the previously defined polling source.",
  "type": "object",
  "additionalProperties": false,
  "required": [],
  "properties": {}
}
```

The `source` identifier is the same one as used in `SourceState::source`, meaning that `AddData::sourceState` can be used by the implementations to store the state of the push source for resuming the consumption and implementing "exactly-once" semantics.

## Compatibility
This change is **backwards-compatible**.

This change is **forwards-incompatible** as older implementation will fail to parse the new core events (we still lack the mechanism to mark some events as "OK to disregard").

## Drawbacks
- More complexity in metadata

## Alternatives
N/A

## Prior art
N/A

## Unresolved questions
- Non-trivial ingestion merge strategies (snapshot, ledger) currently require access to past stream data to perform CDC and deduplication. We were considering to avoid the need to read past data eventually by storing all necessary information in the checkpoints. However, allowing for multiple push sources would mean that we need separate checkpoints per source as sources can have different merge strategies. Given that it's still not clear if storing state in checkpoints for things like CDC is even practical for large datasets - we decided to not let this block this RFC, as the need for several push sources per dataset does seem like a practical necessity.

## Future possibilities
N/A
