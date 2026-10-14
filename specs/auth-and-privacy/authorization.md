# Authorization <!-- omit in toc -->

**Domain**: Auth & Privacy
**Depends on**: [Resource Framework](../automation/resource-framework.md)
**RFCs**: [RFC-020](../../rfcs/020-auth.md)


## Abstract <!-- omit in toc -->

This spec defines the authorization model of ODF nodes - how accounts, groups, auth attributes, actions, and policies are expressed as resources of the [Resource Framework](../automation/resource-framework.md). The model is engine-agnostic: a minimal node can implement simple ownership checks, while a full node can plug in an RBAC / ABAC / ReBAC policy engine, with both remaining interoperable at the level of resources.


## Table of Contents <!-- omit in toc -->

- [Terminology](#terminology)
- [Maturity Levels](#maturity-levels)
- [Accounts](#accounts)
  - [Node](#node)
  - [User](#user)
  - [Organization](#organization)
  - [Service Account](#service-account)
  - [Account Names](#account-names)
- [Ownership](#ownership)
- [Principals](#principals)
- [Subjects](#subjects)
- [Groups](#groups)
- [Auth Attributes](#auth-attributes)
  - [Standard Auth Attributes](#standard-auth-attributes)
- [Actions](#actions)
  - [Resource Actions](#resource-actions)
  - [Dataset Actions](#dataset-actions)
  - [Config Actions](#config-actions)
- [Policies and Policy Bindings](#policies-and-policy-bindings)
  - [Generic Policies](#generic-policies)
  - [Engine-specific Policies](#engine-specific-policies)
- [Access Token Scopes](#access-token-scopes)
- [Reserved Names](#reserved-names)
- [Schemas](#schemas)
- [Appendix A: Cedar Example](#appendix-a-cedar-example)
- [Appendix B: OpenFGA Example](#appendix-b-openfga-example)


## Terminology
This spec builds on the [Node](../glossary.md#node), [Resource](../glossary.md#resource), [Entity](../glossary.md#entity), [Account](../glossary.md#account), and [Principal](../glossary.md#principal) concepts defined in the glossary, and introduces the following terms:
- **Subject** - the party a grant is given to in a policy binding: a principal, a group, or a well-known set of principals
- **Group** - a named, possibly nested set of subjects that policies can be bound to, so that permissions are granted to all members at once
- **Action** - an operation the principal attempts to perform, identified by a schema URI (e.g. `View`, `Read`, `Decrypt`)
- **Object** - the target of an action: a resource, an entity, or a resource type (for actions like `Create` and `List` where no instance exists yet)


## Maturity Levels
Implementations may support authorization at one of the following maturity levels:

| Level | Description |
| --- | --- |
| **0 - No Auth** | All resources and actions are available to the caller |
| **1 - Personal Ownership** | Support for `User` accounts and access based on [ownership](#ownership) only |
| **2 - Policies** | Support for `Generic` [policies](#policies-and-policy-bindings), per-account `PolicyBinding`, and [auth attributes](#auth-attributes) |
| **3 - Groups and Organizations** | Support for [`Group`](#groups) membership, binding policies to groups, [`Organization`](#organization) and [`ServiceAccount`](#service-account) accounts |


## Accounts

### Node
An ODF node. Acts as a principal in system tasks and node-to-node communications, and may own a few node-wide resources (e.g. the `node/admins` group).

```yaml
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/Node
headers:
  name: node
spec:
  did: did:odf:fed0..bc
  displayName: Current Node
```


### User
An individual person who can authenticate and act as a principal.

```yaml
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/User
headers:
  name: alice
spec:
  email: alice@example.com
  displayName: Alice
```

A user may also specify a `did` associated with the account by ODF or an external system, and a `password` for local authentication, which is stored as a hash upon apply and is absent for SSO or DID-based accounts. A user can be suspended by setting `disabled: true` - the effects of suspension are implementation-specific.


### Organization
An account that owns resources on behalf of a group of users, but cannot authenticate or act itself, so its resources are always accessed through its members.

```yaml
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/Organization
headers:
  name: acme
spec:
  displayName: Acme Corp
```

Upon creation of an organization, the node automatically creates two groups under its account:
- `Group:acme/admins` - organization administrators, initially containing the principal that created the organization. Members of this group have the same rights over resources owned by the organization as an owner would.
- `Group:acme/members` - organization members

These are ordinary [groups](#groups) that can be edited to customize membership (e.g. by nesting team groups). Their names are [reserved](#reserved-names), so they can't be created or deleted independently of the organization. Whether admins are implicitly considered members is left to implementations.


### Service Account
A non-human identity used by automation (scheduled flows, CI, installed applications) that can act as a principal but doesn't own resources. Service accounts are always scoped under the owning account.

```yaml
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/ServiceAccount
headers:
  account: acme
  name: ingest-prod
spec:
  displayName: Ingest (prod)
```


### Account Names
To avoid ambiguity and impersonation, nodes must ensure that names of nodes, users, and organizations are unique regardless of their resource type, making it possible to refer to any of these accounts by name alone. Service account names are exempt from this rule, as they are always scoped under the owning account.

Some names cannot be used by accounts - see [Reserved Names](#reserved-names).


## Ownership
Every resource is owned by the account specified in its `headers.account` (see [Resource Framework: Ownership](../automation/resource-framework.md#ownership)). Only `User`, `Organization`, and `Node` accounts can own resources, and are referenced using `ResourceOwnerRef`, whose short form is the account name.

Owners hold all actions on the resources they own and on the entities materialized from them. For resources owned by an organization, the same rights are held by members of its `{org}/admins` group.


## Principals
A principal is the identity performing an action in an authorization request. It is established through authentication and is always a single concrete identity.

| Type | Can act as principal | Can own resources |
| --- | --- | --- |
| `User` | ✅ | ✅ |
| `ServiceAccount` | ✅ | ❌ |
| `Node` | ✅ | ✅ |
| `Organization` | ❌ | ✅ |
| `Anonymous` | ✅ | ❌ |

- `Organization` cannot authenticate - its resources are always accessed through its members.
- `Anonymous` represents unauthenticated requests. It is not a resource and carries no identity.
- When a node executes work that belongs to a user or service account (e.g. a scheduled flow run) it should do so as that principal rather than as itself.

Principals are referenced using `PrincipalRef`, e.g. to specify the service account a flow runs as, or to record the initiator of an operation. Its short form requires a type prefix:

```yaml
serviceAccount: ServiceAccount:acme/ingest-prod  # PrincipalRef
```


## Subjects
A subject is the party a grant is given to in a `PolicyBinding`. Unlike a principal, a subject may stand for many principals at once.

Subjects are referenced using `SubjectRef`, which is a union of:

| Kind | Matches | Short form |
| --- | --- | --- |
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

A `Resource` subject can alternatively be identified by `id` or `did`.

`Authenticated`, `Anonymous`, and `Everyone` can be used in policy bindings, but not as group members - otherwise nesting such a subject into a group would extend the group's permissions to all principals.


## Groups
A `Group` resource represents a named set of subjects. Membership is declared inline in the spec. Groups support nesting - a group may include other groups as members. Only the `Resource` subject kind is allowed as a member.

```yaml
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/Group
headers:
  account: acme
  name: engineering
spec:
  members:
    - User:alice  # SubjectRef
    - User:bob
    - Group:acme/engineering-contractors  # nested group
```

Groups owned by the `Node` account, like `Group:node/admins`, can be used to represent node-wide predefined roles.


## Auth Attributes
Properties of a resource that directly affect access control (e.g. whether a dataset allows public reads) are expressed using resource labels (see [Resource Framework: Labels and Annotations](../automation/resource-framework.md#labels-and-annotations)).

A label schema that declares `labelProperties.isAuthAttribute: true` signals that any resource carrying that label should have its value materialized into an auth attribute:

```json
{
  "$id": "https://opendatafabric.org/schemas/datasets/v1alpha1/AllowPublicRead",
  "$schema": "https://opendatafabric.org/schemas/resources/v1alpha1/meta/ResourceLabel",
  "description": "Controls whether the dataset is readable by any authenticated principal.",
  "type": "boolean",
  "labelProperties": {
    "isAuthAttribute": true,
    "resourceTypes": [
      "https://opendatafabric.org/schemas/datasets/v1alpha1/Dataset"
    ]
  }
}
```

Auth attributes are set like any other label:

```yaml
$schema: https://opendatafabric.org/schemas/datasets/v1alpha1/Dataset
headers:
  account: bob
  name: bobs-dataset
  labels:
    # Short form - resolved to https://opendatafabric.org/schemas/datasets/v1alpha1/AllowPublicRead
    AllowPublicRead: true
    # Full URI form
    https://opendatafabric.org/schemas/datasets/v1alpha1/AllowAnonymousRead: false
spec:
  kind: Root
  metadata: []
```

Rules:
- Auth attributes must be labels, not annotations - validators must reject a resource that places an `isAuthAttribute` label under `annotations`
- The value is validated against the label schema's `type` at apply time
- Implementations discover the set of auth attributes by enumerating label schemas with `isAuthAttribute: true`, rather than hardcoding a list
- Changing an auth attribute may require elevated permissions beyond those needed to update the rest of the resource (see [`AddLabel`](#resource-actions))
- Auth attributes are propagated from resources to the entities they define, e.g. `AllowPublicRead` on a dataset resource (control plane) also appears as an attribute of the dataset entity (data plane)

Since auth attributes are labels, they are also indexed and queryable in the resource listing API.


### Standard Auth Attributes

| Label | Type | Applies to | Description |
| --- | --- | --- | --- |
| `datasets/v1alpha1/AllowPublicRead` | `boolean` | `Dataset` | Dataset is readable by any authenticated principal |
| `datasets/v1alpha1/AllowAnonymousRead` | `boolean` | `Dataset` | Dataset is readable without authentication |

The effects of these attributes can also be achieved by binding policies to `Authenticated` or `Anonymous` [subjects](#subjects). Attributes, however, are the primary way to make datasets public, as they are atomic with the resource manifest and can be efficiently filtered on when listing resources. Bindings to these subjects are better suited for granting other actions publicly, e.g. making a resource manifest viewable by everyone.


## Actions
Authorization decisions are made against a set of well-defined actions. Each action is defined by a JSON schema using the [`Action`](../../schemas/resources/v1alpha1/meta/Action.json) metaschema:
- The schema URI is the canonical action identifier used in authorization requests and policies, e.g. `https://opendatafabric.org/schemas/resources/v1alpha1/actions/View`
- The schema describes the context values needed to authorize the action (e.g. the label being added for `AddLabel`)
- `actionProperties.objectTypes` lists the types of objects the action applies to. Since resource types are an open set, every action must list its object types explicitly. Generic resource actions use the `Resource` type to apply to resources of any type.


### Resource Actions
Operate on resource instances. Exceptions are `Create` and `List`, which operate on a resource type and receive the account under which resources are created or listed in the `targetOwner` context value.

| Action | Object | Context | Description |
| --- | --- | --- | --- |
| [`resources/v1alpha1/actions/Create`](../../schemas/resources/v1alpha1/actions/Create.json) | Resource type | `targetOwner` | Create a new resource instance under an account |
| [`resources/v1alpha1/actions/List`](../../schemas/resources/v1alpha1/actions/List.json) | Resource type | `targetOwner` | List resource instances of a type under an account |
| [`resources/v1alpha1/actions/View`](../../schemas/resources/v1alpha1/actions/View.json) | `Resource` | | Read the resource manifest |
| [`resources/v1alpha1/actions/Edit`](../../schemas/resources/v1alpha1/actions/Edit.json) | `Resource` | | Modify the resource manifest |
| [`resources/v1alpha1/actions/Delete`](../../schemas/resources/v1alpha1/actions/Delete.json) | `Resource` | | Delete the resource instance |
| [`resources/v1alpha1/actions/AddLabel`](../../schemas/resources/v1alpha1/actions/AddLabel.json) | `Resource` | `labelType`, `labelValue` | Apply a specific label to the resource |
| [`resources/v1alpha1/actions/RemoveLabel`](../../schemas/resources/v1alpha1/actions/RemoveLabel.json) | `Resource` | `labelType`, `labelValue` | Remove a specific label from the resource |
| [`auth/v1alpha1/actions/BindPolicy`](../../schemas/auth/v1alpha1/actions/BindPolicy.json) | `Resource` | `subject` | Create `PolicyBinding` resources that reference this resource as `object` |

`AddLabel` and `RemoveLabel` operate at the sub-resource level and are distinct from `Edit` to allow fine-grained control - a principal may be allowed to edit a resource, but be restricted from changing auth-sensitive labels like `AllowPublicRead`. Their context carries the type and the new (when adding) or existing (when removing) value of the label.

`BindPolicy` receives the `subject` of the binding being created in its context, so policies can restrict who is allowed to grant access to e.g. `Everyone`.


### Dataset Actions
Operate on dataset entities in the data plane, separately from dataset resources.

| Action | Object | Description |
| --- | --- | --- |
| [`datasets/v1alpha1/actions/Read`](../../schemas/datasets/v1alpha1/actions/Read.json) | `Dataset` | Read dataset data (query, download) |
| [`datasets/v1alpha1/actions/Write`](../../schemas/datasets/v1alpha1/actions/Write.json) | `Dataset` | Append new data to the dataset |
| [`datasets/v1alpha1/actions/AlterHistory`](../../schemas/datasets/v1alpha1/actions/AlterHistory.json) | `Dataset` | Rewrite or truncate dataset history |


### Config Actions
Operate on configuration entities.

| Action | Object | Description |
| --- | --- | --- |
| [`config/v1alpha1/actions/Decrypt`](../../schemas/config/v1alpha1/actions/Decrypt.json) | `SecretSet` | Decrypt a secret from a `SecretSet` |


## Policies and Policy Bindings
Beyond [ownership](#ownership), `Policy` and `PolicyBinding` resources are the primary mechanism for granting permissions:
- A `Policy` is a reusable, potentially parameterized grant template. It declares what actions are permitted, but may leave the subject and object as slots to be filled in later.
- A `PolicyBinding` instantiates a `Policy` by supplying the concrete `subject` ([`SubjectRef`](#subjects)) and `object` (`ResourceRef`). The `object` refers to both the resource and the entity materialized from it.

Creating a `PolicyBinding` is itself an authorized operation - the principal creating it must hold the `BindPolicy` action on the referenced `object`. Resource owners implicitly hold `BindPolicy` on their own resources and can grant it to others, enabling delegated access management without transferring ownership.

A `Policy` resource's `kind` field selects the policy language. The `Generic` kind must be supported by all nodes at maturity level 2 and above. Other kinds are engine-specific.

Higher-level features like dataset roles (`Reader`, `Editor`, `Maintainer`) are not part of this spec - implementations can build them using groups, `Generic` policy templates, and `PolicyBinding` resources.


### Generic Policies
A `Generic` policy grants a set of ODF actions. It is the portable building block - any node at maturity level 2 and above can evaluate it without a specialized policy engine. Its subject and object are always supplied by a `PolicyBinding`.

```yaml
# A reusable "dataset reader" policy template
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/Policy
headers:
  account: node
  name: dataset-reader
spec:
  kind: Generic
  actions:
    - https://opendatafabric.org/schemas/resources/v1alpha1/actions/View
    - https://opendatafabric.org/schemas/datasets/v1alpha1/actions/Read
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

The binding above grants `View` on the dataset resource and `Read` on the dataset entity.


### Engine-specific Policies
Nodes with a full policy engine may support richer `Policy` kinds that can express attribute-based conditions, organization-scoped grants, and custom logic that `Generic` cannot. Engine-specific policies are not portable between implementations.

Engine-specific policies may declare slots for the subject and object, in which case they are templates instantiated via `PolicyBinding`. Policies that don't declare slots take effect on their own without a binding (e.g. a node-wide policy granting all actions to members of `Group:node/admins`).

The `Cedar` kind carries a policy in the [Cedar](https://cedarpolicy.com/) language, where `?principal` and `?resource` slots are filled in by the `subject` and `object` of a binding:

```yaml
$schema: https://opendatafabric.org/schemas/auth/v1alpha1/Policy
headers:
  account: node
  name: dataset-reader-acme
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

Nodes that do not support a given `Policy` kind must reject the resource with a clear error rather than silently ignoring it.


## Access Token Scopes
Access tokens may be restricted to a set of scopes (e.g. read-only access to specific datasets). Scopes are not part of the authorization model: they can only narrow what the principal is allowed to do, so implementations check whether an action on an object falls within the token's scopes before policy evaluation. Policies then decide based on the principal's own permissions, so a token can never grant more than its owner has.


## Reserved Names
Names that implementations must reserve and treat consistently.

**Accounts** - the following names cannot be used by `User` or `Organization` accounts, to prevent impersonation of the node or its administrators:

| Name | Reason |
| --- | --- |
| `node` | Conventional name of the local `Node` account |
| `admin`, `admins`, `root`, `system` | Suggest administrative privileges |
| `authenticated`, `anonymous`, `everyone` | Match subject keywords |

Matching is case-insensitive. Implementations may reserve additional names (e.g. their own product name).

**Groups** - reserved groups are created together with the account that owns them and cannot be created or deleted independently:

| Name | Created with | Description |
| --- | --- | --- |
| `{node}/admins` | `Node` | Node administrators |
| `{org}/admins` | `Organization` | Organization administrators, have owner rights over all resources of the organization |
| `{org}/members` | `Organization` | Organization members |

**Subject keywords** - cannot collide with resource names, as resource references in short form always include a type prefix, a DID, or an ID:

| Keyword | Description |
| --- | --- |
| `Authenticated` | `SubjectRef` matching any authenticated principal |
| `Anonymous` | `SubjectRef` matching any unauthenticated principal |
| `Everyone` | `SubjectRef` matching any principal |


## Schemas

| Schema | Description |
| --- | --- |
| [`Node`](../../schemas/auth/v1alpha1/Node.json) | Node account |
| [`User`](../../schemas/auth/v1alpha1/User.json) | User account |
| [`Organization`](../../schemas/auth/v1alpha1/Organization.json) | Organization account |
| [`ServiceAccount`](../../schemas/auth/v1alpha1/ServiceAccount.json) | Service account |
| [`Group`](../../schemas/auth/v1alpha1/Group.json) | Group of subjects |
| [`Policy`](../../schemas/auth/v1alpha1/Policy.json) | Policy template |
| [`PolicyBinding`](../../schemas/auth/v1alpha1/PolicyBinding.json) | Policy instantiation for a subject and object |
| [`PrincipalRef`](../../schemas/auth/v1alpha1/PrincipalRef.json) | Reference to a principal |
| [`SubjectRef`](../../schemas/auth/v1alpha1/SubjectRef.json) | Reference to a subject |
| [`ResourceOwnerRef`](../../schemas/auth/v1alpha1/ResourceOwnerRef.json) | Reference to an account that can own resources |
| [`Action`](../../schemas/resources/v1alpha1/meta/Action.json) | Metaschema for action schemas |
| [`ResourceLabel`](../../schemas/resources/v1alpha1/meta/ResourceLabel.json) | Metaschema for label schemas, including auth attributes |


## Appendix A: Cedar Example
> **Non-normative:** This appendix illustrates how the authorization model maps onto the [Cedar](https://cedarpolicy.com/) policy engine.


Full prototype is available [here](https://github.com/kamu-data/tmp-cedar-prototype).

A schema can be code-generated based on types registered in the system allowing policies to be validated against known resource types and attributes:

> **Note:** For brevity, actions here use short names (e.g. `"read"`). A generated schema would use full action schema URIs (e.g. `"https://opendatafabric.org/schemas/datasets/v1alpha1/actions/Read"`) as in the examples above.

```cedar
namespace impl {
    // Base context is implementation-specific and not part of ODF,
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


## Appendix B: OpenFGA Example
> **Non-normative:** This appendix illustrates how the authorization model maps onto the [OpenFGA](https://openfga.dev/) ReBAC engine.


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
