# Open Data Fabric

Version: 0.15.0

# Abstract
**Open Data Fabric** is an open protocol specification for decentralized exchange and transformation of semi-structured data that aims to holistically address many shortcomings of the modern data management systems and workflows.

# Problem Statement
Develop a method of semi-structured data exchange that would:
- Enable worldwide collaboration around data cleaning, enrichment, and transformation.
- Create an environment of verifiable trust between participants without the need for a central authority.
- Enable a high degree of data reuse, making quality data more readily available.
- Reduce the time it takes data to propagate from publishers to consumers.
- Introduce a feedback loop between the consumers and publishers of data that would facilitate improvements around data availability, recency, and design.
- Provide foundation for implementing data anonymization techniques to simplify and speed up the exchange of vital but sensitive data.

# Table of Contents
<!-- toc -->

# Requirements
Functional:
- **Complete historical account** - when data is used to gain insight and to drive decision making discarding or modifying data is akin to rewriting history. The history of all data observed by the system must be preserved.
- **Reproducibility** - the ability to reproduce the results is a cornerstone of the scientific method without which the process and findings of one party cannot be verified by others. Therefore we require that all the transformations performed within the system must be fully reproducible, and it must be possible to get a reference to data that is frozen in time and never changes to achieve reproducibility of any process that uses it.
- **Verifiability** - any party that plans to use some data must be able to verify its validity.
- **Provenance** - regardless of how many transformation stages the data went through, it should be possible to trace any individual data cell back to its ultimate source and understand which data contributed to its existence and its value.

Non-Functional:
- **Transparency** - the method itself should be developed in the open, easy to understand, and reason about.
- **Timeliness** - the method should aim to minimize the propagation time of data through the transformation pipeline.
- **Transactionality** - once data is in the system all operations should have [ACID properties](https://en.wikipedia.org/wiki/ACID) and exactly-once semantics.
- **Decentralization** - there must be no entity in the system with any kind of special privileges over data, or abilities to circumvent any of the above requirements.

# Scope
The primary focus of this specification is the **mission-critical data** such as:
- Business performance data (sales, CRM)
- Governmental data
- Financial data
- Healthcare data
- Scientific data
- As well as any other data used for decision making

This specification does not target very high volume sources like IoT and sensor data where infinite retention of the entire history may be cost-prohibitive. We do, however, hope to shift the mindset of the data science community towards thinking about such cases as **design compromises**. Routine data loss should no longer be our default mode of operations.

# Design Decisions

## Nature of Data
In modern data science "data" is a very broad term that includes many shapes and forms of quantitative and qualitative records. We will not attempt to provide a taxonomy here but will claim that the large portion of data published today carries in it the limitations and design compromises of systems that generated it.

> Examples:
>
> **Snapshot data** - many [OLTP](https://en.wikipedia.org/wiki/Online_transaction_processing) systems are designed to only maintain the current state of the information. The data sourced from OLTP systems is usually in the form of periodic database dumps (snapshots) or [change data capture](https://en.wikipedia.org/wiki/Change_data_capture) logs at best.
>
> Such data has many limitations:
> - It's often accompanied by data loss (e.g. all data changes between the two snapshots are lost).
> - Data is anemic and doesn't carry the business context of the changes.
>
> **Aggregated data** is the form of data in which information from several measurements is combined and expressed in a collective or summary form. This form of data is very common in governmental and healthcare sources, where aggregations often hide important deprivations and inequalities between gender, ethnic, and other groups.
>
> Although such styles of data have become the norm for many major data publishers today, we believe that such treatment is **inadequate** and at odds with many of the requirements we put towards the system. Data is our modern-day history book and should be treated as such.

To achieve the desired qualities we choose the following narrow definition of data:

> Data is a set of events, observations, or propositions believed to be true at a certain time.

Properties:
- Data is a set of event records / relational tuples
- Events are **immutable**
- Event set can only grow
- Every event set can be viewed as a potentially infinite event stream

Such representation poses many additional challenges when working with data, but, as we will show further - the benefits by far outweigh the added complexity, and that complexity can in most cases be addressed by better tooling.

## Nature of Transformations
The state-of-the-art approach to transforming data today is to version the source data (using a checksum or a stable reference) and version the code that transforms it (using a version control system). The result of transformations is then uploaded to some shared storage and made available to others. There are many tools that improve the reproducibility of such workflows, but all of them treat data as a mere binary blob, deprived of any of its intrinsic qualities.

This leads to many problems:
- Data is routinely copied
- Modified versions of data essentially create new datasets that cannot be easily traced to their origin
- The history of transformations (provenance) cannot be easily established
- It is practically impossible to verify that no accidental or malicious alterations were made

As a result, our workflows create large quantities of datasets that are disjoint from their sources, that contribute to the overall "noise" when searching for data but cannot be meaningfully reused. Any data scientist that cares about the validity of their results therefore mush begin with a clean slate and work with the primary authoritative source of data.

> Modern data science is stuck in a loop where all forward progress is constantly lost because no mechanism exists for making incremental improvements to data.

### Components of Trust
We believe that the most important factor in enabling reuse and collaboration on data is **trust**.

While the implicit trust may be enough within an enterprise, trust on the global scale cannot be blind - it is only possible through transparency, reproducibility, and verifiability of the results. Those qualities cannot be easily layered on top of the existing tools, so we designed the system from the ground up with those qualities at its core.

In our design we achieve trust with following solution criteria:
- When using any data you must be able to trace its source to a trusted authority that originally published it
- All transformations that were applied to it must be known and can be audited
- The results of those transformations can always be reproduced by following the same transformation steps

![Diagram: Transformation](images/transform.svg)

Components:
- Transformations are expressed using [Queries](#query) written in any of the supported languages.
- Queries combine and transform one or many input data streams into an output stream.
- Queries are executed by the [Engines](#engine) - a software that knows how to interpret and execute a certain query dialect.
- The results of queries must be deterministic - always produce the same results for the same set of inputs.
- Engines run in a "sandbox" environment, where they don't have access to a network or any other external resources.
- Engines are strictly versioned as their implementation contributes to the total determinism of the operation.

A transformation executed with the same query, on the same inputs, and using the same version of an engine is **guaranteed to be reproducible**.

This is indeed a very hard departure from the ways of working with data that we are used to. It prohibits using anything external to the system - there can be no API calls, no manual editing of data, no use of 3rd party libraries that are not part of the engine.

The benefits we get in exchange for this strictness are, however, immense:
- **Complete transparency** on where data comes from.
- Ability to audit all transformations.
- Ability to easily **reproduce** and **verify** all results.

All this combined allows us to build **verifiable trust** between parties.

### Derivative Data Transience
Since past data in the input datasets is immutable and all transformations are required to be deterministic - derivative datasets are **functionally dependent** on their inputs. This means that for any given transformation graph it is possible to fully reconstruct data of all derivative datasets from the data of root datasets by simply re-applying all transformations.

> Data in derivative datasets can be considered a **form of caching** and does not require durable storage.

### Stream Processing Model
As we model all data as potentially infinite event streams (see [Nature of Data](#nature-of-data)), it's quite natural to use the **stream processing** techniques for all data transformations.

When working with temporal events, stream processing has the following benefits:
- Lets users define a query once and potentially run it forever. This nicely addresses the **timeliness** requirement, allowing us to minimize the latency with which data propagates through the system.
- Streaming queries are expressive and are closer to _"which question is being asked"_ as opposed to _"how to compute the result"_. They are usually much more concise than equivalent batch queries.
- They can be expressed in a way that is agnostic of how and how often the new data arrives. Whether the data is ingested once a month in Gigabyte batches, in micro-batches every hour, or as a true near real-time stream - processing logic can stay the same, produce the same results, and guarantee the best propagation times possible.
- Modern stream processing techniques make complex problems like handling late and out-of-order data much simpler, more explicit, and significantly less error-prone than equivalent batch operations.

### Essential Role of Metadata
Metadata is the cornerstone of our design. It contains every aspect of where the data came from, how it was transformed, and everything that ever influenced how data looks like throughout its entire lifetime. In a way it's like a digital passport of data using which we can audit and verify its validity (see [Data Sharing](#data-sharing)).

The metadata is always updated whenever data gets updated and stays fully consistent with it. The two are cryptographically linked, making spoofing it near impossible.

Metadata also has an immutable append-only nature. While the rest of the document describes it as a standalone technology, its format is designed to make it easy to integrate with existing ledger-based systems such as blockchain.

## Evolution of Data
As the nature of businesses change, as new requirements arrive, as defects are detected and fixed - it's not a matter of *if* but *when* the time comes to make changes to the data. Calling data a potentially infinite stream would not make much sense without providing a way to improve and evolve it over time. Having to create a new dataset every time you need to change the schema or update the transformation would jeopardize the timeliness requirement, as the entire chain of transformations would have to be rebuilt from scratch after every such change.

Our design goals are, therefore:
- To seamlessly support all backward-compatible data schema changes
- To support the evolution of transformations over time
- To provide a way to correct past mistakes in data

While for simplicity we previously considered derivative datasets to be only defined by one transformation query, in fact it can comprise of multiple queries that were active during different periods of time. The metadata keeps track of which parts of inputs were processed by which query and which parts of output were produced as the result, ensuring reproducibility. This idea will be further explored when we discuss the [Metadata Chain](#metadata-chain), but for now, get used to the idea that **almost anything that defines a dataset can change over time**.

## Data Sharing
The guiding factors in designing how data is shared were:
- Minimal storage requirements - while a certain degree of replication is necessary to keep data highly available, we don't want to duplicate data more than it's necessary.
- Decentralization - there should be no central authority that decides what the "right data" is.
- Verifiable trust - even when data is hosted by a single party any tampering or alterations must still be impossible.
- Minimize the movement of data - when a trusted party keeps data in an environment that also provides compute resources, we would like to be able to process and query data without the need to always download it locally.

The way we designed [Metadata](#essential-role-of-metadata) plays a crucial role in data sharing.

Metadata is usually several orders of magnitude smaller than the data it describes, so it's ideally suited to be widely shared. The exact way it's shared is out of scope of this document, but in most simple cases it can be published openly by the provider of data and cryptographically signed to ensure authenticity.

When the metadata of a certain dataset is reliably known, a peer can then download the data from any untrusted source and use the metadata validate the authenticity of every data slice that composes it (see [Hash Function](#hash)) - this means that we need to only ensure the validity of metadata, while the data can be stored almost anywhere with minimal replication factor, as long as one compromised party does not result in the complete loss of data.

Metadata also provides us with a way to establish the trustworthiness of any dataset by reviewing the transformations declared in it, re-applying those transformations in a trusted environment, and comparing the results to the original data. In a distributed system, having peers cross-validate each others' published data can guarantee trusted results and allow them to promptly identify and exclude malicious peers from the network.

## Transactional Semantics
To achieve a perfect reproducibility the system needs to satisfy very strong transactional properties:
- All coordination operations including things like ingesting, downloading, and sharing data, creating and updating datasets must have [ACID properties](https://en.wikipedia.org/wiki/ACID).
- All data transformations must have **exactly-once** semantics.

# Concepts and Components

## Event
As described in the [Nature of Data](#nature-of-data) section, the system operates only on data expressed as past events, observations, or propositions believed to be true at a certain time. For simplicity, we will use the term "event" throughout to refer to such data tuples.

## Data
Data is a set of [Events](#event) stored in the system. Since events are immutable data can only grow over time. Conceptually it's best to think of data as a full log of a potentially infinite event stream.

Data never appears in the system alone as we would not be able to tell where it came from and whether it can be trusted. Data always appears as part of a [Dataset](#dataset).

![Diagram: Dataset/Data](images/dataset.svg)

See also:
- [Data Format](#data-format)
- [Common Data Schema](#common-data-schema)

## Schema
Schema describes the shape of the [data](#data) by associating names and data types to columns that data is composed of. Schema can change over time and its changes are tracked in the [Metadata Chain](#metadata-chain).

Example:
```
registration_time TIMESTAMP(3),
registration_id UUID,
email STRING,
first_name STRING,
last_name STRING,
date_of_birth DATE,
```

See also:
- [Schema Format](#schema-format)
- [Schema Evolution](#schema-evolution)

## Data Slice
[Data](#data) arrives into the system as the arbitrary large sets of events. We refer to them as "slices".

More formally, a slice is a:
- Continuous part of [Data](#data)
- That has the same [Schema](#schema)
- Defined by its `[start; end]` [System Time](#system-time) interval

![Diagram: Data Slices](images/metadata_transform.svg)

## Metadata Chain
Metadata chain captures all essential information about the [Dataset](#dataset), including:
- Where the data comes from (see [Data Ingestion](#data-ingestion))
- How data was processed (see [Query](#query))
- Its [Schema](#schema)
- Log of all modifications made to the data, including information used to verify the integrity of data
- Current [Watermark](#watermark)

Just like [Data](#data), the metadata chain also has a historical nature. It consists of individual **Metadata Blocks** that are linked together, forming a full timeline of how data was evolving. Much [Events](#event), all metadata blocks are immutable.

![Diagram: Metadata Chain](images/metadata_chain.svg)

Metadata can be extended to carry other kinds of information like:
- Extra meaning and structure of knowledge that data represents (glossary, semantics, ontology)
- Relevant policies, terms, rules, compliance, and regulations (governance)
- License, privacy and security concerns (stewardship)
- Information that aids discovery
- Collaboration information

These extensions are out of scope of this document.

See also:
- [Metadata Chain Format](#metadata-chain-format)

## Dataset
Dataset is the main unit of data exchange in the system. It's simply a combination of:
- [Identity](#dataset-identity)
- [Data](#data)
- [Metadata Chain](#metadata-chain)
- [Checkpoints](#checkpoint)

Depending on where the data comes from datasets can be of these kinds:
- [Root](#root-dataset)
- [Derivative](#derivative-dataset)

![Diagram: Dataset Kinds](images/dataset_kinds.svg)

### Root Dataset
Root datasets are the points of entry of external data into the system. They are usually owned by the organization that has full authority and responsibility over that data, i.e. a trusted source.

Root dataset definition includes:
- Where to fetch the data from - e.g. source URL, a protocol to use, cache control
- How to prepare the binary data - e.g. decompression, file filtering, format conversions
- How to interpret the data - e.g. data format, schema to apply, error handling
- How to combine data ingested in the past with the new data - e.g. append as log or diff as a snapshot of the current state

All this information is stored in the [Metadata Chain](#metadata-chain) and can change over time as the dataset evolves.

See also:
- [Merge Strategy](#merge-strategies)

### Derivative Dataset
Derivative datasets are created by transforming/combining one or multiple existing datasets.

They are defined by the combination of:
- Input datasets
- A [Query](#query) to apply to those
- An [Engine](#engine) used to execute the query

This information is stored in the [Metadata Chain](#metadata-chain) and can change over time as the dataset evolves.

See also:
- [Derivative Data Transience](#derivative-data-transience)

## Query
Queries define how input data is combined, modified, and re-shaped to produce new data.

Queries are used in two contexts:
- When defining new [Derivative Datasets](#derivative-dataset)
- When analyzing and extracting data from an existing [Dataset](#dataset) (local or [Remote](#remote))

The system is agnostic to the exact language used to define the query and the set of supported dialects can be extended by implementing a new [Engine](#engine).

All queries, however, must have the following properties:
- Deterministic
- Pure
- Stream/Batch agnostic

In other words, they should be guaranteed to always produce the same result for the same input data, without side effects.

Example windowed aggregation query in streaming SQL:

```sql
SELECT
  TUMBLE_ROWTIME(event_time, INTERVAL '1' MONTH) as event_time,
  sku_id,
  min(price) as min_monthly_price,
  max(price) as max_monthly_price,
  avg(price) as avg_monthly_price
FROM sku_prices
GROUP BY TUMBLE(event_time, INTERVAL '1' MONTH), sku_id
```

See also:
- [Stream Processing Model](#stream-processing-model)
- [Derivative Data Transience](#derivative-data-transience)
- [Engine Contract](#engine-contract)

## Engine
Engine is an interface shared by all specific implementations of a [Query](#query) dialect. Engine implementations are responsible for applying defined queries to input data and returning the result. For example, some engines allows you to query data using a series of streaming SQL statements.

Engines run in a sandboxed environments and are not permitted to use any external resources to guarantee the reproducibility of all operations.

![Diagram: Derivative Transformation](images/transform_component.svg)

As Engines are in the full control of all data transformations, they are also responsible for answering the [Provenance](#provenance) queries.

See also:
- [Engine Contract](#engine-contract)

## Checkpoint
Checkpoints are used by the [Engines](#engine) to store the computation state between the different invocations of a [Query](#query). They are fully engine-specific and opaque to the system. They are however an essential durable part of a [Dataset](#dataset) as they are necessary to be able to pause and resume the streaming queries, and are essential in implementing "exactly-once" processing semantics.

## Coordinator
Coordinator is an application that implements the common [Dataset](#dataset) management logic.

Core responsibilities:
- Handles all [Metadata Chain](#metadata-chain) operations
- Splits the transformation work into batches based on the dataset's evolution timeline
- Collects relevant data slices of the input datasets
- Delegates data processing to the [Engines](#engine), making all transformations look to them as conventional stream processing
- Commits the resulting data slices and new metadata blocks

See also:
- [Coordinator Contract](#coordinator-contract)

## Ingestion
Ingestion is the process by which external data gets into the system. Typical ingestion steps that describe how data is obtained and read (e.g. fetching data from some URL on the web, decompressing it, and reading it as CSV) are a part of the [Root Dataset](#root-dataset) definition.

See also:
- [Data Ingestion](#data-ingestion)

## Merge Strategy
By [design](#nature-of-data), the system only stores data in the append-only event log format to preserve the entire history. Unfortunately, a lot of data in the world is not stored or exposed this way. Some organizations may expose their data in the form of periodic database dumps, while some choose to provide it as a log of changes between current and the previous export.

When [ingesting data](#ingestion) from external sources, the [Root Datasets](#root-dataset) can choose between different [Merge Strategies](#merge-strategies) that define how to combine the newly-ingested data with the existing one.

For example, when dealing with the daily database dumps, a user can choose the merge strategy that performs [change data capture](https://en.wikipedia.org/wiki/Change_data_capture), transforming dumps into a set of events that signify record creation, update, or deletion.

See also:
- [Merge Strategies](#merge-strategies)

## Hash
[Cryptographic hash functions](https://en.wikipedia.org/wiki/Cryptographic_hash_function) are used by the system in these two scenarios:
- For computing a checksum of a [Data Slice](#data-slice).
- For computing a checksum of a [MetadataBlock](#metadata-chain).

Whenever new events are appended to the [Data](#data) the [Metadata Chain](#metadata-chain) will also be extended with a block containing a checksum of the new data slice. The checksum provides a very quick and reliable way to later validate that the data matches the one that has been written earlier.

The new [MetadataBlock](#metadata-chain) will also be cryptographically signed to guarantee its integrity - this excludes any malicious or accidental alterations to the block.

Usage examples:
- If the [Metadata Chain](#metadata-chain) of a certain dataset is reliably known (e.g. available from many independent peers) a peer can then download the [Data](#data) from any untrusted source and use the hash function to validate the authenticity of every data slice that composes it.
- The trustworthiness of any [Dataset](#dataset) can be established by reviewing the transformations it claims to be performing on data (contained in the [Metadata Chain](#metadata-chain)), re-applying those transformations in a trusted environment, and then comparing the checksums of the result slices.

See also:
- [Data Hashing](#data-hashing)
- [Metadata Block Hashing](#metadata-block-hashing)

## Provenance
Data provenance describes the origins and the history of data and adds value to data by explaining how it was obtained.

[Metadata Chain](#metadata-chain) alone can already significantly narrow down the search space when you want to explain how a certain piece of data came to be, as it keeps track of all the inputs and queries used to create a dataset. But the goal of the provenance system is to make this type of inquiries effortless.

We differentiate the following kinds of provenance:
- **Why-provenance** - tells us which input data elements were inspected to decide that an element should be present in the output at all - i.e. defines a sufficient set of elements needed to produce the output.
- **How-provenance** - tells us the process by which the elements of *why-provenance* caused the output to appear
- **Where-provenance** - narrows down *why-provenance* to input data elements that were copied or transformed to determine the output element value.

Since the [Engines](#engine) are responsible for all data transformations, it's also the Engine's responsibility to answer provenance queries.

There are many different ways to implement provenance:
- By statically analyzing the queries
- By inversing transformations
- By repeating the computations and logging the data used at every step
- By propagating provenance data through all computations

Depending on the language used by an [Engine](#engine) one approach may work better in one situation than the other, so we avoid prescribing the exact method to use but instead standardize the language used for provenance-related queries and responses.

See also:
- [Provenance in Databases: Why, How, and Where](http://homepages.inf.ed.ac.uk/jcheney/publications/provdbsurvey.pdf)
- [Engine Contract: Provenance Query](#provenance-query)

## Time
The system applies the idea of [bitemporal data modelling](https://en.wikipedia.org/wiki/Bitemporal_Modeling) to the event streams. It differentiates two kinds of time:
- [System time](#system-time) - tells us when some event was observed by the system
- [Event time](#event-time) - tells when some event occurred from the perspective of the outside world

Every record in the system has exactly one system time associated with it upon the ingestion but can have zero to many event times.

### System Time
System time gives us a reference point for when something has occurred from the perspective of the system.

[Projecting](#projection) the data onto the system time axis answers the question: *"what did the system know at the time T?"*, meaning that such projections effectively freeze data in time, providing the natural way to achieve **reproducibility**.

For all intents and purposes system time is treated as **ordered monotonically non-decreasing value** that lets us establish a *before-after* relationship between certain events. Note, however, that *before-after* relationship is only meaningful for data within one [Dataset](#dataset) and its upstream dependencies. System time cannot be used to establish an exact *before-after* relationship between the events of the independent datasets.

### Event Time
Event time tells us when something happened from the outside world's perspective. This time, therefore, is usually the most useful one for querying and joining data.

There are no restrictions on the event time in the system - there can be many event times associated with any record, and unlike system time, event times don't have to be monotonic. This allows the system to support many kinds and varieties of event time use, like:
- Post-dated events and predictions - with event time set into the future
- Back-dated events and corrections - with event time set into the past

Depending on the type of transformations these restrictions may be more strict, e.g. joining datasets based on event time may require it to be quasi-monotonic increasing to know when the join operation can be considered complete (see [Watermarks](#watermark)).

See also:
* [Projections](#projection)
* [Watermarks](#watermark)
* [Streaming 101 by Tyler Akidau](https://www.oreilly.com/radar/the-world-beyond-batch-streaming-101/)
* [The Dataflow Model  by Tyler Akidau et al](https://static.googleusercontent.com/media/research.google.com/en//pubs/archive/43864.pdf)

## Watermark
A watermark defines the point in [Event Time](#event-time) for which with a high probability we've already observed all preceding events.

![Diagram: Watermarks in the Stream](images/watermarks_in_stream.svg)

When performing time-based windowed operation, aggregations, or joins it is important to know when a certain time window can be considered closed. Watermark tells the system "You most likely will not get event with time less than `T` any more".

In the [Root Dataset](#root-dataset) events can still arrive even after their time interval has been already been closed by the watermark. Such events are considered "late" and it's up to the individual [Queries](#query) to decide how to handle them. They can be simply ignored, emitted into a side output, or still considered by emitting the "correction" event into the output.

Watermarks in the system are defined per every [Metadata Block](#metadata-chain). By default the [Root Dataset](#root-dataset) will assign the watermark to the maximum observed [Event Time](#event-time) in the [Data Slice](#data-slice). You can and should override this behavior if you expect events to arrive out-of-order to some degree, e.g. offsetting the watermark by `1 day` prior to last seen event.

![Diagram: Watermarks in Time Domains](images/watermarks_vs_time.svg)

Watermarks can also be set based on the [System Time](#system-time) manually or semi-automatically. This is valuable for the slow moving [Datasets](#dataset) where it's normal not to see any events in days or even months. Setting the watermark explicitly allows all computations based on such stream to proceed, knowing that there were no events for that time period, where otherwise the output would be stalled assuming the [Dataset](#dataset) was not updated for a while and old data can still arrive.

## Remote
Remotes let participants of the system to exchange [Datasets](#dataset).

Remote definition includes:
- Location where the remote can be reached (URL)
- Protocols that it supports
- Credentials needed to access it
- Any necessary protocol-specific configuration

In the most basic form, a Remote can simply be a location where the dataset files are hosted over one of the [supported](#supported-protocols) file or object-based data transfer protocols. The owner of a dataset will have push privileges to this location, while other participants can pull data from it.

An advanced remote can support more functionality like:
- Push data API for publishers
- Subscription API for consumers
- Query API for making use of remote's compute resources and reducing the amount of transferred data

See also:
- [Remote Contract](#remote-contract)

## Projection
In relational algebra, a [projection](https://en.wikipedia.org/wiki/Projection_(relational_algebra)) is an operation that removes one or many dimensions from a data tuple. In the context of our system the most common projections are *temporal projections* involving the [System Time](#system-time) and [Event Time](#event-time) dimensions.

Depending on the time axis, we arrive at two most important types of projections in [bitemporal data modelling](https://en.wikipedia.org/wiki/Bitemporal_Modeling):
- **AS AT** or *"As we knew at that time"*. This projection collapses the system time dimension and shows us what the state of the data was at that time  to the best knowledge of the system.
- **AS OF** or *“As we should've known at the time”*. This projection collapses the event time dimension and shows us what **should've happened** at that time if we knew about all potential corrections and compensating events that were added since then.

Understanding the difference between these projections is essential when working with time series data and for achieving the reproducibility of results.

See also:
- [A Brief History of Time in Data Modelling: OLAP Systems](https://www.kamu.dev/blog/a-brief-history-of-time-in-data-modelling-olap-systems/)

# Specification

## Dataset Identity
Identity formats described below are used to unambiguously refer to a certain dataset. Depending on the context we differentiate the following formats of dataset identity:
- **Local Format** - used to refer to a dataset within a local workspace
- **Remote Format** - used to refer to a dataset located in a known [Remote](#remote) provider.
- **Remote Multi-Tenant Format** - used to refer to a dataset located in a known [Remote](#remote) provider and belonging to a particular tenant of that provider.

As you will see in the examples below, we recommend (but not require) using the [reverse domain name notation](https://en.wikipedia.org/wiki/Reverse_domain_name_notation) for [Dataset](#dataset) names - this style has proven to be easy to work with, carries an additional value in identifying the authoritative source of the information or the entity that maintains the [Derivative Dataset](#derivative-dataset), and it largely excludes the possibility of name collisions when working with similarly named datasets from different providers.

Examples:
<pre>
// Local Format - DatasetID is highlighted
<b>ca.vancouver.opendata.property.parcel-polygons</b>
<b>org.geonames.cities</b>
<b>com.naturalearthdata.admin0.countries.10m</b>

// Remote Format - RemoteID prefix is highlighted
<b>statcan.gc.ca/</b>ca.gc.statcan.census.2016.population
<b>nyc.gov/</b>us.cityofnewyork.public-safety.ems-incident-dispatch

// Remote Multi-tenant Format - Username infix is highlighted
opendata.ca/<b>statcan</b>/ca.gc.statcan.census.2016.population
data.gov/<b>ny-newyork</b>/us.cityofnewyork.public-safety.ems-incident-dispatch
</pre>

Full [PEG](https://en.wikipedia.org/wiki/Parsing_expression_grammar) grammar:
```
DatasetRef = (RemoteID "/")? (Username "/")? DatasetID
DatasetID = Hostname

RemoteID = Hostname
Username = Subdomain

Hostname = Subdomain ("." Subdomain)*
Subdomain = [a-zA-Z0-9]+ ("-" [a-zA-Z0-9]+)*
```

## Data Format
We differentiate two data formats by their purpose:
- **In-memory format** - used when passing data around between the subsystems (e.g. when [Coordinator](#coordinator) communicates with the [Engine](#engine)).
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

## Schema Format
We chose SQL-like [DDL](https://en.wikipedia.org/wiki/Data_definition_language) syntax for defining data [Schemas](#schema), as it:
- Operates with logical types, abstracting the physical data layout from the user
- Widely familiar
- Has high interoperability with modern data processing systems

Example:
```
registration_time TIMESTAMP(3),
registration_id UUID,
email STRING,
first_name STRING,
last_name STRING,
date_of_birth DATE,
```

Supported types:

|    DDL Type    |                                        Parquet Type                                        |
| :------------: | :----------------------------------------------------------------------------------------: |
|   `BOOLEAN`    |                                         `boolean`                                          |
|     `INT`      |                                          `int32`                                           |
|    `BIGINT`    |                                          `int64`                                           |
| `DECIMAL(p,s)` |  see [spec](https://github.com/apache/parquet-format/blob/master/LogicalTypes.md#decimal)  |
|    `FLOAT`     |                                          `float`                                           |
|    `DOUBLE`    |                                          `double`                                          |
|     `UUID`     |                             `fixed_len_byte_array(16) (UUID)`                              |
|    `STRING`    |                                      `binary (UTF8)`                                       |
| `TIMESTAMP(p)` | see [spec](https://github.com/apache/parquet-format/blob/master/LogicalTypes.md#timestamp) |
|     `DATE`     |                                       `int32 (DATE)`                                       |
|     `TIME`     |   see [spec](https://github.com/apache/parquet-format/blob/master/LogicalTypes.md#time)    |

> **TODO:**
> - Standardize DDL for nested data structures (nested data support highly varies between vendors)
> - Investigate support for lists/arrays
> - Investigate support for schema-less data (JSON/BSON)
> - `VARCHAR`

## Common Data Schema
All data in the system is guaranteed to have the following columns:

|    Column     |            Type             | Description                                                                                                                                                                                                                             |
| :-----------: | :-------------------------: | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `system_time` |       `TIMESTAMP(6)`        | [System Time](#system-time) denotes when an event first appeared in the dataset. This will be an ingestion time for events in the [Root Dataset](#root-dataset) or transformation time in the [Derivative Dataset](#derivative-dataset) |
| `event_time`  | `TIMESTAMP(3..6)` or `DATE` | [Event Time](#event-time) denotes when to our best knowledge an event has ocurred in the real world. This time is used for most time-based windowed computations, aggregations, and joins                                               |

## Metadata Format
The requirements we put towards the metadata format are:
- Compactness
- Fast read performance
- Complex type support (particularly nested structures and unions)
- Allows for forward and backward compatibility
- Allows for a controlled way of making breaking changes
- Possibility to read and write in human-friendly easily editable format
- Schema-based validation for user-specified data

We use the following combination of formats to satisfy these requirements:
- [YAML](https://yaml.org/) is used for human-friendly input and output
  - This format is concise and very readable and widely supported
  - Only a [JSON-compatible](https://json.org/) subset of `YAML` will be used
- [JSON Schema](https://json-schema.org/) may be used for validating user-specified `YAML` files
  - Widely supported and expressive grammar for validating `JSON` and `YAML`
  - Generates readable and easy to understand errors
- [FlatBuffers](https://google.github.io/flatbuffers/) format is used for binary representation
  - Highly efficient and cross-platform serialization format

The `JSON Schemas` and `FlatBuffers Schemas` for all metadata objects are provided as part of this specification (see [Metadata Reference](#metadata-reference)).

## Dataset Layout
The recommended layout of the dataset on disk is:

![Diagram: Dataset Layout](images/dataset_layout.svg)

This layout must be used when sharing datasets via file or object-based [Remotes](#remote) (e.g. local FS, S3, IPFS, DAT...).

When a [Dataset](#dataset) is imported locally, the exact layout is left entirely up to the [Coordinator](#coordinator) implementation, as we expect all interactions with the [Dataset](#dataset) to go through it.

See Also:
- [Dataset Sharing](#dataset-sharing)

## Engine Contract
This section provides the details on the contract between an [Engine](#engine) and the [Coordinator](#coordinator).

### Execution Model
[Engine](#engine) executes in a very restricted environment (a "sandbox") that prevents it from accessing any external resources other than those explicitly provided by the [Coordinator](#coordinator). Isolating the [Engine](#engine) from making network calls and accessing any random files ensures both that the [Data](#data) being processed cannot be leaked, and that non-deterministic behavior is harder to introduce accidentally.

Our isolation technology of choice is [Docker](https://www.docker.com/) because:
- It's currently the most popular containerization tool
- It supports most major operating systems
- [Docker Hub](https://hub.docker.com/) provides a convenient way to reliably distribute the images

> **TODO:** Add exact `docker` commands to reproduce the sandboxed environment

### Communication Interface
This section describes the [RPC](https://en.wikipedia.org/wiki/Remote_procedure_call) mechanism used in communications between the [Coordinator](#coordinator) and the [Engine](#engine).

The requirements we put towards the RPC mechanism are:
- Use common network protocols
- Wide support in many programming languages
- Support streaming responses (e.g. for streaming back the operation status and logs)

Our RPC technology of choice is [gRPC](https://grpc.io/) because:
- It's cross-platform, mature, and available in many languages
- Natively supports [FlatBuffers](https://google.github.io/flatbuffers/flatbuffers_grpc_guide_use_cpp.html) which is our [metadata format](#metadata-format) of choice, letting us reuse object schemas
- Supports streaming
- For our narrow use case, its convenience is more important than interoperability, as we don't expect to have many server and client implementations

[Engine](#engine) implementations must expose the `gRPC` API on port `5544`.

> Note: When starting an [Engine](#engine) container, the [Coordinator](#coordinator) will consider it fully initialized when it can establish a TCP connection to the port `5544`.

#### Data Exchange
The exchange of raw data happens out-of-band of the `gRPC` API. Input and output [Data Slices](#data-slice) are exchanged between the [Coordinator](#coordinator) and the [Engine](#engine) using the memory-mapped files (`mmap`) containing data in the common [in-memory format](#data-format). This avoids any intermediate IO and minimizes the copying of data for the best performance.

> **TODO:*
> - Needs more details on accessing input data and writing output

### Operations
Engine implementation should support the following operations:
- [Validate query](#validate-query) - Validates the user-specified query for basic syntax and schema correctness.
- [Execute query](#execute-query) - Performs the next iteration of the transformation.
- [Migrate query](#validate-query) - Updates the transformation state from one query to another.
- [Derive Provenance](#derive-provenance) - Explains the origin of some data produced in the past.

See also:
- [Engine API Reference](#engine-api-reference)

#### Validate Query
This operation may be used by the [Coordinator](#coordinator) when creating a new [Derivative Dataset](#derivative-dataset) or when changing the [Query](#query) of an existing one to validate the user-specified query for basic syntax and schema correctness.

Inputs:
- [Schemas](#schema) of the input [Datasets](#dataset)
- [Query](#query)
- (optional) Previous [Query](#query) in case of an existing [Dataset](#dataset) modification

Outputs:
- [Schema](#schema) of the result
- (alternatively) Validation errors

#### Execute Query
This operation is used by the [Coordinator](#coordinator) to perform the next iteration of the transformation.

The [Coordinator](#coordinator) is designed to isolate the [Engines](#engine) from the complexity of managing the [Metadata Chain](#metadata-chain) and make processing look as close as possible to a regular stream processing.

![Sequence Diagram: Execute Query](images/engine_transform.svg)

Inputs:
- Transaction ID
- Input [Data Slices](#data-slice)
- Input [Watermakrs](#watermark)
- [Query](#query)
- Previous [Checkpoint](#checkpoint)

Outputs:
- Output [Data Slice](#data-slice)
- Output [Watermark](#watermark)
- New [Checkpoint](#checkpoint)
- Operation progress, status, and logs

This operation must be [idempotent](https://en.wikipedia.org/wiki/Idempotence) (see [Implementing Exactly-Once Semantics](#implementing-exactly-once-semantics)).

When an operation is committed by the [Coordinator](#coordinator), the [Engine](#engine) will not see the same input data again. If due to the nature of the query (e.g. windowing or watermarking configuration) [Engine](#engine) cannot fully process and discard the input data - it should use the [Checkpoint](#checkpoint) directory to buffer it.

##### Implementing Exactly-Once Semantics
Because the [Engine](#engine) passes back the resulting data in memory (see [Data Exchange](#data-exchange)) it is possible that the [Coordinator](#coordinator) will crash before fully committing it. This may result in an inconsistent state where the [Engine](#engine) has already updated the persistent [Checkpoint](#checkpoint) data on disk, but the result was lost.

To support the ["exactly-once" semantics](#transactional-semantics) the [Coordinator](#coordinator) includes the `Transaction ID` parameter that uniquely identifies the processing step being performed. The [Engine](#engine) should maintain its [Checkpoint](#checkpoint) data in a way that permits re-running the last transaction multiple times while producing the same result (i.e. recovery by rolling forward).

Only the last processing step can be retried like this using the same exact input parameters. Upon seeing a different `Transaction ID` the [Engine](#engine) is free to clean up whatever [Checkpoint](#checkpoint) state it used to implement the idempotence.

#### Migrate Query
This operation is used by the [Coordinator](#coordinator) when data processing hits the point where one transformation [Query](#query) is being replaced by another one. It gives the [Engine](#engine) all information needed to handle the transition as gracefully as possible, e.g. by reconciling the existing [Checkpoints](#checkpoint) with the new [Query](#query), or, at the very least, finalizing the processing of the old [Query](#query) and clearing the [Checkpoints](#checkpoint) before the new one starts to execute.

Inputs:
- Transaction ID
- Previous [Query](#query)
- Next [Query](#query)
- Previous [Checkpoint](#checkpoint)

Outputs:
- New [Checkpoint](#checkpoint)
- (optional) [Data](#data) that has been produced when finalizing the old [Query](#query)

#### Derive Provenance
This operations is used to trace back a set of events in the output [Dataset](#dataset) to input events that contributed to their values or their existence (see [Provenance](#provenance)).

> **TODO:** The design of this operation is in progress.

### Engine Versioning
As [described previously](#components-of-trust), to guarantee the reproducibility and verifiability of the results a transformation must be associated with an exact version of an [Engine](#engine) that is used to perform it. We want to exclude any possibility that the code changes in the [Engine](#engine) will break this guarantee.

Whenever the [Coordinator](#coordinator) uses an [Engine](#engine) to execute a query it must specify the full digest of the Docker image in the resulting [Metadata Block](#metadata-chain). [Engine](#engine) maintainers are therefore responsible for treating the images as immutable and ensure old versions are never lost.

See also:
- [Engine Deprecation](#engine-deprecation)

## Coordinator Contract
The main functions of the [Coordinator](#coordinator) are:
- Maintain the [invariants](#requirements) and the [semantics](#transactional-semantics) of the system
- Guarantee the validity and integrity of the [Metadata](#metadata-chain)
- Provide means for ingesting external data into the system
- Interface with the [Engines](#engine) to implement data transformations
- Enable data sharing functionality via [Remotes](#remote)
- Provide additional functionality to simplify working with [Datasets](#dataset) and their metadata

### Common Metadata Operations
This section describes the operations performed by the [Coordinator](#coordinator) as part of the usual [Metadata Chain](#metadata-chain) maintenance.

#### Data Hashing
The procedure for calculating the stable [Hash](#hash) of a [Data Slice](#data-slice) is:

1. Drop the `system_time` column
2. Sort all rows based on the `event_time` column
3. Convert rows to the canonical string representation
4. Calculate [SHA3-256](https://en.wikipedia.org/wiki/SHA-3) digest of the file

> **TODO:** This is a stop-gap implementation. Release version of the hashing function should have a streaming nature while also be tolerant of row reordering, as many processing engine are concurrent and don't enforce ordering between outputs of independent calculations.

#### Metadata Block Hashing

Metadata structures that are part of the [MetadataBlock](#metadata-chain) are hashed as follows:

1. Iterate over block fields in depth-first order
2. Convert fields into their canonical representation
3. Feed data into [SHA3-256](https://en.wikipedia.org/wiki/SHA-3) digest algorithm.

> **TODO:** Add details on format-agnostic hashing

### Data Ingestion
It should be made clear that it's not the goal of this document to standardize the data ingestion techniques. Information here is given only to illustrate *how* new data can be continuously added into the system in alignment with the properties we want to see as a result.

The process of ingestion itself plays a very limited role in the system. When interacting with external data we cannot make any assumptions about the guarantees that the external source provides - we cannot rely on the external data being immutable, on being highly-available, we can't even be sure the domain that hosts it will still be there the next day. So the ingestion step is all about getting the data into the system as fast as possible, where all its properties can be preserved.

The reason we include the ingest configuration in this document at all is that we see it as an important part of the data [Provenance](#provenance).

#### Pull vs. Push
Although we aspire to reach a state where all authoritative data publishers **push** new events into the system as soon as those occur, we realize that this level of integration is hard to achieve. We believe that for a long time the vast majority of data will be ingested via the **pull** model, where the system periodically scans a remote data source and ingests the latest updates.

#### Ingestion Phases
Ingestion is composed of the following phases:
- **Fetch phase** - Obtains the data from some external source (e.g. HTTP/FTP) in its original raw form.
- **Prepare phase** (optional) - Transforms the raw data into one of the supported formats. This can include extracting an archive, filtering and rearranging files, using external tools to convert between formats.
- **Read phase** - Reads the data into a structured form.
- **Preprocess phase** (optional) - Shapes the data into the presentable form. This can include renaming columns, type conversions, nesting, etc.
- **Merge phase** - Combines the new data from the source with the history of previously seen data.

#### Merge Strategies
[Merge Strategies](#merge-strategy) deserve special attention as they bridge the gap between the [wide variety of data](#nature-of-data) in the external world with the strict event-based world of our system.

##### Append Merge Strategy
Under this strategy, the new data will be appended to the [Dataset](#dataset) in its original form without any modifications.

##### Ledger Merge Strategy
This strategy should be used for data sources containing append-only event streams. New data exports can have new rows added, but once data already made it into one export it should never change or disappear. A user-specified primary key is used to identify which events were already seen, not to duplicate them.

##### Snapshot Merge Strategy
This strategy can be used for data exports that are taken periodically and contain only the latest state snapshot of the observed entity or system. Over time such exports can have new rows added, and old rows either removed or modified.

This strategy transforms snapshot data into an append-only event stream by performing the [change data capture](https://en.wikipedia.org/wiki/Change_data_capture). It relies on a user-specified primary key to correlate the rows between the two snapshots. A new event is added into the output stream whenever:

- A row with a certain primary key appears for the first time
- A row with a certain key disappears from the snapshot
- A row data associated with a certain key has changed

Each event will have an additional column that signifies the kind of observation that was encountered.

The `Snapshot` strategy also requires special treatment in regards to the [Event Time](#event-time). Since snapshot-style data exports represent the state of some system at a certain time - it is important to know what that time was. This time is usually captured in some form of metadata (e.g. in the name of the snapshot file, in the URL, or the HTTP caching headers. It should be possible to extract and propagate this time into a data column.

### Derivative Transformations
[Previously](#execute-query) we looked at how an [Engine](#engine) executes the transformation query. Here we will look at what happens on the [Coordinator](#coordinator) side.

![Sequence Diagram: Execute Query](images/engine_transform.svg)

Here are the steps that the [Coordinator](#coordinator) performs during the transformation:
- **Batch step** - Analyze the [Metadata Chains](#metadata-chain) of the [Dataset](#dataset) being transformed and all of the inputs. The goal here is to decide how far the processing can progress before hitting one of the special conditions, such as a change of schema in one of the inputs or a change of the transformation query.
- **Run migrations** (when needed) - If a special condition is encountered - call the [Engine's](#engine) [Migrate Query](#migrate-query) operation to make necessary adjustments for the new transformation parameters.
- **Run query** - Pass the input [Data Slices](#data-slice) into the [Engine's](#engine) [Execute Query](#execute-query) operation
- **Hash resulting data** - Obtain a stable [Hash](#hash) of the output [Data Slice](#data-slice) (see [Data Hashing](#data-hashing))
- **Prepare commit** - Creates the next [Metadata Block](#metadata-chain) describing the output data
- **Commit** - Atomically adds the new [Data Slice](#data-slice) and the [Metadata Block](#metadata-chain) to the [Dataset](#dataset)

### Dataset Evolution
Most parts of the [Dataset](#dataset) can [change over time](#evolution-of-data). To protect downstream consumers of data from braking changes the [Coordinator](#coordinator) has to validate the changes for backward compatibility before accepting them. This section covers the allowed transitions for different parts of the [Metadata](#metadata-chain).

#### Schema Evolution
[Dataset](#dataset) [Schema](#schema) may change when:
- A different [Ingestion](#ingestion) process is specified in the [Root Dataset](#root-dataset)
- A transformation [Query](#query) of the [Derivative Dataset](#derivative-dataset) is changed
- A [Schema](#schema) of one of the inputs of the [Query](#query) changes

In general, [Schema](#schema) changes are restricted to only adding new columns. Removing and renaming columns or changing data types would break downstream data consumers and therefore not allowed. A migration procedure should be used instead when it's desired to introduce breaking changes in the [Schema](#schema).

#### Query Evolution
The procedure for modifying the [Query](#query) of a [Derivative Dataset](#derivative-dataset) is as following:
1. The [Coordinator](#coordinator) invokes the [Engine's](#engine) [Validate Query](#validate-query) operation to validate the new [Query](#query) and determine the validity of the transition according to the engine-specific rules.
2. The new [Schema](#schema) returned in the previous step is validated according to the [Schema Evolution](#schema-evolution) rules.
3. A new [Metadata Block](#metadata-chain) is created and committed.

> **TODO**: Specify how Schema changes in the upstream datasets should be handled
> - Do we stop processing and ask user to validate that the query takes into account the new columns?

### Dataset Sharing
Dataset sharing involves uploading the data to some [Remote](#remote) where it can be discovered and accessed by other peers. Due to immutable nature of data and metadata it is very easy to keep shared data up-to-date as only new blocks and part files need to be uploaded.

It is important to ensure the atomicity of the sharing, or at least create the perception of atomicity to someone who happens to be downloading the data concurrently with the upload.

For [Remotes](#remote) that do not support atomic file/object operations the following sequence of operations can ensure that concurrent downloads will always see the [Dataset](#dataset) in a consistent state:
1. Upload data part files
2. Upload metadata blocks
3. Update references in `metadata/refs` directory

#### Dataset Validation
The [Derivative Dataset](#derivative-dataset) validation confirms that the event data is in fact produced by applying all the transformations stored in the [Metadata](#metadata-chain) to the inputs and was not maliciously altered.

The process of validation is almost identical to the [Derivative Transformation](#derivative-transformations) except instead of the `Commit` phase the [Coordinator](#coordinator) compares the output data hash of the local transform to the hash stored in the [Metadata](#metadata-chain).

### Engine Deprecation
Pinning the exact [Engine](#engine) version used by every transformation guarantees the reproducibility and verifiability of the results (see [Components of Trust](#components-of-trust)). In the long run, however, this creates a problem where in order to [validate a dataset](#dataset-validation) the [Coordinator](#coordinator) will have to use a very large number of different versions of an [Engine](#engine) that might've accumulated over the years. This slows down the validation procedure and can result in a significant disk space required for storing these images.

To mitigate this problem the [Coordinator](#coordinator) offers the **engine deprecation** operation that works as follows:
1. The [Coordinator](#coordinator) reconstructs the dataset from scratch by repeating all transformation in the [Metadata Chain](#metadata-chain).
2. Instead of the [Engine](#engine) version specified in the block, it uses the latest (or provided) version.
3. If the results produced in every step match the recorded ones - we can safely consider that the newer version of the [Engine](#engine) is fully compatible with the older one.
4. A special [Metadata](#metadata-chain) record is created to record this compatibility, therefore, allowing all users to use the latest version of the [Engine](#engine) instead of downloading all the versions seen previously.

## Remote Contract
> **TODO:**
> - Supported protocols
> - Querying data in advanced remote

## Future Topics

### Anonymization
- Portal Dataset
  - Exposes any dataset as root without disclosing details

# Reference Information

## Metadata Reference

![](build/schemas/Manifest.md)
![](build/schemas/MetadataBlock.md)
![](build/schemas/DataSlice.md)
![](build/schemas/DatasetSource.md)
![](build/schemas/Transform.md)
![](build/schemas/FetchStep.md)
![](build/schemas/EventTimeSource.md)
![](build/schemas/SourceCaching.md)
![](build/schemas/PrepStep.md)
![](build/schemas/ReadStep.md)
![](build/schemas/MergeStrategy.md)
![](build/schemas/DatasetSnapshot.md)
![](build/schemas/DatasetVocabulary.md)

## Engine API Reference
> **TODO**: Provide `gRPC + FlatBuffers` IDL

## Remote API Reference
> **TODO**: Provide `gRPC + FlatBuffers` IDL

