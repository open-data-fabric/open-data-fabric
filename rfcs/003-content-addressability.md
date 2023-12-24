# RFC-003: Content Addressability

**Start Date**: 2021-11-20

[![RFC Status](https://img.shields.io/github/issues/detail/state/kamu-data/open-data-fabric/6?label=RFC%20Status)](https://github.com/kamu-data/open-data-fabric/issues/6)

[![Spec PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/17?label=Spec%20PR)](https://github.com/kamu-data/open-data-fabric/pull/17)

## Summary

This RFC specifies how Datasets are uniquely identified across the ODF network, decoupling identity from the aliases.


## Motivation

Datasets are currently identified using their symbolic names. These names are then used, for example, when defining inputs of a derivative dataset or when referring to a dataset in a repository.

Two parties, however, can independently create datasets with conflicting names resulting in high likelihood of name collisions. Allowing users to rename datasets to resolve collisions will inevitably result in breaking the link between derivative dataset and its inputs.

There needs to be a **way to uniquely identify a dataset** on the network, with this identifier being immutable throughout dataset's lifetime and resolvable to find the location(s) of data.


## Guide-level explanation

Per [rationale](#rationale-and-alternatives), we have established that:
- A hash of the last Metadata Block of the dataset can be sufficient to download an entire (subset of a) dataset from a content-addressable storage
- A named reference that points to the last Metadata Block in the chain is needed for identifying dataset as a whole

This RFC therefore suggests some tweaks to the `MetadataBlock` schema to **align it with content-addressability** (see [reference section](#reference-level-explanation)).

Additionally it will **introduce a globally unique dataset identifier** that can be used to refer to a dataset as a whole. This identifier will follow the [W3C DID Identity Scheme](https://w3c.github.io/did-core/). It will be created by hashing a public key of a `ed25519` key pair, practically guaranteeing its uniqueness.

Symbolic names will become aliases for such identities, meaning that:
- Same dataset can have different names in different repositories (e.g. mirrors) while still sharing same identity
- Inputs of the derivative datasets will also be independent of renaming

Conforming to the `DID` specification will allow us in future to expand related mechanisms like proof of control and ownership.


## Reference-level explanation

To make `MetadataBlock` content-addressable we will remove `blockHash` from it to avoid chicken-egg problem of hashing. The metadata hashing procedure will be updated accordingly.

We will also expand the use of `multihash` format proposed in [RFC-002](./002-logical-data-hashes.md) to all hashes, removing the use of `sha3-256` schema format.

New `dataset-name` format will be introduced for dataset names (symbolic aliases) in the same manner as the existing `dataset-id` format.

Existing `dataset-id` schema format will be changed to expect a DID, following these rules:
- ODF datasets will use a custom DID method: `did:odf:...`
- The method-specific identifier will replicate the structure of [`did:key` method](https://w3c-ccg.github.io/did-method-key/) as a self-describing and upgradeable way to store dataset identity represented by a public key.

Dataset identity will be stored in the new `seed` fields in the `MetadataBlock` schema. Seed will be present (only) in the first block of every dataset.

Dataset creation procedure will involve:
- Generating a new cryptographic key pair (defaulting to `ed25519` algorithm)
- Prefixing it with and appropriate `multicodec` identifier (like `ed25519-pub`)
- Storing this data in the first Metadata Block's `seed` field.

When representing dataset ID as a string the DID format `did:odf:<multibase>` will be used, where the binary data will use `multibase` format and `base58btc` encoding just like in `did:key` method.

The `DatasetSource::Derivative` schema will be updated so that inputs specify:
- `id` - unique identity of a dataset
- `name` - symbolic name of an input to be used in queries only.

With the separation of Dataset IDs from Names we will update our [PEG](https://en.wikipedia.org/wiki/Parsing_expression_grammar) grammar to:

```
DatasetRefLocal = DatasetID / DatasetName
DatasetRefRemote = DatasetID / RemoteDatasetName
DatasetRefAny = DatasetRemoteRef / DatasetLocalRef

RemoteDatasetName = RepositoryName "/" (AccountName "/")? DatasetName
AccountName = Subdomain
RepositoryName = Hostname

DatasetName = Hostname
DatasetID = "did:odf:" Multibase

Hostname = Subdomain ("." Subdomain)*
Subdomain = [a-zA-Z0-9]+ ("-" [a-zA-Z0-9]+)*

Multibase = [a-zA-Z0-9+/=]+
```


## Drawbacks

- We are adding more fields to `MetadataBlock` which is already anemic - this will be addressed separately in a follow-up RFC.
- A seemingly unavoidable break in abstraction layers exists where a named reference (higher-level concept) is used from within derivative dataset inputs from metadata (lower-level concept). This is, however, similar to having a HTML page that contains a relative URL of another page. 


## Rationale and alternatives

### Identity of static data in content-addressable systems
The most widespread form of resource identity in decentralized systems today is **content addressability**. Git, Docker & OCI image registries, DHTs, Blockchain, IPFS - resources in these systems are uniquely identifies by (hashes of) their content.

Assuming no hash collisions, this approach allows creating an identity for a resource without any central authority and a risk of collisions. If hashes collide this means resources are in fact identical, leading to a very natural de-duplication of data in the system.

This form of identity, however, is applicable only to static data. When you share a file via IPFS - it's identity is a hash of file's contents. When you modify and share the file again - you get a new identity.

Such form of identity is already perfectly suited for many components of ODF:
- Data part files
- Checkpoints
- Metadata blocks

If we align ODF's hashing with the content-addressable system like IPFS we can get a cool effect:
- A hash of data part file stored in a Metadata Block could be directly used to find and download that data file from IPFS.
- Same goes for the previous metadata block identified by its hash - you could "walk" the metadata chain stored in IPFS in the same way you do on the local drive.

Entire ODF dataset can be stored inside IPFS as its structure maps onto content-addressable storage seamlessly.

### Identity of dynamic data
Unlike all individual components of datasets that are immutable, ODF datasets themselves are dynamic.

In a content-addressable system dataset can be thought of as a "pointer" to the last Metadata Block in the chain. Every time new block is appended - dataset is updated to "point" to the new block. How to represent such "pointer" is ofter referred to as the "naming" problem.

This idea of named pointers is similar to:
- References (e.g. `HEAD`), branches, and tags in `git`
- Image names in Docker / OCI
- IPNS naming service in IPFS

In case of Docker / OCI the naming is centralized - first person to create an organization and push an image with a certain name into the registry wins.

IPNS takes a much more decentralized and conflict-free approach:
- For every "name" a new key pair is generated
- Hash of the public key serves as the name of the pointer
- Private key is used to sign the value to later prove the ownership over it
- Owner writes an entry into DHT with signed value pointing to the hash of IPFS resource
- Entry can be updated when needed

Note that this is just one implementation. Alternative naming schemes using services like DNS and Blockchain exist, but they all have these parts in common:
- A single global-scale system that performs the name resolution (e.g. DNS)
- A mechanism to prove the ownership over a record (e.g. DNS registrar)

Out of this commonality a [W3C Decentralized Identifiers (DIDs) specification](https://w3c.github.io/did-core/) have emerged that provides a common naming scheme, a data model of objects that names resolve into, and mechanisms of proving the control over decentralized identity. IPNS, Blockchains, etc. act as specific implementations of the "verifiable data registries" conforming to DID spec.


## Prior art

- Content addressability in Git, Docker / OCI
- [IPFS](https://ipfs.io/)
  - [naming via IPNS](https://docs.ipfs.io/concepts/ipns/)
  - [naming via DNSLink](https://docs.ipfs.io/concepts/dnslink/)
- [W3C Decentralized Identifiers (DIDs) Specification](https://w3c.github.io/did-core/)
  - [DID Specification Registries](https://www.w3.org/TR/did-spec-registries/)
  - [DID Method Rubric](https://w3c.github.io/did-rubric/)
  - Use of DIDs in the [Ocean Protocol](https://docs.oceanprotocol.com/concepts/did-ddo/)
  - [IPID DID Method](https://did-ipid.github.io/ipid-did-method/)

These are covered in [rationale](#rationale-and-alternatives) above for better flow.


## Unresolved questions


## Future possibilities

- A key pair generated during creation of the dataset identity in future can be used to implement access control and proof of control schemes described in the [W3C DID-Core](https://w3c.github.io/did-core/) specification.

- [IPLD Format](https://ipld.io/) looks a lot similar to what ODF language-independent schemas are trying to accomplish, so we might consider using it in future for better interoperability.
