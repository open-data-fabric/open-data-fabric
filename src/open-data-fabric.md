# Open Data Fabric

Version: 0.41.0

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
- **Complete historical account** - when data is used to gain insight and to drive decision-making discarding or modifying data is akin to rewriting history. The history of all data observed by the system must be preserved.
- **Reproducibility** - the ability to reproduce the results is a cornerstone of the scientific method without which the process and findings of one party cannot be verified by others. Therefore, we require that all the transformations performed within the system must be fully reproducible, and it must be possible to get a reference to data that is frozen in time and never changes to achieve reproducibility of any process that uses it.
- **Verifiability** - any party that plans to use some data must be able to verify its validity.
- **Provenance** - regardless of how many transformation stages the data went through, it should be possible to trace any individual data cell back to its ultimate source and understand which data contributed to its existence and its value.

Non-Functional:
- **Transparency** - the method itself should be developed in the open, easy to understand, and reason about.
- **Timeliness** - the method should aim to minimize the propagation time of data through the transformation pipeline. The frequency with which data is presented to consumers should be optimized for their experience and usability, not dictated by limitations of the data pipeline.
- **Transactionality** - once data is in the system all operations should have [ACID properties](https://en.wikipedia.org/wiki/ACID) and exactly-once semantics.
- **Decentralization** - there must be no entity in the system with any kind of special privileges over data, or abilities to circumvent any of the above requirements.

# Scope
The primary focus of this specification is the **mission-critical data** such as:
- Business performance data (sales, CRM)
- Governmental data
- Financial data
- Healthcare data
- Scientific data
- As well as any other data used for decision-making

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
The state-of-the-art approach to transforming data today is to version the source data (using a hash sum or a stable reference) and version the code that transforms it (using a version control system). The result of transformations is then uploaded to some shared storage and made available to others. There are many tools that improve the reproducibility of such workflows, but all of them treat data as a mere binary blob, deprived of its intrinsic qualities.

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
- Transformations are expressed using [Queries](specs/glossary.md#query) written in any of the supported languages.
- Queries combine and transform one or many input data streams into an output stream.
- Queries are executed by the [Engines](specs/glossary.md#engine) - a software that knows how to interpret and execute a certain query dialect.
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

While for simplicity we previously considered derivative datasets to be only defined by one transformation query, in fact it can comprise multiple queries that were active during different periods of time. The metadata keeps track of which parts of inputs were processed by which query and which parts of output were produced as the result, ensuring reproducibility. This idea will be further explored when we discuss the [Metadata Chain](specs/glossary.md#metadata-chain), but for now, get used to the idea that **almost anything that defines a dataset can change over time**.

## Data Sharing
The guiding factors in designing how data is shared were:
- Minimal storage requirements - while a certain degree of replication is necessary to keep data highly available, we don't want to duplicate data more than it's necessary.
- Decentralization - there should be no central authority that decides what the "right data" is.
- Verifiable trust - even when data is hosted by a single party any tampering or alterations must still be impossible.
- Minimize the movement of data - when a trusted party keeps data in an environment that also provides compute resources, we would like to be able to process and query data without the need to always download it locally.

The way we designed [Metadata](#essential-role-of-metadata) plays a crucial role in data sharing.

Metadata is usually several orders of magnitude smaller than the data it describes, so it's ideally suited to be widely shared. The exact way it's shared is out of scope of this document, but in most simple cases it can be published openly by the provider of data and cryptographically signed to ensure authenticity.

When the metadata of a certain dataset is reliably known, a peer can then download the data from any untrusted source and use the metadata validate the authenticity of every data slice that composes it (see [Hash Function](specs/glossary.md#hash)) - this means that we need to only ensure the validity of metadata, while the data can be stored almost anywhere with minimal replication factor, as long as one compromised party does not result in the complete loss of data.

Metadata also provides us with a way to establish the trustworthiness of any dataset by reviewing the transformations declared in it, re-applying those transformations in a trusted environment, and comparing the results to the original data. In a distributed system, having peers cross-validate each other's published data can guarantee trusted results and allow them to promptly identify and exclude malicious peers from the network.

## Transactional Semantics
To achieve a perfect reproducibility the system needs to satisfy very strong transactional properties:
- All coordination operations including things like ingesting, downloading, and sharing data, creating and updating datasets must have [ACID properties](https://en.wikipedia.org/wiki/ACID).
- All data transformations must have **exactly-once** semantics.

# Specification

## Dataset Identity
The identity of a dataset consists of:
- [Unique identifiers](#unique-identitifiers) - used to unambiguously identify datasets on the network, e.g. when referencing one dataset as an input of another.
- [Aliases and references](#aliases-and-references) - used for human-friendly naming.

### Unique Identifiers
ODF is designed to be compatible with [content-addressable storage](https://en.wikipedia.org/wiki/Content-addressable_storage). When stored in such storage, all parts of a [Dataset](specs/glossary.md#dataset) can be accessed by having only a hash of the last [MetadataBlock](specs/glossary.md#metadata-chain).

As [Dataset](specs/glossary.md#dataset) grows, however, a reference to a single [MetadataBlock](specs/glossary.md#metadata-chain) will only provide access to a subset of its history. To refer to a [Dataset](specs/glossary.md#dataset) as a whole a different identifier is needed that can be resolved into a hash of the latest block. Such identifiers must be globally unique, issuable with no central authority, and tied to a proof of ownership over a dataset.

ODF follows the [W3C DID Identity Scheme](https://w3c.github.io/did-core/) using a custom `did:odf` method. This method is based closely on [`did:key` method](https://w3c-ccg.github.io/did-method-key/) that derives the unique identity from a public key of a cryptographic key pair.

Example:

<pre>
did:odf:fed012126262ba49e1ba8392c26f7a39e1ba8d756c7469786d3365200c68402ff65dc
</pre>

The identifier is formed by:
- Generating a cryptographic key pair (e.g. using `ed25519` algorithm)
- Taking the public key part
- Prepending an appropriate [multicodec](https://github.com/multiformats/multicodec) value to identify the algorithm (e.g. `ed25519-pub`)
- Encoding data with [multibase](https://github.com/multiformats/multibase) encoding using `base16` scheme

Or in pseudocode:

<pre>
did-odf-format := 'did:odf:' + MULTIBASE(
  base16,
  MULTICODEC(
    public_key_type,
    public_key_bytes,
  )
)
</pre>

The resulting DID is stored in the first [MetadataBlock](specs/glossary.md#metadata-chain) in the chain of every [Dataset](specs/glossary.md#dataset), called [Seed](#seed-schema).

Tying the identity of a dataset to a cryptographic key pair provides a way to create unique identity in a fully decentralized way. The corresponding private key can be used to prove ownership and control over a [Dataset](specs/glossary.md#dataset) and to delegate access.

> Note that the only difference between `did:odf` and `did:key` method is the encoding scheme. ODF uses `base16` instead of `base58btc` for reasons explained in the [Hash Representation](#hash-representation) section. Because conversion between the two encodings is trivial, ODF is compatible with authorization frameworks that work with `did:key` method. We expect that in future the `did:key` spec will be extended to allow `base16` encoding too, further minimizing the difference.

See also:
- [RFC-003: Content Addressability](/rfcs/003-content-addressability.md)
- [RFC-012: Recommend base16 encoding](/rfcs/012-recommend-base16-encoding.md)

### Aliases and References
Formats described below provide human-friendly ways to refer to a certain dataset. Note that they are only meaningful within the boundaries of a [Repository](specs/glossary.md#repository). Unlike [Dataset IDs](#unique-identitifiers) they are not collision-free and mutable.

Depending on the context we differentiate the following types of references:
- **Local Format** - used to refer to a dataset within a local workspace (can be single- and multi-tenant)
- **Remote Format** - used to refer to a dataset located in a known [Repository](specs/glossary.md#repository) provider.

As you will see in the examples below, we recommend (but not require) using the [reverse domain name notation](https://en.wikipedia.org/wiki/Reverse_domain_name_notation) for [Dataset](specs/glossary.md#dataset) names.

Examples:
<pre>
// Dataset ID
did:odf:fed012126262ba49e1ba8392c26f7a39e1ba8d756c7469786d3365200c68402ff65dc

// Local Format - DatasetName is highlighted
<b>property.parcel-polygons</b>
<b>cities</b>
<b>admin0.countries.10m</b>

// Local Multi-tenant Format - AccountName is highlighted
<b>statcan.gc.ca</b>/census.2016.population
<b>ny-newyork</b>/public-safety.ems-incident-dispatch

// Remote Format - RepoName is highlighted
<b>statcan.gc.ca</b>/census.2016.population
<b>us.cityofnewyork</b>/public-safety.ems-incident-dispatch
<b>data.gov</b>/ny-newyork/public-safety.ems-incident-dispatch

// Remote Url Format
https://opendata.ca/odf/census-2016-population/
ipfs://bafkreie3hfshd4ikinnbio3kewo2hvj6doh5jp3p23iwk2evgo2un5g7km/
</pre>

Note that name components such as `DatasetName`, `AccountName`, `RepoName` are case-insensitive, i.e. names `ny-newyork` and `NY-NewYork` are considered identical during lookups and uniqueness checks within the repository.

Full [PEG](https://en.wikipedia.org/wiki/Parsing_expression_grammar) grammar:
```
DatasetRefAny = 
  DatasetRefRemote /
  DatasetRef

DatasetRef = 
  DatasetId /
  DatasetAlias

DatasetRefRemote =
  (RepoName "/")? DatasetId /
  DatasetAliasRemote /
  Url

DatasetAlias = 
  (AccountName "/")? DatasetName

DatasetAliasRemote = 
  RepoName "/" (AccountName "/")? DatasetName

DatasetId = "did:odf:" Multibase
DatasetName = Hostname
AccountName = Hostname
RepoName = Hostname

Hostname = Subdomain ("." Subdomain)*
Subdomain = [a-zA-Z0-9]+ ("-" [a-zA-Z0-9]+)*

Multibase = [a-zA-Z0-9+/=]+
Url = Scheme "://" [^\n]+
Scheme = [a-zA-Z0-9]+ ("+" [a-zA-Z0-9]+)*
```

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

See Also:
- [Metadata Block Hashing](#metadata-block-hashing)

> **TODO:*
> - Add a separate section specifying the serialization rules or point to codegen as the reference implementation

## Dataset Layout
The recommended layout of the dataset on disk is:

![Diagram: Dataset Layout](images/dataset_layout.svg)

This layout must be used when sharing datasets via file or object-based [Repositories](specs/glossary.md#repository) (e.g. local FS, S3, IPFS, DAT...).

When a [Dataset](specs/glossary.md#dataset) is imported locally, the exact layout is left entirely up to the [Coordinator](specs/glossary.md#coordinator) implementation, as we expect all interactions with the [Dataset](specs/glossary.md#dataset) to go through it.

See Also:
- [Dataset Sharing](#dataset-sharing)

## Engine Contract
This section provides the details on the contract between an [Engine](specs/glossary.md#engine) and the [Coordinator](specs/glossary.md#coordinator).

### Execution Model
[Engine](specs/glossary.md#engine) executes in a very restricted environment (a "sandbox") that prevents it from accessing any external resources other than those explicitly provided by the [Coordinator](specs/glossary.md#coordinator). Isolating the [Engine](specs/glossary.md#engine) from making network calls and accessing any random files ensures both that the [Data](specs/glossary.md#data) being processed cannot be leaked, and that non-deterministic behavior is harder to introduce accidentally.

Our isolation technology of choice is  [OCI containers](https://opencontainers.org/) - a lightweight mechanism that provides good performance and strong isolation guarantees. We rely on OCI-compatible images as the distribution mechanism for the engines.

> **TODO:** Add exact `podman` and `docker` commands to reproduce the sandboxed environment

### Communication Interface
This section describes the [RPC](https://en.wikipedia.org/wiki/Remote_procedure_call) mechanism used in communications between the [Coordinator](specs/glossary.md#coordinator) and the [Engine](specs/glossary.md#engine).

The requirements we put towards the RPC mechanism are:
- Use common network protocols
- Wide support in many programming languages
- Support streaming responses (e.g. for streaming back the operation status and logs)

Our RPC technology of choice is [gRPC](https://grpc.io/) because:
- It's cross-platform, mature, and available in many languages
- Natively supports [FlatBuffers](https://google.github.io/flatbuffers/flatbuffers_grpc_guide_use_cpp.html) which is our [metadata format](#metadata-format) of choice, letting us reuse object schemas
- Supports streaming
- For our narrow use case, its convenience is more important than interoperability, as we don't expect to have many server and client implementations

[Engine](specs/glossary.md#engine) implementations must expose the `gRPC` API on port `5544`.

> Note: When starting an [Engine](specs/glossary.md#engine) container, the [Coordinator](specs/glossary.md#coordinator) will consider it fully initialized when it can establish a TCP connection to the port `5544`.

#### Data Exchange
The exchange of raw data happens out-of-band of the `gRPC` API. Input and output [Data Slices](specs/glossary.md#data-slice) are exchanged between the [Coordinator](specs/glossary.md#coordinator) and the [Engine](specs/glossary.md#engine) using the memory-mapped files (`mmap`) containing data in the common [in-memory format](specs/datasets/data-format.md#in-memory-and-on-disk-formats). This avoids any intermediate IO and minimizes the copying of data for the best performance.

> **TODO:*
> - Needs more details on accessing input data and writing output

### Operations
Engine implementation should support the following operations:
- [Validate query](#validate-query) - Validates the user-specified query for basic syntax and schema correctness.
- [Execute query](#execute-query) - Performs the next iteration of the transformation.
- [Migrate query](#migrate-query) - Updates the transformation state from one query to another.
- [Derive Provenance](#derive-provenance) - Explains the origin of some data produced in the past.

See also:
- [Engine API Reference](#engine-api-reference)

#### Validate Query
This operation may be used by the [Coordinator](specs/glossary.md#coordinator) when creating a new [Derivative Dataset](specs/glossary.md#derivative-dataset) or when changing the [Query](specs/glossary.md#query) of an existing one to validate the user-specified query for basic syntax and schema correctness.

Inputs:
- [Schemas](specs/glossary.md#schema) of the input [Datasets](specs/glossary.md#dataset)
- [Query](specs/glossary.md#query)
- (optional) Previous [Query](specs/glossary.md#query) in case of an existing [Dataset](specs/glossary.md#dataset) modification

Outputs:
- [Schema](specs/glossary.md#schema) of the result
- (alternatively) Validation errors

#### Execute Query
This operation is used by the [Coordinator](specs/glossary.md#coordinator) to perform the next iteration of the transformation.

The [Coordinator](specs/glossary.md#coordinator) is designed to isolate the [Engines](specs/glossary.md#engine) from the complexity of managing the [Metadata Chain](specs/glossary.md#metadata-chain) and make processing look as close as possible to a regular stream processing.

![Sequence Diagram: Execute Query](images/engine_transform.svg)

Inputs:
- Transaction ID
- Input [Data Slices](specs/glossary.md#data-slice)
- Input [Watermarks](specs/glossary.md#watermark)
- [Query](specs/glossary.md#query)
- Previous [Checkpoint](specs/glossary.md#checkpoint)

Outputs:
- Output [Data Slice](specs/glossary.md#data-slice)
- Output [Watermark](specs/glossary.md#watermark)
- New [Checkpoint](specs/glossary.md#checkpoint)
- Operation progress, status, and logs

This operation must be [idempotent](https://en.wikipedia.org/wiki/Idempotence) (see [Implementing Exactly-Once Semantics](#implementing-exactly-once-semantics)).

When an operation is committed by the [Coordinator](specs/glossary.md#coordinator), the [Engine](specs/glossary.md#engine) will not see the same input data again. If due to the nature of the query (e.g. windowing or watermarking configuration) [Engine](specs/glossary.md#engine) cannot fully process and discard the input data - it should use the [Checkpoint](specs/glossary.md#checkpoint) to buffer it.

#### Migrate Query
This operation is used by the [Coordinator](specs/glossary.md#coordinator) when data processing hits the point where one transformation [Query](specs/glossary.md#query) is being replaced by another one. It gives the [Engine](specs/glossary.md#engine) all information needed to handle the transition as gracefully as possible, e.g. by reconciling the existing [Checkpoints](specs/glossary.md#checkpoint) with the new [Query](specs/glossary.md#query), or, at the very least, finalizing the processing of the old [Query](specs/glossary.md#query) and clearing the [Checkpoints](specs/glossary.md#checkpoint) before the new one starts to execute.

Inputs:
- Transaction ID
- Previous [Query](specs/glossary.md#query)
- Next [Query](specs/glossary.md#query)
- Previous [Checkpoint](specs/glossary.md#checkpoint)

Outputs:
- New [Checkpoint](specs/glossary.md#checkpoint)
- (optional) [Data](specs/glossary.md#data) that has been produced when finalizing the old [Query](specs/glossary.md#query)

#### Derive Provenance
This operation is used to trace back a set of events in the output [Dataset](specs/glossary.md#dataset) to input events that contributed to their values or their existence (see [Provenance](specs/glossary.md#provenance)).

> **TODO:** The design of this operation is in progress.

### Engine Versioning
As [described previously](#components-of-trust), to guarantee the reproducibility and verifiability of the results a transformation must be associated with an exact version of an [Engine](specs/glossary.md#engine) that is used to perform it. We want to exclude any possibility that the code changes in the [Engine](specs/glossary.md#engine) will break this guarantee.

Whenever the [Coordinator](specs/glossary.md#coordinator) uses an [Engine](specs/glossary.md#engine) to execute a query it must specify the full digest of the OCI container image in the resulting [Metadata Block](specs/glossary.md#metadata-chain). [Engine](specs/glossary.md#engine) maintainers are therefore responsible for treating the images as immutable and ensure old versions are never lost.

See also:
- [Engine Deprecation](#engine-deprecation)

## Coordinator Contract
The main functions of the [Coordinator](specs/glossary.md#coordinator) are:
- Maintain the [invariants](#requirements) and the [semantics](#transactional-semantics) of the system
- Guarantee the validity and integrity of the [Metadata](specs/glossary.md#metadata-chain)
- Provide means for ingesting external data into the system
- Interface with the [Engines](specs/glossary.md#engine) to implement data transformations
- Enable data sharing functionality via [Repositories](specs/glossary.md#repository)
- Provide additional functionality to simplify working with [Datasets](specs/glossary.md#dataset) and their metadata

### Common Metadata Operations
This section describes the operations performed by the [Coordinator](specs/glossary.md#coordinator) as part of the usual [Metadata Chain](specs/glossary.md#metadata-chain) maintenance.

#### Data Hashing

##### Physical and Logical Hashes
Because the on-disk Parquet data format is non-deterministic (serializing same logical data may result in different binary layouts depending on heuristics and implementation) two different hash sums are maintained by the coordinator:
- **Physical hash** - a (non-reproducible) hash sum of an entire binary file as originally produced by the owner of the dataset
- **Logical hash** - a stable (reproducible) hash sum computed on data records, resistant to encoding variations

**Physical hash** is used only in the context of uploading and downloading data files to/from content-addressable systems, where hash sum acts as an identifier of the data file. Physical hash is calculated using [SHA3-256](https://en.wikipedia.org/wiki/SHA-3) algorithm on entire data part file.

**Logical hash** is used for integrity and equivalence checks. We use a specially-designed algorithm [arrow-digest](https://github.com/sergiimk/arrow-digest) that computes a hash sum over in-memory representation of data in Apache Arrow format. Refer to the repository for algorithm details.

##### Hash Representation
To be able to evolve hashing algorithms over time we encode the identity of the algorithm used along with the hash sum itself using the [multiformats](https://github.com/multiformats/multiformats) specification, specifically:
- [multihash](https://github.com/multiformats/multihash) - describes how to encode algorithm ID alongside the hash value.
- [multibase](https://github.com/multiformats/multibase) - describes encoding used to represent a binary value as text (e.g. in YAML format or as a file name) without ambiguity of which encoding was used.

It is expected that the specific `multibase` encoding is largely a presentation-layer concern that can vary between implementations. For protocol compatibility all implementations must support all encodings in the `final` state of approval in the [`multibase` specification](https://github.com/multiformats/multibase), transcoding the representations when needed.

For consistency, we recommended implementations to prefer the `base16` encoding. Although it produces longer hashes than other encodings, it:
- Is case-insensitive and can appear in subdomains
- Does not contain symbols that can be easily confused
- Does not have a risk of forming an accidental obscenity
- Does not require padding
- Is easier to use for partitioning by prefix

For representing the identity of hashing algorithms the official [multicodec](https://github.com/multiformats/multicodec/) table is used.

The `multicodec` table is extended with the following codes in the "private use area":

| Codec             | Code       |
| ----------------- | ---------- |
| `arrow0-sha3-256` | `0x300016` |

See also:
- [RFC-002](/rfcs/002-logical-data-hashes.md)
- [RFC-012](/rfcs/012-recommend-base16-encoding.md)

#### Checkpoint Hashing
[Checkpoints](specs/glossary.md#checkpoint) are stored as opaque files and referenced by [Metadata Blocks](specs/glossary.md#metadata-chain) using their physical hash. The process of computing a hash sum is identical to computing a physical hash for a data part file (see [Data Hashing](#data-hashing)).

#### Metadata Block Hashing
Blocks of the [MetadataChain](specs/glossary.md#metadata-chain) are referred to and linked together using their cryptographic hashes. The process of serializing and computing a stable hash is as follows:

1. The [MetadataBlock](#metadatablock-schema) is serialized into [FlatBuffers](https://google.github.io/flatbuffers/) format following a two-step process to ensure that all variable-size buffers are laid out in memory in a consistent order:
   1. First, we iterate over all fields of the block in the same order they appear in the [schemas](#metadata-reference) serializing into buffers all vector-like and variable-size fields and recursing into nested data structures (tables) in the depth-first order.
   2. Second, we iterate over all fields again this time serializing all leftover fixed-size fields
2. Block content is then nested into a [Manifest](#manifest-schema) object using same serialization rules as above
3. The resulting `flatbuffer` data is fed into [SHA3-256](https://en.wikipedia.org/wiki/SHA-3) digest algorithm.

For use in the [Manifest](#manifest-schema)'s `kind` field the `multicodec` table is extended with the following codes in the "private use area":

| Codec                | Code       |
| -------------------- | ---------- |
| `odf-metadata-block` | `0x400000` |

The block hashes are represented using [multihash](https://github.com/multiformats/multihash) and [multibase](https://github.com/multiformats/multibase) described [above](#hash-representation).

### Data Ingestion
It is not the goal of this document to standardize the data ingestion techniques. This section exists only to illustrate *how* new data can be continuously added into the system in alignment with the properties we want to see in [Root Datasets](specs/glossary.md#root-dataset).

When interacting with data on the web we cannot make any assumptions about the guarantees the external source provides - we cannot tell if data is immutable, or if it's replicated for availability, we can't even be sure the domain that hosts it will still exist the next day. The ingestion step is about getting the data into the system as efficiently as possible, where all such properties can be guaranteed and made explicit by [Root Datasets](specs/glossary.md#root-dataset).

#### Source Types

##### Push Source
In the simplest scenario a [Root Dataset](specs/glossary.md#root-dataset) is created and periodically written to, i.e. data being "pushed" into it. This can happen in variety of forms like exporting data from external system as a CDC stream, or IoT device reporting measurements, or recording domain events that occur in a business process.

Push ingestion may support the following phases:
- **Read phase** - Reads the data in some format into a structured form.
- **Preprocess phase** (optional) - Reshapes data into another form. This can include renaming columns, type conversions, nesting, etc.
- **Merge phase** - Combines the new data with the history of previously seen data. This can include deduplication, CDC etc.

##### Polling Sources
Although we aspire to reach a state where all authoritative data publishers **push** new events into the datasets as soon as those occur, we realize that this level of integration will take time to achieve. We believe that for a long time the vast majority of data will be ingested via the **polling** model, where the system periodically scans a remote data source and ingests the latest updates.

Polling ingestion may support the following phases:
- **Fetch phase** - Obtains the data from some external source (e.g. HTTP/FTP) in its original raw form.
- **Prepare phase** (optional) - Transforms the raw data into one of the supported formats. This can include extracting an archive, filtering and rearranging files, using external tools to convert between formats.
- **Read phase** - Reads the data in some format into a structured form.
- **Preprocess phase** (optional) - Reshapes data into another form. This can include renaming columns, type conversions, nesting, etc.
- **Merge phase** - Combines the new data from the source with the history of previously seen data.

#### Merge Strategies
[Merge Strategies](specs/glossary.md#merge-strategy) deserve special attention as they bridge the gap between the [wide variety of data](#nature-of-data) in the external world with the strict event-based world of our system.

##### Append Merge Strategy
Under this strategy, the new data will be appended to the [Dataset](specs/glossary.md#dataset) in its original form without any modifications.

##### Ledger Merge Strategy
This strategy should be used for data sources containing append-only event streams. New data exports can have new rows added, but once data already made it into one export it should never change or disappear. A user-specified primary key is used to identify which events were already seen, not to duplicate them.

##### Snapshot Merge Strategy
This strategy can be used for data exports that are taken periodically and contain only the latest state snapshot of the observed entity or system. Over time such exports can have new rows added, and old rows either removed or modified.

This strategy transforms snapshot data into an append-only event stream by performing the [change data capture](https://en.wikipedia.org/wiki/Change_data_capture). It relies on a user-specified primary key to correlate the rows between the two snapshots. A new event is added into the output stream whenever:

- A row with a certain primary key appears for the first time
- A row with a certain key disappears from the snapshot
- A row data associated with a certain key has changed

Each event will have an additional column that signifies the kind of observation that was encountered.

The `Snapshot` strategy also requires special treatment in regard to the [Event Time](specs/glossary.md#event-time). Since snapshot-style data exports represent the state of some system at a certain time - it is important to know what that time was. This time is usually captured in some form of metadata (e.g. in the name of the snapshot file, in the URL, or the HTTP caching headers). It should be possible to extract and propagate this time into a data column.

### Derivative Transformations
[Previously](#execute-query) we looked at how an [Engine](specs/glossary.md#engine) executes the transformation query. Here we will look at what happens on the [Coordinator](specs/glossary.md#coordinator) side.

![Sequence Diagram: Execute Query](images/engine_transform.svg)

Here are the steps that the [Coordinator](specs/glossary.md#coordinator) performs during the transformation:
- **Batch step** - Analyze the [Metadata Chains](specs/glossary.md#metadata-chain) of the [Dataset](specs/glossary.md#dataset) being transformed and all the inputs. The goal here is to decide how far the processing can progress before hitting one of the special conditions, such as a change of schema in one of the inputs or a change of the transformation query.
- **Run migrations** (when needed) - If a special condition is encountered - call the [Engine's](specs/glossary.md#engine) [Migrate Query](#migrate-query) operation to make necessary adjustments for the new transformation parameters.
- **Run query** - Pass the input [Data Slices](specs/glossary.md#data-slice) into the [Engine's](specs/glossary.md#engine) [Execute Query](#execute-query) operation
- **Hash resulting data and checkpoint** - Obtain a stable [Hash](specs/glossary.md#hash) of the output [Data Slice](specs/glossary.md#data-slice) and [Checkpoint](specs/glossary.md#checkpoint) (see [Data Hashing](#data-hashing) and [Checkpoint Hashing](#checkpoint-hashing))
- **Prepare commit** - Creates the next [Metadata Block](specs/glossary.md#metadata-chain) describing the output data
- **Commit** - Atomically adds the new [Data Slice](specs/glossary.md#data-slice) and the [Metadata Block](specs/glossary.md#metadata-chain) to the [Dataset](specs/glossary.md#dataset)

### Dataset Evolution
Most parts of the [Dataset](specs/glossary.md#dataset) can [change over time](#evolution-of-data). To protect downstream consumers of data from braking changes the [Coordinator](specs/glossary.md#coordinator) has to validate the changes for backward compatibility before accepting them. This section covers the allowed transitions for different parts of the [Metadata](specs/glossary.md#metadata-chain).

#### Query Evolution
The procedure for modifying the [Query](specs/glossary.md#query) of a [Derivative Dataset](specs/glossary.md#derivative-dataset) is as following:
1. The [Coordinator](specs/glossary.md#coordinator) invokes the [Engine's](specs/glossary.md#engine) [Validate Query](#validate-query) operation to validate the new [Query](specs/glossary.md#query) and determine the validity of the transition according to the engine-specific rules.
2. The new [Schema](specs/glossary.md#schema) returned in the previous step is validated according to the [Schema Evolution](specs/datasets/data-format.md#schema-evolution) rules.
3. A new [Metadata Block](specs/glossary.md#metadata-chain) is created and committed.

> **TODO**: Specify how Schema changes in the upstream datasets should be handled
> - Do we stop processing and ask user to validate that the query takes into account the new columns?

### Dataset Sharing
Dataset sharing involves uploading the data to some [Repository](specs/glossary.md#repository) where it can be discovered and accessed by other peers. Due to immutable nature of data and metadata it is very easy to keep shared data up-to-date as only new blocks and part files need to be uploaded.

It is important to ensure the atomicity of the sharing, or at least create the perception of atomicity to someone who happens to be downloading the data concurrently with the upload.

For [Repositories](specs/glossary.md#repository) that do not support atomic file/object operations the following sequence of operations can ensure that concurrent downloads will always see the [Dataset](specs/glossary.md#dataset) in a consistent state:
1. Upload data part files
2. Upload metadata blocks
3. Update references in `metadata/refs` directory

#### Dataset Validation
The [Derivative Dataset](specs/glossary.md#derivative-dataset) validation confirms that the event data is in fact produced by applying all the transformations stored in the [Metadata](specs/glossary.md#metadata-chain) to the inputs and was not maliciously altered.

The process of validation is almost identical to the [Derivative Transformation](#derivative-transformations) except instead of the `Commit` phase the [Coordinator](specs/glossary.md#coordinator) compares the hashes of the data produced by local transform to the hashes stored in the [Metadata](specs/glossary.md#metadata-chain).

Due to non-determinism of Parquet format the physical hash may not match the one in metadata so a fallback to logical hash is required to verify that the resulting records are logically the same.

### Engine Deprecation
Pinning the exact [Engine](specs/glossary.md#engine) version used by every transformation guarantees the reproducibility and verifiability of the results (see [Components of Trust](#components-of-trust)). In the long run, however, this creates a problem where in order to [validate a dataset](#dataset-validation) the [Coordinator](specs/glossary.md#coordinator) will have to use a very large number of different versions of an [Engine](specs/glossary.md#engine) that might've accumulated over the years. This slows down the validation procedure and can result in a significant disk space required for storing these images.

To mitigate this problem the [Coordinator](specs/glossary.md#coordinator) offers the **engine deprecation** operation that works as follows:
1. The [Coordinator](specs/glossary.md#coordinator) reconstructs the dataset from scratch by repeating all transformation in the [Metadata Chain](specs/glossary.md#metadata-chain).
2. Instead of the [Engine](specs/glossary.md#engine) version specified in the block, it uses the latest (or provided) version.
3. If the results produced in every step match the recorded ones - we can safely consider that the newer version of the [Engine](specs/glossary.md#engine) is fully compatible with the older one.
4. A special [Metadata](specs/glossary.md#metadata-chain) record is created to record this compatibility, therefore, allowing all users to use the latest version of the [Engine](specs/glossary.md#engine) instead of downloading all the versions seen previously.

## Repository Contract
> **TODO:**
> - Supported protocols
> - Querying data in advanced repository

### Simple Transfer Protocol

Simple Transfer Protocol specified here is a bare-minimum read-only protocol used for synchronizing [Datasets](specs/glossary.md#dataset) between [Repositories](specs/glossary.md#repository). It requires no ODF-specific logic on the server side and can be easily implemented, for example, by serving a dataset directory under an HTTP server. It's designed for maximal interoperability, not for efficiency.

To describe the protocol we will use HTTP `GET {object-key}` notation below, but note that this protocol can be implemented on top of any block or file-based protocol that supports Unix path-like object keys.

1) Process begins with `GET /refs/head` to get the hash of the last [Metadata Block](specs/glossary.md#metadata-chain)
2) The "metadata walking" process starts with `GET /blocks/{blockHash}` and continues following the `prevBlockHash` links
3) Data part files can be downloaded by using [`DataSlice::physicalHash`](#dataslice-schema) links with `GET /data/{physicalHash}`
4) Checkpoints are similarly downloaded using [`Checkpoint::physicalHash`](#checkpoint-schema) links with `GET /checkpoints/{physicalHash}`
5) The process continues until reaching the first block of the dataset or other termination condition (e.g. reaching the block that has already been synced previously)

See also:
- [RFC-007: Simple Transfer Protocol](/rfcs/007-simple-transfer-protocol.md)

### Smart Transfer Protocol

Smart Transfer Protocol is a superset of Simple Transfer Protocol, which allows both read and writes
and solves performance issues to allow efficient synchronization between remote dataset repositories.

The detailed format of endpoints is described here: [Smart Transfer Protocol: OpenAPI](/protocols/smart-transfer-protocol.openapi.yaml).

Smart Transfer Protocol extends the HTTP operations of the Simple Transfer Protocol with 2 more endpoints:
1) `GET /pull` - to start the smart pull flow on the dataset
2) `GET /push` - to start the smart push flow on the dataset

Both extensions switch from HTTP to a more advanced asynchronous bi-directional message-based protocol,
like [WebSockets](https://websockets.spec.whatwg.org/). Parties first exchange the intent of synchronization,
then switch to transfer of metadata blocks and associated object files. Metadata is transferred as an
archive of the block files, while object files are transferred separately with unrestricted degree of parallelism.
The protocol assumes, but does not require, the use of 3rd-party cloud data storage service to exchange object files. 

The details of the individual asynchronous messages are specified here: [Smart Transfer Protocol: AsyncAPI](/protocols/smart-transfer-protocol.asyncapi.yaml).

See also:
- [RFC-008: Smart Transfer Protocol](/rfcs/008-smart-transfer-protocol.md)


## Future Topics

### Anonymization

> **TODO:**
> - Query Gateways
> - Portal Datasets that expose any dataset as root without disclosing details

# Reference Information

## Metadata Reference

![](build/metadata-reference.md)

## Engine API Reference
> **TODO**: Provide `gRPC + FlatBuffers` IDL

## Repository API Reference
> **TODO**: Provide `gRPC + FlatBuffers` IDL

