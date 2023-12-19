# RFC-015: Unified changelog stream schema

[![Issue](https://img.shields.io/github/issues/detail/state/kamu-data/open-data-fabric/47?label=Issue)](https://github.com/kamu-data/open-data-fabric/issues/47)
[![Spec PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/72?label=PR)](https://github.com/kamu-data/open-data-fabric/pull/72)

**Start Date**: 2023-12-17

**Authors**:
- [Sergii Mikhtoniuk](mailto:sergii.mikhtoniuk@kamu.dev), [Kamu](https://kamu.dev)

**Compatibility**:
- [ ] Backwards-compatible
- [ ] Forwards-compatible

## Summary
Introduces new `op` schema column that will be used across all datasets to differentiate regular appends, corrections, and retractions. To represent corrections a two-event "changelog stream" data model similar to Apache Flink's will be used.

## Motivation
Errors in source data are inevitable and require a mechanism for correcting them post factum. Unlike databases, where one could issue `DELETE` or `UPDATE` queries, our core data model is an immutable append-only stream, and thus requires a different mechanism to issue **retractions and corrections** for past events.

Additionally, in cases when a stream processing operation encounters late data (beyond the current watermark), or late upstream retractions and corrections, it may also need to issue corrections or retractions for previously produced result records that were influenced by the late events.

We consider the correction/retraction model fundamental and essential to making data processing **maximally autonomous**. In turn, only by making data processing autonomous can collaborative data pipelines exist at the **global scale**. This RFC proposes necessary schema changes to make this model uniform across all ODF datasets.

## Guide-level explanation

### Scenario
Consider a video game that writes a `match_scores` dataset as players complete the matches:

| match_time | match_id | player_name | score |
| ---------: | -------: | ----------: | ----: |
|         t1 |        1 |       Alice |   100 |
|         t1 |        1 |         Bob |    80 |
|         t2 |        2 |       Alice |    70 |
|         t2 |        2 |     Charlie |    90 |
|         t3 |        3 |         Bob |    60 |
|         t3 |        3 |     Charlie |   110 |

Let's say we want to create a dataset that continuously tracks the **top two** (for simplicity) highest-scoring players - a leader board.

This is the case of Top-N query, which can be written in Apache Flink SQL as:

```sql
select
  *
from (
  select
    ROW_NUMBER() OVER (ORDER BY score desc) AS place,
    match_time,
    player_name,
    score,
  from match_scores
) where place <= 2
```

### Representing changes in streams
There are several ways to represent how this query behaves when applied to the above input stream.

#### Retract Stream
The most generic way is a "retract stream" that only uses append `+A` and retract `-R` operations (note the new `op` column):

|   op | place | match_time | player_name | score |
| ---: | ----: | ---------: | ----------: | ----: |
|   +A |     1 |         t1 |       Alice |   100 |
|   +A |     2 |         t1 |         Bob |    80 |
|   -R |     2 |         t1 |         Bob |    80 |
|   +A |     2 |         t2 |     Charlie |    90 |
|   -R |     1 |         t1 |       Alice |   100 |
|   -R |     2 |         t2 |     Charlie |    90 |
|   +A |     1 |         t3 |     Charlie |   110 |
|   +A |     2 |         t3 |       Alice |   100 |

In this model the updated state of the leader board is compared with the previous state after each new event, and necessary records first get retracted before being replaced with appends.

#### Upsert Stream
Using the knowledge that the `place` column plays the role of a unique key of the resulting state we could also represent the above as an "upsert stream" using only upsert `+A` and retract `-R` operations:

|   op | place | match_time | player_name | score |
| ---: | ----: | ---------: | ----------: | ----: |
|   +A |     1 |         t1 |       Alice |   100 |
|   +A |     2 |         t1 |         Bob |    80 |
|   +A |     2 |         t2 |     Charlie |    90 |
|   +A |     1 |         t3 |     Charlie |   110 |
|   +A |     2 |         t3 |       Alice |   100 |

This additional knowledge allows us to significantly compact the stream. 

Although the retract operation does not appear in our example, it is needed for completeness, e.g. imagine if the source stream retracted the result of the first match right after `t1` - this would require us to empty the leader board with retractions too.

#### Changelog Stream (single event)
Some systems produce "changelog streams" containing append `+A`, retract `-R`, and update `+U` operations with update carrying both the new values and the old values of the record being changed:

|   op | place | match_time | player_name | score | match_time_old | old_player_name | old_score |
| ---: | ----: | ---------: | ----------: | ----: | -------------: | --------------: | --------: |
|   +A |     1 |         t1 |       Alice |   100 |                |                 |           |
|   +A |     2 |         t1 |         Bob |    80 |                |                 |           |
|   +U |     2 |         t2 |     Charlie |    90 |             t1 |             Bob |        80 |
|   +U |     1 |         t3 |     Charlie |   110 |             t1 |           Alice |       100 |
|   +U |     2 |         t1 |       Alice |   100 |             t2 |         Charlie |        90 |

This format is also used by CDC systems like [Debezium](https://debezium.io/) and as an internal data representation in [Arroyo](https://github.com/ArroyoSystems/arroyo).

This format is the most "informative" one, as it differentiates retractions from corrections, and provides access to both the new and the previous state within one event. The drawback is that it significantly impacts the schema to essentially allow carrying two events (old and new) in one record.

#### Changelog Stream (two events)
The Apache Flink's "changelog streams" variant is using append `+A`, retract `-R`, update-before `-U`, and update-after `+U` operations. Here, the "update-before" events carry the previous values of the record about to be updated and "update-after" events carry the new values, with the restriction that these events must always appear side by side and in order.

|   op | place | match_time | player_name | score |
| ---: | ----: | ---------: | ----------: | ----: |
|   +A |     1 |         t1 |       Alice |   100 |
|   +A |     2 |         t1 |         Bob |    80 |
|   -U |     2 |         t1 |         Bob |    80 |
|   +U |     2 |         t2 |     Charlie |    90 |
|   -U |     1 |         t1 |       Alice |   100 |
|   +U |     1 |         t3 |     Charlie |   110 |
|   -U |     2 |         t2 |     Charlie |    90 |
|   +U |     2 |         t1 |       Alice |   100 |

By splitting the update operation in two events this format does not require extending the schema with multiple columns.

Due to the restriction that update before/after events appear side by side, this format can be easily converted into single-event changelog stream form upon reading.

### Conclusion
Considering all the above we decide to:
- Use **Two-event Changelog Stream format** as the most complete and the least intrusive format for representing corrections and retractions
  - Retract and Upsert streams can still be supported as subsets of the Changelog Stream model
- Extend the set of standard columns with `op` column to define the operation
- Avoid the use of the word `update` and favor the word `correction` to further distantiate ourselves from the CDC and CRUD mentality
- Generalize `MergeSchema::Snapshot` to use this column.

## Reference-level explanation
The set of system columns will be extended with `op` column:
- Arrow type: `uint8`
- Parquet type: `INT32`
- Recommended Parquet encoding: `RLE_DICTIONARY`

| value |   operation    | code  |
| :---: | :------------: | :---: |
|   0   |    `append`    | `+A`  |
|   1   |   `retract`    | `-R`  |
|   2   | `correct-from` | `-C`  |
|   3   |  `correct-to`  | `+C`  |

`DatasetVocabulary` schema will be updated to include `operationTypeColumn`.

`MergeStrategy::Snapshot` behavior will be modified as follows:
- The `obsv` column will be removed in favor of `op` column
- Users will no longer be able to specify custom vocabulary for observations
- Strategy will emit changelog-compliant stream, i.e. will emit two correction events (`-C/+C`) instead of one (`U`) as previously

## Compatibility
This change will be executed as part of the backwards compatibility breaking changes.

## Drawbacks

### More columns
Additional system column may be hard on user eyes. In datasets where retractions/corrections don't appear or rare UI layer may optimize the presentation to exclude them, or find different approaches to visualize them.

### More complex CDC
Previously `MergeStrategy::Snapshot` produced an [upsert stream](#upsert-stream). This resulted in a short diff, but to reconstruct state from an upsert stream one needs to know the primary key of the events, which is currently known only to the merge strategy.

Migration to [changelog stream](#changelog-stream-two-events) format will produce more records and slow down the CDC operation, but make stream more versatile, as it will be possible to feed it directly into engines without propagating the knowledge of primary keys. We therefore decide to accept the associated costs.

### Affect on current batch engines
Some ODF engine implementations, like Kamu Spark and DataFusion engines, are operating in batch mode. This is a transitional measure and they explicitly warn users that they are not complete ODF engine implementations and should be used only for `map` / `filter` style queries. Those engines often reorder rows due to their parallel nature and resort to re-sorting records by `event_time` after processing. While this was mostly fine before, with these changes the event order becomes very important, and in the presence of corrections and retractions order cannot be easily restored by re-sorting, as that would require some kind of primary key which not every dataset has.

We accept this added complexity and recommend that transitional batch-mode engines:
- preserve the order of records during processing (e.g. by coalescing inputs into one partition)
- propagate operation type column as-is, allowing `map` / `filter` operations to act identically on both append and retract/correct records, resulting in a valid changelog stream.

## Alternatives
- Treat retractions and corrections as completely separate data blocks.
  - Given how frequent they can occur in some situations - it's best to keep them as part of the core data model.

## Prior art
- **Apache Flink**:
  - [RowKind type](https://github.com/apache/flink/blob/f7ababa8f0bab87267b101efe2ecc77b700c2222/flink-core/src/main/java/org/apache/flink/types/RowKind.java) that is used to represent a changelog stream uses:
    - `+I INSERT` - for inserts
    - `-U UPDATE_BEFORE` - for updates to carry the previous value that needs to be retracted first (must occur together with `UPDATE_AFTER`)
    - `+U UPDATE_AFTER` - for updates to carry the new value
    - `-D DELETE` - for retractions
  - [Versioned Tables](https://nightlies.apache.org/flink/flink-docs-release-1.18/docs/dev/table/concepts/versioned_tables/)
  - [Top-N](https://nightlies.apache.org/flink/flink-docs-release-1.18/docs/dev/table/sql/queries/topn/) and [Window Top-N](https://nightlies.apache.org/flink/flink-docs-release-1.18/docs/dev/table/sql/queries/window-topn/) examples in Flink's documentation
  - [Table to Stream Conversion](https://nightlies.apache.org/flink/flink-docs-release-1.18/docs/dev/table/concepts/dynamic_tables/#table-to-stream-conversion) docs explain three types of streams :
    - append-only
    - retract
    - upsert (requires unique key)
  - See also [ChangelogMode type in table connector](https://github.com/apache/flink/blob/1c67cccd2fdd6c674a38e0c26fe990e1dd7b62ae/flink-table/flink-table-common/src/main/java/org/apache/flink/table/connector/ChangelogMode.java#L36)
- **Debezium**:
  - [Format description](https://debezium.io/documentation/reference/stable/tutorial.html) uses:
    - `c` - for create / insert
    - `u` - for update
    - `d` - for delete
    - `r` - for read (in the case of a snapshot)
  - Flink docs [call out](https://nightlies.apache.org/flink/flink-docs-release-1.18/docs/connectors/table/formats/debezium/) that they currently don't support merging their `UPDATE_BEFORE` and `UPDATE_AFTER` messages into a single Debezium update message
- **Arroyo**:
  - [UpdatingData type](https://github.com/ArroyoSystems/arroyo/blob/master/arroyo-types/src/lib.rs#L469)
    - `Retract(T)`
    - `Update { old: T, new: T }`
    - `Append(T)`
- **Apache Spark**:
  - [Structured Streaming](https://spark.apache.org/docs/latest/structured-streaming-programming-guide.html) only supports append and update modes and does not support retractions
- **Differential Dataflow / Materialize**:
  - Seems to only support retraction model in the form of `(data, timestamp, diff)` records, where `diff` is a record count difference (`1` for append, and `-1` for retraction) [[1]](https://github.com/MaterializeInc/materialize/blob/ea9338a1d007bf594bfb1a8d068095cd64090666/src/catalog/src/durable/impls/persist/state_update.rs#L53C52-L53C67) [[2]](https://materialize.com/docs/integrations/python/#stream) [[3]](https://materialize.com/blog/power-user/)

## Unresolved questions
Ideally, every retraction/correction in a dataset can be easily associated back with the original event that is being retracted or corrected. One could imagine this working via `offset` system column, that uniquely identifies all records in a dataset.

Such association would allow:
- Fast navigation between retractions/corrections and original records
- Fast lookups of whether a particular range of records was affected by any retractions/corrections in the future
  - e.g. for generating warnings during `AS OF` queries or about stale notebooks

In practice this may be hard to implement. A record identifier such as `offset` would need to be propagated through all streaming queries. We wouldn't want to leave such a delicate and error-prone detail to the user, so we would need to analyze and dynamically rewrite queries to add offset propagation. And since the only thing that knows how to interpret engine's dialect is the engine itself - this rewrite would need to be implemented individually by each engine. 

At this stage of ODF development we decided that introducing query rewrite would be too costly, but we will consider it in future.

Additionally, query rewrites and associations between records are already a part of the vision for ODF's **fine-grain provenance**, and must be designed together, holistically.

## Future possibilities
- `MergeStrategy::Snapshot` may allow omitting the `primaryKey`, in which case we would default to **retract stream** CDC scheme
- Retractions/corrections can be associated with the records they affect via `offset` that would allow fast navigation and affected range queries (see [Unresolved questions](#unresolved-questions) above)
- `Top-N` and similar queries may be a better fit for "Projections" feature where a state is maintained in some separate data structure that is optimized for querying
- We may want to support [upsert](#upsert-stream) streams as they can significantly reduce the amount of events needed to reconstruct the state, but they require knowledge of primary key. We should consider whether upsert streams are useful in our core model or more suitable for projection upkeep.
