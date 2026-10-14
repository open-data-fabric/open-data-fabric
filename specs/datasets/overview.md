# Datasets Overview <!-- omit in toc -->

**Domain**: Datasets
**Depends on**: -
**RFCs**: -


## Abstract

This document introduces the general structure of an ODF [Dataset](../glossary.md#dataset) - the main unit of data exchange in the system. It explains the components a dataset consists of, the two kinds of datasets, and how datasets evolve over time, serving as an entry point into the specs of the Datasets domain.


## Table of Contents <!-- omit in toc -->

- [Abstract](#abstract)
- [Dataset Structure](#dataset-structure)
  - [Data and Data Slices](#data-and-data-slices)
  - [Metadata](#metadata)
  - [Root and Derivative Datasets](#root-and-derivative-datasets)
- [Evolution of Data](#evolution-of-data)
- [Datasets Specs](#datasets-specs)


## Dataset Structure
ODF operates on data expressed as immutable events (see [Nature of Data](../overview.md#nature-of-data)). A [Dataset](../glossary.md#dataset) is a combination of:
- **Identity** - a globally unique identifier tied to a cryptographic key pair, and human-friendly aliases (see [Identity](identity.md))
- **Data** - a potentially infinite stream of events, stored as a sequence of data slices (see [Data Model](data-model.md))
- **Metadata Chain** - an append-only log of blocks that captures the entire history of the dataset (see [Metadata Chain](metadata-chain.md))
- **Checkpoints** - engine-specific state used to resume stream processing (see [Processing Model](../transformations/processing-model.md#checkpoints))

![Diagram: Dataset/Data](../../images/dataset.svg)

All components of a dataset except the block references are immutable and content-addressed, so having only a hash of the last metadata block one can discover and verify all of them (see [Storage Layout](storage-layout.md)).


### Data and Data Slices
[Data](../glossary.md#data) is a set of [Events](../glossary.md#event). Since events are immutable, data can only grow over time. Conceptually it's best to think of data as a full log of a potentially infinite event stream.

Data never appears in the system alone, as we would not be able to tell where it came from and whether it can be trusted - it always appears as part of a dataset.

Data arrives into the system in arbitrarily large sets of events called [Data Slices](../glossary.md#data-slice). Every slice is a continuous part of data with the same [Schema](../glossary.md#schema), defined by its interval of [Offsets](../glossary.md#offset), and is stored as a separate data part file.

![Diagram: Data Slices and Metadata](../../images/metadata.svg)

All records carry the system columns of the common data schema, including the offset, the [Operation Type](../glossary.md#operation-type), the [System Time](../glossary.md#system-time), and the [Event Time](../glossary.md#event-time) (see [Data Model](data-model.md)). The logical structure of data is described using the ODF schema format (see [Data Schema](data-schema.md)).


### Metadata
Metadata plays an essential role in ODF (see [Essential Role of Metadata](../overview.md#essential-role-of-metadata)) - it contains every aspect of where the data came from, how it was transformed, and everything that ever influenced how data looks like throughout its entire lifetime.

Every data slice and checkpoint added to the dataset is recorded in a block of the [Metadata Chain](metadata-chain.md) together with its [hash](data-and-checkpoint-hashing.md), linking data and metadata cryptographically.


### Root and Derivative Datasets
Depending on where the data comes from, datasets can be of two kinds:
- [Root Datasets](../glossary.md#root-dataset) - the points of entry of external data into the system, populated via [Ingestion](ingestion.md)
- [Derivative Datasets](../glossary.md#derivative-dataset) - created by transforming and combining one or multiple existing datasets using deterministic queries (see [Processing Model](../transformations/processing-model.md))

Datasets connected through transformations form a graph, where data of every derivative dataset can be traced back to the root datasets it was derived from.

![Diagram: Dataset Graph](../../images/dataset_graph.svg)


## Evolution of Data
As the nature of businesses change, as new requirements arrive, as defects are detected and fixed - it's not a matter of *if* but *when* the time comes to make changes to the data. Calling data a potentially infinite stream would not make much sense without providing a way to improve and evolve it over time. Having to create a new dataset every time you need to change the schema or update the transformation would jeopardize the timeliness requirement, as the entire chain of transformations would have to be rebuilt from scratch after every such change.

Our design goals are, therefore:
- To seamlessly support all backward-compatible data schema changes
- To support the evolution of transformations over time
- To provide a way to correct past mistakes in data
w
While for simplicity we previously considered derivative datasets to be only defined by one transformation query, in fact it can comprise multiple queries that were active during different periods of time. The metadata keeps track of which parts of inputs were processed by which query and which parts of output were produced as the result, ensuring reproducibility. This is captured by the [Metadata Chain](metadata-chain.md) - **almost anything that defines a dataset can change over time**.


## Datasets Specs

| Spec | Description |
| --- | --- |
| [Identity](identity.md) | `did:odf` identifiers, aliases, and references |
| [Data Model](data-model.md) | Event model, system columns, retractions and corrections, schema evolution |
| [Data Schema](data-schema.md) | Logical schema format of data |
| [Metadata Chain](metadata-chain.md) | Blocks, events, consistency rules, and block hashing |
| [Data & Checkpoint Hashing](data-and-checkpoint-hashing.md) | Physical and logical hashing of data and checkpoints |
| [Storage Layout](storage-layout.md) | Layout of dataset components in file systems, object stores, and content-addressable storage |
| [Ingestion](ingestion.md) | Sources, ingestion phases, and merge strategies of root datasets |
