# RFC-002: Logical Data Digests

**Start Date**: 2021-11-15

[![RFC Status](https://img.shields.io/github/issues/detail/state/kamu-data/open-data-fabric/1?label=RFC%20Status)](https://github.com/kamu-data/open-data-fabric/issues/1)

[![Spec PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/16?label=Spec%20PR)](https://github.com/kamu-data/open-data-fabric/pull/16)

## Summary

This RFC standardizes how equality, equivalence, and integrity checks are performed on data part files.

## Motivation

Use cases we want to cover are:
- **Integrity Checks** - ensuring that data is not tampered after downloading it from untrusted source
- **Equivalence Checks** - after replaying the transformation ensure result is the same as one presented by the dataset owner (transformation result is not spoofed)
- **Content Addressability** - being able to store data in a content-addressable system (e.g. IPFS)

A standard practice for these is through use of **cryptographic digests** (hash sums). Git, blockchain, Docker, IPFS are few of many examples.

The one big problem with this approach is **reproducibility**. ODF's core guarantee that repeating the same transformation always produces identical result. Reproducibility requires perfect determinism. While ODF goes a long way in improving likelihood of determinism in the engines, some non-determinism may also come from the coordinator.

The task of writing output data is delegated to the coordinator. We are currently using Parquet format for on-disk representation. Parquet format is **non-reproducible** (non-deterministic) - writing logically identical data with different implementations or different versions of the libraries may produce files that are different on the binary level.

Things that contribute to this are: heuristics around encodings (e.g. RLE vs bitpack), heuristics around batch sizes, different settings for statistics, etc.

Therefore, if coordinators would use a hash sum of a Parquet file for integrity/equivalence checks they would be likely to break with any changes to Parquet libraries and produce different hashes between different coordinator implementations.

## Guide-level explanation

### Logical Hashes
To isolate ourselves from non-determinism of Parquet storage format we will use a combination of two types of data hashes:
- **Physical hash** - is a (non-reproducible) digest of an entire binary file as originally produced by the owner of the dataset
- **Logical hash** - is a stable (reproducible) digest computed on data records, resistant to encoding variations

**Physical hash** will be used only in the context of uploading and downloading data files to/from content-addressable systems.

**Logical hash** will be used for integrity and equivalence checks. It should maximize the chances that logically identical data results in an identical hash sum.

Luckily, we already use a data format that specifies binary layouts of data in the memory - **Apache Arrow**. It has a lot of similar sources of non-determinism as Parquet (e.g. data cell marked as `null` by validity mask might have uninitialized memory in it), but can be used as a foundation for **efficiently** implementing a stable hash algorithm.

Due to absence of existing solutions it was decided to implement our own hashing scheme on top of Apache Arrow format as described in detail in [arrow-digest](https://github.com/sergiimk/arrow-digest) repository.

### Future-Proofing
This is likely not the last time we change the hashing algorithm, but we should aim for such changes not to be breaking in the future.

As algorithms may have vulnerabilities, or as hash lengths become less secure with advances in computing power, we need an ability to evolve the algorithms we use over time. To future-proof metadata we will be storing the hashing algorithm identifier alongside the hash value, and coordinator implementations will be able to differentiate between versions and decide how to deal with them.

This problem is already addressed by the [multiformats](https://github.com/multiformats/multiformats) project, specifically:
- [multihash](https://github.com/multiformats/multihash) - describes how to encode algorithm ID alongside the hash value.
- [multibase](https://github.com/multiformats/multibase) - describes how to represent binary value in a string (e.g. for YAML format) without ambiguity of which encoding was used.

In the future, we will apply this scheme to all hashes, but for now only limiting them to data-related.

## Reference-level explanation

New schema format `multihash` is introduced:
- In binary form it is using [multihash](https://github.com/multiformats/multihash) specification
- In string form it is encoded using [multibase](https://github.com/multiformats/multibase) specification, using `base58` codec

The official `multicodec` table will be extended with the following codes in the "private use area":

| Codec             | Code       |
|-------------------|------------|
| `arrow0-sha3-256` | `0x300016` |

Because `multihash` format has a static prefix the short (8-character) hash representation should use tail bytes of multihash instead of the head.

`OutputSlice` schema is updated:
- `dataLogicalHash` updated to use `multihash` format
- new field `dataPhysicalHash` is added with `multihash` format

`dataLogicalHash` is updated to use [arrow-digest](https://github.com/sergiimk/arrow-digest) with `sha3-256` base algorithm.

`dataPhysicalHash` will contain a `sha3-256` hash of the data part file.

`ExecuteQueryResponse` schema is updated not to carry hash information - this was a temporary solution before coordinator hashing support is available.

## Drawbacks

- Logical hash equality doesn't guarantee that data file was not tampered with (see [unresolved questions](#unresolved-questions))
- Maintaining two hashes add a lot of complexity - it would be much nicer to have just physical hash and a reproducible file format (see [future possibilities](#future-possibilities))

## Rationale and alternatives

- Maintain separate logical and physical hashes?
- Hash Arrow in-memory representation for stability and performance?
- Achieve deterministic layout of Parquet and only have physical hash that is stable?

## Prior art

Hashing of structured data is mostly used in the context of buckets for hash-based join operations etc. We haven't seen Arrow/Parquet hashing used in cryptographic context.

We did not consider any other formats as we have strong reasons to continue using Arrow and Parquet and encoding in a third format will be wasteful.

- [Our Apache Arrow User mailing list thread on format reproducibility](https://lists.apache.org/thread/fndxck757vfkxd8wx5smspmov9nrzvft)
- Relevant issues:
  - https://issues.apache.org/jira/browse/ARROW-3978
  - https://issues.apache.org/jira/browse/ARROW-8991
  - https://issues.apache.org/jira/browse/ARROW-6024
  - https://issues.apache.org/jira/browse/ARROW-6030
  - https://issues.apache.org/jira/browse/ARROW-11266

## Unresolved questions

- Checking only the logical hash to verify the part file may expose us to attacks that tamper non-data blocks like statistics. Tampering the Parquet statistics may result in exclusion of some records from query results that use predicate pushdown
  - To mitigate this we may in future incorporate statistics sanity checks into data validation process

## Future possibilities

- To avoid maintaining two hashes we might want instead to implement a structured data format that is fully deterministic
  - It can be a complete subset of Parquet, to stick with a widespread format
  - Coordinator implementations will have to use a special writer that will have to be implemented in different languages
    - This is already the case with `arrow-digest`, but hashing is a smaller problem to tackle
- We may want to remove the existing limitation that data part files correspond 1-to-1 with metadata blocks (e.g. for compactions) - this is left out for future RFCs
