# Data & Checkpoint Hashing <!-- omit in toc -->

**Domain**: Datasets
**Depends on**: [Data Format](data-format.md)
**RFCs**: [RFC-002](../../rfcs/002-logical-data-hashes.md)


## Abstract

This spec defines how hash sums of data part files and checkpoints are computed, including the physical hashes used to identify files in content-addressable storage and the logical hashes of data records that remain stable regardless of how data is physically encoded.


## Table of Contents <!-- omit in toc -->

- [Abstract](#abstract)
- [Physical vs. Logical Hashes](#physical-vs-logical-hashes)
- [Logical Hashing Algorithm](#logical-hashing-algorithm)
  - [Properties](#properties)
  - [Value Encoding](#value-encoding)
  - [Data Type Encoding](#data-type-encoding)
  - [Record Batch Hashing](#record-batch-hashing)
- [Checkpoint Hashing](#checkpoint-hashing)
- [Hash Codes](#hash-codes)


## Physical vs. Logical Hashes
Because the on-disk Parquet data format is non-deterministic (serializing same logical data may result in different binary layouts depending on heuristics and implementation) two different hash sums are maintained by the coordinator:
- **Physical hash** - a (non-reproducible) hash sum of an entire binary file as originally produced by the owner of the dataset
- **Logical hash** - a stable (reproducible) hash sum computed on data records, resistant to encoding variations

**Physical hashing** is used in the context of verifying integrity of the datasets and uploading and downloading data files to/from content-addressable systems, where hash sum acts as an identifier of the data file. Physical hash is calculated using [SHA3-256](https://en.wikipedia.org/wiki/SHA-3) algorithm on entire data part file.

**Logical hashing** is used for equivalence checks when reproducing results of computations. It is computed over the in-memory representation of data in [Apache Arrow](https://arrow.apache.org/) format using the algorithm described below.


## Logical Hashing Algorithm
The logical hashing algorithm computes a stable hash of structured data based on its [Apache Arrow](https://arrow.apache.org/docs/format/Columnar.html#physical-memory-layout) in-memory representation. The algorithm is parametrized by the underlying digest function, which in ODF is [SHA3-256](https://en.wikipedia.org/wiki/SHA-3).

The current version of the algorithm is `0`, as reflected in the `arrow0-sha3-256` [hash code](#hash-codes). Future versions may change the resulting hashes, in which case they will be assigned new hash codes.

The reference implementation is provided by the [arrow-digest](https://github.com/sergiimk/arrow-digest) Rust crate.


### Properties
- The hash is the same no matter how many record batches the data was split into
- The hash is the same no matter which Arrow encoding variant of a type is used (e.g. `Utf8`, `LargeUtf8`, and `Utf8View`) - the difference between them is purely an encoding concern
- The hash is the same for arrays without a validity bitmap and arrays where a validity bitmap marks all values as valid
- The hash is **order-dependent** - reordering records will produce a different hash


### Value Encoding
Values are fed into the digest function according to the following rules:
- **Endianness** - all multi-byte values are encoded as little-endian
- **Fixed size types** (`Int`, `FloatingPoint`, `Decimal`, `Date`, `Time`, `Timestamp`) - hashed using their in-memory binary representation
- **Booleans** - hashed as byte-sized values: `1` for `false` and `2` for `true`
- **Binaries and strings** (`Binary`, `LargeBinary`, `FixedSizeBinary`, `BinaryView`, `Utf8`, `LargeUtf8`, `Utf8View`) - length of the value in bytes as `u64`, followed by the bytes of the value
- **Lists** (`List`, `LargeList`, `FixedSizeList`) - length of the list as `u64`, followed by the items of the list hashed according to their data type
- **Nulls** - every null value is represented by a single `0` byte


### Data Type Encoding
When hashing an array, its data type is hashed once at the start of the hashing session, followed by all items of the array hashed sequentially.

Data type is encoded as a type identifier (`u16`) followed by type parameters:

| Arrow Type | Type ID | Followed by |
| --- | --- | --- |
| `Null` | `0` | |
| `Int8`, `Int16`, `Int32`, `Int64` | `1` | `1` (signed) as `u8`, bit width as `u64` |
| `UInt8`, `UInt16`, `UInt32`, `UInt64` | `1` | `0` (unsigned) as `u8`, bit width as `u64` |
| `Float16`, `Float32`, `Float64` | `2` | bit width as `u64` |
| `Binary`, `LargeBinary`, `FixedSizeBinary`, `BinaryView` | `3` | |
| `Utf8`, `LargeUtf8`, `Utf8View` | `4` | |
| `Boolean` | `5` | |
| `Decimal32`, `Decimal64`, `Decimal128`, `Decimal256` | `6` | bit width as `u64`, precision as `u64`, scale as `u64` |
| `Date32` | `7` | `32` as `u64`, date unit `Day` |
| `Date64` | `7` | `64` as `u64`, date unit `Millisecond` |
| `Time32`, `Time64` | `8` | bit width as `u64`, time unit |
| `Timestamp` | `9` | time unit, time zone (a single `0` byte if absent, otherwise its length as `u64` followed by its UTF-8 bytes) |
| `List`, `LargeList`, `FixedSizeList` | `11` | item data type |

Type IDs `10` (`Interval`), `12` (`Struct`), `13` (`Union`), `16` (`Map`), and `17` (`Duration`) are reserved.

Units are encoded as `u16`:

| Date Unit | ID |
| --- | --- |
| `Day` | `0` |
| `Millisecond` | `1` |

| Time Unit | ID |
| --- | --- |
| `Second` | `0` |
| `Millisecond` | `1` |
| `Microsecond` | `2` |
| `Nanosecond` | `3` |


### Record Batch Hashing
Data consisting of multiple columns is hashed as follows:
1. *(once per hashing session)* The schema is hashed into the combined digest by traversing all fields recursively in **depth-first** order and hashing for every field:
   - Field name as length in bytes (`u64`) followed by its UTF-8 bytes
   - Nesting level of the field (zero-based) as `u64`
2. Every leaf column (a column that is not a struct) is hashed by a separate column digest using the rules above:
   - Nested columns use a **combined nullability bitmap** produced from the nullability of the column and all of its parent structs
   - Subsequent record batches update the same column digests
3. *(final step)* Digests of all leaf columns are fed into the combined digest in the depth-first order to produce the final hash.


## Checkpoint Hashing
[Checkpoints](../glossary.md#checkpoint) are stored as opaque files and referenced by [Metadata Blocks](../glossary.md#metadata-chain) using their physical hash. The process of computing a hash sum is identical to computing a physical hash for a data part file (see [Physical and Logical Hashes](#physical-and-logical-hashes)).


## Hash Codes
Hashes are represented using [multihash](https://github.com/multiformats/multihash) and [multibase](https://github.com/multiformats/multibase) as described in [Hash Representation](../../open-data-fabric.md#hash-representation).

The `multicodec` table is extended with the following codes in the "private use area":

| Codec | Code |
| --- | --- |
| `arrow0-sha3-256` | `0x300016` |

See also:
- [RFC-002](../../rfcs/002-logical-data-hashes.md)
