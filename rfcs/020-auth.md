# RFC-020: Authorization Model <!-- omit in toc -->

[![PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/133?label=PR)](https://github.com/kamu-data/open-data-fabric/pull/133)

**Start Date**: 2026-09-23

**Published Date**: ???

**Authors**:
- [Sergii Mikhtoniuk](mailto:sergii.mikhtoniuk@kamu.dev), [Kamu](https://kamu.dev)
- [Sergiy Zaychenko](mailto:sergiy.zaychenko@kamu.dev), [Kamu](https://kamu.dev)


**Compatibility**:
- [X] Backwards-compatible
- [ ] Forwards-compatible


## Summary <!-- omit in toc -->

This RFC defines the authorization model for ODF nodes. It specifies how accounts, groups, auth attributes, and policies are expressed within the [Resource Framework](./018-iac-resource-framework.md). The data model is designed to be engine-agnostic. A minimal node can implement simple ownership checks, while a full node can plug in a RBAC/ABAC policy engine (such as Cedar) or a ReBAC engine (such as OpenFGA).


## Table of Contents <!-- omit in toc -->

- [Motivation](#motivation)
- [Proposed Solution](#proposed-solution)
  - [Auth Maturity Levels](#auth-maturity-levels)
  - [Accounts](#accounts)
  - [Groups](#groups)
  - [Attributes](#attributes)
  - [Actions](#actions)
    - [`AddLabel` action](#addlabel-action)
  - [Policies and Policy Bindings](#policies-and-policy-bindings)
    - [Generic policies](#generic-policies)
    - [Engine-specific policies](#engine-specific-policies)
  - [Relations (ReBAC)](#relations-rebac)
- [Compatibility](#compatibility)
- [Drawbacks](#drawbacks)
- [Rationale and alternatives](#rationale-and-alternatives)
- [Prior art](#prior-art)
- [Unresolved questions](#unresolved-questions)
- [Future possibilities](#future-possibilities)
- [Schema changes required](#schema-changes-required)


## Motivation

ODF nodes host datasets and other resources that belong to different accounts. A flexible, standards-aligned access control model is needed to control:
- Who can view and manage resources under a given account (control plane)
- Who can decrypt secrets and use signing keys
- Who can read or write the data of a dataset (data plane)
- Whether a dataset is public and who can apply labels to make dataset publicly readable
- Which accounts belong to privileged groups (e.g. node administrators)

The model must be expressive enough to support GitHub-style organization permissions while remaining implementable by minimal nodes that may not have a full policy engine available.


## Proposed Solution

### Auth Maturity Levels

We loosely define several authorization maturity levels that ODF implementations may choose to support:

| Maturity Level | Description |
|---|---|
| **0 - No Auth** | All resources and actions are available to the caller |
| **1 — Personal Ownership** | Support for `Account`s and access based on ownership only |
| **2 — Policies** | Support for generic `Policy`, per-account `PolicyBinding`, and resource attributes |
| **3 — Groups** | Support for `Group` mebership and binding policies to groups |
| **4 — ReBAC** | Support for `Relations` and graph-based policies |


### Accounts

An `Account` resource represents an identity that can own resources. ODF recognizes two kinds of accounts:

- **User** — an individual authenticated identity (human or service account)
- **Organization** — a group identity that owns resources but cannot itself perform actions; access is always exercised through its member users

Both share a single `Account` resource type, using a discriminated `spec.kind` field:

```yaml
# A user account
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/Account
headers:
  name: alice
spec:
  kind: User
  email: alice@example.com
  displayName: Alice
```

```yaml
# An organization account
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/Account
headers:
  name: acme
spec:
  kind: Organization
  displayName: Acme Corp
  admins:
    - alice # AccountRef
```

Only `User` accounts are valid principals in authorization requests — an `Organization` cannot directly perform actions.

Unauthenticated requests are represented by a special `UserAnonymous` principal type, which is distinct from `Account` and carries no identity or ownership.


### Groups

A `Group` resource represents a named set of accounts. Membership is declared inline in the spec. Groups support nesting — a group may include other groups as members.

```yaml
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/Group
headers:
  name: engineering
  account: acme
spec:
  members:
    - Account:alice
    - Account:bob
    - Group:acme/engineering-contractors  # nested group
```

Groups owned by the special `node` organization account can be used to represent node-wide predefined roles like `node/admin`.


### Attributes

Some properties of a resource directly affect access control — for example, whether a dataset allows public reads. ODF expresses these as **labels** on the resource they describe rather than as separate ACL objects. This keeps visibility declaration atomic with the resource manifest.

We propose to define this initial set of auth attributes:

| Label | Type | Description |
|---|---|---|
| `dataset/v1alpha1/AllowPublicRead` | `boolean` | Readable by any authenticated user |
| `dataset/v1alpha1/AllowAnonymousRead` | `boolean` | Readable without authentication |

In resource manifest:
```yaml
$schema: https://opendatafabric.org/schemas/dataset/v1alpha1/Dataset
headers:
  name: bobs-dataset
  account: bob
  labels:
    AllowPublicRead: true
    AllowAnonymousRead: false
spec:
  kind: Root
  metadata: []
```

A label schema declares itself as an auth attribute by setting `labelProperties.isAuthAttribute: true`. The node's resource controller materializes such labels into the auth engine automatically on reconciliation.

> **Note:** Labels like `AllowPublicRead: true` may require special permissions to set, as not every maintainer of the dataset should have the power to make data public. Per-label access control is discussed in [`AddLabel` action](#addlabel-action) section.

Auth attributes are propagated from resources to entities they define, e.g. an `AllowPublicRead` attribute on the dataset resource (control plane) will also appear as attribute on `Dataset` entity (data plane).


### Actions

Authorization decisions are made against a set of well-defined actions. Like label schemas, each action is defined by a JSON schema using the `Action` metaschema. The schema URI is the canonical action identifier used in authorization requests and policies, for example `https://opendatafabric.org/schemas/resources/v1alpha1/actions/View`. Action schemas may also define `contextSchema` to declare additional values they need to perform decisions.

Below is a set of initial actions we'll support.

**Resource actions** — operate on specific resource instances or entire resource types (by URI). The target account is supplied in the request context (`targetAccount`), since no instance exists yet:

| Action | Object | Description |
|---|---|---|
| `Create` | `TypeUri` | Create a new resource instance under an account |
| `List` | `TypeUri` | List resource instances of this type under an account |
| `View` | `Resource` | Read the resource manifest (metadata) |
| `Edit` | `Resource` | Modify the resource manifest |
| `Delete` | `Resource` | Delete the resource instance |
| `AddLabel` | `Resource` | Apply a specific label to the resource |
| `BindPolicy` | `Resource` | Create `PolicyBinding` resources that reference this resource as `object` |

**Dataset actions** — operate on dataset instances in the data plane, separate from dataset resources:

| Action | Object | Description |
|---|---|---|
| `Read` | `Dataset` | Read dataset data (query, download) |
| `Write` | `Dataset` | Append new data to the dataset |
| `AlterHistory` | `Dataset` | Rewrite or truncate dataset history |

**Config actions** — operate on configuration entities:

| Action | Object | Description |
|---|---|---|
| `Decrypt` | `SecretSet` | Decrypt a secret from a `SecretSet` |


#### `AddLabel` action

`AddLabel` is an example action that operates on the sub-resource level. It authorizes applying a specific label to a resource. It is distinct from `edit` to allow fine-grained control. A user may have `edit` permission but be restricted from changing auth-sensitive labels like `AllowPublicRead`.

An example of Cedar policy using this action may look like this:

```cedar
// Only "acme" organization admins may mark a dataset public
permit(
  principal in odf::auth::Group::"acme/admin",
  action in [odf::resources::Action::"https://opendatafabric.org/schemas/resources/v1alpha1/actions/AddLabel"],
  resource is odf::resources::Resource
) when {
  context.labelUri == "https://opendatafabric.org/schemas/dataset/v1alpha1/AllowPublicRead" &&
  resource.owner in odf::auth::Account::"acme"
};
```


### Policies and Policy Bindings

`Policy` and `PolicyBinding` resources are the primary mechanism for granting access beyond simple ownership.

A `Policy` resource is a reusable, potentially parameterized grant template. It declares what actions are permitted but leaves `principal` and `object` as slots to be filled in later.

A `PolicyBinding` instantiates a `Policy` by supplying the concrete `principal` and `object`. This separation mirrors the Kubernetes `Role`/`RoleBinding` pattern: a node admin or org admin defines policy templates once; a resource owner creates bindings to apply them to specific resources.

Creating a `PolicyBinding` is itself an authorized operation — the principal creating it must hold the `BindPolicy` action on the referenced `object`. Resource owners implicitly hold `BindPolicy` on their own resources and can grant it to others, enabling delegated access management without transferring ownership.

A `Policy` resource's `kind` field selects the policy language. ODF spec only defines the `Generic` policy type — an engine-agnostic way to express grants directly in terms of ODF action URIs. Additional kinds like `Cedar` and `Rego` may be added by implementations.


#### Generic policies

A `Generic` policy grants a set of ODF actions on a resource or resource type. It is the portable building block — any node that implements Level 2+ can evaluate it without a specialized policy engine:

```yaml
# A reusable "dataset reader" policy template
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/Policy
headers:
  name: dataset-reader
  account: node
spec:
  kind: Generic
  generic:
    actions:
      - https://opendatafabric.org/schemas/resources/v1alpha1/actions/View
      - https://opendatafabric.org/schemas/datasets/v1alpha1/actions/Read
    # principal and object are filled in by PolicyBinding
```

```yaml
# Bob grants Alice reader access to his dataset
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/PolicyBinding
headers:
  account: bob
  name: alice-foo-reader
spec:
  policy: Policy:node/dataset-reader
  principal: Account:alice
  object: Dataset:bob/dataset-foo
```

Higher-level features like dataset roles (Reader, Editor, Maintainer) are not part of the ODF spec — implementations can build them using groups, `Generic` policy templates, and `PolicyBinding` resources automatically.


#### Engine-specific policies

Nodes with a full policy engine may support richer `Policy` kinds that can express attribute-based conditions, org-scoped grants, and custom logic that `Generic` cannot.

Here is an example of a custom Cedar policy template of `acme` organization:

```yaml
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/Policy
headers:
  name: dataset-reader-acme
  account: node
spec:
  kind: Cedar
  body: |
    permit(
      principal == ?principal,
      action in [
        odf::resources::Action::"https://opendatafabric.org/schemas/resources/v1alpha1/actions/View",
        odf::datasets::Action::"https://opendatafabric.org/schemas/datasets/v1alpha1/actions/Read"
      ],
      resource == ?resource
    ) when {
      resource.owner in odf::auth::Account::"acme"
    };
```

The `?principal` and `?resource` slots above are filled in by the `PolicyBinding`:

```yaml
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/PolicyBinding
headers:
  account: bob
  name: alice-reader-binding
spec:
  policy: Policy:node/dataset-reader-acme
  principal: Account:alice
  object: Dataset:acme/weather
```

Nodes that do not support a given `Policy` kind MUST reject the resource with a clear error rather than silently ignoring it.


### Relations (ReBAC)

Nodes that implement a full ReBAC engine (e.g. OpenFGA, SpiceDB) may additionally support the `Relations` resource for expressing arbitrary relation triples of the form `(subject, relation, object)`:

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

`Relations` support is optional. Nodes that do not support it MUST reject `Relations` resources.


## Compatibility

This RFC is backwards-compatible. No existing metadata event schemas are changed. All new resource types are additive. Nodes that do not implement auth enforcement will ignore the new resources.


## Drawbacks

- `Policy` bodies for engine-specific kinds are not portable — a Cedar policy means nothing to an OpenFGA node. Interoperability at the policy layer is not a goal. Interoperability at the data layer (`Account`, `Group`, `Generic` policies) is.
- Inline group membership (vs. separate `Member` relation triples) creates write contention if many members are being added concurrently. Acceptable at ODF node scale - will revisit if needed.


## Rationale and alternatives

**Why a single `Account` resource with `kind: User | Organization` instead of separate types?**
ODF's code-generation toolchain maps one resource type to one API endpoint and CLI command family. Separate types would make `kamu list accounts` awkward to express. The `kind` discriminant pattern is already established by `Dataset` (`kind: Root | Derivative`).

**Why inline `spec.members` on `Group` instead of separate `Member` relations?**
Simpler to reason about, easier to diff in git, maps directly to auth engine entity parents. The `Member` relation approach carries Zanzibar-scale complexity that ODF nodes don't need. A group's membership is a property of the group, not a separate lifecycle object.

**Why no `RoleAssignment` resource?**
Named dataset roles (Reader, Editor, Maintainer) are a useful UX abstraction but are not universal — different implementations may define different roles for different resource types. Rather than standardizing a role vocabulary in the ODF spec, implementations like Kamu build role UX on top of `Group`, `Generic` policy templates, and `PolicyBinding` resources. The spec stays minimal; the implementation provides the ergonomics.

**Why a `Generic` policy kind instead of just engine-specific kinds?**
Engine-specific policy bodies (Cedar, Rego) are not portable across implementations. `Generic` policies — expressed directly in terms of ODF actions — can be evaluated by any Level 2+ node without a specialized engine, preserving interoperability for the common cases (grant action X to principal Y on resource Z).

**Why `Policy`/`PolicyBinding` instead of a flat ACL?**
Policies and bindings have different lifecycles and different authors. A system admin writes `Policy` templates once; a data owner creates `PolicyBinding` resources to instantiate them per resource. Separating them mirrors the Cedar template/link model and the Kubernetes `Role`/`RoleBinding` pattern.

**Why engine-agnostic instead of mandating Cedar?**
ODF is a specification, not an implementation. Mandating a specific policy engine would exclude implementations that use Rego, custom evaluators, or pure ReBAC engines. The layered model lets each implementation choose what fits its stack while remaining interoperable at the data level.


## Prior art

- [Google Zanzibar](https://research.google/pubs/zanzibar-googles-consistent-global-authorization-system/) — the canonical ReBAC system; ODF's `Relations` model is directly inspired by it.
- [OpenFGA](https://openfga.dev/) — open-source ReBAC engine implementing the Zanzibar model.
- [Cedar](https://cedarpolicy.com/en) — open-source policy engine by AWS; the reference implementation for Layer 3 nodes.
- [Open Policy Agent (OPA)](https://www.openpolicyagent.org/) / [Rego](https://www.openpolicyagent.org/docs/latest/policy-language/) — general-purpose policy engine and its declarative language; an alternative Layer 3 engine for nodes that prefer Rego's flexibility over Cedar's formal verification guarantees.
- [Kubernetes RBAC](https://kubernetes.io/docs/reference/access-authn-authz/rbac/) — `Role` + `RoleBinding` pattern; directly analogous to `Policy` + `PolicyBinding`.
- [GitHub repository permissions](https://docs.github.com/en/get-started/learning-about-github/access-permissions-on-github) — teams (groups) + per-repo role assignments; the UX model ODF targets.
- Other projects: [Auth0 FGA](https://auth0.com/fine-grained-authorization), [Topaz](https://www.topaz.sh/), [StrongDM](https://www.strongdm.com/), [SpiceDB](https://authzed.com/spicedb), [Amazon Verified Permissions](https://aws.amazon.com/verified-permissions/)


## Unresolved questions

- Interplay between this model and decentralized capabilities-based auth (UCAN).
- How should the admission layer enforce that `headers.account` on a `PolicyBinding` or `Relations` resource matches the owner of all referenced objects — reject at admission or flag as a validation error in status?
- Should `PolicyBinding` support scoping to a `TypeUri` (resource type) rather than a specific resource instance, to express "this group can create any Dataset under this account"?


## Future possibilities

- **Cross-node relations:** A subject on one ODF node granting access to an object on another, using DIDs for stable cross-node identity.
- **Audit log:** Surface all auth resource changes as events in the event sourcing store for compliance and forensics.
- **Transfer ownership:** A dedicated action and policy mechanism for transferring resource ownership between accounts.


## Schema changes required

The following schema work is needed to implement this RFC. None of these changes affect existing schemas — all are additive new resource types or new optional fields.

| Schema | Change |
|---|---|
| `auth/v1alpha1/Account` | New resource type with `spec.kind: User \| Organization` |
| `auth/v1alpha1/Group` | Add `spec.members: [AccountRef \| GroupRef]` |
| `auth/v1alpha1/meta/Action` | New metaschema for action schemas |
| `resources/v1alpha1/actions/*` | New action schemas: `Create`, `List`, `View`, `Edit`, `Delete`, `AddLabel` |
| `datasets/v1alpha1/actions/*` | New action schemas: `Read`, `Write`, `AlterHistory` |
| `config/v1alpha1/actions/*` | New action schemas: `Decrypt` |
| `auth/v1alpha1/Policy` | New resource type with `spec.kind: Generic \| Cedar \| Rego` |
| `auth/v1alpha1/PolicyBinding` | New resource type |
| `auth/v1alpha1/Relations` | Existing — no changes needed |
