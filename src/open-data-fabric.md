# Open Data Fabric

# Table of Contents
<!-- toc -->

# Specification

## Coordinator Contract
The main functions of the [Coordinator](specs/glossary.md#coordinator) are:
- Maintain the [invariants](specs/overview.md#requirements) and the [semantics](specs/overview.md#transactional-semantics) of the system
- Guarantee the validity and integrity of the [Metadata](specs/glossary.md#metadata-chain)
- Provide means for ingesting external data into the system
- Interface with the [Engines](specs/glossary.md#engine) to implement data transformations
- Enable data sharing functionality via [Repositories](specs/glossary.md#repository)
- Provide additional functionality to simplify working with [Datasets](specs/glossary.md#dataset) and their metadata

### Dataset Validation
The [Derivative Dataset](specs/glossary.md#derivative-dataset) validation confirms that the event data is in fact produced by applying all the transformations stored in the [Metadata](specs/glossary.md#metadata-chain) to the inputs and was not maliciously altered.

The process of validation is almost identical to the [Derivative Transformation](specs/transformations/processing-model.md#derivative-transformations) except instead of the `Commit` phase the [Coordinator](specs/glossary.md#coordinator) compares the hashes of the data produced by local transform to the hashes stored in the [Metadata](specs/glossary.md#metadata-chain).

Due to non-determinism of Parquet format the physical hash may not match the one in metadata so a fallback to logical hash is required to verify that the resulting records are logically the same.

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
