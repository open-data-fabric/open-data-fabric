# RFC-021: ODF Spec Restructuring <!-- omit in toc -->

[![PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/?label=PR)](https://github.com/kamu-data/open-data-fabric/pull/)

**Start Date**: 2026-10-13

**Published Date**: ???

**Authors**:
- [Sergii Mikhtoniuk](mailto:sergii.mikhtoniuk@kamu.dev), [Kamu](https://kamu.dev)


**Compatibility**:
- [x] Backwards-compatible
- [x] Forwards-compatible


## Summary <!-- omit in toc -->

Proposes a reorganization of the ODF specification from a single document into a set of interconnected standards.


## Table of Contents <!-- omit in toc -->

- [Motivation](#motivation)
- [Proposed Solution](#proposed-solution)
  - [Top-level Documents](#top-level-documents)
  - [Domains](#domains)
    - [Datasets](#datasets)
    - [Transformations](#transformations)
    - [Exchange](#exchange)
    - [Auth \& Privacy](#auth--privacy)
    - [Automation](#automation)
    - [Reference](#reference)
  - [Repository Structure](#repository-structure)
  - [Naming and Versioning](#naming-and-versioning)
  - [Common Spec Template](#common-spec-template)
- [Drawbacks](#drawbacks)
- [Rationale and alternatives](#rationale-and-alternatives)
- [Prior art](#prior-art)
- [Unresolved questions](#unresolved-questions)
- [Future possibilities](#future-possibilities)


## Motivation

The current spec has grown very large and covers an extensive breadth of topics starting from common terminology, metadata structure, and data layouts, and to transformations, checkpointing, engine protocols and many more. This is too much information in a single document already.

We also see several more areas developing that are currently not reflected in the spec. From low-level things like encryption, privacy-preserving computations, and logical data hashing, to high-level things like IaC resource framework, node-level and decentralized authorization etc. These areas also need to be covered by a spec, but represent their own domains that would be better covered in standalone documents that reference other domains rather than being mixed with the rest.


## Proposed Solution

We propose to split the current ODF specification into a set of interconnected standards, similarly to [W3C Credentials Community Group](https://w3c-ccg.org/specifications/standards/), [Open Container Initiative](https://opencontainers.org/), and others.

### Top-level Documents

Domain-independent documents that serve as an entry point into the specs:

| Document | Scope | Sources | Status |
| --- | --- | --- | --- |
| **Overview** | Problem statement, requirements, design decisions, map of all specs and their dependencies, conformance profiles. Mostly non-normative. | Abstract, Problem Statement, Requirements, Scope, Design Decisions | Existing |
| ✅ **Glossary** | Definitions of core concepts and terms shared by all specs | Concepts and Components | Existing |

### Domains

Specs are grouped into domains that roughly follow the protocol stack, from the dataset structure up to network-level and node-level concerns. Rather than defining general-purpose primitives like hashing and identity in standalone specs, each spec explains how it applies established standards like [Multiformats](https://multiformats.io/) and [W3C DID](https://www.w3.org/TR/did-core/) where it uses them. Each spec is a standalone document that can reference specs in its own or lower domains.

Status legend:
- **Existing** - content already exists in the current spec and will be moved
- **RFC** - content exists in accepted RFCs but was never fully integrated into the spec
- **New** - area not yet covered, the spec will be created as it matures

#### Datasets

| Spec | Scope | Sources | Status |
| --- | --- | --- | --- |
| **Overview** | Overview of the dataset logical structure and its components | Nature of Data, Evolution of Data, Dataset, Data, Data Slice, Metadata | Existing |
| **Identity** | `did:odf` DID method, key pairs and proof of ownership, aliases and references | Dataset Identity, [RFC-012] | Existing |
| ✅ **Data Model** | Bitemporal event model, common data schema (offset, operation type, system / event time), changelog representation of retractions and corrections, Parquet layout, schema evolution rules | Data Format, Common Data Schema, Retractions and Corrections, Schema Evolution, [RFC-001], [RFC-015] | Existing |
| ✅ **Data Schema** | ODF logical schema format for describing data independently of its physical format (Parquet, Avro, etc.), logical types and their mapping to physical types | Schema Format, [RFC-010], [RFC-016] | Existing |
| **Metadata Chain** | Blocks, events, semantics and consistency rules, binary and human-readable encodings, block hashing, hash representation (multihash / multibase), extensibility and annotations | Metadata, Metadata Chain, Metadata Format, Metadata Block Hashing, Hash Representation, [RFC-004], [RFC-005], [RFC-012] | Existing |
| ✅ **Data & Checkpoint Hashing** | Physical hashing of data and checkpoint files, stable hashing of data records independent of physical encoding, canonical representation of logical types, hashing algorithm (currently defined in [arrow-digest](https://github.com/sergiimk/arrow-digest)) and its multicodec code | Physical and Logical Hashes, Checkpoint Hashing, [RFC-002] | Existing |
| ✅ **Storage Layout** | Dataset structure, refs, tags, branches, named vs. content-addressable layouts, linked large files | Dataset Layout, [RFC-003], [RFC-006], [RFC-017] | Existing |
| **Compactions** | Soft compactions (merging data files while preserving history) and hard compactions (permanent removal of history for compliance or storage optimization) |  | New |
| ✅ **Ingestion** | Root datasets, polling and push sources, source state, merge strategies | Root Dataset, Ingestion, Merge Strategy, Data Ingestion, [RFC-009], [RFC-011], [RFC-014] | Existing |

#### Transformations

| Spec | Scope | Sources | Status |
| --- | --- | --- | --- |
| **Temporal Processing Model** | Derivative datasets, queries, event time, watermarks, checkpoints, query evolution and migrations, provenance | Derivative Dataset, Query, Time, Watermark, Checkpoint, Provenance, Derivative Transformations, Query Evolution | Existing |
| ✅ **Engine Contract** | Engine execution model, communication interface, operations, sandboxing, engine versioning and deprecation | Engine, Engine Contract, Engine Deprecation | Existing |
| **Projections** | Temporal projections (AS AT / AS OF), collapsing changelog streams into state | Projection | Existing |

#### Exchange

| Spec | Scope | Sources | Status |
| --- | --- | --- | --- |
| ✅ **Transfer Protocols** | Repositories, simple and smart transfer protocols, direct-to-storage transfers, replication | Repository, Data Sharing, Dataset Sharing, Repository Contract, [RFC-007], [RFC-008] | Existing |
| **Query Protocols** | Standard query interfaces (Flight SQL / ADBC), federated query planning |  | New |
| **Verification** | Components of trust, determinism and reproducibility, dataset validation and verification procedures | Verifiability, Components of Trust, Dataset Validation | Existing |
| ✅ **Verifiable Batch Queries** | Query commitments and proofs that allow consumers to verify that query results correspond to specific dataset states | [RFC-022] | RFC |
| **Discovery** | Dataset registries, anchoring of dataset state to external ledgers |  | New |


#### Auth & Privacy

| Spec | Scope | Sources | Status |
| --- | --- | --- | --- |
| ✅ **Authorization** | Accounts and their identifiers, groups, auth attributes, policies and policy bindings within a node | [RFC-020] | RFC |
| **Decentralized Permissions** | Access control spanning multiple nodes and organizations: delegated authorization (UCAN), threshold cryptography |  | New |
| **Layered Encryption** | Encryption of data and metadata at rest and in transit, key management and rotation, interaction with content hashing |  | New |
| **Confidential Computing** | Integration of TEEs, ZK proofs, FHE as alternative verification mechanisms; anonymization and other privacy-preserving transformations | Future Topics / Anonymization | New |

#### Automation

| Spec | Scope | Sources | Status |
| --- | --- | --- | --- |
| ✅ **Resource Framework** | Manifest format, resource model, desired vs. observed state, extensibility of resource types | Manifest, [RFC-018] | RFC |
| **Event Bus** | The delivery mechanism and common types of events flowing in the system | | New |
| ✅ **Tasks and Flows** | Workload scheduling and execution resources within ODF nodes | [RFC-019] | RFC |
| **Webhooks** | Delivery of dataset events to external HTTP endpoints | `sinks/v1alpha1` schemas | New |

#### Reference

Schemas and API definitions that domain specs refer to, largely generated from machine-readable sources in the repository.

| Reference | Scope | Sources | Status |
| --- | --- | --- | --- |
| **Metadata Schemas** | Metadata blocks, events, and their nested types | Metadata Reference, `MetadataBlock` / `MetadataEvent` and related types in `schemas/{datasets,data}` | Existing |
| **Resource Schemas** | Resource types and their specs, grouped by API group and version | `schemas/*` | RFC |
| **Engine API** | Engine communication interface IDL and messages | Engine API Reference, `schemas/engines` | Existing |
| **Transfer Protocol API** | Simple and smart transfer protocol endpoints and messages | Repository API Reference, `protocols/smart-transfer-protocol.{openapi,asyncapi}.yaml` | Existing |

### Repository Structure

Current layout of the repository:

```
.
├── open-data-fabric.md      # Generated spec (from src/open-data-fabric.md + build/metadata-reference.md)
├── src/
│   ├── open-data-fabric.md  # Spec source
│   └── images/              # Diagram sources (.puml, .svg, .png)
├── images/                  # Generated diagrams referenced by the spec
├── rfcs/
│   ├── {xxx}-{name}/        # Images of {xxx}-{name}.md
│   └── {xxx}-{name}.md
├── schemas/                 # JSON Schemas for resources and metadata
│   └── {context}/{version}/
│       └── {schema}.json
├── schemas-generated/       # Generated artifacts
│   ├── flatbuffers/opendatafabric.fbs
│   └── mermaid/erd.{mmd,svg}
├── protocols/               # API definitions
│   ├── smart-transfer-protocol.asyncapi.yaml
│   └── smart-transfer-protocol.openapi.yaml
├── examples/                # Example manifests, one directory per scenario
│   └── {scenario}/{name}.yaml
├── tools/
├── authoring_schemas.md
├── CHANGELOG.md
├── CONTRIBUTING.md
├── Makefile
└── README.md
```

Proposed layout of the repository:

```
.
├── specs/                           # Spec sources split by domain
│   ├── {domain}/
│   │   ├── {spec}/                  # Images of {spec}.md
│   │   └── {spec}.md
│   ├── overview.md
│   ├── glossary.md
│   └── reference.md
├── rfcs/
│   ├── {xxx}-{name}/                # Images of {xxx}-{name}.md
│   └── {xxx}-{name}.md
├── schemas/
│   └── {context}/{version}/
│       └── {schema}.json
├── contracts/                       # Serde and API contracts
│   ├── erd.{mmd,svg}                # Generated from schemas
│   ├── metadata/opendatafabric.fbs  # Generated from schemas
│   └── transfer/
│       ├── smart-transfer-protocol.asyncapi.yaml
│       └── smart-transfer-protocol.openapi.yaml
├── examples/
│   └── {scenario}/{name}.yaml
├── tools/
├── authoring_schemas.md
├── CHANGELOG.md
├── CONTRIBUTING.md
├── Makefile
└── README.md
```


### Naming and Versioning

Each spec is stored as `/specs/{domain}/{spec}.md`, where both domain and spec names are the kebab-cased titles from the [Domains](#domains) tables, e.g. `/specs/auth-and-privacy/authorization.md`. Domain-independent documents (`overview.md`, `glossary.md`, `reference.md`) are stored at the root of `/specs`.

All specs are versioned together as a single set, i.e. a release of ODF includes all specs at their current state, and the version is not tracked per individual spec.


### Common Spec Template

All spec files follow a common structure:

```markdown
# {Spec Title}

**Domain**: {Domain}
**Depends on**: [{Spec}](../{domain}/{spec}.md), ...
**RFCs**: [RFC-{NNN}](../../rfcs/{NNN}-{rfc}.md), ...

## Abstract
One paragraph describing what this spec covers.

## Table of Contents

## ...
Spec-specific sections.
```

The **RFCs** field lists the RFCs that introduced or changed the spec, mirroring the **Specs** field in RFC headers, so the two are linked in both directions.


## Drawbacks

- Existing links to sections of the monolithic spec (from RFCs, docs, websites, and implementations) will break unless redirects or a stub document with pointers to new locations is maintained.
- Readers lose a single linear narrative and need to navigate between multiple documents to understand the system as a whole. The **Overview** and **Glossary** specs are meant to mitigate this.
- More documents means more maintenance overhead and a higher risk of duplication or inconsistency between specs. This increases the need for tooling that checks cross-references.
- Publishing a set of specs requires more elaborate build and website tooling than rendering a single markdown file.


## Rationale and alternatives

Splitting by domain aligns specs with their audiences and their rate of change: an engine implementer, a repository operator, and a node administrator care about different subsets of ODF, and the data format evolves much more slowly than node-level resources. It also allows new areas to start as standalone drafts without destabilizing mature parts of the spec, and makes it clear which parts of the spec an RFC affects.

Alternatives considered:
- **Single document split into files at build time** - would reduce the size of individual source files, but still presents one large document to readers and does not allow different parts of the spec to have different maturity levels.
- **Split by implementer role** (e.g. engine, coordinator, repository contracts, as partially done in the current spec) - roles cut across many domains (e.g. a coordinator touches ingestion, hashing, metadata, and transfer), which would lead to duplication. Instead, roles are expected to be expressed as conformance profiles in the **Overview** spec that reference relevant domain specs.
- **Few coarse-grained specs** (e.g. format, processing, and protocols, similarly to OCI) - simpler to navigate, but new areas like privacy, authorization, and automation would again bloat these documents.


## Prior art

- [W3C Credentials Community Group](https://w3c-ccg.org/specifications/standards/)
- [Open Container Initiative](https://opencontainers.org/) - separate, independently versioned specs for [image format](https://github.com/opencontainers/image-spec), [runtime](https://github.com/opencontainers/runtime-spec), and [distribution](https://github.com/opencontainers/distribution-spec)
- [Apache Arrow](https://arrow.apache.org/docs/format/index.html) - format specs (Columnar, IPC, C Data Interface, Flight, Flight SQL, ADBC) versioned separately from libraries
- [Apache Iceberg](https://iceberg.apache.org/spec/) - table spec alongside [view spec](https://iceberg.apache.org/view-spec/), [Puffin spec](https://iceberg.apache.org/puffin-spec/), and [REST catalog API](https://github.com/apache/iceberg/blob/main/open-api/rest-catalog-open-api.yaml)
- [AT Protocol](https://atproto.com/specs/atp) - content-addressed repositories and DID-based identity split into small specs (data model, cryptography, DID, repository, sync, etc.)
- [Eclipse Dataspace Protocol](https://eclipse-dataspace-protocol-base.github.io/DataspaceProtocol/) - abstract protocol specs (catalog, contract negotiation, transfer process) with separate protocol bindings


## Unresolved questions


## Future possibilities

- Versioning specs independently, each with its own maturity level (similarly to W3C CSS modules), once some domains stabilize while others continue to evolve rapidly
- Rust linter to ensure referential integrity


[RFC-001]: ./001-record-offsets.md
[RFC-002]: ./002-logical-data-hashes.md
[RFC-003]: ./003-content-addressability.md
[RFC-004]: ./004-metadata-extensibility.md
[RFC-005]: ./005-dataset-annotations.md
[RFC-006]: ./006-checkpoints-as-files.md
[RFC-007]: ./007-simple-transfer-protocol.md
[RFC-008]: ./008-smart-transfer-protocol.md
[RFC-009]: ./009-ingest-source-state.md
[RFC-010]: ./010-data-schema-in-metadata.md
[RFC-011]: ./011-push-ingest-sources.md
[RFC-012]: ./012-recommend-base16-encoding.md
[RFC-014]: ./014-minimize-offset-scanning.md
[RFC-015]: ./015-unified-changelog-stream-schema.md
[RFC-016]: ./016-odf-schema.md
[RFC-017]: ./017-large-files-linking.md
[RFC-018]: ./018-iac-resource-framework.md
[RFC-019]: ./019-iac-task-flow-system.md
[RFC-020]: ./020-auth.md
[RFC-022]: ./022-batch-query-commitments.md
