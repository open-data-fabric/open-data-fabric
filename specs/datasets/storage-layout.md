# Storage Layout <!-- omit in toc -->

**Domain**: Datasets
**Depends on**: [Data Model](data-model.md), [Data & Checkpoint Hashing](data-and-checkpoint-hashing.md)
**RFCs**: [RFC-003](../../rfcs/003-content-addressability.md), [RFC-006](../../rfcs/006-checkpoints-as-files.md), [RFC-017](../../rfcs/017-large-files-linking.md)


## Abstract <!-- omit in toc -->

This spec defines how components of a [Dataset](../glossary.md#dataset) - metadata blocks, references, data part files, checkpoints, and linked objects - are laid out in file systems, conventional object stores, and content-addressable and permanent storage networks.


## Table of Contents <!-- omit in toc -->

- [Dataset Components](#dataset-components)
  - [Checkpoints](#checkpoints)
  - [Linked Objects](#linked-objects)
- [Layout in File Systems and Object Stores](#layout-in-file-systems-and-object-stores)
- [Layout in Content-Addressable Stores](#layout-in-content-addressable-stores)
  - [Incremental Updates](#incremental-updates)
  - [Naming and Resolution](#naming-and-resolution)
  - [Immutability Considerations](#immutability-considerations)


## Dataset Components
A dataset is stored as a set of objects grouped into the following sections:

| Section | Contents | Object key |
| --- | --- | --- |
| `blocks` | Blocks of the [Metadata Chain](../glossary.md#metadata-chain) | Hash of the block |
| `refs` | Named references to blocks | Name of the reference |
| `data` | [Data Slice](../glossary.md#data-slice) part files and [linked objects](#linked-objects) | Physical hash of the file |
| `checkpoints` | [Checkpoint](../glossary.md#checkpoint) files | Physical hash of the file |

Hashes used as object keys are represented as [multihash](https://github.com/multiformats/multihash) values encoded with [multibase](https://github.com/multiformats/multibase) (see [Hash Representation](metadata-chain.md#hash-representation)), e.g. `f16207e15328e0fcfe47339273e4a00f59f210891d45026d84b71c6b40eecf237ec24`. For data part files and checkpoints this is the physical hash (see [Data & Checkpoint Hashing](data-and-checkpoint-hashing.md)).

Reference objects contain the hash of the block they point to. Every dataset has a `head` reference that points to the last block in the Metadata Chain. Other references (e.g. for branches and tags) may appear in the `refs` section, but their semantics are not yet specified.

All objects except references are **immutable** and **content-addressed** - they are uniquely identified by the hash of their content. Having only the hash of the `head` block one can traverse the entire Metadata Chain following the `prevBlockHash` links and discover all data part files and checkpoints referenced by the blocks. Hash of the `head` block thus acts as a Merkle root of the entire dataset.


### Checkpoints
A checkpoint must always be a single file. Engines that need to persist state as multiple files or directories must combine them into one file (e.g. using `tar`).

This ensures that checkpoints can be hashed the same way as data part files, and that a dataset can be transferred via protocols that don't support listing directories.


### Linked Objects
> **Experimental:** See [RFC-017](../../rfcs/017-large-files-linking.md).

Large binary objects (e.g. images, videos, documents) linked to a dataset by hash from data columns are stored in the `data` section alongside data part files, keyed by their multihash. Since links are part of the data, the hash of the `head` block also acts as a Merkle root of all linked objects, preserving tamper-proof properties of the dataset.


## Layout in File Systems and Object Stores
When storing a dataset in a file system or a conventional object store (e.g. S3), sections are represented as directories (or key prefixes) and objects as files within them:

```
{dataset-root}/
├── blocks/
│   └── {block-hash}
├── refs/
│   └── head
├── data/
│   ├── {data-file-physical-hash}
│   └── {linked-object-hash}
└── checkpoints/
    └── {checkpoint-physical-hash}
```

This layout must be used when sharing datasets via file or object-based [Repositories](../glossary.md#repository) (e.g. local FS, S3, HTTP servers).

Accessing a dataset in this layout only requires the ability to **read an object by key** - it purposefully does not rely on any advanced storage features like listing directories. This allows datasets to be transferred using the [Simple Transfer Protocol](../exchange/transfer-protocols.md#simple-transfer-protocol) on top of any storage that supports Unix path-like keys.

When a [Dataset](../glossary.md#dataset) is imported locally, the exact layout is left entirely up to the [Coordinator](../glossary.md#coordinator) implementation, as we expect all interactions with the [Dataset](../glossary.md#dataset) to go through it. Implementations may also store additional implementation-specific information (e.g. configuration, caches, summaries) alongside the dataset, but such information is not a part of the dataset and must not be relied upon by other implementations.

See Also:
- [Dataset Sharing](../exchange/transfer-protocols.md#dataset-sharing)


## Layout in Content-Addressable Stores
Since all dataset components except references are immutable and content-addressed, datasets map naturally onto content-addressable and permanent storage networks like [IPFS](https://ipfs.tech/) and [Arweave](https://www.arweave.org/).

A dataset is stored in such networks using the same [layout](#layout-in-file-systems-and-object-stores) as above, represented via the network's conventional file and directory format (e.g. UnixFS in IPFS, ArFS in Arweave). This allows datasets to be stored and retrieved using generic tooling of the network that has no knowledge of ODF.

Storage networks identify stored data by their own **locators** (e.g. IPFS CIDs, Arweave transaction IDs). Locators generally differ from ODF hashes of the same objects, as they depend on network-specific settings like chunking of large files, DAG structure, or transaction boundaries. Objects are therefore located by their paths within the dataset directory rather than by their ODF hashes directly.

Networks that provide HTTP gateways allow reading datasets using the [Simple Transfer Protocol](../exchange/transfer-protocols.md#simple-transfer-protocol), as it only requires the ability to read objects by their paths.

Because components of a dataset are identical regardless of where they are stored, the same dataset can be stored in multiple places simultaneously, e.g. in a conventional object store for low-latency access and in a permanent storage network for archival and censorship-resistant access. Consistency between such replicas can be verified by comparing the hashes of their `head` blocks.


### Incremental Updates
When a dataset is updated, only the new blocks, data part files, and checkpoints (the **delta**) are added to the network - objects stored previously are reused without duplication.

Depending on the network:
- In networks that support directory structures with structural sharing (e.g. IPFS), every update produces a new directory locator, while previous locators remain valid and point to older states of the dataset
- In append-only networks where every upload is a separate transaction (e.g. Arweave), every upload containing a delta should reference the previous upload of the same dataset, so that having only the latest locator one can discover all uploads that constitute the dataset

Uploads may be batched - a single upload can contain deltas of multiple updates of a dataset, which may reduce the cost of storage in networks where every transaction incurs fees.


### Naming and Resolution
Since locators change with every update, an additional mechanism is needed to resolve a stable dataset identity (see [Identity](../../open-data-fabric.md#dataset-identity)) or name into the locator of its latest state. Such mechanisms are network-specific, e.g. mutable names in IPFS ([IPNS](https://docs.ipfs.tech/concepts/ipns/), [DNSLink](https://docs.ipfs.tech/concepts/dnslink/)), or transaction tags indexed by gateways in Arweave.

Resolution should not depend on infrastructure operated by a single party. Wherever possible, information needed for resolution (e.g. dataset ID and the hash of the `head` block) should be stored alongside the data in the network itself, so that it can be independently verified, and the mapping can be reconstructed by indexing the network if needed.


### Immutability Considerations
Data stored in permanent storage networks cannot be reliably deleted:
- Hard compactions and other forms of erasure of history will not remove previously stored objects from the network
- Data that was published unencrypted cannot be taken down

Sensitive data should therefore only be stored in such networks in encrypted form. In this case, erasure can be achieved by permanently revoking access to decryption keys.
