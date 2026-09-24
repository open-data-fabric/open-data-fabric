# RFC-020: Authorization Model <!-- omit in toc -->

[![Issue](https://img.shields.io/github/issues/detail/state/kamu-data/open-data-fabric/1?label=Issue)](https://github.com/kamu-data/open-data-fabric/issues/1)
[![PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/12?label=PR)](https://github.com/kamu-data/open-data-fabric/pull/12)

**Start Date**: 2026-09-23

**Published Date**: ???

**Authors**:
- [Sergii Mikhtoniuk](mailto:sergii.mikhtoniuk@kamu.dev), [Kamu](https://kamu.dev)
- [Sergiy Zaychenko](mailto:sergiy.zaychenko@kamu.dev), [Kamu](https://kamu.dev)


**Compatibility**:
- [X] Backwards-compatible
- [ ] Forwards-compatible


## Summary <!-- omit in toc -->
This RFC defines the authorization model for ODF nodes. It introduces Relationship-Based Access Control (ReBAC) and specifies how auth attributes and relations are expressed within the [Resource Framework](./018-iac-resource-framework.md).

## Table of Contents <!-- omit in toc -->

- [Motivation](#motivation)
- [Guide-level explanation](#guide-level-explanation)
  - [Attributes](#attributes)
  - [Relations](#relations)
  - [Groups](#groups)
- [Reference-level explanation](#reference-level-explanation)
  - [Auth Attribute Labels](#auth-attribute-labels)
  - [Relation Triples](#relation-triples)
  - [Relation Ownership](#relation-ownership)
  - [Groups and Membership](#groups-and-membership)
- [Compatibility](#compatibility)
- [Drawbacks](#drawbacks)
- [Rationale and alternatives](#rationale-and-alternatives)
- [Prior art](#prior-art)
- [Unresolved questions](#unresolved-questions)
- [Future possibilities](#future-possibilities)
- [TODO](#todo)


## Motivation

ODF nodes host datasets and other resources that belong to different accounts. A flexible, standards-aligned access control model is needed to express:
- Whether a dataset is publicly readable (no login required) or open to any authenticated user
- Which accounts have specific roles on a dataset (e.g. Reader, Editor, Maintainer)
- Which accounts belong to privileged groups (e.g. node administrators)

ODF adopts Relationship-Based Access Control (ReBAC) — the model used by Google Zanzibar and systems like OpenFGA and SpiceDB. In ReBAC, all permissions are derived from a graph of relations between resources. This makes the model expressive, auditable, and extensible without requiring schema changes when new resource types or permission types are introduced.


## Guide-level explanation

### Attributes

Some properties of a resource directly affect access control — for example, whether a dataset allows public reads. Rather than storing these in a separate permission object, ODF expresses them as **labels** on the resource they describe.

A label schema declares itself as an auth attribute by setting `labelProperties.isAuthAttribute: true`. The node's resource controller materializes such labels into the ReBAC attribute store automatically on reconciliation.

Example: Bob's dataset declares its visibility via labels:

```yaml
$schema: https://opendatafabric.org/schemas/dataset/v1alpha1/Dataset
headers:
  name: bobs-dataset
  account: bob
  labels:
    AllowPublicRead: true  # https://opendatafabric.org/schemas/dataset/v1alpha1/AllowPublicRead
    AllowAnonymousRead: false  # https://opendatafabric.org/schemas/dataset/v1alpha1/AllowAnonymousRead
spec:
  kind: Root
  metadata: []
```

Labels can be referenced by their full URI or by their short name (the last path segment of the URI). Both forms are equivalent; the full URI is resolved by the node.

The auth attribute value lives with the resource it describes. This means the account that owns the resource controls its visibility — no separate permission object needs to be managed.

> **Admission control note:** Labels with `isAuthAttribute: true` are validated by the node's admission layer. The owning account is the only one that can change them. This prevents users from elevating their own access by patching another account's resource labels.


### Relations

Access roles are declared using `Relations` resources — a list of relation triples of the form `(subject, relation, object)`. Each triple grants a subject (an account or group) a specific relation to an object (a resource).

Example: Bob grants Alice the `Maintainer` role on his dataset:

```yaml
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/Relations
headers:
  account: bob
  name: alice-bobs-dataset
spec:
  relations:
    - subject: Account:alice
      relation: DatasetRole
      value: Maintainer
      object: Dataset:bob/bobs-dataset
```

The `relation` field resolves to a full schema URI (e.g. `https://opendatafabric.org/schemas/dataset/v1alpha1/DatasetRole`). The `value` is validated against the corresponding schema — in this case a string enum `Reader | Editor | Maintainer`.

The `Relations` resource lives under the account that owns the **object** being protected (Bob, in this case). This ties the lifecycle of the permission grant to the data owner: if Bob's account is deleted, his relations are deleted too, and Alice's access is automatically revoked via cascading deletion through `ownerReferences`.


### Groups

For managing permissions at scale — for example, granting admin access to a set of accounts — ODF provides a `Group` resource. Groups have no properties of their own; their meaning is entirely derived from the members assigned to them via `Member` relations.

Example: creating an admin group and adding Alice to it:

```yaml
# groups/admin.yaml
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/Group
headers:
  name: admin
  account: system
spec: {}
```

```yaml
# relations/admins.yaml
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/Relations
headers:
  account: system
  name: admins
spec:
  relations:
    - subject: Account:alice
      relation: Member
      object: Group:system/admin
```

The `Member` relation carries no value (`type: null`) — membership is binary. Groups can then be assigned roles on resources using `DatasetRole` or other relation schemas, and the ReBAC engine resolves membership transitively.


## Reference-level explanation

### Auth Attribute Labels

A label schema that declares `labelProperties.isAuthAttribute: true` uses the `ResourceLabel` metaschema:

```json
{
  "$id": "https://opendatafabric.org/schemas/dataset/v1alpha1/AllowPublicRead",
  "$schema": "https://opendatafabric.org/schemas/metaschemas/v1alpha1/ResourceLabel",
  "description": "Controls whether the dataset is readable by any authenticated user.",
  "type": "boolean",
  "labelProperties": {
    "isAuthAttribute": true,
    "resourceTypes": [
      "https://opendatafabric.org/schemas/dataset/v1alpha1/Dataset"
    ]
  }
}
```

The `resourceTypes` field restricts which resource types may carry this label. When a resource is reconciled, the controller scans its labels, finds all schemas with `isAuthAttribute: true`, and upserts the corresponding entries in the ReBAC attribute store.

Auth attributes must be declared as `labels` (indexed), not `annotations` (non-indexed). The node's admission webhook enforces this.

Currently defined auth attribute labels:

| Label | Type | Description |
|---|---|---|
| `dataset/v1alpha1/AllowPublicRead` | `boolean` | Readable by any authenticated user |
| `dataset/v1alpha1/AllowAnonymousRead` | `boolean` | Readable without authentication |


### Relation Triples

A `Relations` resource contains one or more triples. Each triple is:

| Field | Description |
|---|---|
| `subject` | `ResourceRef` — the account or group being granted access |
| `relation` | Short name or full URI of the relation schema |
| `value` | Optional value; type is validated against the relation schema |
| `object` | `ResourceRef` — the resource being protected |

The `relation` field resolves to a schema URI. The node looks up that schema to:
1. Validate the `value` field against the schema's type definition
2. Check that `subject` and `object` resource types match `relationProperties.subjectResourceTypes` and `objectResourceTypes`

Example relation schema (`DatasetRole`):

```json
{
  "$id": "https://opendatafabric.org/schemas/dataset/v1alpha1/DatasetRole",
  "$schema": "https://opendatafabric.org/schemas/metaschemas/v1alpha1/Relation",
  "type": "string",
  "enum": ["Reader", "Editor", "Maintainer"],
  "relationProperties": {
    "subjectResourceTypes": [
      "https://opendatafabric.org/schemas/auth/v1alpha1/Account"
    ],
    "objectResourceTypes": [
      "https://opendatafabric.org/schemas/dataset/v1alpha1/Dataset"
    ]
  }
}
```


### Relation Ownership

A `Relations` resource must be owned by the account that owns the **object** resources referenced in its triples. The `headers.account` field on the `Relations` resource is enforced by the node's admission layer.

This design has two important properties:
- **Lifecycle coupling:** When the object owner's account is deleted, all their `Relations` resources are cascade-deleted, automatically revoking all grants they made.
- **Delegation control:** An account cannot grant access to resources they do not own. Alice cannot create a `Relations` resource under Bob's account to grant herself access to Bob's datasets.


### Groups and Membership

`Group` is a first-class resource with an empty spec. Its identity is its name and owning account. Groups owned by the `system` account are created by node administrators and represent node-wide roles.

The `Member` relation schema:

```json
{
  "$id": "https://opendatafabric.org/schemas/auth/v1alpha1/Member",
  "$schema": "https://opendatafabric.org/schemas/metaschemas/v1alpha1/Relation",
  "type": "null",
  "relationProperties": {
    "subjectResourceTypes": [
      "https://opendatafabric.org/schemas/auth/v1alpha1/Account"
    ],
    "objectResourceTypes": [
      "https://opendatafabric.org/schemas/auth/v1alpha1/Group"
    ]
  }
}
```

`type: null` means membership carries no value — the relation itself is the fact. The ReBAC engine resolves group membership transitively when evaluating access.


## Compatibility

This RFC is backwards-compatible. No existing metadata event schemas are changed. The authorization model is entirely additive — nodes without an auth enforcement layer will simply ignore `Relations` resources and auth attribute labels.


## Drawbacks

- Auth attributes tied to resource labels means changing visibility requires updating the resource manifest. This is intentional (visibility is part of the resource's declared state) but may feel surprising to users accustomed to separate ACL objects.
- The relation ownership rule (owner of object controls grants) prevents delegation. An account cannot grant someone else the ability to further grant access. If delegation is needed in the future it will require extending the model.


## Rationale and alternatives

**Why ReBAC over RBAC?**
Classic role-based access control assigns roles to users globally. ReBAC scopes relations to specific resource instances, making it far more expressive for a multi-tenant dataset system where Alice may be a Reader of one dataset and a Maintainer of another.

**Why labels for auth attributes instead of a separate AuthPolicy resource?**
Keeping visibility on the resource itself means a single `kamu apply` declares both the dataset and its access policy atomically. A separate policy object would require two resources to stay in sync and would make it easy to accidentally publish a dataset without setting its visibility.

**Why is relation ownership tied to the object owner?**
This mirrors how physical access control works — the owner of a thing decides who can access it. It also simplifies lifecycle management: deleting an account cleans up all its grants automatically.


## Prior art

- [Google Zanzibar](https://research.google/pubs/zanzibar-googles-consistent-global-authorization-system/) — the canonical ReBAC system; ODF's relation triple model is directly inspired by it.
- [OpenFGA](https://openfga.dev/), [SpiceDB](https://authzed.com/spicedb) — open-source ReBAC engines that implement the Zanzibar model.
- [Kubernetes RBAC](https://kubernetes.io/docs/reference/access-authn-authz/rbac/) — `Role` + `RoleBinding` pattern.
- [GitHub repository permissions](https://docs.github.com/en/get-started/learning-about-github/access-permissions-on-github) — teams (groups) + per-repo role assignments.


## Unresolved questions

- How should the node enforce that `headers.account` on a `Relations` resource matches the owner of all referenced objects? Should violations be rejected at admission or flagged as a validation error in status?
- How are roles defined for non-dataset resources (e.g. `Flow`, `SecretSet`)? Should there be a generic `ResourceRole` or per-resource-type role schemas?
- Should groups support nesting (a group as a member of another group)?


## Future possibilities

- **Custom roles:** Replace the fixed `DatasetRole` enum with a `DatasetRole` resource that defines granular permissions per action (read data, read metadata, write, admin). The `DatasetRole` relation schema would then reference these resources rather than a hardcoded enum.
- **Delegation:** Allow an account to grant another account the ability to further grant access, by adding a `canDelegate: true` flag to relation triples.
- **Cross-node relations:** A subject on one ODF node granting access to an object on another, using DIDs for stable cross-node identity.
- **Audit log:** Surface all relation changes as events in the event sourcing store for compliance and forensics.


## TODO
Stress test:
- Resources
  - list
  - read
  - write / delete
  - change certain auth labels?
  - transfer ownership?
- Datasets
  - read metadata / data
  - write metadata / data
- Admin stuff
- Orgs, Teams, Groups