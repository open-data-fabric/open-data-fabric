# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
