# RFC-004: Metadata Extensibility

**Start Date**: 2020-12-29

[![RFC Status](https://img.shields.io/github/issues/detail/state/kamu-data/open-data-fabric/8?label=RFC%20Status)](https://github.com/kamu-data/open-data-fabric/issues/8)

[![Spec PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/19?label=Spec%20PR)](https://github.com/kamu-data/open-data-fabric/pull/19)

## Summary

This RFC proposes a new `MetadataBlock` schema that makes metadata less ambiguous and easier to extend.

## Motivation

`MetadataBlock` schema today consists of multiple optional fields that appear in metadata under different conditions. For example, the `outputSlice` field is present when a transformation resulted in some new data added to the datasets, the `outputWatermark` tells us that transformation had bumbed up the watermark of a dataset - it can be present even when no new output data was produced. So when you want to process only metadata blocks that correspond to transformations - you get into a position of wondering what's the "definitive sign" of transformation taking place, is it the presence of `outputWatermark`, `ouputSlice`, or `inputSclices`?

These are the clear signs of "anemic data" - the problem ODF was designed to prevent in data, as compared to approaches like Change Data Capture. As the set of features ODF supports grows, such anemic ever-growing schema will become unsustainable.

This RFC will explore how to transition metadata into a descriptive event-based format that can be easily extended by different parties to implement higher-level features.

## Guide-level explanation

This RFC proposes to replace the anemic fields of the `MetadataBlock` schema with a extensible set of `MetadataEvent`s, with every `MetadataBlock` containing a single `MetadataEvent` of a certain type. `MetadataEvent` will correspond to a certain transaction on the dataset (e.g. executing query, specifying polling source, updating transformation query). Every event will contain only fields that are relevant to the transaction.

A similar transformation will be done to the `DatasetSnapshot` schema. Instead of directly representing the first `MetadataBlock` it will contain an array of `MetadataEvent`s that bring the dataset into a desired state. This means that in most cases a dataset newly-created from a `DatasetSnapshot` will have more than one block.

For example, creating dataset from the following snapshot:

```yaml
kind: DatasetSnapshot
version: 1
content:
  name: ca.bccdc.covid19.case-details
  kind: root
  metadata:
    - kind: setPollingSource
      fetch: ...
      read: ...
      merge: ...
    - kind: setVocab
      eventTimeColumn: reported_date
```

will produce metadata chain with three blocks:

```yaml
- Seed
- SetPollingSource
- SetVocab
```

## Reference-level explanation

Core events:

| Event Type     | Description                                                                                 |
| -------------- | ------------------------------------------------------------------------------------------- |
| `Seed`         | Contains identity information of a dataset and allways appears as the first metadata block. |
| `AddData`      | Signifies that data was added into the `root` dataset.                                      |
| `SetWatermark` | Signigies that watermar of the dataset has been advanced.                                   |
| `SetTransform` | Defines transformation of the `derivative` dataset.                                         |
| `ExecuteQuery` | Signifies data processing step on the `derivative` dataset.                                 |

Extension events:

| Event Type         | Description                                                                                      |
| ------------------ | ------------------------------------------------------------------------------------------------ |
| `SetPollingSource` | (Optional extension) Defines how externally-hosted data can be ingested into the `root` dataset. |

## Drawbacks
[drawbacks]: #drawbacks

- Makes `DatasetSnapshot` schema a little more verbose
- There will be some events that contain a similar set of fields (e.g. `AddData` and `ExecuteQuery` both contain data and watermark info) but will now need to be processed separately. This is a small price to pay for a robust domain model though.

## Rationale and alternatives

- As in case of dataset creation, we can see that it's no longer possible to move dataset into desired state with just one metadata block. Does this hurt consistency?
  - One considered alternative was to allow multiple `MetadataEvent`s per `MetadataBlock` - this would ensure per-block consistency but at the expense of higher complexity
  - It was decided that this complexity is not warranted. Just like with `git`, code is not guaranteed to compile when looking at it on per-commit basis - as long as a group of commits that constitutes a consistent state is pushed atomically consistency overall can be achieved.

## Prior art

## Unresolved questions

- **Forward-compatibility** requires a way to differentiate between events that coordinator has to understand and events that can be safely ignored if coordinator does not support them. 
  - Currently we don't have a good way to achieve this without running into too many issues with `flatbuffers`.
  - We postpone this issue until we get better clarity on the long-term serialization format - we're getting more indications that `flatbuffers` is not the best way forward and may consider alternatives. This also ties in with considering `IPLD`.


## Future possibilities

- This RFC opens up the path for introducing many new `MetadataEvent` types such as for:
  - Storing description / readme
  - Linking to additional content (e.g. notebooks)
  - Embedding a license
