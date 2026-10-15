# Resource Framework <!-- omit in toc -->

**Domain**: Automation
**Depends on**: -
**RFCs**: [RFC-018](../../rfcs/018-iac-resource-framework.md)


## Abstract <!-- omit in toc -->

This spec defines the declarative Infrastructure-as-Code framework used to configure ODF nodes. It defines the common manifest format of [Resources](../glossary.md#resource), how resource types are identified and versioned, how resources are identified, owned, labeled, and reference one another, and how [Nodes](../glossary.md#node) apply manifests and reconcile the actual state of the system with the desired state they describe. Individual resource types (datasets, sources, flows, accounts, etc.) are defined by the specs of their respective domains.


## Table of Contents <!-- omit in toc -->

- [Terminology](#terminology)
- [Resource Manifests](#resource-manifests)
  - [Input and Canonical Representations](#input-and-canonical-representations)
- [Type Identification](#type-identification)
  - [Versioning](#versioning)
- [Identity](#identity)
  - [Resources and Entities](#resources-and-entities)
- [Ownership](#ownership)
- [Labels and Annotations](#labels-and-annotations)
- [References](#references)
  - [Typed References](#typed-references)
  - [Reference Resolution](#reference-resolution)
- [Selectors](#selectors)
- [Owner References](#owner-references)
- [Generations](#generations)
- [Status](#status)
  - [Phases](#phases)
  - [Conditions](#conditions)
- [Resource Application](#resource-application)
- [Schemas](#schemas)


## Terminology
This spec builds on the [Resource](../glossary.md#resource), [Manifest](../glossary.md#manifest), and [Entity](../glossary.md#entity) concepts defined in the glossary, and introduces the following terms:
- **Controller** - a process that tracks resources of certain types and reconciles them with the state of entities under its control, continuously moving the actual state towards the desired state described by manifests

For example:
- Applying a `Dataset` manifest creates a `Dataset` resource, whose controller creates and maintains an ODF [Dataset](../glossary.md#dataset)
- Applying a high-level resource manifest may result in its controller generating and applying lower-level manifests, which in turn result in provisioning of corresponding entities


## Resource Manifests
All resources share a common manifest structure:

```yaml
$schema: https://opendatafabric.org/schemas/config/v1alpha1/SecretSet
headers: {}
spec: {}
status: {}
```

Where:
- `$schema` - identifies the [type](#type-identification) of the resource
- `headers` - contains identity, ownership, labels, annotations, and lifecycle information
- `spec` - defines the desired state of the resource
- `status` - contains information about the current state and the reconciliation process (see [Status](#status))

The `status` section never appears in user-authored manifests - it is maintained by the node and is writable only by controllers.


### Input and Canonical Representations
Every resource has two representations:
- **Input** ([`ResourceInput`](../../schemas/resources/v1alpha1/ResourceInput.json)) - the user-authored manifest. It contains only the fields users can modify, and allows convenient alternatives, e.g. referencing other resources by name or using short forms.
- **Canonical** ([`Resource`](../../schemas/resources/v1alpha1/Resource.json)) - the representation stored and returned by the node. It includes system-populated fields (e.g. `id`, `generation`, `status`), and all references are fully resolved.

Input types reference their canonical type via the `canonicalType` schema property. Canonical types must be trivially convertible into input types, while converting input types into canonical requires identity and [reference resolution](#reference-resolution).


## Type Identification
The `$schema` field identifies the resource type using a resolvable URL that points to the JSON Schema of the resource:

```yaml
$schema: https://opendatafabric.org/schemas/config/v1alpha1/SecretSet
```

The URL follows the pattern `{base-url}/{context}/{version}/{Name}` and carries:
- Controlling organization domain (e.g. `opendatafabric.org`)
- Bounded context (e.g. `config`)
- Version (e.g. `v1alpha1`)
- Resource type name (e.g. `SecretSet`)

Resource schemas are registered within a node and assigned a **short type name** (e.g. `SecretSet`) that can be used instead of the full URL, e.g. in [references](#references).

Resource schemas are validated against the [`meta/Resource`](../../schemas/resources/v1alpha1/meta/Resource.json) and [`meta/ResourceInput`](../../schemas/resources/v1alpha1/meta/ResourceInput.json) metaschemas.


### Versioning
The version is part of the `$schema` URL in the form of e.g. `v1` or `v1alpha1`:
- A version captures both how a resource is defined and the semantics of how it behaves, i.e. the version may be incremented if the behavior of a resource changes significantly even when its schema stays the same.
- Versions apply to an entire bounded context, not an individual resource type - a version bump applies to all resource types within the context.


## Identity
A manifest usually only defines the resource `name`:

```yaml
$schema: https://opendatafabric.org/schemas/config/v1alpha1/SecretSet
headers:
  name: my-secrets
spec: {}
```

At any point in time the tuple `(account, type, name)` uniquely identifies a resource within a node. Resources of different types under one account may have the same name.

Upon creation the node assigns the resource a unique `id` (UUID v4), which is a stable identifier of the resource **within one node**:

```yaml
$schema: https://opendatafabric.org/schemas/config/v1alpha1/SecretSet
headers:
  id: 6767a4ee-d74d-436e-84f9-709407869a26
  name: my-secrets
spec: {}
```

Resource names are **mutable**. When a manifest includes both `id` and `name`, the resource is matched by `id` and its `name` is updated to the one in the manifest, allowing resources to be renamed. Including `id` ensures that the manifest applies to the exact intended resource, but sacrifices portability of the manifest across nodes.


### Resources and Entities
Resources describe the desired state of the system and are separate from the [Entities](../glossary.md#entity) created to fulfill that state, e.g. a `Dataset` resource is not the same thing as an ODF dataset stored in some S3 bucket.

This is why resources like `Dataset` or `User` may have both:
- `id` - identifies the resource within a node
- `did` - a [W3C DID](https://www.w3.org/TR/did-core/) that identifies the corresponding entity (e.g. a dataset or a person) on a global network

When one node creates a dataset `did:odf:xxyy` and another node replicates it, the two nodes have two `Dataset` resources with the same `did` but different `id`s.


## Ownership
Every resource belongs to an [Account](../glossary.md#account) specified in the `account` header:

```yaml
$schema: https://opendatafabric.org/schemas/config/v1alpha1/SecretSet
headers:
  account: alice
  name: my-secrets
spec: {}
```

When `account` is omitted in a manifest, the resource is created under the account of the current [Principal](../glossary.md#principal).

ODF uses an account-centric model of multi-tenancy, where ownership and access control are defined by the [Authorization](../auth-and-privacy/authorization.md) spec.


## Labels and Annotations
Resources can specify labels and annotations. Both are maps of keys to arbitrary JSON values, but only labels are indexed and can be used for querying and in [selectors](#selectors):

```yaml
$schema: https://opendatafabric.org/schemas/config/v1alpha1/SecretSet
headers:
  name: my-secrets
  labels:
    env: prod
    https://opendatafabric.org/schemas/datasets/v1alpha1/DatasetKind: Root
  annotations:
    repo:
      url: https://github.com/open-data-fabric
      commitHash: 31d318be8a8f1a9ad8f317534a52f37ba89e8771
spec: {}
```

Labels and annotations are fully **mutable**.

Keys can be either short type names or full schema URIs. Label and annotation schemas are defined using the [`meta/ResourceLabel`](../../schemas/resources/v1alpha1/meta/ResourceLabel.json) and [`meta/ResourceAnnotation`](../../schemas/resources/v1alpha1/meta/ResourceAnnotation.json) metaschemas, so their values can be type-checked. Schemas may restrict which resource types they are valid for via `resourceTypes`, and labels can be marked as required for these types via `isRequired`.

Controllers may contribute their own labels to simplify common filtering scenarios, e.g. a `Dataset` resource may automatically get the `DatasetKind` label.

Labels can also carry information used in access control - see [Auth Attributes](../auth-and-privacy/authorization.md#auth-attributes).


## References
Resources can link to other resources using **references** ([`ResourceRef`](../../schemas/resources/v1alpha1/ResourceRef.json)). A reference can identify the target by:
- `id` - unique within a node
- `did` - unique globally
- `type`, `name`, and optional `account` - when `account` is not specified, the name is resolved within the account of the current principal

Example of referencing a resource by type, name, and account:

```yaml
volume:
  type: PersistentVolume
  name: my-s3-bucket
  account: my-org
```

Or in short form:

```yaml
volume: PersistentVolume:my-org/my-s3-bucket
```

Example of referencing by ID:

```yaml
volume:
  type: PersistentVolume
  id: 6767a4ee-d74d-436e-84f9-709407869a26
```

Resource references form a DAG - cyclical references are not allowed.

All references and selectors share common schemas, so they can be handled uniformly by tools without knowing the specifics of individual resource types.


### Typed References
Domains may define extended versions of resource references that:
- Restrict target resources to a specific type (e.g. `DatasetRef`)
- Provide additional features (e.g. referencing a sub-path of a secret via `Secret:postgres#password`)

Such references are validated against the [`meta/ResourceRef`](../../schemas/resources/v1alpha1/meta/ResourceRef.json) metaschema.


### Reference Resolution
Referencing resources by name is convenient when setting up resource graphs, especially when target resources are not created yet. Over the long term, however, as resources get renamed and deleted, name-based references become unreliable and can pose security risks when old names get reused.

Nodes therefore resolve all references to stable IDs upon first apply. Resolved references are represented as **handles** ([`ResourceHandle`](../../schemas/resources/v1alpha1/ResourceHandle.json)) that always include the `id`, while `type`, `name`, and `account` remain for human readability:

```yaml
volume:
  id: 6767a4ee-d74d-436e-84f9-709407869a26  # Stable reference
  type: PersistentVolume
  name: my-s3-bucket  # Remains only for human readability
  account: my-org
```

**Dangling references are allowed** - implementations may warn users about them, but:
- It must be possible to create a resource that references a non-existing resource and have the target created later
- It must be possible to delete a resource referenced by others


## Selectors
Multiple resources can be referenced at once using **selectors** ([`ResourceSelector`](../../schemas/resources/v1alpha1/ResourceSelector.json)) that match resources by:
- Owning account
- Resource type
- Name pattern in SQL `LIKE` format
- Label filters ([`LabelFilter`](../../schemas/resources/v1alpha1/LabelFilter.json))

```yaml
$schema: https://opendatafabric.org/schemas/flows/v1alpha1/Flow
headers:
  name: periodic-compaction
spec:
  target:
    type: Dataset
    name: org.opendatafabric.%
    labels:
      DatasetKind: Root
      env: prod
  ...
```

Being a superset of references, selectors can also match singular resources by `id`, `did`, or type and name without wildcards.

Unlike references, selectors are not resolved during apply. Whether a resource matches a selector is determined repeatedly during controller operations, so resources can start and stop matching a selector after the resource containing it was applied.


## Owner References
When a resource is created by a controller of another higher-level resource, the association is recorded in the `ownerReferences` header. This creation provenance can be used for lineage tracking, cascading deletion, and garbage collection.

```yaml
$schema: https://opendatafabric.org/schemas/flows/v1alpha1/FlowRun
headers:
  name: periodic-compaction-aabbcc
  ownerReferences:
    - id: c27331ce-ce88-4ff9-8c5a-4ce8107cc03f
      name: periodic-compaction
spec: {}
```

Owner references are distinct from [ownership](#ownership) by an account.


## Generations
Resource reconciliation is an **eventually-consistent** process - while a controller is reconciling one version of a resource, its desired state may be changed by the user. To reflect this, the node maintains the following headers:
- `generation` - incremented every time the resource headers or spec are updated, signifying changes to the desired state. Does not increment on status changes.
- `statusGeneration` - incremented every time the resource status is updated.
- `createdAt`, `updatedAt` - time when the resource was first applied and last updated (including status updates)

```yaml
$schema: https://opendatafabric.org/schemas/config/v1alpha1/SecretSet
headers:
  name: my-secrets
  generation: 4
  statusGeneration: 9
  createdAt: 2026-01-01T00:00:00Z
  updatedAt: 2026-01-04T00:00:00Z
spec: {}
status: {}
```

Both generations start at `1`. Together they tell whether anything changed in a resource since it was last read, without comparing its contents.

Status may be updated by several controllers concurrently. A controller can pass the `statusGeneration` it read as a **precondition** of a status update, which is rejected as a conflict if the status was modified in the meantime. This allows **compare-and-swap** updates without the check and the update racing with other controllers.


## Status
The `status` section provides information about the reconciliation of the resource:
- `observedGeneration`, `observedAt` - the generation last seen by the main controller of the resource, and when
- `reconciledGeneration`, `reconciledAt` - the generation last successfully reconciled by the main controller, and when
- `phase` - see [Phases](#phases)
- `conditions` - see [Conditions](#conditions)

Example of a resource where reconciliation of `generation: 2` failed because it links to a non-existing secret, leaving the previous generation running:

```yaml
$schema: https://opendatafabric.org/schemas/sources/v1alpha1/Source
headers:
  name: api-polling-source
  generation: 2
spec: {}
status:
  phase: Degraded
  observedGeneration: 2
  observedAt: 2026-01-02T00:00:00Z
  reconciledGeneration: 1
  reconciledAt: 2026-01-01T00:00:00Z
  conditions:
    https://opendatafabric.org/schemas/resources/v1alpha1/ReconciliationError:
      code: unresolved-reference
      message: "Secret `new-api-key` not found"
```


### Phases
The `phase` field ([`ResourcePhase`](../../schemas/resources/v1alpha1/ResourcePhase.json)) is populated as follows:

| `generation` | `observedGeneration` | `reconciledGeneration` | `phase` | Meaning |
| --- | --- | --- | --- | --- |
| 1 | - | - | `Pending` | Controller has not seen the resource yet |
| 3 | 2 | 2 | `Pending` | Controller has not seen generation 3 yet |
| 3 | 3 | 2 | `Reconciling` | Controller is actively applying generation 3 |
| 3 | 3 | 3 | `Ready` | Fully reconciled, running the current spec |
| 3 | 3 | 2 | `Degraded` | Reconciliation of generation 3 failed, but generation 2 is still running |
| 3 | 3 | - | `Failed` | Controller has seen generation 3, but never successfully reconciled anything |
| any | any | any | `Deleting` | The `deletionRequestedAt` header is set and controllers are cleaning up |
| any | any | any | `Deleted` | Controllers cleaned up and the `deletedAt` header is set |

Deletion takes precedence over the other phases. Deleting a resource is a two-step process, so that controllers can clean up the entities they created or finish work in progress before the resource disappears.


### Conditions
The `conditions` field provides extended information about the state of a specific resource type. Conditions can be contributed by multiple controllers and are keyed by schema URIs to avoid name collisions and allow schema checking. Condition schemas are defined using the [`meta/ResourceCondition`](../../schemas/resources/v1alpha1/meta/ResourceCondition.json) metaschema.

When reconciliation fails, the controller reports the cause using the [`ReconciliationError`](../../schemas/resources/v1alpha1/ReconciliationError.json) condition, which carries a machine-readable `code` and a human-readable `message`.

Conditions contributed by controllers other than the main controller of the resource should carry their own `observedGeneration` and `reconciledGeneration` fields, as they may lag behind or be ahead of the main controller.


## Resource Application
When a batch of manifests is applied, the node performs the following steps:
1. Validate each manifest against its schema
2. Look up whether target resources already exist (by `id` or `name`)
3. Assign `id`s to resources that don't yet exist
4. [Resolve](#reference-resolution) all references to `id`s of existing resources or of those about to be created
5. Order resources in depth-first dependency order
6. Pass every resource to the pre-apply step of its main controller, which may:
   - Perform additional validation
   - Contribute additional labels
   - Rewrite parts of the spec (e.g. to encrypt raw secrets before the spec is stored)
7. Increment the resource `generation`
8. Store the resource
9. Initiate reconciliation asynchronously


## Schemas

| Schema | Description |
| --- | --- |
| [`Resource`](../../schemas/resources/v1alpha1/Resource.json) | Canonical representation of a resource |
| [`ResourceInput`](../../schemas/resources/v1alpha1/ResourceInput.json) | User-authored representation of a resource |
| [`ResourceHeaders`](../../schemas/resources/v1alpha1/ResourceHeaders.json) | Canonical identity, ownership, and lifecycle headers |
| [`ResourceHeadersInput`](../../schemas/resources/v1alpha1/ResourceHeadersInput.json) | User-authored headers |
| [`ResourceLabels`](../../schemas/resources/v1alpha1/ResourceLabels.json) | Indexed key-value pairs |
| [`ResourceAnnotations`](../../schemas/resources/v1alpha1/ResourceAnnotations.json) | Non-indexed key-value pairs |
| [`ResourceStatus`](../../schemas/resources/v1alpha1/ResourceStatus.json) | Reconciliation status |
| [`ResourcePhase`](../../schemas/resources/v1alpha1/ResourcePhase.json) | Reconciliation or deletion phase |
| [`ResourceConditions`](../../schemas/resources/v1alpha1/ResourceConditions.json) | Conditions contributed by controllers |
| [`ReconciliationError`](../../schemas/resources/v1alpha1/ReconciliationError.json) | Condition describing a failed reconciliation |
| [`ResourceRef`](../../schemas/resources/v1alpha1/ResourceRef.json) | Reference to another resource |
| [`ResourceHandle`](../../schemas/resources/v1alpha1/ResourceHandle.json) | Resolved reference to another resource |
| [`ResourceSelector`](../../schemas/resources/v1alpha1/ResourceSelector.json) | Matches zero or many resources |
| [`LabelFilter`](../../schemas/resources/v1alpha1/LabelFilter.json) | Filter on resource labels |
| [`meta/Resource`](../../schemas/resources/v1alpha1/meta/Resource.json) | Metaschema for canonical resource schemas |
| [`meta/ResourceInput`](../../schemas/resources/v1alpha1/meta/ResourceInput.json) | Metaschema for user-authored resource schemas |
| [`meta/ResourceRef`](../../schemas/resources/v1alpha1/meta/ResourceRef.json) | Metaschema for reference types |
| [`meta/ResourceHandle`](../../schemas/resources/v1alpha1/meta/ResourceHandle.json) | Metaschema for resolved reference types |
| [`meta/ResourceLabel`](../../schemas/resources/v1alpha1/meta/ResourceLabel.json) | Metaschema for label schemas |
| [`meta/ResourceAnnotation`](../../schemas/resources/v1alpha1/meta/ResourceAnnotation.json) | Metaschema for annotation schemas |
| [`meta/ResourceCondition`](../../schemas/resources/v1alpha1/meta/ResourceCondition.json) | Metaschema for condition schemas |

Example manifests of various resource types are available in the [`examples`](../../examples) directory, and the relationships between core resource types are shown in the [entity-relationship diagram](../../contracts/erd.svg).
