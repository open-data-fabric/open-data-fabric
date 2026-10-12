# RFC-020: Authorization Model <!-- omit in toc -->

[![PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/133?label=PR)](https://github.com/kamu-data/open-data-fabric/pull/133)

**Start Date**: 2026-09-23

**Published Date**: 2026-10-12

**Authors**:
- [Sergii Mikhtoniuk](mailto:sergii.mikhtoniuk@kamu.dev), [Kamu](https://kamu.dev)
- [Sergiy Zaychenko](mailto:sergiy.zaychenko@kamu.dev), [Kamu](https://kamu.dev)


**Compatibility**:
- [X] Backwards-compatible
- [ ] Forwards-compatible


## Summary <!-- omit in toc -->

This RFC defines the authorization model for ODF nodes. It specifies how accounts, groups, auth attributes, and policies are expressed within the [Resource Framework](./018-iac-resource-framework.md). The data model is designed to be engine-agnostic. A minimal node can implement simple ownership checks, while a full node can plug in a RBAC / ABAC / ReBAC policy engine.


## Table of Contents <!-- omit in toc -->

- [Motivation](#motivation)
- [Proposed Solution](#proposed-solution)
  - [Terminology](#terminology)
  - [Auth Maturity Levels](#auth-maturity-levels)
  - [Accounts](#accounts)
  - [Principals](#principals)
  - [Subjects](#subjects)
  - [Groups](#groups)
  - [Attributes](#attributes)
  - [Actions](#actions)
    - [`AddLabel` action](#addlabel-action)
  - [Policies and Policy Bindings](#policies-and-policy-bindings)
    - [Generic policies](#generic-policies)
    - [Engine-specific policies](#engine-specific-policies)
- [Compatibility](#compatibility)
- [Drawbacks](#drawbacks)
- [Rationale and alternatives](#rationale-and-alternatives)
- [Prior art](#prior-art)
- [Unresolved questions](#unresolved-questions)
- [Future possibilities](#future-possibilities)
- [Appendix A: Cedar - Example Schema and Policies](#appendix-a-cedar---example-schema-and-policies)
- [Appendix B: OpenFGA - Example Schema](#appendix-b-openfga---example-schema)
- [Appendix C: Reserved Names](#appendix-c-reserved-names)


## Motivation

ODF nodes host datasets and other resources that belong to different accounts. A flexible, standards-aligned access control model is needed to govern:
- Who can view and manage resources under a given account (control plane)
- Who can decrypt secrets and use signing keys
- Who can read or write the data of a dataset (data plane)
- Whether a dataset is public and who can apply labels to make dataset publicly readable
- Which accounts belong to privileged groups (e.g. node administrators)

The model must be expressive enough to support GitHub-style organization permissions while remaining implementable by minimal nodes that may not have a full policy engine available.


## Proposed Solution

### Terminology
- **Node** - an entire ODF deployment acting under a single identity
- **Resource** - a control plane manifest that describes the desired state and status of a data plane entity
- **Entity** - a data plane object that is materialized from a resource (e.g. an ODF dataset, secret, encryption key)
- **User** - an authenticated account of an individual person
- **Organization** - an account that owns resources on behalf of a group of users but can't authenticate or act itself, so its resources are always accessed through its members
- **Service account** - a non-human identity used by automation (scheduled flows, CI, installed applications); it can act as a principal but doesn't own resources
- **Account** - a general term for a named identity (node, user, organization, service account)
- **Anonymous** - the principal of an unauthenticated request, which has no identity and owns nothing
- **Group** - a named, possibly nested set of principals that policies can be bound to, so that permissions are granted to all members at once
- **Principal** - the identity performing an action in an authorization request, established through authentication (a user, service account, node, or anonymous)
- **Subject** - the party a grant is given to in a policy binding or relation: a principal or a group
- **Action** - an operation the principal attempts to perform, identified by a schema URI (e.g. `View`, `Read`, `Decrypt`)
- **Object** - the target of an action: a resource, an entity, or a resource type (`TypeUri`) for actions like `Create` and `List` where no instance exists yet


### Auth Maturity Levels

We loosely define several authorization maturity levels that ODF implementations may choose to support:

| Maturity Level | Description |
|---|---|
| **0 - No Auth** | All resources and actions are available to the caller |
| **1 — Personal Ownership** | Support for `User` accounts and access based on ownership only |
| **2 — Policies** | Support for generic `Policy`, per-account `PolicyBinding`, and resource attributes |
| **3 — Groups and Organizations** | Support for `Group` membership, binding policies to groups, `Organization` and `ServiceAccount` accounts |


### Accounts

ODF recognizes the following kinds of accounts.

**Node** — an ODF node:

```yaml
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/Node
headers:
  name: node
spec:
  did: did:odf:0xfa..bc
  displayName: Current Node
```

**User** — an individual person who can authenticate and act as a principal:

```yaml
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/User
headers:
  name: alice
spec:
  email: alice@example.com
  displayName: Alice
```

**Organization** — a group identity that owns resources but cannot itself perform actions:

```yaml
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/Organization
headers:
  name: acme
spec:
  displayName: Acme Corp
```

Upon creation of an organization, the node automatically creates two groups under its account:
- `Group:acme/admins` — organization administrators, initially containing the principal that created the organization. Members of this group have the same rights over resources owned by the organization as an owner would
- `Group:acme/members` — organization members

These are ordinary groups that can be edited to customize membership (e.g. by nesting team groups). Their names are reserved, so they can't be created or deleted independently of the organization. Whether admins are implicitly considered members is left to implementations.

**ServiceAccount** — a non-human identity used by automation (scheduled flows, CI, installed applications) that can act as a principal but doesn't own resources:

```yaml
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/ServiceAccount
headers:
  account: acme
  name: ingest-prod
spec:
  displayName: Ingest (prod)
```

> **Note:** To avoid any ambiguity or impersonation, node implementations should ensure that names of nodes, users, and organizations are unique regardless of resource type, making it possible to refer to any account only by its name. Service account names don't follow this rule as they are always scoped under the owning account.


### Principals

A principal is the identity performing an action in an authorization request. It is established through authentication and is always a single concrete identity.

| Type | Can act as principal | Can own resources |
|---|---|---|
| `User` | ✅ | ✅ |
| `ServiceAccount` | ✅ | ❌ |
| `Node` | ✅ | ✅ |
| `Organization` | ❌ | ✅ |
| `Anonymous` | ✅ | ❌ |

- `Organization` cannot authenticate - its resources are always accessed through its members.
- `Anonymous` represents unauthenticated requests. It is not a resource and carries no identity.
- `Node` acts as a principal in system tasks and node-to-node communications. When a node executes work that belongs to a user or service account (e.g. a scheduled flow run) it should do so as that principal rather than as itself.

Principals are referenced using `PrincipalRef`, e.g. to specify the service account a flow runs as, or to record the initiator of an operation:

```yaml
serviceAccount: ServiceAccount:acme/ingest-prod  # PrincipalRef
```


### Subjects

A subject is the party a grant is given to in a `PolicyBinding`. Unlike a principal, a subject may stand for many principals at once.

Subjects are referenced using `SubjectRef`, which is a union of:

| Kind | Matches | Short form |
|---|---|---|
| `Resource` | A specific `User`, `ServiceAccount`, `Node`, or members of a `Group` | `User:alice`, `Group:acme/engineering` |
| `Authenticated` | Any authenticated principal | `Authenticated` |
| `Anonymous` | Any unauthenticated request | `Anonymous` |
| `Everyone` | Any principal, authenticated or not | `Everyone` |

Short forms expand as follows:

```yaml
subject: Group:acme/engineering
# Expands to:
subject:
  kind: Resource
  type: Group
  account: acme
  name: engineering

subject: Authenticated
# Expands to:
subject:
  kind: Authenticated
```

`Authenticated`, `Anonymous`, and `Everyone` can be used in policy bindings, but not as group members - otherwise nesting such a subject into a group would extend the group's permissions to all principals.


### Groups

A `Group` resource represents a named set of subjects. Membership is declared inline in the spec. Groups support nesting — a group may include other groups as members.

```yaml
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/Group
headers:
  name: engineering
  account: acme
spec:
  members:
    - User:alice  # SubjectRef
    - User:bob
    - Group:acme/engineering-contractors  # nested group
```

Groups owned by the system `Node` account like `Group:node/admins` can be used to represent node-wide predefined roles.


### Attributes

Some properties of a resource directly affect access control — for example, whether a dataset allows public reads. We expresses these properties using the existing [labels](./018-iac-resource-framework.md#labels--annotations) mechanism.

A label schema that declares `labelProperties.isAuthAttribute: true` signals that any resource carrying that label should have its value materialized into an auth attribute.

Example auth attribute label:
```json
{
  "$id": "https://opendatafabric.org/schemas/datasets/v1alpha1/AllowPublicRead",
  "$schema": "https://opendatafabric.org/schemas/resources/v1alpha1/meta/ResourceLabel",
  "description": "Controls whether the dataset is readable by any authenticated user.",
  "type": "boolean",
  "labelProperties": {
    "isAuthAttribute": true,
    "resourceTypes": [
      "https://opendatafabric.org/schemas/datasets/v1alpha1/Dataset"
    ]
  }
}
```

It now can be defined like any other resource label:
```yaml
$schema: https://opendatafabric.org/schemas/datasets/v1alpha1/Dataset
headers:
  name: my-dataset
  labels:
    # Short form - resolved to https://opendatafabric.org/schemas/datasets/v1alpha1/AllowPublicRead
    AllowPublicRead: true
    # Full URI form
    https://opendatafabric.org/schemas/datasets/v1alpha1/AllowAnonymousRead: false
spec:
  kind: Root
  metadata: []
```

Key properties:
- **Single authoring surface** — attributes live on the resource they describe, eliminating cross-ownership ambiguity
- **Typed and validated** — the label schema's `type` field is enforced at apply time
- **Indexed** — because auth attributes are labels, they are also queryable in the resource listing API
- **Schema-discoverable** — implementations enumerate label schemas with `isAuthAttribute: true` at startup to know which labels to materialize, without hardcoding a list
- **Auth attributes must be labels, not annotations** — validators reject a resource that places an `isAuthAttribute` label under `annotations`
- **Admission control** — changing a label with `isAuthAttribute: true` may require elevated permissions beyond those needed to update the rest of the resource spec.

We propose to define this initial set of auth attributes:

| Label | Type | Description |
|---|---|---|
| `datasets/v1alpha1/AllowPublicRead` | `boolean` | Readable by any authenticated principal |
| `datasets/v1alpha1/AllowAnonymousRead` | `boolean` | Readable without authentication |

In resource manifest:
```yaml
$schema: https://opendatafabric.org/schemas/datasets/v1alpha1/Dataset
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

Labels like `AllowPublicRead: true` may require special permissions to set, as not every maintainer of the dataset should have the power to make data public. Per-label access control is discussed in [`AddLabel` action](#addlabel-action) section.

Auth attributes are propagated from resources to entities they define, e.g. an `AllowPublicRead` attribute on the dataset resource (control plane) will also appear as attribute on ODF dataset entity (data plane).

> **Note:** The effects of `AllowPublicRead` and  `AllowAnonymousRead` can be achieved by defining policies for `Authenticated` or `Anonymous` [subjects](#subjects). Attributes, however, remain the primary way to make datasets public, as they are atomic with the resource manifest and can be efficiently filtered on when listing resources. Bindings to these subjects are better suited for granting other actions publicly, e.g. making a resource manifest viewable by everyone. Since the `BindPolicy` action receives the `subject` in its context, policies can restrict who is allowed to create such bindings.


### Actions

Authorization decisions are made against a set of well-defined actions. Like label schemas, each action is defined by a JSON schema using the `Action` metaschema. The schema URI is the canonical action identifier used in authorization requests and policies, for example `https://opendatafabric.org/schemas/resources/v1alpha1/actions/View`. The schema itself describes the context values needed to authorize the action (e.g. the label being added for `AddLabel`), while `actionProperties.objectTypes` lists the types of objects the action applies to. Every action lists the object types it applies to.

Below is a set of initial actions ODF will specify.

**Resource actions** — operate on resource instances. Exceptions are `Create` and `List`, which operate on a resource type (`TypeUri`) and supply the account under which resources are created or listed in the request context (`targetOwner`):

| Action | Object | Description |
|---|---|---|
| `Create` | `TypeUri` | Create a new resource instance under an account |
| `List` | `TypeUri` | List resource instances of this type under an account |
| `View` | `Resource` | Read the resource manifest (metadata) |
| `Edit` | `Resource` | Modify the resource manifest |
| `Delete` | `Resource` | Delete the resource instance |
| `AddLabel` | `Resource` | Apply a specific label to the resource |
| `RemoveLabel` | `Resource` | Remove a specific label from the resource |
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
  principal in odf::auth::Group::"acme/admins",
  action in [odf::resources::Action::"https://opendatafabric.org/schemas/resources/v1alpha1/actions/AddLabel"],
  resource is odf::resources::Resource
) when {
  context.labelType == odf::resources::TypeUri::"https://opendatafabric.org/schemas/datasets/v1alpha1/AllowPublicRead" &&
  resource.owner in odf::auth::ResourceOwner::"acme"
};
```


### Policies and Policy Bindings

Beyond resource ownership, `Policy` and `PolicyBinding` resources are the primary mechanism for granting permissions.

A `Policy` resource is a reusable, potentially parameterized grant template. It declares what actions are permitted but may leave `subject` and `object` as slots to be filled in later.

A `PolicyBinding` instantiates a `Policy` by supplying the concrete `subject` and `object`. This separation mirrors the Kubernetes `Role`/`RoleBinding` pattern: a node admin or org admin defines policy templates once; a resource owner creates bindings to apply them to specific resources.

Engine-specific policies that don't declare `subject` and `object` slots are not templates — they take effect on their own without a binding (e.g. a node-wide policy granting all actions to members of `Group:node/admins`).

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
  actions:
    - https://opendatafabric.org/schemas/resources/v1alpha1/actions/View
    - https://opendatafabric.org/schemas/datasets/v1alpha1/actions/Read
  # subject and object are filled in by PolicyBinding
```

```yaml
# Bob grants Alice reader access to his dataset
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/PolicyBinding
headers:
  account: bob
  name: alice-foo-reader
spec:
  policy: Policy:node/dataset-reader
  subject: User:alice  # SubjectRef
  object: Dataset:bob/dataset-foo  # ResourceRef
```

The `object` refers to both the resource and the entity materialized from it, so the policy above grants `View` on the dataset resource and `Read` on the dataset entity.

Higher-level features like dataset roles (`Reader`, `Editor`, `Maintainer`) are not part of the ODF spec — implementations can build them using groups, `Generic` policy templates, and `PolicyBinding` resources.


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
      resource.owner in odf::auth::ResourceOwner::"acme"
    };
```

The `?principal` and `?resource` slots above are filled in by the `subject` and `object` of the `PolicyBinding`:

```yaml
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/PolicyBinding
headers:
  account: bob
  name: alice-reader-binding
spec:
  policy: Policy:node/dataset-reader-acme
  subject: User:alice
  object: Dataset:acme/weather
```

Nodes that do not support a given `Policy` kind MUST reject the resource with a clear error rather than silently ignoring it.


## Compatibility

This RFC is backwards-compatible. No existing metadata event schemas are changed. All new resource types are additive, so existing resources remain valid. Nodes that do not implement auth enforcement will reject the new resources as unknown types.


## Drawbacks

- `Policy` bodies for engine-specific kinds are not portable — a Cedar policy means nothing to an OpenFGA node. Interoperability at the policy layer is not a goal. Interoperability at the data layer (accounts, groups, `Generic` policies) is.

- Inline group membership (vs. separate `Member` relation triples) creates write contention if many members are being added concurrently. Acceptable at ODF node scale - will revisit if needed.


## Rationale and alternatives

**Why a separate `ServiceAccount` type instead of a `User` with a flag?**
Service accounts differ from users in ways that matter for authorization: they are scoped under an owning account, cannot own resources, and have no email or interactive login. A separate type lets policies distinguish them explicitly (e.g. `principal is ServiceAccount`) and keeps user-only fields out of their manifests. Installing an application creates a service account under the installing account, so applications don't need a principal type of their own.

**Why are access token scopes not part of the authorization model?**
A token may be restricted to a set of scopes (e.g. read-only access to specific datasets), but scopes can only narrow what the principal is allowed to do. Checking whether an action on an object falls within the token's scopes is a pure function of the token, action, and object that needs no policies or hierarchies, so implementations perform it before policy evaluation. Policies then decide based on the principal's own permissions, so a token can never grant more than its owner has.

**Why are organizations part of the spec while teams and roles are not?**
Teams and roles can be fully expressed with groups, `Generic` policy templates, and bindings, so implementations can build them on top of the spec. Organizations can't: they are a type of resource owner that cannot act on its own, and granting their admins owner rights over all organization resources is not expressible with single-object `Generic` bindings. Since ownership is part of every resource header, owner types must also be consistent between implementations for manifests to be portable.

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

- [Google Zanzibar](https://research.google/pubs/zanzibar-googles-consistent-global-authorization-system/) — the canonical ReBAC system.
- [OpenFGA](https://openfga.dev/) — open-source ReBAC engine implementing the Zanzibar model.
- [Cedar](https://cedarpolicy.com/en) — open-source policy engine by AWS; used by the reference implementation for engine-specific policies.
- [Open Policy Agent (OPA)](https://www.openpolicyagent.org/) / [Rego](https://www.openpolicyagent.org/docs/latest/policy-language/) — general-purpose policy engine and its declarative language; an alternative engine for nodes that prefer Rego's flexibility over Cedar's formal verification guarantees.
- [Kubernetes RBAC](https://kubernetes.io/docs/reference/access-authn-authz/rbac/) — `Role` + `RoleBinding` pattern; directly analogous to `Policy` + `PolicyBinding`.
- [GitHub repository permissions](https://docs.github.com/en/get-started/learning-about-github/access-permissions-on-github) — teams (groups) + per-repo role assignments; the UX model ODF targets.
- Other projects: [Auth0 FGA](https://auth0.com/fine-grained-authorization), [Topaz](https://www.topaz.sh/), [StrongDM](https://www.strongdm.com/), [SpiceDB](https://authzed.com/spicedb), [Amazon Verified Permissions](https://aws.amazon.com/verified-permissions/)


## Unresolved questions

- Interplay between this model and decentralized capabilities-based auth (UCAN).
- How should the admission layer enforce that `headers.account` on a `PolicyBinding` resource matches the owner of all referenced objects — reject at admission or flag as a validation error in status?
- Who can author `Policy` resources? Initially only node admins should be allowed to create policies, as an engine-specific policy (e.g. `Cedar`) can grant arbitrary permissions across the node. Delegating policy authoring to e.g. organization admins will be addressed in future RFCs.


## Future possibilities

- **Cross-node relations:** A subject on one ODF node granting access to an object on another, using DIDs for stable cross-node identity.
- **Audit log:** Surface all auth resource changes as events in the event sourcing store for compliance and forensics.
- **Transfer ownership:** A dedicated action and policy mechanism for transferring resource ownership between accounts.
- **Bulk bindings:** Allowing `PolicyBinding` objects to target a resource type within an account (e.g. all `Dataset`s of `acme`) so that organization admins could express org-wide grants with `Generic` policies, without resorting to engine-specific policies.


## Appendix A: Cedar - Example Schema and Policies

Full prototype is available [here](https://github.com/kamu-data/tmp-cedar-prototype).

A schema can be code-generated based on types registered in the system allowing policies to be validated against known resource types and attributes:

> **Note:** For brevity, actions here use short names (e.g. `"read"`). A generated schema would use full action schema URIs (e.g. `"https://opendatafabric.org/schemas/datasets/v1alpha1/actions/Read"`) as in the examples above.

```cedar
namespace impl {
    // Base context is implmentation-specific and not part of ODF,
    // but it will be present in all actions as `context.base`.
    // It's a workaround for missing type inheritance in Cedar.
    type Context = {
        ip?: String,
        country_code_a2?: String,
    };
}

namespace odf {
    // Common context type for ODF actions that don't need anything extra
    type Context = {
        base: impl::Context,
    };
}

namespace odf::auth {
    // Abstract type that can own resources
    // User and Organization entities specify this type as parent
    entity ResourceOwner = {
        // Set for organizations to their reserved `{org}/admins` group,
        // whose members get the same rights as the owner
        admins?: odf::auth::Group,
    };

    // Marker for any authenticated principal: User, ServiceAccount, or Node
    // Allows checking `principal in Authenticated::""` instead of creating
    // separate policies for each principal type.
    entity Authenticated;

    // Unauthenticated users are represented as separate type
    // to ensure they are handled in policies explicitly
    entity Anonymous;

    // The node is a principal used for local system tasks and node-to-node communications.
    // When node performs actions on behalf of users and service accounts
    // (e.g. scheduled flow runs) the principal will be the user or service account
    // that task belongs to, i.e. node acts on behalf of the account.
    // The local node may own a few system-wide resources, e.g. the `admin` group.
    entity Node in [ResourceOwner, Authenticated];

    // Authenticated users
    entity User in [ResourceOwner, Group, Authenticated];

    // Organizations can own resources but can't perform actions (not principals)
    entity Organization in [ResourceOwner];

    // Principal for automation that lives under a user or organization
    // used to bind permissions to org-scoped flows and application installations.
    // Can act but does not own any resources.
    entity ServiceAccount in [Group, Authenticated] = {
        owner: odf::auth::ResourceOwner,
    };

    // Groups are a set of users, service accounts or other groups (nesting)
    // used to manage permissions in bulk.
    entity Group in [Group] = {
        owner: odf::auth::ResourceOwner,
    };
}

namespace odf::resources {
    entity TypeUri;

    entity Resource in [TypeUri] = {
        owner: odf::auth::ResourceOwner,

        // All possible auth attributes are accessible via full URIs
        "https://opendatafabric.org/schemas/datasets/v1alpha1/AllowPublicRead"?: Bool,
        "https://opendatafabric.org/schemas/datasets/v1alpha1/AllowAnonymousRead"?: Bool,
    };

    action create, list appliesTo {
        principal: [
            odf::auth::Node,
            odf::auth::User,
            odf::auth::Anonymous,
            odf::auth::ServiceAccount
        ],
        resource: [odf::resources::TypeUri],
        context: odf::resources::ContextCreateList,
    };

    type ContextCreateList = {
        // Owner under which resources are being created / listed
        targetOwner: odf::auth::ResourceOwner,
        base: impl::Context,
    };

    action view, edit, delete, bindPolicy appliesTo {
        principal: [
            odf::auth::Node,
            odf::auth::User,
            odf::auth::ServiceAccount,
            odf::auth::Anonymous
        ],
        resource: [odf::resources::Resource],
        context: odf::Context,
    };

    action addLabel, removeLabel appliesTo {
        principal: [
            odf::auth::Node,
            odf::auth::User,
            odf::auth::ServiceAccount,
            odf::auth::Anonymous
        ],
        resource: [odf::resources::Resource],
        context: odf::resources::ContextAddRemoveLabel,
    };

    type ContextAddRemoveLabel = {
        // Type of the label being added / removed
        labelType: odf::resources::TypeUri,
        // New value of the label when adding or existing value when removing (as stringified json)
        labelValue: String,
        base: impl::Context,
    };
}

namespace odf::datasets {
    // Represents a data plane entity, not the control plane resource
    entity Dataset = {
        owner: odf::auth::ResourceOwner,

        // Relevant attributes are propagated into the data plane
        // as short names allowing for type-checked access
        allowPublicRead?: Bool,
        allowAnonymousRead?: Bool,
    };

    action read, write, alterHistory appliesTo {
        principal: [
            odf::auth::Node,
            odf::auth::User,
            odf::auth::ServiceAccount,
            odf::auth::Anonymous
        ],
        resource: [odf::datasets::Dataset],
        context: odf::Context,
    };
}

namespace odf::config {
    // Represents a data plane entity, not the control plane resource
    entity SecretSet = {
        owner: odf::auth::ResourceOwner,
    };

    action decrypt appliesTo {
        principal: [
            odf::auth::Node,
            odf::auth::User,
            odf::auth::ServiceAccount,
            odf::auth::Anonymous
        ],
        resource: [odf::config::SecretSet],
        context: odf::Context,
    };
}
```

Example policies:

```cedar
// System admins can do anything
permit(
    principal in odf::auth::Group::"node/admins",
    action,
    resource,
);

// The local node can do anything. Implementations drop privileges
// by executing work as a user or service account.
permit(
    principal == odf::auth::Node::"${local_node_id}",
    action,
    resource,
);

// Users can create and list resources under their own account
permit(
    principal is odf::auth::User,
    action in [
        odf::resources::Action::"create",
        odf::resources::Action::"list",
    ],
    resource is odf::resources::TypeUri,
) when {
    principal in context.targetOwner
};

// Users can do anything to resources they own
permit(
    principal is odf::auth::User,
    action,
    resource is odf::resources::Resource
) when {
    principal in resource.owner
};

// Users can do anything to data plane entities they own
permit(
    principal is odf::auth::User,
    action,
    resource is ${entity_type}
) when {
    principal in resource.owner
};

// Org admins can do anything to resources owned by the organization
permit(
    principal is odf::auth::User,
    action,
    resource is odf::resources::Resource
) when {
    resource.owner has admins && principal in resource.owner.admins
};

// Public datasets can be read by any authenticated principal (users and service accounts)
permit(
    principal in odf::auth::Authenticated::"",
    action in [odf::datasets::Action::"read"],
    resource is odf::datasets::Dataset
) when {
    resource has allowPublicRead && resource.allowPublicRead
};

// Anonymous users can read datasets that explicitly allow it
permit(
    principal is odf::auth::Anonymous,
    action in [odf::datasets::Action::"read"],
    resource is odf::datasets::Dataset
) when {
    resource has allowPublicRead && resource.allowPublicRead &&
    resource has allowAnonymousRead && resource.allowAnonymousRead
};
```

`PolicyBinding` resources become template-linked policies, with the `?principal` slot filled based on the `SubjectRef` variant:

| `SubjectRef` | Cedar |
|---|---|
| `Resource` (e.g. `Group:acme/engineering`) | Entity UID, e.g. `odf::auth::Group::"acme/engineering"` |
| `Authenticated` | `odf::auth::Authenticated::""` |
| `Anonymous` | `odf::auth::Anonymous::""` |
| `Everyone` | No slot — compiled into a static policy with an unconstrained `principal` |


## Appendix B: OpenFGA - Example Schema

Full prototype is available [here](https://github.com/kamu-data/tmp-openfga-prototype).

> **Note:** This model predates the `ServiceAccount` and `Node` principals and is kept as an illustration of how the model maps onto a ReBAC engine.

Example schema:

```openfga
model
  schema 1.1

# Singleton representing the kamu node. Carries the node-wide admin group.
# Users and organizations link to the node.
type node
  relations
    define admin: [group#member]

# An authenticated human user.
type user
  relations
    define node: [node]
    define node_admin: admin from node

# An unauthenticated (anonymous) caller. Kept separate from user so that
# user:* wildcards never accidentally grant anonymous access.
type anonymous

# An organization can own resources but cannot act.
type organization
  relations
    define node: [node]
    define node_admin: admin from node

# A set of users or other groups (supports nesting).
type group
  relations
    define direct_member: [user, group]
    define member: direct_member or member from direct_member

# Resource stores exist under every account per every resource type
# (e.g. `alice/dataset`, `bob/dataset`, `alice/secretset`, `bob/secretset`).
# They allow to express permissions for an entire resource type within
# an account or the whole node.
type resource_store
  relations
    define node_store: [resource_store]
    define owner: [user, organization]
    define lister: [user, group#member]
    define creator: [user, group#member]
    define deleter: [user, group#member]
    define viewer: [user, group#member]
    define editor: [user, group#member]
    define label_assignee: [user with label_type_matches, group#member with label_type_matches]

    define can_create: owner or node_admin from owner or creator or can_create from node_store
    define can_delete: owner or node_admin from owner or deleter or can_delete from node_store
    define can_list: owner or node_admin from owner or lister or can_list from node_store
    define can_view: owner or node_admin from owner or viewer or editor or can_view from node_store
    define can_edit: owner or node_admin from owner or editor or can_edit from node_store
    define can_set_label: owner or node_admin from owner or label_assignee or can_set_label from node_store

# A control-plane resource record (metadata / lifecycle object).
# Linked to a resource_store for org-wide viewer/editor grants and node_admin
# traversal. Per-resource viewer/editor tuples are also supported for individual grants.
type resource
  relations
    define resource_store: [resource_store]
    define owner: [user, organization]
    define viewer: [user, group#member]
    define editor: [user, group#member]

    define can_view: owner or viewer or editor or can_view from resource_store
    define can_edit: owner or editor or can_edit from resource_store
    define can_delete: owner or can_delete from resource_store
    define can_set_label: owner or can_set_label from resource_store

# Permission container for datasets, scoped to node, org, or user.
# Stores link to a parent node_store to inherit node-wide grants.
type dataset_store
  relations
    define node_store: [dataset_store]
    define owner: [user, organization]
    define reader: [user, group#member]
    define writer: [user, group#member]
    define maintainer: [user, group#member]

    define can_read: owner or node_admin from owner or reader or writer or maintainer or can_read from node_store
    define can_write: owner or node_admin from owner or writer or maintainer or can_write from node_store
    define can_alter_history: owner or node_admin from owner or maintainer or can_alter_history from node_store

# A data-plane dataset entity. Permissions are independent of the control-plane
# resource record — a reader role here grants data access only.
type dataset
  relations
    define dataset_store: [dataset_store]
    define owner: [user, organization]
    define reader: [user, group#member]
    define writer: [user, group#member]
    define maintainer: [user, group#member]
    define public_reader: [user:*]       # any authenticated user can read
    define anonymous_reader: [anonymous:*]  # unauthenticated callers can read

    define store_can_read: can_read from dataset_store
    define store_can_write: can_write from dataset_store
    define store_can_alter_history: can_alter_history from dataset_store

    define can_read: owner or reader or writer or maintainer or public_reader or anonymous_reader or store_can_read
    define can_write: owner or writer or maintainer or store_can_write
    define can_alter_history: owner or maintainer or store_can_alter_history

# Permission container for secret sets, scoped to node, org, or user.
# Stores link to a parent node_store to inherit node-wide grants.
type secret_set_store
  relations
    define node_store: [secret_set_store]
    define owner: [user, organization]
    define decryptor: [user, group#member]

    define can_decrypt: node_admin from owner or owner or decryptor or can_decrypt from node_store

# A data-plane secret set entity (e.g. encryption keys).
# Linked to a secret_set_store so that store-level grants (node admin,
# org group) propagate without needing per-secret-set tuples.
type secret_set
  relations
    define owner: [user, organization]
    define decryptor: [user, group#member]
    define secret_set_store: [secret_set_store]

    define can_decrypt: owner or decryptor or can_decrypt from secret_set_store

condition label_type_matches(requested: string, allowed: list<string>) {
  requested in allowed
}
```


## Appendix C: Reserved Names

Names that implementations must reserve and treat consistently.

**Accounts**

The following names cannot be used by `User` or `Organization` accounts, to prevent impersonation of the node or its administrators:

| Name | Reason |
|---|---|
| `node` | Conventional name of the local `Node` account |
| `admin`, `admins`, `root`, `system` | Suggest administrative privileges |
| `authenticated`, `anonymous`, `everyone` | Match subject keywords |

Matching is case-insensitive. Implementations may reserve additional names (e.g. their own product name).

**Groups**

| Name | Created with | Description |
|---|---|---|
| `{node}/admins` | `Node` | Node administrators |
| `{org}/admins` | `Organization` | Organization administrators, have owner rights over all resources of the organization |
| `{org}/members` | `Organization` | Organization members |

Reserved groups are created together with the account that owns them and cannot be created or deleted independently.

**Subject keywords**

| Keyword | Description |
|---|---|
| `Authenticated` | `SubjectRef` matching any authenticated principal |
| `Anonymous` | `SubjectRef` matching any unauthenticated principal |
| `Everyone` | `SubjectRef` matching any principal |

Subject keywords cannot collide with resource names, as resource references in short form always include a type prefix, a DID, or an ID.
