# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
- Renamed `remote` into `repository` to avoid some terminology pitfals.

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
