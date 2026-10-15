# Verifiable Batch Queries <!-- omit in toc -->

**Domain**: Exchange\
**Depends on**: -\
**RFCs**: [RFC-022](../../rfcs/022-batch-query-commitments.md)


## Abstract <!-- omit in toc -->

This spec defines a commitment scheme for results of batch queries executed by ODF nodes. A commitment cryptographically binds the result of a [Query](../glossary.md#query) to all of its inputs, including the exact states of the queried [Datasets](../glossary.md#dataset), and to the identity of the node that executed it. This allows consumers to process query results optimistically and hold every party involved in producing the result accountable long after the response was received, by verifying and disputing the results later.


## Table of Contents <!-- omit in toc -->

- [Commitment Properties](#commitment-properties)
- [Response Structure](#response-structure)
  - [Input](#input)
  - [Output](#output)
  - [Sub-queries](#sub-queries)
  - [Commitment](#commitment)
  - [Proof](#proof)
- [JSON Hashing](#json-hashing)
- [Response Validation](#response-validation)
- [Storing Commitments](#storing-commitments)
- [Dispute Resolution](#dispute-resolution)
- [Use in Other Protocols](#use-in-other-protocols)


## Commitment Properties
The commitment scheme provides the following properties:
- **Hiding** - The commitment does not reveal any information about the operation. The hashes in the [`commitment`](#commitment) section don't reveal inputs or outputs, while the [`proof`](#proof) section reveals only the identity of the node that performed the computation.
- **Binding** - It is impossible to change any inputs or outputs of the operation without invalidating the commitment.
- **Succinct** - The commitment is small enough to be stored cheaply (e.g. on a blockchain), and disputing the result does not require storing and transferring the resulting data.
- **Deterministic** - The same query results in the same commitment, as long as the query itself is deterministic.
- **Reproducible** - Given the same inputs a third party can repeat the query, arrive at the same results, and validate them against the original commitment.
- **Non-repudiable** - A node cannot deny performing the operation, as the commitment is signed with the key tied to the node's identity.
- **Composable** - Computation split into several steps, possibly executed by nodes belonging to different parties (e.g. when joining datasets of different owners), produces commitments that compose into one final commitment. In case of a dispute the responsibility can be assigned to the specific party, not only to the party that aggregated the final result.
- **Multi-protocol** - The commitment structure can be reused across different protocols without significant alterations.

Until zero-knowledge computation proofs are introduced, reproducibility (see [Verifiability](../glossary.md#verifiability)) is the primary way of verifying the commitments.


## Response Structure
A node executing a query that was requested to be verifiable returns a response consisting of the following sections:

```json
{
  "input": {
    "query": "select * from foo",
    "queryDialect": "SqlDataFusion",
    "dataFormat": "JsonAoS",
    "schemaFormat": "ArrowJson",
    "datasets": [{
      "id": "did:odf:fed0..ee02",
      "alias": "foo",
      "blockHash": "f162..3e0b"
    }]
  },
  "output": {
    "data": [
      {"foo": 1, "bar": "A"},
      {"foo": 2, "bar": "B"}
    ],
    "dataFormat": "JsonAoS",
    "schema": {"fields": [...]},
    "schemaFormat": "ArrowJson"
  },
  "subQueries": [{
    "input": {...},
    "subQueries": [...],
    "commitment": {...},
    "proof": {...}
  }],
  "commitment": {
    "inputHash": "f162..3e0b",
    "outputHash": "f162..baaf",
    "subQueriesHash": "f162..8dae"
  },
  "proof": {
    "type": "Ed25519Signature2020",
    "verificationMethod": "did:key:fed0..3e41",
    "proofValue": "u3MvG...ouQk"
  }
}
```


### Input
The `input` section contains all information needed to fully reproduce the result of the operation:
- The query and its dialect
- All parameters that affect the output, including the requested data and schema formats
- The list of queried datasets, each with:
  - `id` - the [unique identifier](../datasets/identity.md#unique-identifiers) of the dataset
  - `alias` - the name under which the dataset was referred to in the query
  - `blockHash` - hash of the [Metadata Chain](../glossary.md#metadata-chain) block that represents the exact state of the dataset the query was executed against

Datasets that were referenced in the request by aliases are resolved by the node, and states of all datasets are pinned to specific blocks, so that the query can later be repeated against the exact same data.


### Output
The `output` section contains the result data and its schema in the requested formats.


### Sub-queries
The `subQueries` section contains information about parts of the operation that were executed by other nodes (e.g. in federated queries). Every entry has the same structure as the top-level response, excluding the `output` section, making the scheme recursive. Sub-queries are empty when the entire operation was executed by a single node.


### Commitment
The `commitment` section is a succinct commitment to the operation, consisting of:
- `inputHash` - `multihash(canonicalize(input))`
- `outputHash` - `multihash(canonicalize(output))`
- `subQueriesHash` - `multihash(canonicalize(subQueries))`

Hashes are represented as described in [Hash Representation](../datasets/metadata-chain.md#hash-representation), and `canonicalize` is defined in [JSON Hashing](#json-hashing).

The `commitment` section is technically redundant, as the consumer has to recompute all hashes to [validate](#response-validation) the response anyway. It is included to protect consumers from implementing hashing in a way that is inconsistent with this spec, and to help isolate inconsistencies to specific parts of the response during validation and dispute resolution.


### Proof
The `proof` section ties the commitment to the identity of the node that performed the computation:
- `type` - signature scheme (e.g. `Ed25519Signature2020`)
- `verificationMethod` - DID of the node, from which the public key used to verify the signature can be derived
- `proofValue` - `multibase(sign(canonicalize(commitment)))`


## JSON Hashing
To exclude variations in whitespace, property order, and number representation between JSON libraries, JSON objects are converted into canonical form as specified in [RFC 8785: JSON Canonicalization Scheme](https://datatracker.ietf.org/doc/html/rfc8785) before hashing or signing.


## Response Validation
Immediately upon receiving the response the consumer must validate it to ensure that the commitment is not bogus and is disputable:
1. Verify that the `proof.verificationMethod` DID belongs to the node the data was requested from
2. Verify `commitment.outputHash` by recomputing the hash
3. Verify `commitment.inputHash` by recomputing the hash
4. Verify `commitment.subQueriesHash` by recomputing the hash
5. Verify the signature in `proof.proofValue` using:
   - Message: `canonicalize(commitment)`
   - Public key: `key_from_did(proof.verificationMethod)`
6. Repeat steps 3-5 recursively for every entry in `subQueries`


## Storing Commitments
To be able to dispute the result in future, the consumer should store the following sections:
- `input`
- `subQueries`
- `commitment`
- `proof`

The `output` section doesn't need to be stored, as it can be reproduced from the `input`.

These sections provide different layers of verifiability and can be used separately:
- `input` - provides reproducibility
- `commitment` - seals the response against tampering
- `proof` - adds non-repudiation via signature
- `subQueries` - provide the above for multi-party computations


## Dispute Resolution
The procedure of resolving a dispute using a commitment via reproducibility is as follows:
1. Consumer submits a claim to the verifier, providing the `input`, `subQueries`, `commitment`, and `proof` sections
2. Verifier obtains the datasets listed in `input.datasets` at the specified block hashes
3. Verifier validates the claim according to the [Response Validation](#response-validation) rules
   - If any check fails, the claim is **rejected** as invalid
4. Verifier checks that all nodes that signed the `subQueries` are known (registered) nodes
   - If an unknown node is found, the claim is **accepted** and the blame is assigned to the root node for using a bogus node to compute part of the response
5. Verifier generates the logical query plan and checks that it is identical or equivalent to the plan composed from the `subQueries`
   - If the plan does not correspond to the original query, the claim is **accepted** and the blame is assigned to the root node for providing a malicious query plan
6. In a bottom-up fashion, verifier repeats all computation steps using their `input` sections and compares hashes of the outputs to the corresponding `commitment.outputHash` values
   - If a hash doesn't match, the claim is **accepted** and the blame is assigned to the node that signed the corresponding commitment for providing malicious output data


## Use in Other Protocols
While described here using JSON responses of a REST API, the same structure can be used in other protocols, e.g. Flight SQL using canonical structures in a binary format, or blockchain oracles.

In protocols that already provide some of these properties some sections may be redundant. E.g. a blockchain oracle response is already signed by the provider's transaction, making the root-level `proof` unnecessary. Including all sections nevertheless allows using the same dispute resolution procedure across protocols, without the need to map provider addresses on different blockchains to node identities.
