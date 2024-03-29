<div align="center">

<img src="./src/images/odf-logo.png" alt="Open Data Fabric">

<p><strong><i>Open protocol for decentralized exchange and transformation of data</i></strong></p>

[Website](https://docs.kamu.dev/odf/) |
[Reference Implementation](https://github.com/kamu-data/kamu-cli) |
[Original Whitepaper](https://arxiv.org/abs/2111.06364) |
[Chat](https://discord.gg/nU6TXRQNXC)


[![Spec](https://img.shields.io/github/v/tag/open-data-fabric/open-data-fabric?include_prereleases&logo=gitbook&logoColor=white&label=Spec&style=for-the-badge)](https://github.com/open-data-fabric/open-data-fabric/blob/master/open-data-fabric.md)
[![Metadata Reference](https://img.shields.io/static/v1?label={}&message=Schemas&color=blue&style=for-the-badge)](./open-data-fabric.md#reference-information)
[![Chat](https://shields.io/discord/898726370199359498?style=for-the-badge&logo=discord&label=Discord)](https://discord.gg/nU6TXRQNXC)


</p>
</div>

## Introduction

**Open Data Fabric** is an open protocol specification for decentralized exchange and transformation of semi-structured data, that aims to holistically address many shortcomings of the modern data management systems and workflows.

The goal of this specification is to develop a method of data exchange that would:
- Enable worldwide collaboration around data cleaning, enrichment, and derivation
- Create an environment of verifiable trust between participants without the need for a central authority
- Enable high degree of data reuse, making quality data more readily available
- Improve liquidity of data by speeding up the data propagation times from publishers to consumers
- Create a feedback loop between data consumers and publishers, allowing them to collaborate on better data availability, recency, and design

<p align="center">
<img src="./src/images/dataset_graph.png" alt="Dataset Graph">
</p>

`ODF` protocol is a **Web 3.0 technology** that powers a distributed structured data supply chain for providing timely, high-quality, and verifiable data for data science, smart contracts, web and applications.

<p align="center">
<img src="./src/images/distributed_world.png" alt="Web 3.0" width="350">
</p>

### Protocol Map

<p align="center">
<img src="images/odf-dataset.svg" alt="Dataset" width="400">
</p>
<p align="center">
<img src="images/odf-protocols.svg" alt="Dataset" width="700">
</p>

### Introductory materials
- [Original Whitepaper (July 2020)](https://arxiv.org/abs/2111.06364)
- [Kamu Blog: Introducing Open Data Fabric](https://www.kamu.dev/blog/introducing-odf/)
- [Talk: Open Data Fabric for Research Data Management](https://www.youtube.com/watch?v=Ivh-YDDmRf8)
- [PyData Global 2021 Talk: Time: The most misunderstood dimension in data modelling](https://www.youtube.com/watch?v=XxKnTusccUM)
- [Data+AI Summit 2020 Talk: Building a Distributed Collaborative Data Pipeline](https://databricks.com/session_eu20/building-a-distributed-collaborative-data-pipeline-with-apache-spark)

More tutorials and articles can be found in [kamu-cli documentation](https://docs.kamu.dev/cli/learn/learning-materials/).

## Current State

The specification is currently in actively evolving and welcomes feedback.

See also our [Roadmap](https://github.com/kamu-data/open-data-fabric/projects/1) for future direction and [RFC archive](/rfcs) for the record of changes.

## Implementations

`Coordinator` implementations:
- [kamu-cli](https://github.com/kamu-data/kamu-cli/) - data management tool that serves as the reference implementation.

`Engine` implementations:
- [kamu-engine-spark](https://github.com/kamu-data/kamu-engine-spark) - engine based on Apache Spark.
- [kamu-engine-flink](https://github.com/kamu-data/kamu-engine-flink) - engine based on Apache Flink.


## History

The specification was originally developed by [Kamu](https://kamu.dev) as part of the [kamu-cli](https://github.com/kamu-data/kamu-cli/) data management tool. While developing it, we quickly realized that the very essence of what we're trying to build - a collaborative open data processing pipeline based on verifiable trust - requires full transparency and openness on our part. We strongly believe in the potential of our ideas to bring data management to the next level, to provide better quality data faster to the people who need it to innovate, fight deceases, build better businesses, and make informed political decisions. Therefore, we saw it as our duty to share these ideas with the community and make the system as inclusive as possible for the existing technologies and future innovations, and work together to build momentum needed to achieve such radical change.

## Contributing
See [Contribution Guidelines](./CONTRIBUTING.md)

## RFC List
- [RFC-000: RFC Template](rfcs/000-template.md)
- [RFC-001: Record Offsets](rfcs/001-record-offsets.md)
- [RFC-002: Logical Data Hashes](rfcs/002-logical-data-hashes.md)
- [RFC-003: Content Addressability](rfcs/003-content-addressability.md)
- [RFC-004: Metadata Extensibility](rfcs/004-metadata-extensibility.md)
- [RFC-005: New Annotation Metadata Events](rfcs/005-dataset-annotations.md)
- [RFC-006: Store Checkpoints as Files](rfcs/006-checkpoints-as-files.md)
- [RFC-007: Simple Transfer Protocol](rfcs/007-simple-transfer-protocol.md)
- [RFC-008: Smart Transfer Protocol](rfcs/008-smart-transfer-protocol.md)
- [RFC-009: Ingest Source State](rfcs/009-ingest-source-state.md)
- [RFC-010: Data Schema in Metadata](rfcs/010-data-schema-in-metadata.md)
- [RFC-011: Push Ingest Sources](rfcs/011-push-ingest-sources.md)
- [RFC-012: Recommend `base16` encoding for textual representation of hashes and DIDs](rfcs/012-recommend-base16-encoding.md)
- [RFC-013: Enum representation in YAML encoding](rfcs/013-yaml-enum-representation.md)
- [RFC-014: Minimizing scanning for last offset and block ](rfcs/014-minimize-offset-scanning.md)
- [RFC-015: Unified changelog stream schema](rfcs/015-unified-changelog-stream-schema.md)
