# Data Model <!-- omit in toc -->

**Domain**: Datasets
**Depends on**: [Data Schema](data-schema.md)
**RFCs**: [RFC-001](../../rfcs/001-record-offsets.md), [RFC-015](../../rfcs/015-unified-changelog-stream-schema.md)


## Abstract <!-- omit in toc -->

This spec defines how data of a dataset is represented in memory and at rest, the system columns that all data in ODF carries to support the bitemporal event model, the changelog representation of retractions and corrections, and the rules according to which a dataset's schema is allowed to evolve over time.


## Table of Contents <!-- omit in toc -->

- [In-memory and On-disk Formats](#in-memory-and-on-disk-formats)
- [Common Data Schema](#common-data-schema)
- [Representation of Retractions and Corrections](#representation-of-retractions-and-corrections)
- [Schema Evolution](#schema-evolution)


## In-memory and On-disk Formats
We differentiate two data formats by their purpose:
- **In-memory format** - used when passing data around between the subsystems (e.g. when [Coordinator](../glossary.md#coordinator) communicates with the [Engine](../glossary.md#engine)).
- **On-disk format** - used for data at rest and by query engines when querying data from the entire dataset.

For our **in-memory format** we chose [Apache Arrow](https://arrow.apache.org/), as it is:
- Purposefully designed for minimal overhead interoperability between data processing systems
- Minimizes copying of data
- Hardware efficient
- Supports streaming

For our **on-disk format** we choose [Apache Parquet](https://parquet.apache.org/), as it is:
- Fast to decode into [Apache Arrow](https://arrow.apache.org/)
- Space and IO efficient thanks to the built-in compression
- Efficient for querying thanks to the columnar structure and built-in basic indexing

Data on disk is stored in the multiple "part" files. Once a part file is written it is immutable for the entire lifetime of the dataset.


## Common Data Schema
All data in the system is guaranteed to have the following columns:

|    Column     | Description                                                                                                                                                                                                                                                                                                                                       |
| :-----------: | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|   `offset`    | [Offset](../glossary.md#offset) is a sequential identifier of a row relative to the start of the dataset (first row has an offset of `0`)                                                                                                                                                                                                                       |
|     `op`      | [Operation Type](../glossary.md#operation-type) is used to differentiate regular append events from retractions and corrections                                                                                                                                                                                                                                 |
| `system_time` | [System Time](../glossary.md#system-time) denotes when an event first appeared in the dataset. This will be an ingestion time for events in the [Root Dataset](../glossary.md#root-dataset) or transformation time in the [Derivative Dataset](../glossary.md#derivative-dataset)                                                                                                           |
| `event_time`  | [Event Time](../glossary.md#event-time) denotes when to our best knowledge an event has occurred in the real world. By default all temporal computations (windowing, aggregations, joins) are done in the event time space thus giving the user query an appearance of a regular flow of events even when data is backfilled or frequently arrives out-of-order |

Representation:

|    Column     |      Arrow Type      |                            Parquet Type                             | Recommended Parquet Encoding |
| :-----------: | :------------------: | :-----------------------------------------------------------------: | :--------------------------: |
|   `offset`    |       `uint64`       |                               `INT64`                               |    `DELTA_BINARY_PACKED`     |
|     `op`      |       `uint8`        |                               `INT32`                               |       `RLE_DICTIONARY`       |
| `system_time` | `timestamp(ms, UTC)` |           `INT64, TIMESTAMP(MILLIS, AdjustedToUTC=true)`            |      `PLAIN_DICTIONARY`      |
| `event_time`  | `timestamp(ms, UTC)` | `INT64, TIMESTAMP(MILLIS, AdjustedToUTC=true)`<br/>or `int32, DATE` |                              |


> **TODO:**
> - We are not allowing non-UTC-adjusted timestamps yet as Parquet does not offer a way to encode the timezone, meaning we need a reliable way to pass timezone information between different engines through some other means (e.g. Parquet metadata). Having naive/local timestamps without enforcing that they are accompanied by the specific timezone would be too error-prone.


## Representation of Retractions and Corrections
[Retractions and corrections](../glossary.md#retractions-and-corrections) are differentiated from regular [Events](../glossary.md#event) in the [Data Slice](../glossary.md#data-slice) via special `op` column, carrying an [Operation Type](../glossary.md#operation-type).

Valid operation types are:

| Value | Operation name | Operation short code |
| :---: | :------------: | :------------------: |
|   0   |    `append`    |         `+A`         |
|   1   |   `retract`    |         `-R`         |
|   2   | `correct-from` |         `-C`         |
|   3   |  `correct-to`  |         `+C`         |

The `retract` and `correct-from` events must carry the same exact data fields (all fields excluding `offset`, `op`, and `system_time`) as the event that is being retracted.

The `correct-to` event carries the new values for the event being corrected. This event must immediately follow the `correct-from` event. This model allows data engines that operate on changelog / diff events that simultaneously carry both old and new values for event during a correction to reconstruct such events easily by sequentially reading the [Data Slice](../glossary.md#data-slice).

See also:
- [RFC-015: Unified Changelog Stream Schema](../../rfcs/015-unified-changelog-stream-schema.md)


## Schema Evolution
[Dataset](../glossary.md#dataset) [Schema](../glossary.md#schema) may change when:
- A different [Ingestion](../glossary.md#ingestion) process is specified in the [Root Dataset](../glossary.md#root-dataset)
- A transformation [Query](../glossary.md#query) of the [Derivative Dataset](../glossary.md#derivative-dataset) is changed
- A [Schema](../glossary.md#schema) of one of the inputs of the [Query](../glossary.md#query) changes

In general, [Schema](../glossary.md#schema) changes are restricted to only adding new columns. Removing and renaming columns or changing data types would break downstream data consumers and therefore not allowed. A migration procedure should be used instead when it's desired to introduce breaking changes in the [Schema](../glossary.md#schema).
