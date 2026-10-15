# RFC-022: Batch Query Commitments <!-- omit in toc -->

**Start Date**: 2024-07-26\
**Published Date**: 2024-10-01

**Specs**:
- [Verifiable Batch Queries](../specs/exchange/verifiable-batch-queries.md)

**Authors**:
- [Sergii Mikhtoniuk](mailto:sergii.mikhtoniuk@kamu.dev), [Kamu](https://kamu.dev)


**Compatibility**:
- [X] Backwards-compatible
- [ ] Forwards-compatible


## Table of Contents <!-- omit in toc -->

- [Summary](#summary)
- [Motivation](#motivation)
  - [Requirements](#requirements)
  - [Current API](#current-api)
  - [Commitments in the ODF Oracle](#commitments-in-the-odf-oracle)
  - [Need for REST Commitments](#need-for-rest-commitments)
- [Reference-level explanation](#reference-level-explanation)
  - [Structured Recursive Commitments](#structured-recursive-commitments)
  - [JSON Hashing](#json-hashing)
  - [Response Validation](#response-validation)
  - [Storing Commitments](#storing-commitments)
  - [Dispute Resolution](#dispute-resolution)
  - [Requirements Checklist](#requirements-checklist)
  - [Effects on ODF Oracle](#effects-on-odf-oracle)
- [Compatibility](#compatibility)
- [Drawbacks](#drawbacks)
- [Rationale and alternatives](#rationale-and-alternatives)
  - [HTTP Message Signatures](#http-message-signatures)
- [Prior art](#prior-art)
- [Unresolved questions](#unresolved-questions)
- [Future possibilities](#future-possibilities)


## Summary
Proposes a new commitment scheme for REST Batch Query API results and the process of opening (verifying) such commitments.


## Motivation
When a user queries data, we want to establish an unbreakable cryptographic commitment that allows the user to **hold every party in the supply chain accountable** for veracity of the result.

This accountability should persist long after the response is provided, so that participants can process data fast in "**optimistic**" fashion, and can **verify and dispute the results later**.

This mechanism in future will allow us to build **automatic dispute resolution** mechanisms within the ODF network that financially penalize malicious actors.


### Requirements
- **Hiding** - Commitment should not reveal any information about the operation.
- **Binding** - It should be impossible to change any inputs or outputs of the operation without invalidating the commitment.
- **Succinct** - Commitment itself should be small enough to e.g. store on-chain, and the process of disputing the operation results should not require storing and transferring the resulting data.
- **Deterministic** - Same query should result in the same commitment as long as the query itself is deterministic.
- **Reproducible** - Given all the same inputs of the operation a third party should be able to repeat the query, arrive at the same results, and be able to successfully validate them against the original commitment.
  - Reproducibility will be the *primary way of verifying the commitments* for now, until we introduce zero-knowledge computation proofs.
- **Non-repudiable** - A prover cannot dispute performing the operation. There exists a mechanism that ties its identity to the provided commitment. This mechanism may be separate from the commitment itself to keep the latter pure zero-knowledge.
- **Composable** - It should be possible to split computation into several steps and compose their commitments into one final commitment. When joining two datasets from different owners the entire execution may be split into sub-plans that are executed by different nodes belonging to different parties. In the event of a dispute, we should be able to assign responsibility to the specific party and not just stop at the party that performed the final result aggregation.
- **Multi-protocol** - Ideally the commitment structure can be reused without significant alterations in other protocols like Blockchain Oracle, Flight SQL, Kafka egress, WebSockets etc.


### Current API
Our initial attempt to add a resemblance of cryptographic commitments to the REST Batch Query API looked like this:

```
POST /query
Host: node.demo.kamu.dev

{
  "query": "select offset, city, population from populations order by offset desc",
  "dataFormat": "JsonAoA",
  "aliases": [{
    "alias": "populations",
    "id": "did:odf:fed0..ee02"
  }]
}
```

```
200 OK

{
  "data": [
    [1, "B", 200],
    [0, "A", 100]
  ],
  "schema": "...",
  "resultHash": "f968..656e",
  "state": {
    "inputs": [{
      "id": "did:odf:fed0..ee02",
      "blockHash": "f162..3e0b"
    }]
  }
}
```

In this case the commitment consists of `resultHash` and `state` properties.

- **Hiding** - **NO**
  - `state` reveals in plain text what datasets were queried.
- **Binding** - **NO**
  - Commitment does not cover the inputs like `query`, `aliases` and other parameters like `dataFormat`, row limits etc.
  - Commitment does not cover outputs like `schema`
- **Succinct** - **YES**
- **Deterministic** - **YES**
- **Reproducible** - **NO**
  - The `state` information allows the user to run the query on the same exact input data
  - But the query itself is not part of the commitment. This creates a loophole: when the result is disputed, a node operator can claim that this result was provided for a slightly different query than what the user is showing.
- **Non-repudiable** - **NO**
  - There is no mechanism that ties the result to a node that executed it.
- **Composable** - **NO**
- **Multi-protocol** - **NO**


### Commitments in the ODF Oracle
Although purely in REST API context this attempt looked like a failure, it actually provided a good starting point for ODF Oracle commitments on the blockchain.

- **Hiding** - **NO**
  - The entire request and response data is publicly available, but this is an inherent limitation of non-ZK blockchains
- **Binding** - **YES**
  - All inputs and outputs are immutably stored on-chain
- **Succinct** - **YES** / **NO**
  - All response data ends up stored which we cannot avoid, but the commitment itself does not add much overhead
- **Deterministic** - **YES**
- **Reproducible** - **YES**
  - Original query with all of its parameters is stored on-chain and can be repeated against the `state` information in response
- **Non-repudiable** - **YES**
  - Automatically achieved as the provider signs the response transaction with its private key
- **Composable** - **NO**
- **Multi-protocol** - **NO**


### Need for REST Commitments
As you can see, in combination with properties added by blockchain, the existing API fulfills the oracle use case pretty well.

However, now we are starting to work with clients that require commitments from REST API alone. This is, for example, important in the scenario where ODF nodes are used to provide factual data to LLM chatbots - the calling party will interact only with REST API and needs to have an ability to store the commitment in the conversation history.


## Reference-level explanation

### Structured Recursive Commitments

```
POST /query
Host: node.demo.kamu.dev

{
  "query": "select * from foo",
  "queryDialect": "SqlDataFusion",
  "dataFormat": "JsonAoS",
  "schemaFormat": "ArrowJson",
  "signed": true
}
```

```
200 OK

{
  # Input section that can be used to fully reproduce the result of operation
  "input": {
    "query": "select * from foo",
    "queryDialect": "SqlDataFusion",
    "dataFormat": "JsonAoS",
    "schemaFormat": "ArrowJson",
    "datasets": [{
      "id": "did:odf:fed0..ee02",
      "alias": "foo",
      "blockHash": "f162..3e0b",
    }]
  },

  # The result
  "output": {
    "data": [
      {"foo": 1, "bar": "A"},
      {"foo": 2, "bar": "B"}
    ],
    "schema": "{\"fields\": [...]}",
  },

  # Information about processing performed by other nodes as part of this operation
  # that are identical in structure to the top-level response (recursive)
  "subQueries": [{
    "input": {...},
    "subQueries": [...],
    "commitment": {...},
    "proof": {...},
  }]

  # Succinct commitment
  "commitment": {
    # Input hash: multihash(canonicalize(input))
    "inputHash": "f162..3e0b",

    # Output hash: multihash(canonicalize(output))
    "outputHash": "f162..baaf",

    # Sub-queries hash: multihash(canonicalize(subQueries))
    "subQueriesHash": "f162..8dae",
  }

  # Signature block
  "proof": {
    "type": "Ed25519Signature2020",

    # DID (public key) of the node performing the computation
    "verificationMethod": "did:key:fed0..3e41",

    # Signature: multibase(sign(canonicalize(commitment)))
    "proofValue": "u3MvG...ouQk"
  }
}
```

Note: Technically the `commitment` block is redundant and the caller could compute all hashes themselves. In fact they have to do so to validate the response (see below). We include this block purely to:
- Protect clients from implementing hashing in a way that is not consistent with the specification
- Provide more information during the validation and dispute stages to isolate inconsistencies to specific parts.


### JSON Hashing
Note that the proposed scheme involves hashing JSON objects. To exclude variations like whitespace characters and property order differences between JSON library implementations we need to convert it into canonical form as specified in [RFC 8785: JSON Canonicalization Scheme](https://datatracker.ietf.org/doc/html/rfc8785).


### Response Validation
Immediately upon receiving the response the caller must validate it to ensure that the commitment is not bogus and is disputable:
1. Verify that `proof.verificationMethod` DID belongs to the node you are requesting data from
2. Verify `commitment.outputHash` by re-computing the hash
3. Verify `commitment.inputHash` by re-computing the hash
4. Verify `commitment.subQueriesHash` by re-computing the hash
5. Verify the signature in `proof.proofValue` using
   1. Message: `canonicalize(commitment)`
   2. Public key: `key_from_did(proof.verificationMethod)`
6. **Repeat the steps 3-5 recursively** for every entry in `subQueries`


### Storing Commitments
To be able to dispute the result in future, consumer should store the following fields:
- `input`
- `subQueries`
- `commitment`
- `proof`

These different parts can be also used separately for different purposes as they all provide different "layers" of verifiability:
- `input` - provides reproducibility
- `commitment` - seals the response against tampering
- `proof` - adds non-repudiation via signature
- `subQueries` - provide the above for multi-party computations


### Dispute Resolution
The procedure of resolving a dispute using a commitment via reproducibility is as follows:
1. Consumer submits a claim to the verifier by providing:
   1. `input`
   2. `commitment`
   3. `subQueries`
   4. `proof`
2. Verifier obtains the relevant `input.datasets`
3. Verifier validates the claim according to the same rules as described in [Response Validation](#response-validation)
   1. If any check fails the claim is **rejected** as invalid
4. Verifier checks that all `authority` specified in `subQueries` are registered nodes
   1. If an unregistered node is found the claim is **accepted** and the blame is assigned to the root `authority` for using a bogus node for computing part of the response
5. Verifier generates the logical query plan and checks that the plan is identical or equivalent to the plan composed from the `subQueries`
   1. If plan does not correspond to the original query the claim is **accepted** and the blame is assigned to the root `authority` for providing a malicious query plan
6. In a bottom-up fashion Verifier repeats all the computation steps using `input` and compares hashes of outputs to those captured in `commitment.outputHash`
   1. If a hash doesn't match the claim is **accepted** and the blame is assigned to the corresponding `authority` for providing a malicious output data


### Requirements Checklist
- **Hiding** - **YES** / **NO**
  - The hashes in the `commitment` block can be used as zero-knowledge commitments that do not reveal any information
  - The `proof` reveals the identity of the node performing the computation
- **Binding** - **YES**
- **Deterministic** - **YES**
  - Assuming canonical response format
- **Reproducible** - **YES**
  - Using the `input` field gives us all the necessary info to reproduce the request
- **Non-repudiable** - **YES**
  - Through the identity and signature in `proof`
- **Composable** - **YES**
  - Through `subQueries`
- **Multi-protocol** - **YES**
  - A similar scheme can be implemented in other protocols e.g. Flight SQL using canonical structs in FlatBuffers format.


### Effects on ODF Oracle
Oracle response structure will be modified to include all the elements of the new REST API response.

Some of them may be a little redundant: e.g. root-level authority and signature are not needed, because we have the address and transaction signature of the provider.

But storing these fields will allow us to use the same dispute resolution mechanism for oracle responses as for other protocols, and avoid the need to map provider addresses to node DIDs on multiple blockchains.


## Compatibility
The new response structure replaces the `resultHash` and `state` properties of the existing REST Batch Query API response, so clients relying on them will need to be updated.


## Drawbacks
- Responses become larger and more nested, as they now include the full `input` section, sub-queries, and the signature block.
- Validating a response requires clients to implement JSON canonicalization, hashing, and signature verification.


## Rationale and alternatives

### HTTP Message Signatures
[RFC 9421: HTTP Message Signatures](https://datatracker.ietf.org/doc/rfc9421/) proposes a standard for signing HTTP requests and responses.

Under this scheme a query operation could look like this:

```
POST /query
Host: node.demo.kamu.dev
Accept-Signature: sig=(@status, ...)

{
  "query": "select * from foo",
  "queryFormat": "SqlDataFusion",
  "aliases": [{"alias": "foo", "id": "did:odf:fed0..ee02"}],
  "dataFormat": "JsonAoS"
}
```

```
200 OK
Signature-Input: (@status, ...)
Signature: sig=:...:

{
  "data": [
    {"foo": 1, "bar": "A"},
    {"foo": 2, "bar": "B"}
  ],
  "dataFormat": "JsonAoS",
  "query": "select * from foo",
  "queryFormat": "SqlDataFusion",
  "aliases": [{"alias": "foo", "id": "did:odf:fed0..ee02"}],
  "state": [{"id": "did:odf:fed0..ee02", "blockHash": "f162..3e0b"}]
}
```

In this case the commitment would consist of the entire HTTP response (excluding only some headers that are not part of the `Signature-Input`).

- **Hiding** - **YES** / **NO**
  - Depending on the algorithm, signature may reveal the public key of the signer and therefore the identity of a node that provided the response.
  - RFC specifies `Content-Digest` header that may be included into the signature instead of the entire payload, thus hiding the response data
- **Binding** - **YES**
- **Succinct** - **NO**
  - The signature itself is small and constant in size, however to dispute the operation a client will have to hold on to the entire response data to show the validity of a signature before verifier attempts to reproduce the request - this would be impractical at scale
- **Deterministic** - **YES**
  - Assuming stable / canonical response format
- **Reproducible** - **YES**
  - Note that the response would include the query and all other input parameters, so removing the output fields like `data` and `schema` gives us all the necessary inputs to reproduce the request
- **Non-repudiable** - **YES**
  - Through the signature
- **Composable** - **NO**
  - Including commitments from other nodes that performed parts of the request will require some different mechanism where HTTP signatures are included as parts of the JSON response
  - Nodes may not even use HTTP when communicating with one another and use Flight SQL instead.
- **Multi-protocol** - **NO**
  - This scheme is HTTP specific - other protocols will have to use other methods.


## Prior art
- [RFC 9421: HTTP Message Signatures](https://datatracker.ietf.org/doc/rfc9421/) - see [Rationale and alternatives](#http-message-signatures)
- [RFC 8785: JSON Canonicalization Scheme](https://datatracker.ietf.org/doc/html/rfc8785) - used for hashing JSON objects
- [W3C Verifiable Credentials Data Integrity](https://www.w3.org/TR/vc-data-integrity/) - structure of the `proof` block (`type`, `verificationMethod`, `proofValue`)


## Unresolved questions
- Do we need to include a proof that a specific `blockHash` actually exists, e.g. by including a signature? Otherwise would it be too easy for a malicious prover to say that the dataset altered its history and they cannot be held accountable for a non-reproducible result?
- Checking that the root SQL query corresponds to the `subQueries` plans will be a difficult task, but there doesn't seem to be an alternative. Even with ZK proofs we will likely hit this problem of computation equivalence.


## Future possibilities
- Zero-knowledge proofs are currently very limited in terms of data types and functions they can support, with JOINs being especially problematic. They can be introduced progressively, where possible for simpler queries, as an alternative to verification through signing and reproducibility.
- Commitments can serve as a foundation for automatic dispute resolution in the ODF network that financially penalizes malicious actors.
