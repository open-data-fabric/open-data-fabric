# RFC-018: Infrastructure-as-Code Resource Framework <!-- omit in toc -->

[![Issue](https://img.shields.io/github/issues/detail/state/kamu-data/open-data-fabric/108?label=Issue)](https://github.com/kamu-data/open-data-fabric/issues/108)
[![PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/116?label=PR)](https://github.com/kamu-data/open-data-fabric/pull/116)

**Start Date**: 2025-06-14

**Published Date**: 2025-08-25

**Authors**:
- [Sergiy Zaychenko](mailto:sergiy.zaychenko@kamu.dev), [Kamu](https://kamu.dev)
- [Sergii Mikhtoniuk](mailto:smikhtoniuk@kamu.dev), [Kamu](https://kamu.dev)


**Compatibility**:
- [X] Backwards-compatible
- [ ] Forwards-compatible

<!--
Backwards-compatible means:
- whether software updated to this RFC will be able to operate on old data
- in case of protocol updates - whether older clients will be able to communicate with newer servers

Forwards-compatible means:
- whether data written by new software still can be used by older software
- if newer clients will be able to communicate with older servers
-->


## Summary <!-- omit in toc -->
<!--
One paragraph explanation of the feature.
-->

This RFC proposes a new Open Data Fabric manifests format and a set of resource types aiming to re-center ODF around the declarative approach for defining the desired state of the system, achieve better functional decomposition of the system, and provide better framework for extensibility.

## Table of Contents <!-- omit in toc -->

- [Motivation](#motivation)
- [General Direction](#general-direction)
- [Terminology](#terminology)
- [Proposal](#proposal)
  - [Resource Manifests](#resource-manifests)
    - [Versioning](#versioning)
    - [Multi-tenancy](#multi-tenancy)
    - [Identity](#identity)
    - [Labels \& Annotations](#labels--annotations)
    - [References](#references)
    - [Selectors](#selectors)
    - [Ownership](#ownership)
  - [APIs](#apis)
    - [Current state of ODF APIs](#current-state-of-odf-apis)
    - [Kubernetes API Overview](#kubernetes-api-overview)
    - [REST API strategy](#rest-api-strategy)
    - [GraphQL strategy](#graphql-strategy)
- [Compatibility](#compatibility)
- [Drawbacks](#drawbacks)
- [Prior art](#prior-art)
  - [Kubernetes Design Notes](#kubernetes-design-notes)
- [Rationale and alternatives](#rationale-and-alternatives)
  - [ODF objects as Kubernetes CRDs](#odf-objects-as-kubernetes-crds)
  - [Terraform modules for Kamu API](#terraform-modules-for-kamu-api)
- [Unresolved questions](#unresolved-questions)
- [Future possibilities](#future-possibilities)
- [Appendix A: Example Manifests](#appendix-a-example-manifests)
  - [Dataset](#dataset)
  - [Storage](#storage)
  - [Source](#source)
  - [Ingress](#ingress)
  - [Push/PullAlias](#pushpullalias)
  - [Flow](#flow)
  - [VariableSet](#variableset)
  - [RebacSet](#rebacset)
  - [MaterializedView](#materializedview)
  - [Data Test](#data-test)


# Motivation
<!--
Why are we doing this? What use cases does it support? What is the expected outcome?
-->

1. 
   Open Data Fabric spec started and revolved around the single `DatasetSnapshot` manifest that defines the desired state of dataset metadata. We have used `MetadataEvent` types in the snapshot type to define quite complex features like polling data ingestion and push sources.
   
   With gained experience, we started to realize that some of these features don’t belong in dataset metadata as they not only don’t contribute to dataset provenance or validity, but may **reveal sensitive information about publisher’s infrastructure** that must remain private. This RFC proposes to move some of this information out of the dataset metadata chain to be kept private and at the same time **make core ODF format spec more lean** and easier to adopt.

2. 
   Reference implmenetations like Kamu have been adding a lot of new features that are strongly related to reusable data pipelines but clearly don’t belong in the dataset metadata:

   * Flow schedules and configuration  
   * Dataset variables and secrets  
   * ReBAC permissions

   Because it didn’t make sense to encode these into metadata events, configuration started relying on the growing amount of custom GQL and REST APIs **sidetracking us into imperative configuration**, while ODF’s intention was always to be maximally declarative. This makes configuring the state of the system significantly more challenging, and makes configuration process **non-portable** between ODF implementations.
   
   With this RFC we want to provide all ODF implementations a single resource configuration framework with a clear way to upstream established resources later into ODF, and correct the course back towards the **declarative Infrastructure-as-Code approach**.


# General Direction
ODF is quickly becoming a ***“Kubernetes for Data”*** \- a high-level integration framework for different data formats, compute engines, privacy technologies, and data access APIs.

We are embracing **Infrastructure-as-Code** philosophy:

* Manifest files will be the primary way to instantiate and update all resources in data pipelines
* Configurations can be managed in git
* Configuration will describe "desired state" of the system
* Reconciliation mechanism will be responsible for matching actual state with the desired state

We introduce a **unified model of resource, identity, references, ownership**. We want to provide similar API experience whether you are configuring datasets, flows, variables, or ReBAC relations. We want representation work similarly across different flavors of APIs (REST / GQL) to reduce maintenance burden - adding new resource type should not require extending the API surface.


# Terminology
We will stick to the following terminology:

* **Resource** - is a managed back-end entity that has an identity and consumes or allocates some compute or storage resources
* **Manifest** - is a declarative configuration (e.g. a YAML file) that describes the desired resource state.  
* **Controller** - a process that keeps track of certain manifests types and reconciles them with the state of objects under its control. The purpose of controllers is to constantly move the current resource state towards the desired state described by manifests.

Examples:

* `Dataset` YAML file is a manifest applying which will create a corresponding `Dataset` resource  
* `VariableSet` is a high-level manifest applying which will create a set of `Variable` resources  
* `Ingress` is a manifest application of which will create a high-level `Ingress` resource. The controller of `Ingress` will then generate and apply lower-level manifests for `ApiEndpoint`, a `Buffer` (e.g. Kafka topic), and a `Source`, which will in turn result in provisioning of corresponding resources.

# Proposal
The proposal will often reference Kubernetes as one of the best example of declarative IaC approach. We will draw many parallels with its design and build on its learnings.


## Resource Manifests
In Kubernetes manifests are structured as:
```yaml
apiVersion: external-secrets.io/v1beta1
kind: SecretStore
metadata: {}
spec: {}
status: {}
```

In ODF we propose a format like this:
```yaml
context: secrets.opendatafabric.org/v1
kind: SecretSet
header: {}
spec: {}
status: {}
```

Here:
- `context` represents the "bounded context" of the resource, similar to JSON-LD `@context`
  - `context` is used instead of `apiVersion` in K8s as latter is a REST-level concept and is too narrow compared to what we're trying to represent
- `kind` is used for consistency with k8s and the current ODF enums
- `header` contains identity and ownership information
  - `header` is used instead of `metadata` because the latter is already an overloaded term in ODF and is generic to the point of being meaningless
- `spec` defines the desired state of the resource
- `status` contains information about the current state and the reconciliation process


### Versioning
Version suffix is part of the `context` in the form of `v1` or `v1alpha1`.

- Version should NOT be thought of only as manifest schema. It captures both how resource is defined and the semantics of how it behaves, i.e. version may be incremented if resource behavior changes significantly even when the schema stays the same.
- Versions apply to the level of entire bounded context, not an individual resource, so if one domain contains multiple releated resources a version bump would apply to all of them.


### Multi-tenancy
Resources can explicitly define which `account` they belong to:

```yaml
context: secrets.opendatafabric.org/v1
kind: SecretSet
header:
  account: alice  # Short form can parse DID or name
  account:  # Full form
    id: did:odf:123..321
    name: alice
spec: {}
```

Unlike Kubernetes that uses RBAC and `namespace`-based isolation - ODF is based on **ReBAC account-centric model** that allows complex ownership and access control hierachies, e.g. teams, organizations, flexible permissions for accounts outside of organizations.


### Identity
A manifest file will usually only define the `name`:

```yaml
context: secrets.opendatafabric.org/v1
kind: SecretSet
header:
  name: my-secrets
spec: {}
```

Resource **names are immutable** - changing the name requires deleting and re-creating the resource.

The ODF node will assign a unique `id` (UUID v4) to resources upon creation:

```yaml
context: secrets.opendatafabric.org/v1
kind: SecretSet
header:
  id: 6767a4ee-d74d-436e-84f9-709407869a26
  name: my-secrets
spec: {}
status: {}
```

Including `id` in the manifest can be used to ensure the manifest applies to exact needed resource, but sacrifices portability of the manifest across ODF nodes.


### Labels & Annotations
A resource can specify custom labels and annotations. Both are maps of string keys to any JSON values, but only labels get indexed and can be used for querying:

```yaml
context: secrets.opendatafabric.org/v1
kind: SecretSet
header:
  name: my-secrets
  labels:
    env: prod
  annotations:
    owner: https://github.com/open-data-fabric
    repo: https://github.com/open-data-fabric/spec
spec: {}
```

Labels and annotations are **mutable**.


### References
Resource manifests can link to other resources using **references**. 

Resourecs thus form a DAG. Cyclical references are not allowed - this can enforced by implementation via linters.

Unlike Kubernetes that doesn't specify a common reference format - in ODF all refereces and selectors will have a common format and thus can be easily picked up by linters and other automation uniformly, without knowing the specifics of individual resource schemas.

Resources can be referenced by:
- ID
- Alias (`{name}` or `{account-name}/{resource-name}`)

A `context` and `kind` can be included for additional validation.

Example:

```yaml
context: datasets.opendatafabric.org/v1
kind: Dataset
header:
  name: my-dataset
spec:
  storage: my-bucket  # Short form reference can parse IDs and aliases
  storage:  # Long form reference
    id: 6767a4ee-d74d-436e-84f9-709407869a26
    alias: my-account/my-bucket
    context: storage.opendatafabric.org/v1
    kind: Storage
```


### Selectors
Multiple resources can be referenced at once using **selectors**.

```yaml
context: flows.opendatafabric.org/v1
kind: Flow
header:
  name: periodic-compaction
spec:
  triggers:
    - kind: Cron
      cron: "@daily"
  steps:
    - kind: Compaction
      maxSliceSize: 100MiB
      maxSliceRecords: 10_000
      targetDatasets:
        kind: Root
        name: org.opendatafabric.%
        labels:
          env: prod
```


### Ownership
When an object is created by another higher-level object it can write the association into header as `ownerReferece`. This creation provenance trail can be used for automatic cascading deletion and garbage collection.

```yaml
context: ingest.opendatafabric.org/v1
kind: Buffer
header:
  name: buffer-aabbcc
  ownerReferences:
  - id: c27331ce-ce88-4ff9-8c5a-4ce8107cc03f
    name: ingest-f76666445
    context: ingest.opendatafabric.org/v1
    kind: Ingest
spec: {}
```

See also:
- [Kubernetes Owners and Dependents](https://kubernetes.io/docs/concepts/overview/working-with-objects/owners-dependents/)



## APIs

### Current state of ODF APIs
Current API of the Node evolved as several groups of functionality:

- REST API is composed of semi-overlapping groups like:  
  - Simple transfer protocol  
  - Smart transfer protocol  
  - Data query and commitments  
- Standalone APIs 
  - GraphQL
  - FlightSQL

![][image2]

Our REST API is currently missing the functionality for listing datasets, inspecting metadata, and manipulating other objects like accounts, flows, variables etc - this role is only filled by GraphQL.

As we evolve our APIs we would like:

- REST API to become a superset of GraphQL API  
- Minimize the burden of maintaining two APIs (reuse object schemas as much as possible)  
- Establish a good versioning strategy

### Kubernetes API Overview

Kubernetes provides a unified API that is very extensible and provides a uniform way to work with all object resources.

![][image3]

Basic Kubernetes API scheme is:

* `/apis/<group>/<version>/<kind>/<name>`  
  * `/apis/apps/v1/deployments`  
  * `/apis/apps/v1/deployments/my-deployment`  
* `/apis/<group>/<version>/namespaces/<namespace>/<kind>/<name>`  
  * `/apis/apps/v1/namespaces/my-namespace/deployments/my-deployment`

Benefits of Kubernetes REST API:

* Proven, mature model  
* API-based versioning  
* API groups allowing different subsystems to evolve independently

Problems with Kubernetes REST API:

* Namespace is the only unit of multi-tenancy  
  * In Kamu we aim for more flexible multi-tenancy model based on accounts  
* Objects are addressed by names \- it’s not possible to use `uid`  
  * The names in k8s are immutable, but in Kamu they can not only be changed within a node, but also be different across nodes

### REST API strategy

The core idea is to introduce another core REST API protocol group: **Object protocol**.

* Object protocol will define how to list, create, update, delete, and get the state of various objects in an ODF system  
* It will define the top-level **object addressing scheme** and serve as a **nesting point** for other protocols like simple/smart transfer and data querying

![][image4]

Proposed object protocol endpoint scheme:

* Listing: `/<group>/<version>/<kind-plural>`  
  * `/odf/v1/datasets`  
  * `/rebac/v1/relations`  
* By object ref: `/<group>/<version>/<kind-plural>/<object-ref>`  
  * `/odf/v1/datasets/did:odf:123..321`  
  * `/odf/v1/datasets/my-dataset` (searches current account only)  
  * `/flow/v1/flows/1234-123-12`  
* By account and object ref: `/<group>/<version>/accounts/<account-ref>`  
  * `/odf/v1/accounts/sergiimk/datasets/my-dataset`  
  * `/odf/v1/accounts/did:key:321..321/datasets/did:odf:123..123`  
* Other HTTP-based protocols: `/<protocol>/...`  
  * `/graphql`

Kubernetes compatibility can be achieved by:

* Adding `/apis` prefix  
* Replacing `accounts` as `namespaces` \- i.e. in Kamu it’s as if every account has its own namespace and shared namespaces can be created through organization accounts  
* Projects like [kcp.io](http://kcp.io) that explore adding ReBAC support to K8s may in future allow us to completely blur the line between Kamu and K8s models

### GraphQL strategy

We already generate GraphQL types for all ODF manifests. We will continue and expand on this strategy moving forward. Manifest schemas will be defined in one place - the ODF spec and code generation will be used for REST and GraphQL representations.

To make use of the graph nature of the GraphQL protocol we will introduce a special treatment for cross-object references where, for example a manifest like this one:

```yaml
kind: Dataset
apiVersion: odf/v1
metadata:
  uid: did:odf:123..321
  name: foo
  owner:
    id: did:key:123
    name: sergiimk
spec:
  storageRef:
    id: 123-a122-1231
    alias: sergiimk/my-s3-bucket  # Resoved for human readability only
```

Will allow navigation of reference as:

```gql
datasets {
  byId(id: "did:odf:123..321") {
    spec {
      storage {  # storage reference becomes navigable
        id
        alias
        account {
          name
        }
      }
    }
  }
}
```


# Compatibility
<!--
Details on compatibility of these changes.
-->


# Drawbacks
<!--
Why should we *not* do this?
-->


# Prior art
<!--
Discuss prior art, both the good and the bad, in relation to this proposal.
A few examples of what this can include are:

- For core features: Does this feature exist in other technologies and what experience have their community had?
- For community proposals: Is this done by some other community and what were their experiences with it?
- For other teams: What lessons can we learn from what other communities have done here?
- Papers: Are there any published papers or great posts that discuss this? If you have some relevant papers to refer to, this can serve as a more detailed theoretical background.

This section is intended to encourage you as an author to think about the lessons from other projects, provide readers of your RFC with a fuller picture.
If there is no prior art, that is fine - your ideas are interesting to us whether they are brand new or if it is an adaptation from other technologies.
-->

## Kubernetes Design Notes
* [Generated Object API Reference](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#api-overview)  
* OpenAPI Spec
  * [Schema link](https://raw.githubusercontent.com/kubernetes/kubernetes/refs/heads/master/api/openapi-spec/swagger.json)  
  * [OpenAPI Editor](https://editor-next.swagger.io/)  
* [apimachinery](https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go)  
* [Kubernetes API concepts](https://kubernetes.io/docs/reference/using-api/api-concepts/)  
* [kube.rs](http://kube.rs)  
* [https://github.com/Arnavion/k8s-openapi](https://github.com/Arnavion/k8s-openapi)  
* [Kubernetes API Groups](https://github.com/kubernetes/design-proposals-archive/blob/main/api-machinery/api-group.md) (api-based versioning instead of resource-based)



# Rationale and alternatives
<!--
- Why is this design the best in the space of possible designs?
- What other designs have been considered and what is the rationale for not choosing them?
- What is the impact of not doing this?
-->

## ODF objects as Kubernetes CRDs
* Pros:
  * Mature system with carefully thought out APIs  
  * Reusing tools and integrations  
* Cons:
  * Namespaces and RBAC don’t provide real multi-tenancy  
    * We would like to think multi-tenant / multi-region /multi-cloud  
    * See [https://www.kcp.io/](https://www.kcp.io/)  
  * Performance concerns in case of millions of objects  
  * Excessive coupling that would be hard to undo  
  * Templating would require re-inventing something like helm \+ helmfile


## Terraform modules for Kamu API
* Pros:
  * Can wrap existing APIs in TF modules without modifications  
  * Existing tool for complex dependency management and templating  
* Cons:
  * TF feels like a hack to add declarativeness onto services that don’t support it  
  * Very few data engineers have TF experience  
  * High potential for desync between TF state and actual pipeline state  
  * TF state needs to be stored somewhere, requiring more infrastructure on the user’s side  
  * Harder to validate and to report useful errors


# Unresolved questions
<!--
- What parts of the design do you expect to resolve through the RFC process before this gets merged?
- What parts of the design do you expect to resolve through the implementation of this feature before stabilization?
- What related issues do you consider out of scope for this RFC that could be addressed in the future independently of the solution that comes out of this RFC?
-->

* How will secret management work?  
* API/manifest versioning  
* Should we consider JSON-LD as an alternative?  
  * More generic way to define and connect objects  
  * Integrates with [schema.org](http://schema.org) \- if we use RDF in data it may make sense to use it in manifests  
  * Integrates with [verifiable credentials standards](https://vcplayground.org/docs/standards/)


- DatasetID vs ResourceID
  - should we have both?
  - two types of selectors then?


# Future possibilities
<!--
Think about what the natural extension and evolution of your proposal would be and how it would affect ODF as a whole. Try to use this section as a tool to more fully consider all possible interactions with the ODF in your proposal.

This is also a good place to "dump ideas", if they are out of scope for the RFC you are writing but otherwise related.

If you have tried and cannot think of any future possibilities, you may simply state that you cannot think of anything.

Note that having something written down in the future-possibilities section is not a reason to accept the current or a future RFC; such notes should be in the section on motivation or rationale in this or subsequent RFCs. The section merely provides additional information.
-->

We are strongly [considering using JSON-LD](https://github.com/open-data-fabric/open-data-fabric/issues/123) for manifests in future. JSON-LD manifest could look like this:

```yaml
"@context": https://opendatafabric.org/core/v1/context.jsonld
"@type": odf:SecretSet
"@id": https://cluster.example.com/resources/8f3b2c1a-4d5e-6f7a-8b9c-0d1e2f3a4b5c
header: {}
spec: {}
status: {}
```

------------------------------------------

------------------------------------------



# Appendix A: Example Manifests

![][image5]

Manifests in this section are not meant to be final but are here to illustrate the direction and the granularity of decompositions we want to achieve.

## Dataset

Represent the desired state of dataset metadata.

**Applying:**  
If the referenced dataset is absent \- it will be created with the specified metadata events in the chain.

If the referenced dataset exists \- the differences between its current and desired state will be reconciled, which can include:

* Setting / unsetting certain events  
* Schema migrations  
* Operation migrations

**Old:**:

```
kind: DatasetSnapshot
version: 1
content:
  name: etherscan.transactions
  kind: Root
  metadata:
    - kind: SetPollingSource  # moves to Source manifest
      fetch:
        kind: Url
        url: https://api.etherscan.io/api/...
      read:
        kind: Json
        subPath: result
      merge:                 # moves to Flow manifest
        kind: Ledger
        primaryKey:
          - transaction_hash    - kind: SetVocab      eventTimeColumn: block_time    - kind: SetLicense
      shortName: apache-2.0
```

**New:**

```
kind: Dataset
apiVersion: odf/v1
metadata:
  name: etherscan.transactions
spec:
  kind: Root
  events:    - kind: SetVocab      eventTimeColumn: block_time
    - kind: SetLicense
      shortName: apache-2.0
  storageRef:
    alias: sergiimk/my-s3-bucket
```

TODO: where and when schema is defined? Do we allow schema inference upon the first write?

TODO: where the merge strategy belongs? At the source / dataset / or whatever connects the two?

TODO: should we treat these as  templates? (similarly to pod templates in k8s)

TODO: what the future interplay with templating will be? (e.g. helm and dbt’s ninja SQL)

## Storage

Currently the `DatasetRepository` implementation defines where datasets will be stored (e.g. S3, or LocalFS).

Our public platform will need to allow users to specify per dataset where they would like them to be stored. 

Kamu platform will provide several built-in storages to choose from, e.g. S3 / GCS / Azur and some IPFS/Filecoin onramps. We will also support “bring your own” (BYO) where users can specify location and access keys of their own private storage. This means that **storage also becomes an object with its own access scope and permissions**.

**New:**

```
kind: Storage
apiVersion: odf/v1
metadata:
  account:
    name: sergiimk
  name: my-s3-bucket
spec:
  config:
    kind: S3
    bucketId: my-s3-bucket
    region: us-west-2
    accessKey:
      kind: secretRef
      alias: sergiimk/my-s3-key
```

TODO: Here is the need for account-scoped secrets\!

TODO: When pushing a local dataset to a node, how can we specify which storage to use? Is dataset-to-storage association managed outside of manifests? There seems to be a general problem of manifest transferability \- we are not pushing manifests \- we push objects and it’s ok if they result in a different manifest.

## Source

Sources replace `SetPollingSource` / `AddPushSource` events to define where data is ingested from.

With separation of sources, datasets become just collections of data (root) or transformation code and results (deriv). Root datasets will be agnostic of where the data comes from, their job is to only maintain data integrity. This will make the core ODF format a lot leaner, simpler, and closer to existing competitors like Iceberg.

**Object interface:**

```c
trait Source {
  /// Initial configuration of the source
  fn config(&self) -> Config;

  /// Inspect the state of the source before reading
  async fn peek(&self) -> Result<Peek, PeekError>;

  /// Read the next set of records
  async fn read(&self, state: Option<State>, params: ReadParams)
    -> Result<ReadResult, ReadError>;
}

struct Peek {
  /// Number of records ready to be read
  pending_records: Option<usize>,
}

struct ReadParams {
  /// Recommended size of the record batch
  target_batch_size: usize,
  /// Time to wait for data before considering the source exhausted
  polling_timeout: Duration,
  /// Whether to force-fetch the data ignoring the caching state
  fetch_uncacheable: bool,
}

enum ReadResult {
  UpToDate {
    /// Set if source cannot determine the cache validity
    uncacheable: bool,
  },
  Updated {
    /// Raw data read, if any
    data: Option<DataFrame>,
    /// Watermark state after the returned chunk of data
    watermark: Option<DateTime<Utc>>
    /// New state, if advanced
    new_state: Option<SourceState>,
  }
}
```

TODO: non-resumable sources???

Note that such an interface **removes the distinction between polling and push sources**, making it simply a resumable batch iterator of records (*more on push sources below*).

TODO: consider source partitioning

**State:**

Sources need to maintain a state to be cacheable and resumable. Currently this state is stored directly in the dataset metadata chain as part of `AddData` events.

Sources, however, are node-local entities, so:

* State may contain internal sensitive information  
  * *e.g. last update date of an internal database, or an offset of internal kafka topic may let an outsider infer the volume of company sales*  
* Moving a dataset to another node will involve re-creating the source which might not be able to resume from the same state  
  * *e.g. a blockchain source would be able to reuse the same state because block numbers are global, but a Kafka source like the one we’ll have for buffering push ingest may have completely different offsets*

For these reasons we will extract the source state out of the metadata chain. Removing frequent source updates will also lessen the write load on the metadata chain.

TODO: How to move a dataset with a source from local workspace to a node, or from one node to another? Do we support the “kamu push” operation for sources to transfer the state?

TODO: With this change we will lose a nice ability to reset the dataset and have the source state roll back along with it…

**Applying:** ???

* What parts are mutable and what aren’t  
* Whether / when / how do we reset the state

TODO: Do we create sources per dataset? Or have a pool of sources multiple datasets can read from?

TODO: How does a dataset get connected to a source? Through a flow? Separate ingest object? This connection will need to be **stateful**\!

**Previously:**:

```
kind: DatasetSnapshot
version: 1
content:
  name: etherscan.transactions
  kind: Root
  metadata:
    - kind: SetPollingSource
      fetch:
        kind: Url
        url: https://api.etherscan.io/api/...
      read:
        kind: Json
        subPath: result        schema: [..]
      merge:
        kind: Ledger
        primaryKey:
          - transaction_hash    - kind: SetVocab      eventTimeColumn: block_time
```

**New: Dataset with a polling source that is stored in the default storage and ingested on a schedule**

```
kind: Dataset
apiVersion: odf/v1
metadata:
  name: etherscan.transactions
spec:  events:
    - kind: SetVocab      eventTimeColumn: block_time    - kind: SetDataSchema      schemaArrow:        ...  storageRef:    alias: default

# NOTE: The rest of manifests are provided for completenes
# and are explained in detail in further sections

---

kind: Storage
apiVersion: odf/v1
metadata:
  name: default
spec:
  config:
    kind: LocalFS
    path: ./datasets/

---

kind: Source
apiVersion: odf/v1
metadata:
  name: etherscan.transactions
spec:
  config:
    fetch:
      kind: Url
      url: https://api.etherscan.io/api/...
    prepare:
      ...
    schema:
      ...
    read:
      kind: Json
      subPath: result
      readSchema:
        - block_number BIGINT
        ...
    preprocess:  ??? does this belong here or next to merge strategy or both
      kind: Sql
      engine: datafusion
      query: select * from input  # Optional initial state
  # This will allow both declaring initial configuration
  # and to export/transfer sources and their states betwen nodes
  # TODO: Transactional push of dataset & source ?????
state:
  etag: 12345@feda..18df

---

kind: Flow
apiVersion: odf/v1
metadata:
  name: ingest
  dataset:     # scoped under a dataset
    alias: etherscan.transactions
content:
  tasks:
    - kind: Ingest
      sourceRef:
        alias: etherscan.transactions
      sourceParams:
        targetBatchSize: 10_000
      merge:
        kind: Ledger
        primaryKey:
          - transaction_hash
  triggers:
    - kind: Schedule
      schedule:
        kind: cron5
        cron: "*/30 * * * *"
```

In the example above:

* An empty `etherscan.transactions` root dataset is created, with metadata only renaming the `event_time` column  
* An `etherscan.transactions` source is created with configuration of where to pull data from and how to read and pre-process it  
* An `ingest` flow of type `Ingest` is created that ties the source to a target dataset, defines how to merge raw data into the dataset, and sets up a cron trigger

## Ingress

Currently to allow writing data directly into a dataset we use `AddPushSource` metadata events. Push sources act like named endpoints that receive data in certain formats and define how it should be read and merged into the target dataset. They are a vague concept, especially when we see sources like MQTT that are more seen as “push” protocols actually using “pull” in kamu.

**Previously:**

```
kind: DatasetSnapshot
version: 1
content:
  kind: Root
  name: temp.sensor
  metadata:
    - kind: AddPushSource
      sourceName: default
      read:
        kind: NdJson
        schema:
          - t TIMESTAMP
          - long DOUBLE
          - lat DOUBLE
      merge:
        kind: Append
```

New approach introduces `Ingress` objects and manifests. Ingresses represent actual infrastructure components that need to be deployed or configured to provide a write interface for a dataset.

**New: Immediate ingress**

```
kind: Dataset
apiVersion: odf/v1
metadata:
  name: temp.sensor
spec:
  kind: Root

---

kind: Ingress
apiVersion: odf/v1
metadata:
  name: temp.sensor.api
  dataset:
    alias: temp.sensor
spec:
  frontend:
    kind: RestApi
    read:
      kind: NdJson
      schema:
        - t TIMESTAMP
        - temp DOUBLE
  backend:
    kind: Immediate
    merge:
      kind: Append
```

TODO: Where the schema should be defined? If the ingress API doesn’t need any schema coercion / preprocessing could we declare the schema on a dataset and have ingress automatically enforce it? This would mean ingresses are inseparable from datasets.

Above we create a `temp.sensor.api` Ingress object that represents provisionment of a `RestApi` endpoint. This could alternatively be Kafka, WebSocket, MQTT and other types of write APIs.

The `frontend` section specifies how data should be read, similarly to a `Source`. 

The `backend` section \- how it should be added to the dataset. The `Immediate` backend means that on every call to the API we will attempt to write data directly into the dataset.

**New: Buffered ingress**

```
kind: Ingress
apiVersion: odf/v1
metadata:
  name: temp.sensor.api
  dataset:
    alias: temp.sensor
spec:
  frontend:
    kind: RestApi
    read:
      kind: NdJson
      schema:
        - t TIMESTAMP
        - temp DOUBLE
  backend:
    kind: Buffered
    bufferSize: 1000
    overflowPolicy:
      kind: Reject
  flow:
    triggers:
      - kind: Source
        minRecordsToAwait: 100
        maxAwaitIterval: 1h
```

This example shows a `Buffered` backend that will provision and accumulate records in a queue (e.g. Kafka). This essentially creates an internal data storage which we could declare a `Source` for to consume from. However, for convenience, we let `Ingest` to automatically provision a `Source` and a `Flow` with a trigger.

In other words `Ingress` is a higher-level manifest that can provision:

* API endpoints  
* Queues  
* Sources  
* and Flows

TODO: Do we want to decouple ingest from datasets? Probably not, as we want ELT and as little fallible extra steps between data arriving and being written into a dataset.

## Push/PullAlias

TODO: What are we doing with `kamu repo **` commands?

TODO: How do we express `Remote(Root/Derivative)` datasets in the new system?

## Flow

A `Flow` is a template from which individual `FlowRuns` are created. A `FlowRun` is a collection of steps that spawn `Tasks`.

In other words:

* Tasks are a unit of work \- they contain a plan which gets executed by TaskExecutors  
* FlowRuns are a higher-level workflow that contain parameters for spawning tasks \- they are scheduled and driven to completion by the FlowSystem  
* Flows are configuration blueprints for creating FlowRuns.

**New: Ingest flow connecting a source with a dataset**

```
kind: Flow
apiVersion: odf/v1
metadata:
  name: ingest
  dataset:
    alias: etherscan.transactions
spec:
  task:
    kind: Ingest
    sourceRef:
      alias: etherscan.transactions
    sourceParams:
      targetBatchSize: 10_000
    merge:
      kind: Ledger
      primaryKey:
        - transaction_hash
  triggers:
    - kind: Schedule
      schedule:
        kind: cron5
        cron: "*/15 * * * *"
```

TODO: Do we want flows to always be scoped under a dataset? Or should we have account / organization scope flows too? This seems to fit the idea of system flows (scoped under system account).

**New: Transform flow that batches derivative input updates**

```
kind: Flow
apiVersion: odf/v1
metadata:
  name: transform-123
  dataset:
    alias: foobar
spec:
  task:
    kind: Transform
  triggers:
    - kind: Batching
      minRecordsToAwait: 1000
      maxAwaitInterval: 10m
```

**New: Transform flow runs compactions on all root datasets in the account**

```
kind: Flow
apiVersion: odf/v1
metadata:
  name: compact-all
  # note no `dataset` - this flow is scoped under entire account
spec:
  datasetSelector:
    pattern: %
    kind: Root
  task:
    kind: Compaction
    hard: false
    maxSliceSize: 100MiB
    maxSliceRecords: 10_000
  triggers:
    - kind: Schedule
      schedule:
        kind: cron5
        cron: @daily
```

**New: GC system flow**

```
kind: Flow
apiVersion: odf/v1
metadata:
  name: gc
  account:
    name: kamu  # Scoped to an admin account
spec:
  datasetSelector:
    pattern: %/%        # All datasets of all accounts
    kind: Root
  task:
    kind: GarbageCollect
  triggers:
    - kind: Schedule
      schedule:
        kind: cron5
        cron: @weekly
```

TODO: Will such flows be shown to the dataset owner as scheduled?

## VariableSet

```
kind: VariableSet  /// Config /// Secret
apiVersion: odf/v1
metadata:
  name: my-vars
  dataset:
    alias: my-dataset # Scoped to dataset
spec:
  variables:
    A: B
    C: D
  secrets:
    E: F  # ?????? encryption
```

TODO: Do we need variables for anything but ingest parametrization? Perhaps parametrizing flows, although why not simply mutate them?

TODO: How do we encrypt secrets?

## RebacSet

```
kind: RebacSet
apiVersion: odf/v1
metadata:
  name: my-rebac-set # ???
spec:
  relations:
    - subject:
        accountId: did:key:...
      name: role
      value: maintain
      object:
        kind: datasetRef
        alias: foo
    - subject:
        accountRef:
          alias: sergiimk
      name: role
      value: reader
      object:
        kind: datasetRef
        alias: bar
  properties:
    - object:
        kind: datasetRef
        alias: foo
      name: allowPublicRead
      value: true
    - object:
        kind: datasetRef
        alias: bar
      name: allowAnonymousRead
      value: false
```

## MaterializedView

```
kind: MaterializedView
apiVersion: odf/v1
metadata:
  name: current-rates
spec:
  inputs:
    - datasetRef:
        alias: exchange-rates
  transform:
    kind: Sql
    engine: risingwave
    query: |
      select
        currency_from,
        currency_to,
        last(rate) over (
          partition by currency_from, currency_to
          order by event_time desc
        ) as rate
      from "exchange-rates"
  lazy: true
```

## Data Test

```
post ingest / pre-commit QA
```



