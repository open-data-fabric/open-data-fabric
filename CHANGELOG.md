# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.42.0] - 2026-10-15
### Added
- [RFC-021: ODF Spec Restructuring](rfcs/021-spec-restructuring.md)
- [RFC-022: Batch Query Commitments](rfcs/022-batch-query-commitments.md)
- Specification is now split into domain-based specs under `specs/` (see [Overview](specs/overview.md))
  - New specs for areas previously covered only by RFCs: Authorization, Resource Framework, Tasks and Flows, Verifiable Batch Queries
  - Clarified retractions and corrections semantics in Data Model, Processing Model, and Engine Contract
  - Differentiated validation from verification and described verification mechanisms for root data, derivative data, and queries
- Schema reference is generated into `specs/reference.md`, including scalar label schemas
- `ReconciliationError` resource condition
- `Interval` and `InputsUpdated` flow triggers
- `enabled`, `cooldown`, and `cooldownMaxBatch` as common properties of all flow triggers
- `onNoOp` and `onFailure` task properties controlling how a flow run proceeds
- `recentBindingsRetention` and `recentRunsRetention` flow properties
- `retryPolicy` in canonical `WebhookCall` task spec
- `make lint-markdown` target for checking links in markdown files
### Changed
- Generated artifacts moved from `schemas-generated/` and `protocols/` to `contracts/`
- Images moved next to the specs that use them
- Spec documents no longer carry a version - releases are versioned by tagging the repository
### Removed
- **Breaking:** `Dataset` flow trigger in favor of `Event` and `InputsUpdated` triggers
- Monolithic `open-data-fabric.md` spec
### Fixed
- Retraction code in `Snapshot` merge strategy description (`-R` instead of `-D`)
- Smart Transfer Protocol AsyncAPI definition (`DatasetPushMetadata` payload, duplicated channel name, auth scheme)
- Schema URL pattern in `Resource` description no longer includes the `.json` extension

## [0.41.1] - 2026-10-12
- Move `PolicyBinding` action to `auth` domain

## [0.41.0] - 2026-10-12
- [RFC-020: Authorization Model](rfcs/020-auth.md)

## [0.40.1] - 2026-10-12
- Updated RFC-018
- Refining Task & Flow system schemas

## [0.40.0] - 2026-10-12
- Elaborating Task & Flow systems

## [0.39.0] - 2026-06-29
- Major restructuring towards IaC resource framework (#126)

## [0.38.0] - 2026-05-03
### Changed
- **Breaking:** Support for ODF schema in sources
  - `read.schema` now expects ODF and old field was renamed to `read.ddlSchema`
  - YAML manifests will need to be updated
  - Flatbuffer binary compatibility is preserved so existing datasets will continue to work fine
- Support short-form union representation for more concise ODF schema in manifests
- Defaults for `unit` and `timezone` in data types for even more conciseness
- Codegen-level support for `default` properties and equivalence

## [0.37.0] - 2025-09-08
### Added
- Introduced `ChangelogStream` and `UpsertStream` merge strategies.
- [RFC-016: ODF schema format](rfcs/016-odf-schema.md)
- [RFC-017: Large files linking](rfcs/017-large-files-linking.md)

## [0.36.0] - 2024-06-07
### Added
- Introduced the `EthereumLogs` source definition

## [0.35.0] - 2024-05-12
### Added
- Introduced the MQTT source definition

## [0.34.1] - 2024-03-14
### Changed
- Added note on case-insensitivity of dataset's name components

## [0.34.0] - 2024-01-11
### Changed
- [RFC-012: Recommend `base16` encoding for textual representation of hashes and DIDs](rfcs/012-recommend-base16-encoding.md) (#71)
- [RFC-013: Enum representation in YAML encoding](rfcs/013-yaml-enum-representation.md) (#71)
- [RFC-014: Minimizing scanning for last offset and block ](rfcs/014-minimize-offset-scanning.md) (#71)
- [RFC-015: Unified changelog stream schema](rfcs/015-unified-changelog-stream-schema.md) (#72)

## [0.33.1] - 2023-12-11
### Changed
- Renamed `source` field to `sourceName` in `AddPushSource` / `DisablePushSource` events and make it optional

## [0.33.0] - 2023-12-08
### Added
- Explicit data schema in metadata (#63)
- Push ingest Sources (#63)

## [0.32.0] - 2023-08-20
### Changed
- `JsonLines` renamed to `NdJson` for consistency with [ndjson.org](http://ndjson.org). Old name is considered deprecated and will be removed in future versions
- `JsonLines.multiline` is deprecated and removed in `NdJson` type as it contradicts `ndjson` format
### Added
- `Json` format was introduced that can read Array-of-Structures style JSON files
- `NdGeoJson` format was introduced that is similar to `NdJson` and will expect one GeoJSON `Feature` object per line
- `EventTimeSource::FromSystemTime` that assigns event time from the system time and can be used when source metadata cannot be trusted or is invalid.

## [0.31.0] - 2023-07-12
### Changed
- TransformInput extended with optional source dataset reference, potentially multi-tenant.

## [0.30.0] - 2023-05-05
### Changed
- Ingest source state (#50)

## [0.29.0] - 2023-04-12
### Changed
- Multi-tenancy related changes in dataset alias/reference formats 

## [0.28.0] - 2023-01-03
### Added
- Smart transfer protocol specification (#35)

## [0.27.0] - 2022-08-05
### Added
- Sequence number in MetadataBlock

## [0.26.0] - 2022-07-22
### Added
- Read step extended with Parquet format

## [0.25.0] - 2022-04-21
### Added
- Simple transfer protocol specification for syncing datasets between repos (#27).

## [0.24.0] - 2022-04-19
### Changed
- Engine checkpoints can now only be files, not arbitrary directories (#25).

## [0.23.0] - 2022-04-08
### Added
- New metadata events for annotating datasets (#21).

## [0.22.0] - 2022-01-04
### Changed
- Metadata blocks now have event-based format (#19).

## [0.21.0] - 2021-12-28
### Added
- Unique dataset identity and content addressability (#17).

## [0.20.0] - 2021-12-11
### Added
- Logical and physical data hashing specification (#16).

## [0.19.0] - 2021-12-04
### Changed
- Added `offset` system column (#10).

## [0.18.0] - 2021-08-05
### Changed
- Renamed `remote` into `repository` to avoid some terminology pitfalls.

## [0.17.0] - 2020-12-26
### Added
- `DatasetVocabulary` is now a part of the `MetadataBlock`

## [0.16.0] - 2020-10-31
### Added
- Finalized metadata hashing procedure.
- Added `flatbuffers` schema.
- Multiple updates to codegen tools.

## [0.15.0] - 2020-07-12
### Added
- Metadata schemas improvements.

## [0.14.1] - 2020-07-05
### Added
- Initial public release.
