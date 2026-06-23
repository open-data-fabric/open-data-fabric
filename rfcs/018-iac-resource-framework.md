# RFC-018: Infrastructure-as-Code Resource Framework <!-- omit in toc -->

[![Issue](https://img.shields.io/github/issues/detail/state/kamu-data/open-data-fabric/108?label=Issue)](https://github.com/kamu-data/open-data-fabric/issues/108)
[![PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/116?label=PR)](https://github.com/kamu-data/open-data-fabric/pull/116)

**Start Date**: 2025-06-14

**Published Date**: 2026-06-20

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
    - [Type Identification](#type-identification)
    - [Versioning](#versioning)
    - [Multi-tenancy](#multi-tenancy)
    - [Identity](#identity)
    - [Labels \& Annotations](#labels--annotations)
    - [References](#references)
    - [Selectors](#selectors)
    - [Ownership](#ownership)
    - [Generations](#generations)
    - [Status](#status)
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
  - [`VariableSet`](#variableset)
  - [`SecretSet`](#secretset)
- [QQQQQQQQQQQQQQQQQQQQQQQQQQ](#qqqqqqqqqqqqqqqqqqqqqqqqqq)


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
$schema: https://opendatafabric.org/schemas/config/v1alpha1/SecretSet
headers: {}
spec: {}
status: {}
```

Here:
- `$schema` identifies the type of the resource using a resolvable URL that points to the JSON Schema file
  - This replaces separate `apiVersion`/`kind` fields (as in K8s) with a single self-describing identifier
- `headers` contains identity and ownership information
  - `headers` is used instead of `metadata` because the latter is already an overloaded term in ODF and is generic to the point of being meaningless
- `spec` defines the desired state of the resource
- `status` contains information about the current state and the reconciliation process


### Type Identification
We use `$schema` URL to identify resource types:

```yaml
$schema: https://opendatafabric.org/schemas/config/v1alpha1/SecretSet
```

The `$schema` URL is formatted as `{base-url}/{context}/{version}/{Name}` and carries:
- Controlling organization domain (e.g. `opendatafabric.org`)
- Bounded context (e.g. `config`)
- Version (e.g. `v1alpha1`)
- Resource name (e.g. `SecretSet`)

Many IDEs recognize `$schema` field and automatically fetch the associated JSON schema to provide validation and auto-completion.

Resource schemas will be registered within ODF node and, similarly to Kubernetes CRDs, assigned a **short resource type name** (e.g. `SecretSet`) that can be used instead of the schema.


### Versioning
Version is part of the `$schema` URL in the form of `v1` or `v1alpha1`.

- Version should NOT be thought of only as manifest schema. It captures both how resource is defined and the semantics of how it behaves, i.e. version may be incremented if resource behavior changes significantly even when the schema stays the same.
- Versions apply to the level of entire bounded context, not an individual resource, so if one domain contains multiple related resources a version bump would apply to all of them.


### Multi-tenancy
Resources can explicitly define which `account` they belong to:

```yaml
$schema: https://opendatafabric.org/schemas/config/v1alpha1/SecretSet
headers:
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
$schema: https://opendatafabric.org/schemas/config/v1alpha1/SecretSet
headers:
  name: my-secrets
spec: {}
```

Resource **names are immutable** - changing the name requires deleting and re-creating the resource.

The tuple `(Account, ResourceType, ResourceName)` uniquely identifies the resource within an ODF node (see also [references](#references)). There may be multiple resources of different type under one account with the same name.

The ODF node will additionally assign a unique `id` (UUID v4) to resources upon creation:

```yaml
$schema: https://opendatafabric.org/schemas/config/v1alpha1/SecretSet
headers:
  id: 6767a4ee-d74d-436e-84f9-709407869a26
  name: my-secrets
spec: {}
status: {}
```

Including `id` in the manifest can be used to ensure the manifest applies to exact needed resource, but sacrifices portability of the manifest across ODF nodes.


### Labels & Annotations
A resource can specify custom labels and annotations. Both are maps of string keys to any JSON values, but only labels get indexed and can be used for querying:

```yaml
$schema: https://opendatafabric.org/schemas/config/v1alpha1/SecretSet
headers:
  name: my-secrets
  labels:
    env: prod
  annotations:
    owner: https://github.com/open-data-fabric
    repo: https://github.com/open-data-fabric/spec
spec: {}
```

Labels and annotations are **mutable**.

Labels and annotations with URL-like names will be automatically validated against the respective schemas to exclude typos:

```yaml
$schema: https://opendatafabric.org/schemas/config/v1alpha1/Dataset
headers:
  name: my-dataset
  labels:
    https://opendatafabric.org/schemas/dataset/v1alpha1/DatasetKind: Root  # Anything but Root or Derivative will fail valiation
spec:
  kind: Root
  metadata: []
```

Controllers may contribute their own labels to simplify common filtering scenarios. For example a `Dataset` resource above will automatically get the `https://opendatafabric.org/schemas/dataset/v1alpha1/DatasetKind: Root` label without you needing to specify it manually because it's very common to filter datasets by `kind`.


### References
Resource manifests can link to other resources using **references**, forming a DAG. Cyclical references are not allowed - this can enforced by implementation via linters.

Unlike Kubernetes that doesn't specify a common reference format - in ODF all refereces and selectors will have a common format and thus can be easily picked up by linters and other automation uniformly, without knowing the specifics of individual resource schemas.

Resources can be referenced by:
- ID
- Type and name (optionally including the ower account name)

Example:

```yaml
$schema: https://opendatafabric.org/schemas/dataset/v1alpha1/Dataset
headers:
  name: my-dataset
spec:
  metadata: []
  # Short form reference equivalent to `{ type: PersistentVolume, account: { name: my-org }, name: my-s3-bucket }
  volume: PersistentVolume:my-org/my-s3-bucket
```


### Selectors
Multiple resources can be referenced at once with **selectors**.

```yaml
$schema: https://opendatafabric.org/schemas/flow/v1alpha1/Flow
headers:
  name: periodic-compaction
spec:
  target:
    type: Dataset
    name: org.opendatafabric.%
    labels:
      https://opendatafabric.org/schemas/dataset/v1alpha1/DatasetKind: Root
      env: prod
  triggers:
    - kind: Cron
      cron: "@daily"
  steps:
    - kind: Compaction
      maxSliceSize: 100MiB
      maxSliceRecords: 10_000
```


### Ownership
When an object is created by another higher-level object it can write the association into header as `ownerReferece`. This creation provenance trail can be used for automatic cascading deletion and garbage collection.

```yaml
$schema: https://opendatafabric.org/schemas/source/v1alpha1/Buffer
headers:
  name: buffer-aabbcc
  ownerReferences:
  - id: c27331ce-ce88-4ff9-8c5a-4ce8107cc03f
    name: ingest-f76666445
spec: {}
```

See also:
- [Kubernetes Owners and Dependents](https://kubernetes.io/docs/concepts/overview/working-with-objects/owners-dependents/)


### Generations
Resource reconciliation is an **eventually-consistent** process. While a controller is working to reconcile one version of a resource the desired state may be changed by the user. To reflect this lag, a sequential `generation` number is incremented every time the resource header and spec are updated.

```yaml
$schema: https://opendatafabric.org/schemas/config/v1alpha1/SecretSet
headers:
  name: my-secrets
  generation: 4
  createdAt: 2026-01-01T00:00:00Z
  updatedAt: 2026-01-04T00:00:00Z
spec: {}
status: {}
```

In the `status` section `observedGeneration` can be used to see what generation the controller had a chance to process.

Note that `generation` does not increment on status changes as it intended to signify changes to the desired state.


### Status
The `status` section of the resource manifest never appears in user-defined manifests. It is maintained by the ODF nodes and writeable only by resource controllers. It is used to provide detailed information about the reconciliation status of the resource.

The main controller of a resource populates the `phase` and associated top-level fields during reconciliation attempts, while the `conditions` field provides a generic mechanism to attach additional information like error codes and messages. The `conditions` can be contributed by multiple controllers

```yaml
$schema: https://opendatafabric.org/schemas/config/v1alpha1/SecretSet
headers:
  name: my-secrets
spec: {}
status:
  phase: Failed
  observedGeneration: 4
  reconciledAt: 2026-01-01T00:00:00Z
  conditions:
    https://opendatafabric.org/schemas/config/v1/ConditionReady:
      value: false
      reason: decryption-key-not-found
      message: "Decryption key X does not exist"
      lastTransitionTime: 2026-01-01T00:00:00Z
      observedGeneration: 4
```


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
$schema: https://opendatafabric.org/schemas/dataset/v1alpha1/Dataset
headers:
  id: did:odf:123..321
  name: foo
  account: sergiimk
spec:
  volume: PersistentVolume:sergiimk/my-s3-bucket
```

Will allow navigation of reference as:

```gql
datasets {
  byId(id: "did:odf:123..321") {
    spec {
      volume {  # reference becomes navigable
        id
        type
        name
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

- Using `$schema` URL for type identification is non-standard in YAML — editors require either explicit schema associations in workspace settings or per-file `# yaml-language-server: $schema=...` comments to enable validation and autocomplete.


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
headers: {}
spec: {}
status: {}
```


# Appendix A: Example Manifests

![][image5]

Manifests in this section are not meant to be final but are here to illustrate the direction and the granularity of decompositions we want to achieve.


## `VariableSet`
```yaml
$schema: https://opendatafabric.org/schemas/config/v1alpha1/VariableSet
headers:
  name: my-vars
spec:
  variables:
    host: postgres
    port: "5113"
```


## `SecretSet`
A raw unencrypted secret:
```yaml
$schema: https://opendatafabric.org/schemas/config/v1alpha1/SecretSet
headers:
  name: my-secrets
spec:
  secrets:
    password: "postgres-staging-password"
    api_key: "internal-api-key-123"
```

Upon loading into the system it will get encrypted and the spec will look like this:
```yaml
$schema: https://opendatafabric.org/schemas/config/v1alpha1/SecretSet
headers:
  name: my-secrets
spec:
  secrets:
    password:
      value: eyJh..dN5oc
      contentEncoding: jwe
    api_key:
      value: eyJh..dN5oc
      contentEncoding: jwe
```


# QQQQQQQQQQQQQQQQQQQQQQQQQQ
- Extensibility of task types / ingress types ...
- Move ReBAC `attributes` into `spec` or `headers`?
- Make labels / annotations / attributes full URLs?
- DID vs ID
  - will account/dataset have both? should we allow DIDs in references?
  - or should we only use DIDs?
    - might be useful for delegation of control in future
    - but what does it mean for moving pipelines between nodes? 


Future:
- Start moving event bus events schemas into ODF
- Get rid of StructOrString in struct fields in favor of `serde_as`
- Remove `Resource*` prefix from types in `resource` domain?
- Can we replace full urls with JSON-LD like contexts:
  - instead `https://opendatafabric.org/schemas/dataset/v1alpha1/DatasetKind: Root`
  - have `"dataset:DatasetKind": Root`