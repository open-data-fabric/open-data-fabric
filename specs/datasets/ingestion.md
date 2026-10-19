# Ingestion <!-- omit in toc -->

**Domain**: Datasets\
**Depends on**: [Data Model](data-model.md), [Data Schema](data-schema.md), [Resource Framework](../automation/resource-framework.md)\
**RFCs**: [RFC-009](../../rfcs/009-ingest-source-state.md), [RFC-011](../../rfcs/011-push-ingest-sources.md), [RFC-014](../../rfcs/014-minimize-offset-scanning.md)


## Abstract <!-- omit in toc -->

This spec defines how external data is brought into [Root Datasets](../glossary.md#root-dataset). It defines the `Source` resource that describes where data comes from and how it is read, the phases of the ingestion process, the [Merge Strategies](../glossary.md#merge-strategy) that turn data of various nature into an append-only history of events, how [Event Time](../glossary.md#event-time) is determined, and how the state of a source is preserved to allow incremental ingestion.


## Table of Contents <!-- omit in toc -->

- [Purpose](#purpose)
- [Sources](#sources)
  - [Push and Polling Sources](#push-and-polling-sources)
- [Ingestion Phases](#ingestion-phases)
- [Ingress](#ingress)
  - [Event Time Sources](#event-time-sources)
  - [Caching](#caching)
  - [Container Interface](#container-interface)
  - [EVM Logs](#evm-logs)
- [Prepare Steps](#prepare-steps)
- [Read Formats](#read-formats)
- [Preprocessing](#preprocessing)
- [Merge Strategies](#merge-strategies)
  - [Append](#append)
  - [Ledger](#ledger)
  - [Snapshot](#snapshot)
  - [Changelog Stream](#changelog-stream)
  - [Upsert Stream](#upsert-stream)
- [Committing Data](#committing-data)
  - [Source State](#source-state)
- [Validity of Root Data](#validity-of-root-data)
- [Legacy Source Events](#legacy-source-events)
- [Schemas](#schemas)


## Purpose
When interacting with data on the web we cannot make any assumptions about the guarantees the external source provides:
- Just by looking at the data we cannot tell if it is mutable or preserves the history of changes
- We often can't tell where it originates from and who can be held accountable for its validity
- We can't be sure that it's sufficiently replicated - the domain that hosts it may not exist the next day

[Ingestion](../glossary.md#ingestion) is about getting the external data into the system as efficiently as possible, into the strict ledgers of [Root Datasets](../glossary.md#root-dataset), where properties like clear ownership, reproducibility, accountability, and complete historical account can be guaranteed and made explicit.

This spec does not aim to standardize all possible data ingestion techniques. ODF is not meant to replace data workflow and data extraction tools - the ingestion mechanisms described here simplify the very first step in the data's journey through the stream processing pipelines, and implementations may support additional ingress types, formats, and steps.


## Sources
A source of data is described by a `Source` [resource](../glossary.md#resource):

```yaml
$schema: https://opendatafabric.org/schemas/sources/v1alpha1/Source
headers:
  name: ca.bankofcanada
spec:
  # Makes variables and secrets available for templating under `config`
  config:
    apiKey: SecretSet:ca.bankofcanada#apiKey
  # Where the data comes from
  ingress:
    kind: Url
    url: https://www.bankofcanada.ca/valet/observations/group/FX_RATES_DAILY/csv
    headers:
      - name: Authorization
        value: "Bearer ${{ config.apiKey }}"
  # How to read the data into structured form
  read:
    kind: Csv
    header: true
    schema:
      fields:
        - name: date
          type: Timestamp
        - name: FXUSDCAD
          type:
            kind: Decimal
            precision: 19
            scale: 6
  # OPTIONAL: Query that shapes the data
  preprocess:
    kind: Sql
    engine: datafusion
    query: |
      select
        date as event_time,
        FXUSDCAD as fxusdcad
      from input
  # How to combine new data with the history of previously seen data
  merge:
    kind: Ledger
    primaryKey:
      - event_time
```

A source is decoupled from the datasets it writes to. Ingestion is performed by an `Ingest` task that references the `Source` and the target root dataset, and that can be scheduled by a flow (see [Tasks and Flows](../automation/tasks-and-flows.md)):

```yaml
$schema: https://opendatafabric.org/schemas/flows/v1alpha1/Flow
headers:
  name: ca.bankofcanada
spec:
  target: Dataset:ca.bankofcanada
  triggers:
    - kind: Schedule
      cron: "*/30 * * * *"
  tasks:
    - kind: Ingest
      source: Source:ca.bankofcanada
      targetRecordsPerSlice: 10000
```

All data ingested into a dataset must conform to the dataset's schema, as captured by the [`SetDataSchema`](../../schemas/datasets/v1alpha1/SetDataSchema.json) event.


### Push and Polling Sources
Depending on the type of [ingress](#ingress), sources fall into two categories:
- **Polling sources** - the node periodically scans a remote data source and ingests the latest updates. This is suitable for periodic database dumps, data published as files on the web, bulk data APIs, and blockchains.
- **Push sources** - an external actor actively sends data into the node. This is suitable for IoT devices, business processes that report events directly, and streaming data APIs and event queues.

Although we aspire to reach a state where all authoritative data publishers push new events as soon as those occur, we realize that this level of integration will take time to achieve, and for a long time the vast majority of data will be ingested via the polling model.


## Ingestion Phases
The ingestion process consists of the following phases, directly reflected in the structure of the [`SourceSpec`](../../schemas/sources/v1alpha1/SourceSpec.json):

| Phase | Field | Description |
| --- | --- | --- |
| Ingress | `ingress` | Obtains the data from an external source in its original raw form (e.g. via HTTP), or accepts the data pushed by an external actor |
| Prepare (optional) | `prepare` | Prepares raw binary data for reading, e.g. by extracting an archive or converting between formats |
| Read | `read` | Reads the data in some format into a structured form |
| Preprocess (optional) | `preprocess` | Reshapes the structured data with a query, e.g. renaming columns, type conversions, nesting |
| Merge | `merge` | Combines the new data with the history of previously seen data (see [Merge Strategies](#merge-strategies)) |

Additionally:
- `vocab` - specifies the names of the system columns (see [`DatasetVocabulary`](../../schemas/datasets/v1alpha1/DatasetVocabulary.json)) in case the data uses names different from the defaults
- `config` - brings variables and secrets into the `config` context, so they can be used for templating values of other fields (e.g. `${{ config.apiKey }}`)


## Ingress
The `ingress` field ([`Ingress`](../../schemas/sources/v1alpha1/Ingress.json)) determines where data comes from:

| Kind | Type | Description |
| --- | --- | --- |
| `Url` | Polling | Fetches data by its URL using one of the supported protocols (e.g. HTTP, FTP), optionally passing request headers |
| `FilesGlob` | Polling | Matches a growing set of files on the local file system using a glob pattern |
| `Container` | Polling | Runs an OCI container to fetch data from an arbitrary source (see [Container Interface](#container-interface)) |
| `EvmLogs` | Polling | Streams transaction logs from an EVM-compatible blockchain node (see [EVM Logs](#evm-logs)) |
| `Mqtt` | Push | Subscribes to topics of an MQTT broker |
| `RestEndpoint` | Push | Exposes an HTTP endpoint that accepts pushed records |

`FilesGlob` is used for directories containing a growing set of files, e.g. periodic database dumps or batches of new data. Files should never change once added - the source remembers the last ingested file and only considers files that come after it in the order specified by `order`: lexicographically by name (`ByName`) or by [event time](#event-time-sources) (`ByEventTime`). Every file is processed individually and advances the [Watermark](../glossary.md#watermark) of the dataset.

Push ingresses buffer the received records until they are ingested by a task. The `RestEndpoint` ingress specifies the `buffer` ([`IngressBuffer`](../../schemas/sources/v1alpha1/IngressBuffer.json)) with its size and the `overflowPolicy` applied when the buffer is full: rejecting new records (`Reject`) or dropping the oldest ones (`DropOldest`).


### Event Time Sources
Ideally, data records contain [Event Time](../glossary.md#event-time) within them as a column, but many data sources don't. Polling ingresses can specify how event time is determined via the `eventTime` field ([`EventTimeSource`](../../schemas/sources/v1alpha1/EventTimeSource.json)):

| Kind | Description |
| --- | --- |
| `FromMetadata` | Extracts event time from the metadata of the source, e.g. modification time of files or the `Last-Modified` header of HTTP resources |
| `FromPath` | Extracts event time from the path of the source using a regular expression whose first group contains the timestamp, and a timestamp format |
| `FromSystemTime` | Uses the [System Time](../glossary.md#system-time) of ingestion as the event time |

Example of extracting event time from file names:

```yaml
ingress:
  kind: FilesGlob
  path: /data/db-table-dump-*.csv
  eventTime:
    kind: FromPath
    pattern: 'db-table-dump-(\d+-\d+-\d+)\.csv'
    timestampFormat: '%Y-%m-%d'
```

Event time is especially important for the [Snapshot](#snapshot) merge strategy, as snapshot-style exports represent the state of a system at a certain time that is usually captured only in some form of metadata (e.g. in the name of the file, in the URL, or in HTTP caching headers).


### Caching
Implementations should avoid redundant work and not ingest data if the source was not updated since the last poll. The exact mechanism depends on the ingress type and the protocol, e.g. for HTTP it relies on standard caching headers like `ETag` and `Last-Modified`, and the state needed for this is preserved as [source state](#source-state).

Caching behavior can be controlled via the `cache` field ([`SourceCaching`](../../schemas/sources/v1alpha1/SourceCaching.json)). The `Forever` kind specifies that after the source was processed once it will never be ingested again.


### Container Interface
The `Container` ingress allows using a general purpose programming language to deal with particularly complex APIs, or for web scraping. The specified OCI image is expected to conform to the following interface:
- Produce data to `stdout`
- Write warnings and errors to `stderr`
- Use the following environment variables:

| Variable | Description |
| --- | --- |
| `ODF_LAST_MODIFIED` | Last modified time of data from the previous ingest run, if any (in RFC 3339 format) |
| `ODF_ETAG` | Caching tag of data from the previous ingest run, if any |
| `ODF_BATCH_SIZE` | Recommended number of records, for scripts that provide a continuous stream of data and can resume from previous state |
| `ODF_NEW_LAST_MODIFIED_PATH` | Path to a text file where the script may write the new `Last-Modified` timestamp |
| `ODF_NEW_ETAG_PATH` | Path to a text file where the script may write the new `ETag` |
| `ODF_NEW_HAS_MORE_DATA_PATH` | Path to a file which the script can create to indicate that more data is available for the next batch. When the file is created, the new `ETag` or `Last-Modified` must also be written |


### EVM Logs
The `EvmLogs` ingress reads logs of EVM-compatible blockchains directly from a node. The `filter` field is an SQL `WHERE` clause used to create the most efficient RPC request, and the optional `signature` specifies a Solidity event signature used to decode logs.

The output structure corresponds to the output of the [`eth_getLogs`](https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_getlogs) RPC method:

| Column | Type | Nullable | Description |
| --- | --- | --- | --- |
| `block_number` | `UInt64` | | |
| `block_hash` | `Binary` | | |
| `block_timestamp` | `Timestamp` | ✅ | Not returned by all node providers |
| `transaction_index` | `UInt64` | | |
| `transaction_hash` | `Binary` | | |
| `log_index` | `UInt64` | | |
| `address` | `Binary` | | |
| `topic0` ... `topic3` | `Binary` | ✅ | |
| `data` | `Binary` | | |
| `event` | `Struct` | | Decoded event, only present when `signature` is specified |


## Prepare Steps
The `prepare` field specifies a list of steps ([`PrepStep`](../../schemas/sources/v1alpha1/PrepStep.json)) applied to the raw data before reading:

| Kind | Description |
| --- | --- |
| `Decompress` | Extracts data compressed with `Gzip` or `Zip`. For multi-file archives `subPath` (can contain glob patterns) specifies which file to extract |
| `Pipe` | Executes an external command that receives data via `stdin` and outputs the result to `stdout`, e.g. to transcode formats that are not natively supported |


## Read Formats
The `read` field ([`ReadStep`](../../schemas/sources/v1alpha1/ReadStep.json)) specifies how data is read into the structured form:

| Kind | Description |
| --- | --- |
| `Csv` | Comma-separated values, with options for separators, quoting, escaping, headers, null values, and date / timestamp formats |
| `Json` | JSON document containing an array of objects, optionally located at `subPath` within the document |
| `NdJson` | [Newline-delimited JSON](https://github.com/ndjson/ndjson-spec) objects with the same schema |
| `GeoJson` | GeoJSON document with a `FeatureCollection` object in the root, creating a record per each `Feature` with properties extracted into individual columns and the geometry left in its own column |
| `NdGeoJson` | Similar to `GeoJson`, but with every `Feature` object on its own line |
| `EsriShapefile` | ESRI Shapefile, optionally located at `subPath` within an archive |
| `Parquet` | Apache Parquet |

The read step can specify the `schema` of the data in the [Data Schema](data-schema.md) format, used to coerce values into appropriate data types. Formats like JSON can't be split and read efficiently, so newline-delimited variants should be preferred where possible.


## Preprocessing
The optional `preprocess` field specifies an engine-specific query ([`Transform`](../../schemas/datasets/v1alpha1/Transform.json)) that shapes the data after reading, e.g. to parse values, convert types, or rename columns to match the dataset schema. The query refers to the read data as `input`:

```yaml
preprocess:
  kind: Sql
  engine: datafusion
  query: |
    select
      date,
      city,
      cast(replace(population, ',', '') as bigint) as population
    from input
```

Preprocessing queries are executed by [Engines](../glossary.md#engine) using the [Raw Query](../transformations/engine-contract.md#raw-query) operation.

The order of records in the query result is the order in which the subsequent [Merge Strategy](#merge-strategies) receives them. For strategies where this order is significant the engine will be instructed to preserve the order of records read from the input.


## Merge Strategies
By design, ODF stores data only in the append-only event log format to preserve the entire history. Unfortunately, a lot of data in the world is not stored or exposed this way - some publishers expose data as periodic database dumps, while others provide a log of changes between the current and the previous export.

[Merge Strategies](../glossary.md#merge-strategy) ([`MergeStrategy`](../../schemas/sources/v1alpha1/MergeStrategy.json)) bridge the gap between the wide variety of data in the external world and the strict event-based world of ODF, by defining how the newly-ingested data is combined with the existing history.

All strategies except `Append` rely on a user-specified `primaryKey` - the names of columns that uniquely identify a record throughout its lifetime.

Strategies differ in whether the order of records in the new data is significant:
- [Changelog Stream](#changelog-stream) and [Upsert Stream](#upsert-stream) treat the new data as a sequence of changes, each related to a certain `primaryKey`. The order of those changes is significant and must be preserved through all [ingestion phases](#ingestion-phases) up to the merge (see [Preprocessing](#preprocessing)).
- [Append](#append), [Ledger](#ledger), and [Snapshot](#snapshot) treat the new data as a set of records - its order is not significant, and implementations may assign [Offsets](../glossary.md#offset) to the resulting records in any order (e.g. by event time or by primary key).


### Append
Under this strategy the new data is appended to the dataset in its entirety, without any modifications or deduplication.

This strategy is rarely used for polling sources, but can be useful for push sources (e.g. IoT devices).


### Ledger
This strategy should be used for data sources containing append-only ledgers of events. New data exports can have new rows added, but once data made it into one export it should never change or disappear. The primary key is used to identify which events were already seen, so that the source data can contain partially overlapping sets of records and only those records that were not previously seen are appended.

For example, a source with historical populations of cities:

```csv
Year,Country,City,Population
2019,CA,Vancouver,2581000
2019,US,Seattle,3433000
```

Ingested with primary key `(year, country, city)` results in:

| op | system_time | event_time | country | city | population |
| --- | --- | --- | --- | --- | --- |
| +A | s1 | 2019 | CA | Vancouver | 2581000 |
| +A | s1 | 2019 | US | Seattle | 3433000 |

When a new row for 2020 appears in the source, only that row is appended, while rows for 2019 are recognized as already seen:

| op | system_time | event_time | country | city | population |
| --- | --- | --- | --- | --- | --- |
| +A | s1 | 2019 | CA | Vancouver | 2581000 |
| +A | s1 | 2019 | US | Seattle | 3433000 |
| **+A** | **s2** | **2020** | **CA** | **Vancouver** | **2606000** |

Having a time column doesn't always mean that the data is a ledger - to be a true ledger all previous records must be immutable.


### Snapshot
This strategy can be used for data exports that are taken periodically and contain only the latest state snapshot of the observed entity or system. Over time such exports can have new rows added, and old rows either removed or modified.

Publishers that use such formats routinely lose information - when a record is updated or one export replaces another, the history of previous values is lost. This strategy transforms snapshot data into an append-only event stream by performing [change data capture](https://en.wikipedia.org/wiki/Change_data_capture) - diffing the current state of data against the reconstructed previous state. The primary key is used to correlate rows between the two snapshots, and the [Operation Type](../glossary.md#operation-type) column of the output will contain:
- Append (`+A`) - when a row with a certain key appears for the first time
- Retraction (`-R`) - when a row with a certain key disappears from the snapshot
- Correction (`-C`, `+C`) - when the data associated with a certain key has changed, with `-C` carrying the old values and `+C` carrying the new values (see [Retractions and Corrections](../glossary.md#retractions-and-corrections))

To identify whether a row has changed, this strategy compares all columns one by one. If the data contains a column that is guaranteed to change whenever any other column changes (e.g. a modification timestamp, a version, or a data hash), it can be specified in `compareColumns` to speed up the detection of modified rows.

For example, a source with current populations of cities, ingested with primary key `(country, city)` and event time [taken from the source metadata](#event-time-sources):

```csv
Country,City,Population
CA,Vancouver,2581000
US,Seattle,3433000
```

| op | system_time | event_time | country | city | population |
| --- | --- | --- | --- | --- | --- |
| +A | s1 | e1 | CA | Vancouver | 2581000 |
| +A | s1 | e1 | US | Seattle | 3433000 |

After the population of Vancouver changes in the source, ingestion produces a correction:

| op | system_time | event_time | country | city | population |
| --- | --- | --- | --- | --- | --- |
| +A | s1 | e1 | CA | Vancouver | 2581000 |
| +A | s1 | e1 | US | Seattle | 3433000 |
| **-C** | **s2** | **e1** | **CA** | **Vancouver** | **2581000** |
| **+C** | **s2** | **e2** | **CA** | **Vancouver** | **2606000** |


### Changelog Stream
This strategy is used for sources that already provide data in the native ODF changelog format that describes appends, retractions, and corrections (see [Data Model](data-model.md#representation-of-retractions-and-corrections)). No processing except for format validation is performed.

Records are applied in the order they are read: a key can be retracted and appended again, or corrected several times within one portion of new data. The resulting records are therefore assigned [Offsets](../glossary.md#offset) in the order of the input.


### Upsert Stream
This strategy is used for sources containing ledgers of insert-or-update and delete events. Unlike in the changelog stream, insert-or-update events only carry the new values, so this strategy uses the primary key to look up the previous values and re-classify events into appends or correction pairs.

When the new data contains several events for the same key, they are applied in the order they are read, so the last of them determines the new state of the key.


## Committing Data
The result of every ingestion is committed into the [Metadata Chain](../glossary.md#metadata-chain) of the dataset as an [`AddData`](../../schemas/datasets/v1alpha1/AddData.json) event, which carries:
- `prevOffset` and `newData` - the last offset of the previous data slice and the new data slice, if any, allowing the next ingestion to be prepared without deep scanning of the chain
- `prevCheckpoint` and `newCheckpoint` - checkpoint used to restore and persist the ingestion state, if any
- `newWatermark` - the new [Watermark](../glossary.md#watermark) of the dataset, if any
- `newSourceState` - the [state of the source](#source-state), if any

An ingestion may produce no new data but still commit a block, e.g. to advance the watermark or to update the source state.


### Source State
To avoid expensive recomputations, the state of the source the data was ingested from (e.g. an `ETag` of an HTTP resource, the name of the last file matched by a glob pattern, or the height of the last indexed block of a blockchain) is stored in the `newSourceState` field ([`SourceState`](../../schemas/sources/v1alpha1/SourceState.json)) of the `AddData` event:

```yaml
newSourceState:
  sourceName: odf/polling
  kind: odf/etag
  value: W/"95c5fde3918cba7c33eaac7ff9d02b22"
```

Where:
- `sourceName` - identifies the source the state belongs to
- `kind` - identifies the type of the state, similarly to MIME types. Standard kinds are `odf/etag` (an identifier with semantics of the `ETag` HTTP header) and `odf/last-modified` (an RFC 3339 timestamp with semantics of the `Last-Modified` HTTP header)
- `value` - an opaque value of the state

Both `sourceName` and `kind` are plain strings, allowing implementations to define their own extensions. It is safe to ignore source state that an implementation does not understand.

If the state didn't change but is still relevant for subsequent runs it should be carried over into the new event, as only the last state per source is considered when resuming.


## Validity of Root Data
Unlike data of [Derivative Datasets](../glossary.md#derivative-dataset), data of [Root Datasets](../glossary.md#root-dataset) cannot be reproduced - it captures observations of the outside world. Its validity therefore depends entirely on its publisher, who is in full control of the data they present, so measuring and processing errors or even malicious data can make their way into a root dataset.

What can be validated mechanically is that root data was not altered after it was published (see [Metadata Chain: Validation](metadata-chain.md#validation) and [Data & Checkpoint Hashing: Validation and Verification](data-and-checkpoint-hashing.md#validation-and-verification)), and who published it. Trust in the data itself is established through:
- **Authority** - the reputation of the publisher, e.g. organizations that directly operate the systems from which the data is collected
- **Cross-validation** - comparing data against other similar sources to detect anomalies

See also:
- [Verifiability](../glossary.md#verifiability)


## Legacy Source Events
> **Deprecated:** Sources used to be defined in the [Metadata Chain](../glossary.md#metadata-chain) of root datasets using the following events. They are superseded by `Source` resources, as source definitions don't contribute to provenance or validity of a dataset, but may reveal sensitive information about the infrastructure of a publisher (see [RFC-018](../../rfcs/018-iac-resource-framework.md)).

| Event | Description |
| --- | --- |
| [`SetPollingSource`](../../schemas/legacy/v0/SetPollingSource.json) | Defines the polling source of a dataset, with `fetch`, `prepare`, `read`, `preprocess`, and `merge` steps |
| [`DisablePollingSource`](../../schemas/legacy/v0/DisablePollingSource.json) | Disables the polling source |
| [`AddPushSource`](../../schemas/legacy/v0/AddPushSource.json) | Defines a named push source of a dataset, with `read`, `preprocess`, and `merge` steps |
| [`DisablePushSource`](../../schemas/legacy/v0/DisablePushSource.json) | Disables a named push source |

Under this model:
- A dataset may have multiple named push sources active simultaneously (e.g. for devices from different manufacturers producing slightly different data), but push and polling sources are mutually exclusive
- All sources must result in the same final data schema, as captured in the `SetDataSchema` event
- The source state of the polling source uses the `odf/polling` source name


## Schemas

| Schema | Description |
| --- | --- |
| [`Source`](../../schemas/sources/v1alpha1/Source.json) | Source resource |
| [`SourceSpec`](../../schemas/sources/v1alpha1/SourceSpec.json) | Specification of a source |
| [`Ingress`](../../schemas/sources/v1alpha1/Ingress.json) | Where data comes from |
| [`IngressBuffer`](../../schemas/sources/v1alpha1/IngressBuffer.json) | Buffering of pushed records |
| [`EventTimeSource`](../../schemas/sources/v1alpha1/EventTimeSource.json) | How event time is determined |
| [`SourceCaching`](../../schemas/sources/v1alpha1/SourceCaching.json) | Caching of external data |
| [`SourceOrdering`](../../schemas/sources/v1alpha1/SourceOrdering.json) | Ordering of input files |
| [`PrepStep`](../../schemas/sources/v1alpha1/PrepStep.json) | Preparation of raw data |
| [`ReadStep`](../../schemas/sources/v1alpha1/ReadStep.json) | Reading data into structured form |
| [`MergeStrategy`](../../schemas/sources/v1alpha1/MergeStrategy.json) | Combining new data with history |
| [`SourceState`](../../schemas/sources/v1alpha1/SourceState.json) | State of a source for incremental ingestion |
| [`AddData`](../../schemas/datasets/v1alpha1/AddData.json) | Metadata event committing the ingested data |
