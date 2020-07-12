# Open Data Fabric

[Latest Version](./open-data-fabric.md)

## Introduction

**Open Data Fabric** is an open protocol specification for decentralized exchange and transformation of semi-structured data, that aims to holistically address many shortcomings of the modern data management systems and workflows.

The goal of this specification is to develop a method of data exchange that would:
- Enable worldwide collaboration around data cleaning, enrichment, and derivation
- Create an environment of verifiable trust between participants without the need for a central authority
- Enable high degree of data reuse, making quality data more readily available
- Improve liquidity of data by speeding up the data propagation times from publishers to consumers
- Create a feedback loop between data consumers and publishers, allowing them to collaborate on better data availability, recency, and design

## Current State

Latest version: [0.15.0](./open-data-fabric.md)

The specification is currently in the **EXPERIMENTAL** stage and welcomes feedback.

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
