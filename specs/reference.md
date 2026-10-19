# Reference <!-- omit in toc -->

<!-- THIS FILE IS AUTO-GENERATED FROM `schemas/` - DO NOT MODIFY MANUALLY -->

Reference of all schemas that ODF specs refer to, grouped by context. Schemas are defined in the [`schemas`](../schemas) directory.


<a name="table-of-contents"></a>
## Table of Contents <!-- omit in toc -->

- [auth](#reference-auth)
  - [BindPolicy](#bindpolicy-schema)
  - [Group](#group-schema)
  - [GroupSpec](#groupspec-schema)
  - [GroupSpecInput](#groupspecinput-schema)
  - [Node](#node-schema)
  - [NodeSpec](#nodespec-schema)
  - [NodeSpecInput](#nodespecinput-schema)
  - [Organization](#organization-schema)
  - [OrganizationSpec](#organizationspec-schema)
  - [OrganizationSpecInput](#organizationspecinput-schema)
  - [Policy](#policy-schema)
  - [PolicyBinding](#policybinding-schema)
  - [PolicyBindingSpec](#policybindingspec-schema)
  - [PolicyBindingSpecInput](#policybindingspecinput-schema)
  - [PolicyRef](#policyref-schema)
  - [PolicySpec](#policyspec-schema)
  - [PolicySpecInput](#policyspecinput-schema)
  - [PrincipalHandle](#principalhandle-schema)
  - [PrincipalRef](#principalref-schema)
  - [ResourceOwnerHandle](#resourceownerhandle-schema)
  - [ResourceOwnerRef](#resourceownerref-schema)
  - [ServiceAccount](#serviceaccount-schema)
  - [ServiceAccountSpec](#serviceaccountspec-schema)
  - [ServiceAccountSpecInput](#serviceaccountspecinput-schema)
  - [SubjectHandle](#subjecthandle-schema)
  - [SubjectRef](#subjectref-schema)
  - [User](#user-schema)
  - [UserSpec](#userspec-schema)
  - [UserSpecInput](#userspecinput-schema)
- [config](#reference-config)
  - [Decrypt](#decrypt-schema)
  - [Secret](#secret-schema)
  - [SecretSet](#secretset-schema)
  - [SecretSetSpec](#secretsetspec-schema)
  - [SecretSetSpecInput](#secretsetspecinput-schema)
  - [Secrets](#secrets-schema)
  - [ValueHandle](#valuehandle-schema)
  - [ValueRef](#valueref-schema)
  - [ValueRefs](#valuerefs-schema)
  - [Variable](#variable-schema)
  - [VariableSet](#variableset-schema)
  - [VariableSetSpec](#variablesetspec-schema)
  - [VariableSetSpecInput](#variablesetspecinput-schema)
  - [Variables](#variables-schema)
- [data](#reference-data)
  - [DataField](#datafield-schema)
  - [DataSchema](#dataschema-schema)
  - [DataType](#datatype-schema)
  - [ExtraAttributes](#extraattributes-schema)
  - [OperationType](#operationtype-schema)
  - [TimeUnit](#timeunit-schema)
- [datasets](#reference-datasets)
  - [AddData](#adddata-schema)
  - [AllowAnonymousRead](#allowanonymousread-schema)
  - [AllowPublicRead](#allowpublicread-schema)
  - [AlterHistory](#alterhistory-schema)
  - [AttachmentEmbedded](#attachmentembedded-schema)
  - [Attachments](#attachments-schema)
  - [Checkpoint](#checkpoint-schema)
  - [DataSlice](#dataslice-schema)
  - [Dataset](#dataset-schema)
  - [DatasetHandle](#datasethandle-schema)
  - [DatasetKind](#datasetkind-schema)
  - [DatasetRef](#datasetref-schema)
  - [DatasetSelector](#datasetselector-schema)
  - [DatasetSpec](#datasetspec-schema)
  - [DatasetSpecInput](#datasetspecinput-schema)
  - [DatasetVocabulary](#datasetvocabulary-schema)
  - [ExecuteTransform](#executetransform-schema)
  - [ExecuteTransformInput](#executetransforminput-schema)
  - [MetadataBlock](#metadatablock-schema)
  - [MetadataEvent](#metadataevent-schema)
  - [OffsetInterval](#offsetinterval-schema)
  - [Projection](#projection-schema)
  - [ProjectionSpec](#projectionspec-schema)
  - [ProjectionSpecInput](#projectionspecinput-schema)
  - [Read](#read-schema)
  - [Seed](#seed-schema)
  - [SetAttachments](#setattachments-schema)
  - [SetDataSchema](#setdataschema-schema)
  - [SetInfo](#setinfo-schema)
  - [SetLicense](#setlicense-schema)
  - [SetTransform](#settransform-schema)
  - [SetVocab](#setvocab-schema)
  - [SqlQueryStep](#sqlquerystep-schema)
  - [TemporalTable](#temporaltable-schema)
  - [Transform](#transform-schema)
  - [TransformInput](#transforminput-schema)
  - [Watermark](#watermark-schema)
  - [Write](#write-schema)
- [engines](#reference-engines)
  - [RawQueryRequest](#rawqueryrequest-schema)
  - [RawQueryResponse](#rawqueryresponse-schema)
  - [TransformRequest](#transformrequest-schema)
  - [TransformRequestInput](#transformrequestinput-schema)
  - [TransformResponse](#transformresponse-schema)
- [events](#reference-events)
  - [EventFilter](#eventfilter-schema)
- [flows](#reference-flows)
  - [Flow](#flow-schema)
  - [FlowRun](#flowrun-schema)
  - [FlowRunActivationCause](#flowrunactivationcause-schema)
  - [FlowRunActivationCauses](#flowrunactivationcauses-schema)
  - [FlowRunRetry](#flowrunretry-schema)
  - [FlowRunSpec](#flowrunspec-schema)
  - [FlowRunSpecInput](#flowrunspecinput-schema)
  - [FlowRunStatus](#flowrunstatus-schema)
  - [FlowSpec](#flowspec-schema)
  - [FlowSpecInput](#flowspecinput-schema)
  - [FlowStatus](#flowstatus-schema)
  - [FlowTrigger](#flowtrigger-schema)
  - [FlowTriggerInput](#flowtriggerinput-schema)
  - [RecentBindingsRetention](#recentbindingsretention-schema)
  - [RecentRunsRetention](#recentrunsretention-schema)
  - [RetryBackoff](#retrybackoff-schema)
  - [RetryPolicy](#retrypolicy-schema)
- [legacy](#reference-legacy)
  - [AddPushSource](#addpushsource-schema)
  - [DatasetSnapshot](#datasetsnapshot-schema)
  - [DisablePollingSource](#disablepollingsource-schema)
  - [DisablePushSource](#disablepushsource-schema)
  - [FetchStep](#fetchstep-schema)
  - [Manifest](#manifest-schema)
  - [SetPollingSource](#setpollingsource-schema)
- [resources](#reference-resources)
  - [AddLabel](#addlabel-schema)
  - [Create](#create-schema)
  - [Delete](#delete-schema)
  - [Edit](#edit-schema)
  - [LabelFilter](#labelfilter-schema)
  - [List](#list-schema)
  - [ReconciliationError](#reconciliationerror-schema)
  - [RemoveLabel](#removelabel-schema)
  - [Resource](#resource-schema)
  - [ResourceAnnotations](#resourceannotations-schema)
  - [ResourceConditions](#resourceconditions-schema)
  - [ResourceHandle](#resourcehandle-schema)
  - [ResourceHeaders](#resourceheaders-schema)
  - [ResourceHeadersInput](#resourceheadersinput-schema)
  - [ResourceInput](#resourceinput-schema)
  - [ResourceLabels](#resourcelabels-schema)
  - [ResourcePhase](#resourcephase-schema)
  - [ResourceRef](#resourceref-schema)
  - [ResourceSelector](#resourceselector-schema)
  - [ResourceStatus](#resourcestatus-schema)
  - [View](#view-schema)
- [sinks](#reference-sinks)
  - [WebhookEndpoint](#webhookendpoint-schema)
  - [WebhookEndpointSpec](#webhookendpointspec-schema)
  - [WebhookEndpointSpecInput](#webhookendpointspecinput-schema)
- [sources](#reference-sources)
  - [CompressionFormat](#compressionformat-schema)
  - [EnvVar](#envvar-schema)
  - [EventTimeSource](#eventtimesource-schema)
  - [Ingress](#ingress-schema)
  - [IngressBuffer](#ingressbuffer-schema)
  - [MergeStrategy](#mergestrategy-schema)
  - [MqttQos](#mqttqos-schema)
  - [MqttTopicSubscription](#mqtttopicsubscription-schema)
  - [PrepStep](#prepstep-schema)
  - [ReadStep](#readstep-schema)
  - [RequestHeader](#requestheader-schema)
  - [Source](#source-schema)
  - [SourceCaching](#sourcecaching-schema)
  - [SourceOrdering](#sourceordering-schema)
  - [SourceSpec](#sourcespec-schema)
  - [SourceSpecInput](#sourcespecinput-schema)
  - [SourceState](#sourcestate-schema)
- [storage](#reference-storage)
  - [AwsCredentials](#awscredentials-schema)
  - [AwsCredentialsInput](#awscredentialsinput-schema)
  - [PersistentVolume](#persistentvolume-schema)
  - [PersistentVolumeRef](#persistentvolumeref-schema)
  - [PersistentVolumeSpec](#persistentvolumespec-schema)
  - [PersistentVolumeSpecInput](#persistentvolumespecinput-schema)
  - [VolumeCapacity](#volumecapacity-schema)
- [tasks](#reference-tasks)
  - [FailureBehavior](#failurebehavior-schema)
  - [NoOpBehavior](#noopbehavior-schema)
  - [SyncAuth](#syncauth-schema)
  - [SyncAuthHeader](#syncauthheader-schema)
  - [SyncAuthHeaderInput](#syncauthheaderinput-schema)
  - [SyncAuthInput](#syncauthinput-schema)
  - [SyncEndpoint](#syncendpoint-schema)
  - [SyncEndpointInput](#syncendpointinput-schema)
  - [Task](#task-schema)
  - [TaskError](#taskerror-schema)
  - [TaskOutcome](#taskoutcome-schema)
  - [TaskPlan](#taskplan-schema)
  - [TaskSpec](#taskspec-schema)
  - [TaskSpecInput](#taskspecinput-schema)
  - [TaskStatus](#taskstatus-schema)


<a name="reference-auth"></a>
## auth

<a name="bindpolicy-schema"></a>
### BindPolicy
Create `PolicyBinding` resources that reference this resource as `object`.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `subject` | [SubjectHandle](#subjecthandle-schema) | V |  | Subject of the binding being created, allowing policies to restrict granting access to e.g. `Everyone`. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/actions/BindPolicy.json)
[^](#table-of-contents)


<a name="group-schema"></a>
### Group
A named, possibly nested set of principals. Policies can be bound to groups so that permissions are granted to all members at once. Some group names are reserved, e.g. `{org}/admins` and `{org}/members` are created together with an organization.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `$schema` | `string` | V |  | Identifies this resource type. |
| `headers` | [ResourceHeadersInput](#resourceheadersinput-schema) | V |  | Container for identity and ownership information of a resource. |
| `spec` | [GroupSpecInput](#groupspecinput-schema) | V |  | Specifies the desired state of the resource. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/Group.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="groupspec-schema"></a>
### GroupSpec
Group specification.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `members` | array([SubjectHandle](#subjecthandle-schema)) |  |  | Members of the group. Only the `Resource` subject kind is allowed, so that well-known subjects like `Everyone` cannot be nested into a group. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/GroupSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="groupspecinput-schema"></a>
### GroupSpecInput
Group specification.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `members` | array([SubjectRef](#subjectref-schema)) |  |  | Members of the group. Only the `Resource` subject kind is allowed, so that well-known subjects like `Everyone` cannot be nested into a group. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/GroupSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="node-schema"></a>
### Node
An entire ODF deployment acting under a single identity. The node acts as a principal in system tasks and node-to-node communications and owns node-wide resources like the reserved `{node}/admins` group.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `$schema` | `string` | V |  | Identifies this resource type. |
| `headers` | [ResourceHeadersInput](#resourceheadersinput-schema) | V |  | Container for identity and ownership information of a resource. |
| `spec` | [NodeSpecInput](#nodespecinput-schema) | V |  | Specifies the desired state of the resource. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/Node.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="nodespec-schema"></a>
### NodeSpec
Node account specification.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `did` | `string` |  |  | DID associated with the account by ODF or an external system. |
| `displayName` | `string` |  |  | Human-friendly display name. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/NodeSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="nodespecinput-schema"></a>
### NodeSpecInput
Node account specification.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `did` | `string` |  |  | DID associated with the account by ODF or an external system. |
| `displayName` | `string` |  |  | Human-friendly display name. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/NodeSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="organization-schema"></a>
### Organization
An account that owns resources on behalf of a group of users but cannot authenticate or act itself. Upon creation the node creates reserved groups `{org}/admins`, whose members have owner rights over all resources of the organization, and `{org}/members`.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `$schema` | `string` | V |  | Identifies this resource type. |
| `headers` | [ResourceHeadersInput](#resourceheadersinput-schema) | V |  | Container for identity and ownership information of a resource. |
| `spec` | [OrganizationSpecInput](#organizationspecinput-schema) | V |  | Specifies the desired state of the resource. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/Organization.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="organizationspec-schema"></a>
### OrganizationSpec
Organization account specification.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `displayName` | `string` |  |  | Human-friendly display name. |
| `avatarUrl` | `string` |  | `url` | URL of the account's avatar image. |
| `disabled` | `boolean` |  |  | Suspends the account. Effects of suspension are implementation-specific.<br/><br/>Default: false |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/OrganizationSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="organizationspecinput-schema"></a>
### OrganizationSpecInput
Organization account specification.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `displayName` | `string` |  |  | Human-friendly display name. |
| `avatarUrl` | `string` |  | `url` | URL of the account's avatar image. |
| `disabled` | `boolean` |  |  | Suspends the account. Effects of suspension are implementation-specific.<br/><br/>Default: false |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/OrganizationSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="policy-schema"></a>
### Policy
A reusable grant of permissions. Policies with subject and object slots are templates instantiated via `PolicyBinding`, while policies without slots take effect on their own.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `$schema` | `string` | V |  | Identifies this resource type. |
| `headers` | [ResourceHeadersInput](#resourceheadersinput-schema) | V |  | Container for identity and ownership information of a resource. |
| `spec` | [PolicySpecInput](#policyspecinput-schema) | V |  | Specifies the desired state of the resource. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/Policy.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="policybinding-schema"></a>
### PolicyBinding
Instantiates a `Policy` template for a concrete subject and object.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `$schema` | `string` | V |  | Identifies this resource type. |
| `headers` | [ResourceHeadersInput](#resourceheadersinput-schema) | V |  | Container for identity and ownership information of a resource. |
| `spec` | [PolicyBindingSpecInput](#policybindingspecinput-schema) | V |  | Specifies the desired state of the resource. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/PolicyBinding.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="policybindingspec-schema"></a>
### PolicyBindingSpec
Policy binding specification.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `policy` | [ResourceHandle](#resourcehandle-schema) | V |  | Policy template to instantiate. |
| `subject` | [SubjectHandle](#subjecthandle-schema) | V |  | Party the permissions are granted to. |
| `object` | [ResourceHandle](#resourcehandle-schema) | V |  | Resource the permissions are granted on. Refers to both the resource and the entity materialized from it. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/PolicyBindingSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="policybindingspecinput-schema"></a>
### PolicyBindingSpecInput
Policy binding specification.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `policy` | [PolicyRef](#policyref-schema) | V |  | Policy template to instantiate. |
| `subject` | [SubjectRef](#subjectref-schema) | V |  | Party the permissions are granted to. |
| `object` | [ResourceRef](#resourceref-schema) | V |  | Resource the permissions are granted on. Refers to both the resource and the entity materialized from it. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/PolicyBindingSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="policyref-schema"></a>
### PolicyRef
Reference to a `Policy`.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `account` | [ResourceOwnerRef](#resourceownerref-schema) |  |  | Reference to an account that owns the `Policy`. |
| `id` | `string` |  |  | ID of the resource. |
| `name` | `string` |  |  | Name of the resource. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/PolicyRef.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="policyspec-schema"></a>
### PolicySpec
Policy specification. The `kind` selects the policy language.

| Union Type | Description |
| :---: | --- |
| [PolicySpec::Generic](#policyspec-generic-schema) | Engine-agnostic policy that grants a set of actions. Subject and object are always supplied by a `PolicyBinding`. |
| [PolicySpec::Cedar](#policyspec-cedar-schema) | Policy expressed in the Cedar policy language. Policies that declare `?principal` and `?resource` slots are templates instantiated via `PolicyBinding`, otherwise they take effect on their own. Only supported by nodes that use Cedar. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/PolicySpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="policyspec-cedar-schema"></a>
### PolicySpec::Cedar
Policy expressed in the Cedar policy language. Policies that declare `?principal` and `?resource` slots are templates instantiated via `PolicyBinding`, otherwise they take effect on their own. Only supported by nodes that use Cedar.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `body` | `string` | V |  | Cedar policy source. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/PolicySpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="policyspec-generic-schema"></a>
### PolicySpec::Generic
Engine-agnostic policy that grants a set of actions. Subject and object are always supplied by a `PolicyBinding`.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `actions` | array(`string`) | V |  | Schema URIs of actions granted by the policy. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/PolicySpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="policyspecinput-schema"></a>
### PolicySpecInput
Policy specification. The `kind` selects the policy language.

| Union Type | Description |
| :---: | --- |
| [PolicySpecInput::Generic](#policyspecinput-generic-schema) | Engine-agnostic policy that grants a set of actions. Subject and object are always supplied by a `PolicyBinding`. |
| [PolicySpecInput::Cedar](#policyspecinput-cedar-schema) | Policy expressed in the Cedar policy language. Policies that declare `?principal` and `?resource` slots are templates instantiated via `PolicyBinding`, otherwise they take effect on their own. Only supported by nodes that use Cedar. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/PolicySpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="policyspecinput-cedar-schema"></a>
### PolicySpecInput::Cedar
Policy expressed in the Cedar policy language. Policies that declare `?principal` and `?resource` slots are templates instantiated via `PolicyBinding`, otherwise they take effect on their own. Only supported by nodes that use Cedar.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `body` | `string` | V |  | Cedar policy source. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/PolicySpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="policyspecinput-generic-schema"></a>
### PolicySpecInput::Generic
Engine-agnostic policy that grants a set of actions. Subject and object are always supplied by a `PolicyBinding`.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `actions` | array(`string`) | V |  | Schema URIs of actions granted by the policy. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/PolicySpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="principalhandle-schema"></a>
### PrincipalHandle
Link to a resolved identity that can act as a principal: `User`, `ServiceAccount`, or `Node`.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `type` | `string` | V |  | Type URI of the principal. |
| `account` | [ResourceOwnerHandle](#resourceownerhandle-schema) |  |  | Account that owns the service account. |
| `id` | `string` | V |  | ID of the principal resource. |
| `did` | `string` |  | `did` | DID of the principal, if applicable. |
| `name` | `string` | V |  | Name of the principal resource. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/PrincipalHandle.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="principalref-schema"></a>
### PrincipalRef
Reference to an identity that can act as a principal: `User`, `ServiceAccount`, or `Node`. Short form requires a type prefix, e.g. `ServiceAccount:acme/ingest-prod`.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `type` | `string` |  |  | Short type name or full type URI of the principal. |
| `account` | [ResourceOwnerRef](#resourceownerref-schema) |  |  | Reference to an account that owns the service account. |
| `id` | `string` |  |  | ID of the principal resource. |
| `did` | `string` |  | `did` | DID of the principal. |
| `name` | `string` |  |  | Name of the principal resource. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/PrincipalRef.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="resourceownerhandle-schema"></a>
### ResourceOwnerHandle
Link to a resolved account that can own resources: `User`, `Organization`, or `Node`.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `id` | `string` | V |  | ID of the account resource. |
| `did` | `string` | V |  | DID of the account. |
| `name` | `string` | V |  | Name of the account. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/ResourceOwnerHandle.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="resourceownerref-schema"></a>
### ResourceOwnerRef
Reference to an account that can own resources: `User`, `Organization`, or `Node`. Short form is the account name, as names of these accounts are unique within a node.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `id` | `string` |  |  | UUID of the account resource. |
| `did` | `string` |  |  | DID of the account. |
| `name` | `string` |  |  | Name of the account. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/ResourceOwnerRef.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="serviceaccount-schema"></a>
### ServiceAccount
A non-human identity used by automation (scheduled flows, CI, installed applications). Service accounts are scoped under an owning account and can act as principals, but cannot own resources.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `$schema` | `string` | V |  | Identifies this resource type. |
| `headers` | [ResourceHeadersInput](#resourceheadersinput-schema) | V |  | Container for identity and ownership information of a resource. |
| `spec` | [ServiceAccountSpecInput](#serviceaccountspecinput-schema) | V |  | Specifies the desired state of the resource. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/ServiceAccount.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="serviceaccountspec-schema"></a>
### ServiceAccountSpec
Service account specification.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `displayName` | `string` |  |  | Human-friendly display name. |
| `disabled` | `boolean` |  |  | Suspends the account. Effects of suspension are implementation-specific.<br/><br/>Default: false |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/ServiceAccountSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="serviceaccountspecinput-schema"></a>
### ServiceAccountSpecInput
Service account specification.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `displayName` | `string` |  |  | Human-friendly display name. |
| `disabled` | `boolean` |  |  | Suspends the account. Effects of suspension are implementation-specific.<br/><br/>Default: false |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/ServiceAccountSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="subjecthandle-schema"></a>
### SubjectHandle
Resolved reference to a party a grant is given to.

| Union Type | Description |
| :---: | --- |
| [SubjectHandle::Resource](#subjecthandle-resource-schema) | A specific `User`, `ServiceAccount`, `Node`, or members of a `Group`. |
| [SubjectHandle::Authenticated](#subjecthandle-authenticated-schema) | Matches any authenticated principal. |
| [SubjectHandle::Anonymous](#subjecthandle-anonymous-schema) | Matches any unauthenticated principal. |
| [SubjectHandle::Everyone](#subjecthandle-everyone-schema) | Matches any principal, authenticated or not. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/SubjectHandle.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="subjecthandle-anonymous-schema"></a>
### SubjectHandle::Anonymous
Matches any unauthenticated principal.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/SubjectHandle.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="subjecthandle-authenticated-schema"></a>
### SubjectHandle::Authenticated
Matches any authenticated principal.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/SubjectHandle.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="subjecthandle-everyone-schema"></a>
### SubjectHandle::Everyone
Matches any principal, authenticated or not.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/SubjectHandle.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="subjecthandle-resource-schema"></a>
### SubjectHandle::Resource
A specific `User`, `ServiceAccount`, `Node`, or members of a `Group`.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `type` | `string` | V |  | Type URI of the subject resource. |
| `account` | [ResourceOwnerHandle](#resourceownerhandle-schema) |  |  | Account that owns the subject resource (for service accounts and groups). |
| `id` | `string` | V |  | ID of the subject resource. |
| `did` | `string` |  | `did` | DID of the subject resource, if applicable. |
| `name` | `string` | V |  | Name of the subject resource. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/SubjectHandle.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="subjectref-schema"></a>
### SubjectRef
Reference to a party a grant is given to. Unlike a principal, a subject may stand for many principals at once. Short form is either a well-known subject name (e.g. `Authenticated`) or a resource reference with a type prefix, DID, or ID (e.g. `Group:acme/engineering`).

| Union Type | Description |
| :---: | --- |
| [SubjectRef::Resource](#subjectref-resource-schema) | A specific `User`, `ServiceAccount`, `Node`, or members of a `Group`. |
| [SubjectRef::Authenticated](#subjectref-authenticated-schema) | Matches any authenticated principal. |
| [SubjectRef::Anonymous](#subjectref-anonymous-schema) | Matches any unauthenticated principal. |
| [SubjectRef::Everyone](#subjectref-everyone-schema) | Matches any principal, authenticated or not. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/SubjectRef.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="subjectref-anonymous-schema"></a>
### SubjectRef::Anonymous
Matches any unauthenticated principal.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/SubjectRef.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="subjectref-authenticated-schema"></a>
### SubjectRef::Authenticated
Matches any authenticated principal.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/SubjectRef.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="subjectref-everyone-schema"></a>
### SubjectRef::Everyone
Matches any principal, authenticated or not.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/SubjectRef.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="subjectref-resource-schema"></a>
### SubjectRef::Resource
A specific `User`, `ServiceAccount`, `Node`, or members of a `Group`.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `type` | `string` |  |  | Short type name or full type URI of the subject resource. |
| `account` | [ResourceOwnerRef](#resourceownerref-schema) |  |  | Reference to an account that owns the subject resource (for service accounts and groups). |
| `id` | `string` |  |  | ID of the subject resource. |
| `did` | `string` |  | `did` | DID of the subject resource. |
| `name` | `string` |  |  | Name of the subject resource. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/SubjectRef.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="user-schema"></a>
### User
An individual person who can authenticate and act as a principal.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `$schema` | `string` | V |  | Identifies this resource type. |
| `headers` | [ResourceHeadersInput](#resourceheadersinput-schema) | V |  | Container for identity and ownership information of a resource. |
| `spec` | [UserSpecInput](#userspecinput-schema) | V |  | Specifies the desired state of the resource. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/User.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="userspec-schema"></a>
### UserSpec
User account specification.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `did` | `string` |  |  | DID associated with the account by ODF or an external system. |
| `displayName` | `string` |  |  | Human-friendly display name. |
| `email` | `string` |  |  | Email address of the user. |
| `avatarUrl` | `string` |  | `url` | URL of the account's avatar image. |
| `password` | [Secret](#secret-schema) |  |  | Password for local authentication, stored as a hash upon apply. Absent for SSO or DID-based accounts. |
| `disabled` | `boolean` |  |  | Suspends the account. Effects of suspension are implementation-specific.<br/><br/>Default: false |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/UserSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="userspecinput-schema"></a>
### UserSpecInput
User account specification.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `did` | `string` |  |  | DID associated with the account by ODF or an external system. |
| `displayName` | `string` |  |  | Human-friendly display name. |
| `email` | `string` |  |  | Email address of the user. |
| `avatarUrl` | `string` |  | `url` | URL of the account's avatar image. |
| `password` | [Secret](#secret-schema) |  |  | Password for local authentication, stored as a hash upon apply. Absent for SSO or DID-based accounts. |
| `disabled` | `boolean` |  |  | Suspends the account. Effects of suspension are implementation-specific.<br/><br/>Default: false |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/auth/v1alpha1/UserSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="reference-config"></a>
## config

<a name="decrypt-schema"></a>
### Decrypt
Decrypt a secret from a `SecretSet`.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/config/v1alpha1/actions/Decrypt.json)
[^](#table-of-contents)


<a name="secret-schema"></a>
### Secret
Individual secret in raw or encrypted form.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `value` | `string` | V |  | A secret value in raw or encoded form. |
| `contentEncoding` | `string` |  |  | Represents the encoding of the value. Typically will be `jwe` after a raw secret gets encrypted. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/config/v1alpha1/Secret.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="secretset-schema"></a>
### SecretSet
Defines a set of secrets stored and managed by the ODF node.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `$schema` | `string` | V |  | Identifies this resource type. |
| `headers` | [ResourceHeadersInput](#resourceheadersinput-schema) | V |  | Container for identity and ownership information of a resource. |
| `spec` | [SecretSetSpecInput](#secretsetspecinput-schema) | V |  | Specifies the desired state of the secret set. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/config/v1alpha1/SecretSet.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="secretsetspec-schema"></a>
### SecretSetSpec
Defines a set of secrets stored and managed by the ODF node and accessible via embedded sercets provider.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `secrets` | [Secrets](#secrets-schema) | V |  | Key value pairs of secrets. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/config/v1alpha1/SecretSetSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="secretsetspecinput-schema"></a>
### SecretSetSpecInput
Defines a set of secrets stored and managed by the ODF node and accessible via embedded sercets provider.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `secrets` | [Secrets](#secrets-schema) | V |  | Key value pairs of secrets. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/config/v1alpha1/SecretSetSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="secrets-schema"></a>
### Secrets
Container for key-value secrets. Every key must be a string. Values may be strings with raw unencrypted data or objects that signify the encoding.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/config/v1alpha1/Secrets.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="known-extra-attrs"></a>
#### Known Extensions
| Extension | Description |
| --- | --- |
| `opendatafabric.net/description` | Used for human readable schema field descriptions |
| `opendatafabric.net/type` | An extended set of logical types that ODF recommends but does not require every implementation to support |
| `opendatafabric.org/linkedObjects` | When attached to `AddData` event contains a summary of how many external objects were associated with a certain transaction as well as their size |
| `arrow.apache.org/bufferEncoding` | Used to accurately represent buffer encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/dateEncoding` | Used to accurately represent date encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/decimalEncoding` | Used to accurately represent decimal encoding type when converting Arrow schema to ODF schema |


<a name="known-extra-types"></a>
#### Known Extended Types
| Extended Type | Core Type | Description |
| --- | --- | --- |
| `Did` | `String` | Decentralized identifier `did:<method>:<id>` |
| `Multihash` | `String` | Hash in self-describing [multihash](https://github.com/multiformats/multihash) format |
| `ObjectLink` | `String` | Signifies that the value references an external object. The mandatory `linkType` property defines the type of the link (e.g. `Multihash`). |



<a name="valuehandle-schema"></a>
### ValueHandle
Reference to a value within a `VariableSet` or a `SecretSet`.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `account` | [ResourceOwnerHandle](#resourceownerhandle-schema) | V |  | Account that owns the target resource. |
| `type` | `string` | V |  | Type URI of the target resource. |
| `id` | `string` | V |  | ID of the resource within a node. |
| `name` | `string` | V |  | Name of a resource. |
| `path` | `string` |  |  | JSON path to a value within a `VariableSet` or a `SecretSet`. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/config/v1alpha1/ValueHandle.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="valueref-schema"></a>
### ValueRef
Reference to a value within a `VariableSet` or a `SecretSet`.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `account` | [ResourceOwnerRef](#resourceownerref-schema) |  |  | Reference to an account that owns the `VariableSet` or the `SecretSet`. |
| `id` | `string` |  |  | ID of a resource. |
| `type` | `string` |  |  | Short type name or full type URI of the target resource. |
| `name` | `string` |  |  | Name of a resource. |
| `path` | `string` |  |  | JSON path to a value within a `VariableSet` or a `SecretSet`. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/config/v1alpha1/ValueRef.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="valuerefs-schema"></a>
### ValueRefs
Container for key-value variables. Every key must be a string. Values shoud reference fields in `SecretSet`s and `VariableSet`s.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/config/v1alpha1/ValueRefs.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="known-extra-attrs"></a>
#### Known Extensions
| Extension | Description |
| --- | --- |
| `opendatafabric.net/description` | Used for human readable schema field descriptions |
| `opendatafabric.net/type` | An extended set of logical types that ODF recommends but does not require every implementation to support |
| `opendatafabric.org/linkedObjects` | When attached to `AddData` event contains a summary of how many external objects were associated with a certain transaction as well as their size |
| `arrow.apache.org/bufferEncoding` | Used to accurately represent buffer encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/dateEncoding` | Used to accurately represent date encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/decimalEncoding` | Used to accurately represent decimal encoding type when converting Arrow schema to ODF schema |


<a name="known-extra-types"></a>
#### Known Extended Types
| Extended Type | Core Type | Description |
| --- | --- | --- |
| `Did` | `String` | Decentralized identifier `did:<method>:<id>` |
| `Multihash` | `String` | Hash in self-describing [multihash](https://github.com/multiformats/multihash) format |
| `ObjectLink` | `String` | Signifies that the value references an external object. The mandatory `linkType` property defines the type of the link (e.g. `Multihash`). |



<a name="variable-schema"></a>
### Variable
Individual variable.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `value` | `string` | V |  | A value in raw or encoded form. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/config/v1alpha1/Variable.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="variableset-schema"></a>
### VariableSet
Defines a set of variables stored and managed by the ODF node.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `$schema` | `string` | V |  | Identifies this resource type. |
| `headers` | [ResourceHeadersInput](#resourceheadersinput-schema) | V |  | Container for identity and ownership information of a resource. |
| `spec` | [VariableSetSpecInput](#variablesetspecinput-schema) | V |  | Specifies the desired state of the variable set. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/config/v1alpha1/VariableSet.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="variablesetspec-schema"></a>
### VariableSetSpec
Defines a set of variables stored and managed by the ODF node and accessible via embedded variables provider.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `variables` | [Variables](#variables-schema) | V |  | Key value pairs of variables. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/config/v1alpha1/VariableSetSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="variablesetspecinput-schema"></a>
### VariableSetSpecInput
Defines a set of variables stored and managed by the ODF node and accessible via embedded variables provider.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `variables` | [Variables](#variables-schema) | V |  | Key value pairs of variables. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/config/v1alpha1/VariableSetSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="variables-schema"></a>
### Variables
Container for key-value variables. Every key must be a string. Values may be raw strings or objects that incorporate the encoding.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/config/v1alpha1/Variables.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="known-extra-attrs"></a>
#### Known Extensions
| Extension | Description |
| --- | --- |
| `opendatafabric.net/description` | Used for human readable schema field descriptions |
| `opendatafabric.net/type` | An extended set of logical types that ODF recommends but does not require every implementation to support |
| `opendatafabric.org/linkedObjects` | When attached to `AddData` event contains a summary of how many external objects were associated with a certain transaction as well as their size |
| `arrow.apache.org/bufferEncoding` | Used to accurately represent buffer encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/dateEncoding` | Used to accurately represent date encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/decimalEncoding` | Used to accurately represent decimal encoding type when converting Arrow schema to ODF schema |


<a name="known-extra-types"></a>
#### Known Extended Types
| Extended Type | Core Type | Description |
| --- | --- | --- |
| `Did` | `String` | Decentralized identifier `did:<method>:<id>` |
| `Multihash` | `String` | Hash in self-describing [multihash](https://github.com/multiformats/multihash) format |
| `ObjectLink` | `String` | Signifies that the value references an external object. The mandatory `linkType` property defines the type of the link (e.g. `Multihash`). |



<a name="reference-data"></a>
## data

<a name="datafield-schema"></a>
### DataField
Represents a named field (column) in a root or nested struct schema

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` | V |  | Name of the field |
| `type` | [DataType](#datatype-schema) | V |  | Logical type of the field that defines its semantic behavior and value ranges |
| `extra` | [ExtraAttributes](#extraattributes-schema) |  |  | ODF extensions |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataField.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="dataschema-schema"></a>
### DataSchema
This schema aims to be a human-friendly variant of Arrow. Arrow currently specifies only the [flatbuffer format](https://github.com/apache/arrow/blob/f9301c0ba8a7ed1b0b63275cfdd4c44c26b04675/format/Schema.fbs) which has many legacy to it and is not suited to be defined by humans, so we had to define our own schema format. While inspired by Arrow - this format makes a clear separation between logical data types and encoding (physical layout) of data in the chunks.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `fields` | array([DataField](#datafield-schema)) | V |  | Top-level fields (columns) of the schema. |
| `extra` | [ExtraAttributes](#extraattributes-schema) |  |  | ODF extensions |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataSchema.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-schema"></a>
### DataType
Defines a logical type of the field. Logical type determines the semantics and boudaries of a type and how it can be operated on, without a concern about encoding and physical layout of the data in chunks.

| Union Type | Description |
| :---: | --- |
| [DataType::Binary](#datatype-binary-schema) | A sequence of bytes. Used for arbitrary binary data. |
| [DataType::Bool](#datatype-bool-schema) | A boolean value representing true or false. |
| [DataType::Date](#datatype-date-schema) | A calendar date. |
| [DataType::Decimal](#datatype-decimal-schema) | A fixed-point decimal number with a specified precision and scale. |
| [DataType::Duration](#datatype-duration-schema) | An elapsed time interval with a specified time unit. |
| [DataType::Float16](#datatype-float16-schema) | A floating-point number. |
| [DataType::Float32](#datatype-float32-schema) | A floating-point number. |
| [DataType::Float64](#datatype-float64-schema) | A floating-point number. |
| [DataType::Int8](#datatype-int8-schema) | An integer value. |
| [DataType::Int16](#datatype-int16-schema) | An integer value. |
| [DataType::Int32](#datatype-int32-schema) | An integer value. |
| [DataType::Int64](#datatype-int64-schema) | An integer value. |
| [DataType::UInt8](#datatype-uint8-schema) | An integer value. |
| [DataType::UInt16](#datatype-uint16-schema) | An integer value. |
| [DataType::UInt32](#datatype-uint32-schema) | An integer value. |
| [DataType::UInt64](#datatype-uint64-schema) | An integer value. |
| [DataType::List](#datatype-list-schema) | A list of values, all having the same data type. |
| [DataType::Map](#datatype-map-schema) | A map of key-value pairs, represented as a list of entries (structs with key and value fields). |
| [DataType::Null](#datatype-null-schema) | A type representing the absence of a value (null). |
| [DataType::Option](#datatype-option-schema) | A type representing an optional (nullable) value of another data type. |
| [DataType::Struct](#datatype-struct-schema) | A collection of named fields, each with its own data type. |
| [DataType::Time](#datatype-time-schema) | A time of day value, without a date, with a specified unit of granularity. |
| [DataType::Timestamp](#datatype-timestamp-schema) | A point in time, represented as an offset from the Unix epoch in a specific timezone. |
| [DataType::String](#datatype-string-schema) | A Unicode string. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-binary-schema"></a>
### DataType::Binary
A sequence of bytes. Used for arbitrary binary data.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `fixedLength` | `integer` |  | `uint64` | Number of bytes per value for fixed-size binary. If omitted, the binary is variable-length. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-bool-schema"></a>
### DataType::Bool
A boolean value representing true or false.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-date-schema"></a>
### DataType::Date
A calendar date.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-decimal-schema"></a>
### DataType::Decimal
A fixed-point decimal number with a specified precision and scale.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `precision` | `integer` | V | `uint32` | Total number of decimal digits that can be stored. |
| `scale` | `integer` | V | `int32` | Number of digits after the decimal point. In certain situations, scale could be negative number. For negative scale, it is the number of padding 0 to the right of the digits.<br/><br/>For example the number 12300 could be treated as a decimal has precision 3 and scale -2. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-duration-schema"></a>
### DataType::Duration
An elapsed time interval with a specified time unit.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `unit` | [TimeUnit](#timeunit-schema) |  |  | The unit of the duration measurement.<br/><br/>Default: "Millisecond" |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-float16-schema"></a>
### DataType::Float16
A floating-point number.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-float32-schema"></a>
### DataType::Float32
A floating-point number.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-float64-schema"></a>
### DataType::Float64
A floating-point number.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-int16-schema"></a>
### DataType::Int16
An integer value.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-int32-schema"></a>
### DataType::Int32
An integer value.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-int64-schema"></a>
### DataType::Int64
An integer value.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-int8-schema"></a>
### DataType::Int8
An integer value.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-list-schema"></a>
### DataType::List
A list of values, all having the same data type.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `itemType` | [DataType](#datatype-schema) | V |  | Data type of list items. |
| `fixedLength` | `integer` |  | `uint64` | Number of list items per value for fixed-size lists. If omitted, the list is variable-length. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-map-schema"></a>
### DataType::Map
A map of key-value pairs, represented as a list of entries (structs with key and value fields).

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `keyType` | [DataType](#datatype-schema) | V |  | Data type of the map's keys. |
| `valueType` | [DataType](#datatype-schema) | V |  | Data type of the map's values. |
| `keysSorted` | `boolean` |  |  | Set to true if the keys within each value are sorted. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-null-schema"></a>
### DataType::Null
A type representing the absence of a value (null).

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-option-schema"></a>
### DataType::Option
A type representing an optional (nullable) value of another data type.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `inner` | [DataType](#datatype-schema) | V |  | Inner data type for the optional value. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-string-schema"></a>
### DataType::String
A Unicode string.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-struct-schema"></a>
### DataType::Struct
A collection of named fields, each with its own data type.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `fields` | array([DataField](#datafield-schema)) | V |  | Fields that make up the struct. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-time-schema"></a>
### DataType::Time
A time of day value, without a date, with a specified unit of granularity.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `unit` | [TimeUnit](#timeunit-schema) |  |  | The unit of the time value.<br/><br/>Default: "Millisecond" |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-timestamp-schema"></a>
### DataType::Timestamp
A point in time, represented as an offset from the Unix epoch in a specific timezone.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `unit` | [TimeUnit](#timeunit-schema) |  |  | The unit of the timestamp value that determines its precision.<br/><br/>Default: "Millisecond" |
| `timezone` | `string` |  |  | The timezone is an optional string indicating the name of a timezone<br/>one of<br/><br/>* As used in the Olson timezone database (the "tz database" or<br/>  "tzdata"), such as "America/New_York".<br/>* An absolute timezone offset of the form "+XX:XX" or "-XX:XX",<br/>  such as "+07:30".<br/><br/>Default: "UTC" |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-uint16-schema"></a>
### DataType::UInt16
An integer value.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-uint32-schema"></a>
### DataType::UInt32
An integer value.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-uint64-schema"></a>
### DataType::UInt64
An integer value.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datatype-uint8-schema"></a>
### DataType::UInt8
An integer value.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/DataType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="extraattributes-schema"></a>
### ExtraAttributes
Container for custom key-value extension attributes. Every key must be in the form of `<domain>/<path>` (e.g. `kamu.dev/archetype`) in order to fully disambiguate the value in the face of multiple extensions. Values may be any valid JSON including nested objects.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/ExtraAttributes.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="known-extra-attrs"></a>
#### Known Extensions
| Extension | Description |
| --- | --- |
| `opendatafabric.net/description` | Used for human readable schema field descriptions |
| `opendatafabric.net/type` | An extended set of logical types that ODF recommends but does not require every implementation to support |
| `opendatafabric.org/linkedObjects` | When attached to `AddData` event contains a summary of how many external objects were associated with a certain transaction as well as their size |
| `arrow.apache.org/bufferEncoding` | Used to accurately represent buffer encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/dateEncoding` | Used to accurately represent date encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/decimalEncoding` | Used to accurately represent decimal encoding type when converting Arrow schema to ODF schema |


<a name="known-extra-types"></a>
#### Known Extended Types
| Extended Type | Core Type | Description |
| --- | --- | --- |
| `Did` | `String` | Decentralized identifier `did:<method>:<id>` |
| `Multihash` | `String` | Hash in self-describing [multihash](https://github.com/multiformats/multihash) format |
| `ObjectLink` | `String` | Signifies that the value references an external object. The mandatory `linkType` property defines the type of the link (e.g. `Multihash`). |



<a name="operationtype-schema"></a>
### OperationType
Defines an operation in a changelog stream.

| Enum Value |
| :---: |
| Append |
| Retract |
| CorrectFrom |
| CorrectTo |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/OperationType.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="timeunit-schema"></a>
### TimeUnit
Defines the unit of measurement of time

| Enum Value |
| :---: |
| Second |
| Millisecond |
| Microsecond |
| Nanosecond |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/data/v1alpha1/TimeUnit.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="reference-datasets"></a>
## datasets

<a name="adddata-schema"></a>
### AddData
Indicates that data has been ingested into a root dataset.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `prevCheckpoint` | `string` |  | [multihash](https://github.com/multiformats/multihash) | Hash of the checkpoint file used to restore ingestion state, if any. |
| `prevOffset` | `integer` |  | `uint64` | Last offset of the previous data slice, if any. Must be equal to the last non-empty `newData.offsetInterval.end`. |
| `newData` | [DataSlice](#dataslice-schema) |  |  | Describes output data written during this transaction, if any. |
| `newCheckpoint` | [Checkpoint](#checkpoint-schema) |  |  | Describes checkpoint written during this transaction, if any. If an engine operation resulted in no updates to the checkpoint, but checkpoint is still relevant for subsequent runs - a hash of the previous checkpoint should be specified. |
| `newWatermark` | `string` |  | [date-time](https://json-schema.org/draft/2019-09/json-schema-validation.html#rfc.section.7.3.1) | Last watermark of the output data stream, if any. Initial blocks may not have watermarks, but once watermark is set - all subsequent blocks should either carry the same watermark or specify a new (greater) one. Thus, watermarks are monotonically non-decreasing. |
| `newSourceState` | [SourceState](#sourcestate-schema) |  |  | The state of the source the data was added from to allow fast resuming. If the state did not change but is still relevant for subsequent runs it should be carried, i.e. only the last state per source is considered when resuming. |
| `extra` | [ExtraAttributes](#extraattributes-schema) |  |  | ODF extensions. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/AddData.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="allowanonymousread-schema"></a>
### AllowAnonymousRead
Controls whether the dataset is readable by users who did not authenticate. Materialized into the auth engine by the Dataset controller.

| Type | Format |
| :---: | :---: |
| `boolean` |  |

| Label Property | Value |
| --- | --- |
| Auth attribute | true |
| Resource types | [Dataset](#dataset-schema) |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/AllowAnonymousRead.json)
[^](#table-of-contents)


<a name="allowpublicread-schema"></a>
### AllowPublicRead
Controls whether the dataset is readable by any authenticated principal. Materialized into the auth engine by the Dataset controller.

| Type | Format |
| :---: | :---: |
| `boolean` |  |

| Label Property | Value |
| --- | --- |
| Auth attribute | true |
| Resource types | [Dataset](#dataset-schema) |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/AllowPublicRead.json)
[^](#table-of-contents)


<a name="alterhistory-schema"></a>
### AlterHistory
Rewrite or truncate dataset history.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/actions/AlterHistory.json)
[^](#table-of-contents)


<a name="attachmentembedded-schema"></a>
### AttachmentEmbedded
Embedded attachment item.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `path` | `string` | V |  | Path to an attachment if it was materialized into a file. |
| `content` | `string` | V |  | Content of the attachment. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/AttachmentEmbedded.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="attachments-schema"></a>
### Attachments
Defines the source of attachment files.

| Union Type | Description |
| :---: | --- |
| [Attachments::Embedded](#attachments-embedded-schema) | For attachments that are specified inline and are embedded in the metadata. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/Attachments.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="attachments-embedded-schema"></a>
### Attachments::Embedded
For attachments that are specified inline and are embedded in the metadata.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `items` | array([AttachmentEmbedded](#attachmentembedded-schema)) | V |  | List of embedded items. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/Attachments.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="checkpoint-schema"></a>
### Checkpoint
Describes a checkpoint produced by an engine

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `physicalHash` | `string` | V | [multihash](https://github.com/multiformats/multihash) | Hash sum of the checkpoint file. |
| `size` | `integer` | V | `uint64` | Size of checkpoint file in bytes. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/Checkpoint.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="dataslice-schema"></a>
### DataSlice
Describes a slice of data added to a dataset or produced via transformation

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `logicalHash` | `string` | V | [multihash](https://github.com/multiformats/multihash) | Logical hash sum of the data in this slice. |
| `physicalHash` | `string` | V | [multihash](https://github.com/multiformats/multihash) | Hash sum of the data part file. |
| `offsetInterval` | [OffsetInterval](#offsetinterval-schema) | V |  | Data slice produced by the transaction. |
| `size` | `integer` | V | `uint64` | Size of data file in bytes. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/DataSlice.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="dataset-schema"></a>
### Dataset
Represents a desired state of a dataset.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `$schema` | `string` | V |  | Identifies this resource type. |
| `headers` | [ResourceHeadersInput](#resourceheadersinput-schema) | V |  | Container for identity and ownership information of a resource. |
| `spec` | [DatasetSpecInput](#datasetspecinput-schema) | V |  | Specifies the desired state of the resource. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/Dataset.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datasethandle-schema"></a>
### DatasetHandle
Link to a dataset.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `account` | [ResourceOwnerHandle](#resourceownerhandle-schema) |  |  | Reference to an account that owns the dataset. |
| `id` | `string` | V |  | ID of the dataset resource. |
| `did` | `string` | V | [dataset-id](datasets/identity.md#unique-identifiers) | DID of the dataset. |
| `name` | `string` | V |  | Name of the dataset. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/DatasetHandle.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datasetkind-schema"></a>
### DatasetKind
Represents type of the dataset.

| Enum Value |
| :---: |
| Root |
| Derivative |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/DatasetKind.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datasetref-schema"></a>
### DatasetRef
Reference to a dataset.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `account` | [ResourceOwnerRef](#resourceownerref-schema) |  |  | Reference to an account that owns the dataset. |
| `id` | `string` |  |  | UUID of the dataset resource. |
| `did` | `string` |  | [dataset-id](datasets/identity.md#unique-identifiers) | DID of the dataset. |
| `name` | `string` |  |  | Name of the dataset. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/DatasetRef.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datasetselector-schema"></a>
### DatasetSelector
Selects one or more datasets by name pattern and optional filters.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `account` | [ResourceOwnerRef](#resourceownerref-schema) |  |  | Reference to an account that owns the target resources. |
| `id` | `string` |  |  | ID of the singular resource. |
| `name` | `string` |  |  | Name pattern in SQL `LIKE` format. |
| `labels` | [LabelFilter](#labelfilter-schema) |  |  | Filter by resource labels. |
| `kind` | [DatasetKind](#datasetkind-schema) |  |  | Restricts the selector to datasets of a specific kind. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/DatasetSelector.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datasetspec-schema"></a>
### DatasetSpec
Represents a desired state of the dataset metadata.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `did` | `string` | V | [dataset-id](datasets/identity.md#unique-identifiers) | DID of the dataset in global ODF network |
| `kind` | [DatasetKind](#datasetkind-schema) | V |  | Type of the dataset. |
| `metadata` | array([MetadataEvent](#metadataevent-schema)) | V |  | An array of metadata events that will be used to populate the chain, e.g. to set info, licenses, and attachments. As the dataset evolves, it maintains a snapshot of the key events that describe the dataset in its latest state, excluding intermediate transitions (e.g. previous schema changes) and events related to ingestions and transformations. |
| `volume` | [ResourceHandle](#resourcehandle-schema) | V |  | Reference to a storage volume where dataset data will be stored. If omitted, the node's default storage is used. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/DatasetSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datasetspecinput-schema"></a>
### DatasetSpecInput
Represents a desired state of the dataset metadata.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `did` | `string` |  | [dataset-id](datasets/identity.md#unique-identifiers) | DID of the dataset in global ODF network |
| `kind` | [DatasetKind](#datasetkind-schema) | V |  | Type of the dataset. |
| `metadata` | array([MetadataEvent](#metadataevent-schema)) | V |  | An array of metadata events that will be used to populate the chain, e.g. to set info, licenses, and attachments. As the dataset evolves, it maintains a snapshot of the key events that describe the dataset in its latest state, excluding intermediate transitions (e.g. previous schema changes) and events related to ingestions and transformations. |
| `volume` | [PersistentVolumeRef](#persistentvolumeref-schema) |  |  | Reference to a storage volume where dataset data will be stored. If omitted, the node's default storage is used. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/DatasetSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datasetvocabulary-schema"></a>
### DatasetVocabulary
Specifies the mapping of system columns onto dataset schema.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `offsetColumn` | `string` |  |  | Name of the offset column.<br/><br/>Default: "offset" |
| `operationTypeColumn` | `string` |  |  | Name of the operation type column.<br/><br/>Default: "op" |
| `systemTimeColumn` | `string` |  |  | Name of the system time column.<br/><br/>Default: "system_time" |
| `eventTimeColumn` | `string` |  |  | Name of the event time column.<br/><br/>Default: "event_time" |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/DatasetVocabulary.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="executetransform-schema"></a>
### ExecuteTransform
Indicates that derivative transformation has been performed.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `queryInputs` | array([ExecuteTransformInput](#executetransforminput-schema)) | V |  | Defines inputs used in this transaction. Slices corresponding to every input dataset must be present. |
| `prevCheckpoint` | `string` |  | [multihash](https://github.com/multiformats/multihash) | Hash of the checkpoint file used to restore transformation state, if any. |
| `prevOffset` | `integer` |  | `uint64` | Last offset of the previous data slice, if any. Must be equal to the last non-empty `newData.offsetInterval.end`. |
| `newData` | [DataSlice](#dataslice-schema) |  |  | Describes output data written during this transaction, if any. |
| `newCheckpoint` | [Checkpoint](#checkpoint-schema) |  |  | Describes checkpoint written during this transaction, if any. If an engine operation resulted in no updates to the checkpoint, but checkpoint is still relevant for subsequent runs - a hash of the previous checkpoint should be specified. |
| `newWatermark` | `string` |  | [date-time](https://json-schema.org/draft/2019-09/json-schema-validation.html#rfc.section.7.3.1) | Last watermark of the output data stream, if any. Initial blocks may not have watermarks, but once watermark is set - all subsequent blocks should either carry the same watermark or specify a new (greater) one. Thus, watermarks are monotonically non-decreasing. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/ExecuteTransform.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="executetransforminput-schema"></a>
### ExecuteTransformInput
Describes a slice of the input dataset used during a transformation

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `datasetId` | `string` | V | [dataset-id](datasets/identity.md#unique-identifiers) | Input dataset identifier. |
| `prevBlockHash` | `string` |  | [multihash](https://github.com/multiformats/multihash) | Last block of the input dataset that was previously incorporated into the derivative transformation, if any. Must be equal to the last non-empty `newBlockHash`. Together with `newBlockHash` defines a half-open `(prevBlockHash, newBlockHash]` interval of blocks that will be considered in this transaction. |
| `newBlockHash` | `string` |  | [multihash](https://github.com/multiformats/multihash) | Hash of the last block that will be incorporated into the derivative transformation. When present, defines a half-open `(prevBlockHash, newBlockHash]` interval of blocks that will be considered in this transaction. |
| `prevOffset` | `integer` |  | `uint64` | Last data record offset in the input dataset that was previously incorporated into the derivative transformation, if any. Must be equal to the last non-empty `newOffset`. Together with `newOffset` defines a half-open `(prevOffset, newOffset]` interval of data records that will be considered in this transaction. |
| `newOffset` | `integer` |  | `uint64` | Offset of the last data record that will be incorporated into the derivative transformation, if any. When present, defines a half-open `(prevOffset, newOffset]` interval of data records that will be considered in this transaction. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/ExecuteTransformInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="metadatablock-schema"></a>
### MetadataBlock
An individual block in the metadata chain that captures the history of modifications of a dataset.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `systemTime` | `string` | V | [date-time](https://json-schema.org/draft/2019-09/json-schema-validation.html#rfc.section.7.3.1) | System time when this block was written. |
| `prevBlockHash` | `string` |  | [multihash](https://github.com/multiformats/multihash) | Hash sum of the preceding block. |
| `sequenceNumber` | `integer` | V | `uint64` | Block sequence number, starting from zero at the seed block. |
| `event` | [MetadataEvent](#metadataevent-schema) | V |  | Event data. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/MetadataBlock.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="metadataevent-schema"></a>
### MetadataEvent
Represents a transaction that occurred on a dataset.

| Union Type | Description |
| :---: | --- |
| [AddData](#adddata-schema) | Indicates that data has been ingested into a root dataset. |
| [ExecuteTransform](#executetransform-schema) | Indicates that derivative transformation has been performed. |
| [Seed](#seed-schema) | Establishes the identity of the dataset. Always the first metadata event in the chain. |
| [SetPollingSource](#setpollingsource-schema) | Contains information on how externally-hosted data can be ingested into the root dataset. |
| [SetTransform](#settransform-schema) | Defines a transformation that produces data in a derivative dataset. |
| [SetVocab](#setvocab-schema) | Lets you manipulate names of the system columns to avoid conflicts. |
| [SetAttachments](#setattachments-schema) | Associates a set of files with this dataset. |
| [SetInfo](#setinfo-schema) | Provides basic human-readable information about a dataset. |
| [SetLicense](#setlicense-schema) | Defines a license that applies to this dataset. |
| [SetDataSchema](#setdataschema-schema) | Specifies the complete schema of Data Slices added to the Dataset following this event. |
| [AddPushSource](#addpushsource-schema) | Describes how to ingest data into a root dataset from a certain logical source. |
| [DisablePushSource](#disablepushsource-schema) | Disables the previously defined source. |
| [DisablePollingSource](#disablepollingsource-schema) | Disables the previously defined polling source. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/MetadataEvent.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="offsetinterval-schema"></a>
### OffsetInterval
Describes a range of data as a closed arithmetic interval of offsets

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `start` | `integer` | V | `uint64` | Start of the closed interval [start; end]. |
| `end` | `integer` | V | `uint64` | End of the closed interval [start; end]. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/OffsetInterval.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="projection-schema"></a>
### Projection
Represents a projection of a dataaset history into a state for fast lookups.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `$schema` | `string` | V |  | Identifies this resource type. |
| `headers` | [ResourceHeadersInput](#resourceheadersinput-schema) | V |  | Container for identity and ownership information of a resource. |
| `spec` | [ProjectionSpecInput](#projectionspecinput-schema) | V |  | Specifies the desired state of the resource. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/Projection.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="projectionspec-schema"></a>
### ProjectionSpec
Represents a projection of a dataaset history into a state for fast lookups.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `inputs` | array([TransformInput](#transforminput-schema)) | V |  | Datasets that will be used as sources. |
| `project` | [Transform](#transform-schema) | V |  | Transformation that will be applied to produce new data. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/ProjectionSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="projectionspecinput-schema"></a>
### ProjectionSpecInput
Represents a projection of a dataaset history into a state for fast lookups.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `inputs` | array([TransformInput](#transforminput-schema)) | V |  | Datasets that will be used as sources. |
| `project` | [Transform](#transform-schema) | V |  | Transformation that will be applied to produce new data. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/ProjectionSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="read-schema"></a>
### Read
Read dataset data (query, download).

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/actions/Read.json)
[^](#table-of-contents)


<a name="seed-schema"></a>
### Seed
Establishes the identity of the dataset. Always the first metadata event in the chain.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `datasetId` | `string` | V | [dataset-id](datasets/identity.md#unique-identifiers) | Unique identity of the dataset. |
| `datasetKind` | [DatasetKind](#datasetkind-schema) | V |  | Type of the dataset. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/Seed.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="setattachments-schema"></a>
### SetAttachments
Associates a set of files with this dataset.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `attachments` | [Attachments](#attachments-schema) | V |  | One of the supported attachment sources. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/SetAttachments.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="setdataschema-schema"></a>
### SetDataSchema
Specifies the complete schema of Data Slices added to the Dataset following this event.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `rawArrowSchema` | `string` |  | `flatbuffers` | DEPRECATED: Apache Arrow schema encoded in its native flatbuffers representation. |
| `schema` | [DataSchema](#dataschema-schema) |  |  | Defines the logical schema of the data files that follow this event. Will become a required field after migration. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/SetDataSchema.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="setinfo-schema"></a>
### SetInfo
Provides basic human-readable information about a dataset.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `description` | `string` |  |  | Brief single-sentence summary of a dataset. |
| `keywords` | array(`string`) |  |  | Keywords, search terms, or tags used to describe the dataset. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/SetInfo.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="setlicense-schema"></a>
### SetLicense
Defines a license that applies to this dataset.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `shortName` | `string` | V |  | Abbreviated name of the license. |
| `name` | `string` | V |  | Full name of the license. |
| `spdxId` | `string` |  |  | License identifier from the SPDX License List. |
| `websiteUrl` | `string` | V | `url` | URL where licensing terms can be found. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/SetLicense.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="settransform-schema"></a>
### SetTransform
Defines a transformation that produces data in a derivative dataset.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `inputs` | array([TransformInput](#transforminput-schema)) | V |  | Datasets that will be used as sources. |
| `transform` | [Transform](#transform-schema) | V |  | Transformation that will be applied to produce new data. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/SetTransform.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="setvocab-schema"></a>
### SetVocab
Lets you manipulate names of the system columns to avoid conflicts.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `offsetColumn` | `string` |  |  | Name of the offset column. |
| `operationTypeColumn` | `string` |  |  | Name of the operation type column. |
| `systemTimeColumn` | `string` |  |  | Name of the system time column. |
| `eventTimeColumn` | `string` |  |  | Name of the event time column. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/SetVocab.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="sqlquerystep-schema"></a>
### SqlQueryStep
Defines a query in a multi-step SQL transformation.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `alias` | `string` |  |  | Name of the temporary view that will be created from result of the query. Step without this alias will be treated as an output of the transformation. |
| `query` | `string` | V |  | SQL query the result of which will be exposed under the alias. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/SqlQueryStep.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="temporaltable-schema"></a>
### TemporalTable
Temporary Flink-specific extension for creating temporal tables from streams.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` | V |  | Name of the dataset to be converted into a temporal table. |
| `primaryKey` | array(`string`) | V |  | Column names used as the primary key for creating a table. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/TemporalTable.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="transform-schema"></a>
### Transform
Engine-specific processing queries that shape the resulting data.

| Union Type | Description |
| :---: | --- |
| [Transform::Sql](#transform-sql-schema) | Transform using one of the SQL dialects. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/Transform.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="transform-sql-schema"></a>
### Transform::Sql
Transform using one of the SQL dialects.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `engine` | `string` | V |  | Identifier of the engine used for this transformation. |
| `version` | `string` |  |  | Version of the engine to use. |
| `query` | `string` |  |  | SQL query the result of which will be used as an output. This is a convenience property meant only for defining queries by hand. When stored in the metadata this property will never be set and instead will be converted into a single-iter `queries` array. |
| `queries` | array([SqlQueryStep](#sqlquerystep-schema)) |  |  | Specifies multi-step SQL transformations. Each step acts as a shorthand for `CREATE TEMPORARY VIEW <alias> AS (<query>)`. Last query in the array should have no alias and will be treated as an output. |
| `temporalTables` | array([TemporalTable](#temporaltable-schema)) |  |  | Temporary Flink-specific extension for creating temporal tables from streams. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/Transform.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="transforminput-schema"></a>
### TransformInput
Describes a derivative transformation input

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `datasetRef` | `string` | V | [dataset-ref](datasets/identity.md#grammar) | A local or remote dataset reference. When block is accepted this MUST be in the form of a DatasetId to guarantee reproducibility, as aliases can change over time. |
| `alias` | `string` |  |  | An alias under which this input will be available in queries. Will be populated from `datasetRef` if not provided before resolving it to DatasetId. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/TransformInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="watermark-schema"></a>
### Watermark
Represents a watermark in the event stream.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `systemTime` | `string` | V | [date-time](https://json-schema.org/draft/2019-09/json-schema-validation.html#rfc.section.7.3.1) | Moment in processing time when watermark was emitted. |
| `eventTime` | `string` | V | [date-time](https://json-schema.org/draft/2019-09/json-schema-validation.html#rfc.section.7.3.1) | Moment in event time which watermark has reached. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/Watermark.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="write-schema"></a>
### Write
Append new data to the dataset.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/datasets/v1alpha1/actions/Write.json)
[^](#table-of-contents)


<a name="reference-engines"></a>
## engines

<a name="rawqueryrequest-schema"></a>
### RawQueryRequest
Sent by the coordinator to an engine to perform query on raw input data, usually as part of ingest preprocessing step

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `inputDataPaths` | array(`string`) | V |  | Paths to input data files to perform query over. Must all have identical schema. |
| `transform` | [Transform](#transform-schema) | V |  | Transformation that will be applied to produce new data. |
| `outputDataPath` | `string` | V | `path` | Path where query result will be written. |
| `preserveOrder` | `boolean` |  |  | Requires the engine to keep the order of records read from the input in the query result, or fail the request if it cannot.<br/><br/>Default: false |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/engines/v1alpha1/RawQueryRequest.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="rawqueryresponse-schema"></a>
### RawQueryResponse
Sent by an engine to coordinator when performing the raw query operation

| Union Type | Description |
| :---: | --- |
| [RawQueryResponse::Progress](#rawqueryresponse-progress-schema) | Reports query progress |
| [RawQueryResponse::Success](#rawqueryresponse-success-schema) | Query executed successfully |
| [RawQueryResponse::InvalidQuery](#rawqueryresponse-invalidquery-schema) | Query did not pass validation |
| [RawQueryResponse::InternalError](#rawqueryresponse-internalerror-schema) | Internal error during query execution |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/engines/v1alpha1/RawQueryResponse.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="rawqueryresponse-internalerror-schema"></a>
### RawQueryResponse::InternalError
Internal error during query execution

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `message` | `string` | V |  | Brief description of an error |
| `backtrace` | `string` |  |  | Details of an error (e.g. a backtrace) |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/engines/v1alpha1/RawQueryResponse.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="rawqueryresponse-invalidquery-schema"></a>
### RawQueryResponse::InvalidQuery
Query did not pass validation

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `message` | `string` | V |  | Explanation of an error |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/engines/v1alpha1/RawQueryResponse.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="rawqueryresponse-progress-schema"></a>
### RawQueryResponse::Progress
Reports query progress

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/engines/v1alpha1/RawQueryResponse.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="rawqueryresponse-success-schema"></a>
### RawQueryResponse::Success
Query executed successfully

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `numRecords` | `integer` | V | `uint64` | Number of records produced by the query |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/engines/v1alpha1/RawQueryResponse.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="transformrequest-schema"></a>
### TransformRequest
Sent by the coordinator to an engine to perform the next step of data transformation

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `datasetId` | `string` | V | [dataset-id](datasets/identity.md#unique-identifiers) | Unique identifier of the output dataset. |
| `datasetAlias` | `string` | V | [dataset-alias](datasets/identity.md#grammar) | Alias of the output dataset, for logging purposes only. |
| `systemTime` | `string` | V | [date-time](https://json-schema.org/draft/2019-09/json-schema-validation.html#rfc.section.7.3.1) | System time to use for new records. |
| `vocab` | [DatasetVocabulary](#datasetvocabulary-schema) | V |  | Vocabulary of the output dataset. |
| `transform` | [Transform](#transform-schema) | V |  | Transformation that will be applied to produce new data. |
| `queryInputs` | array([TransformRequestInput](#transformrequestinput-schema)) | V |  | Defines inputs used in this transaction. Slices corresponding to every input dataset must be present. |
| `nextOffset` | `integer` | V | `uint64` | Starting offset to use for new data records. |
| `prevCheckpointPath` | `string` |  | `path` | TODO: This will be removed when coordinator will be speaking to engines purely through Arrow. |
| `newCheckpointPath` | `string` | V | `path` | TODO: This will be removed when coordinator will be speaking to engines purely through Arrow. |
| `newDataPath` | `string` | V | `path` | TODO: This will be removed when coordinator will be speaking to engines purely through Arrow. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/engines/v1alpha1/TransformRequest.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="transformrequestinput-schema"></a>
### TransformRequestInput
Sent as part of the engine transform request operation to describe the input

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `datasetId` | `string` | V | [dataset-id](datasets/identity.md#unique-identifiers) | Unique identifier of the dataset. |
| `datasetAlias` | `string` | V | [dataset-alias](datasets/identity.md#grammar) | Alias of the output dataset, for logging purposes only. |
| `queryAlias` | `string` | V |  | An alias of this input to be used in queries. |
| `vocab` | [DatasetVocabulary](#datasetvocabulary-schema) | V |  | Vocabulary of the input dataset. |
| `offsetInterval` | [OffsetInterval](#offsetinterval-schema) |  |  | Subset of data that goes into this transaction. |
| `dataPaths` | array(`string`) | V |  | TODO: This will be removed when coordinator will be slicing data for the engine. |
| `schemaFile` | `string` | V | `path` | TODO: replace with actual DDL or Parquet schema. |
| `explicitWatermarks` | array([Watermark](#watermark-schema)) | V |  | Watermarks that should be injected into the stream to separate micro batches for reproducibility. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/engines/v1alpha1/TransformRequestInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="transformresponse-schema"></a>
### TransformResponse
Sent by an engine to coordinator when performing the data transformation

| Union Type | Description |
| :---: | --- |
| [TransformResponse::Progress](#transformresponse-progress-schema) | Reports query progress |
| [TransformResponse::Success](#transformresponse-success-schema) | Query executed successfully |
| [TransformResponse::InvalidQuery](#transformresponse-invalidquery-schema) | Query did not pass validation |
| [TransformResponse::InternalError](#transformresponse-internalerror-schema) | Internal error during query execution |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/engines/v1alpha1/TransformResponse.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="transformresponse-internalerror-schema"></a>
### TransformResponse::InternalError
Internal error during query execution

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `message` | `string` | V |  | Brief description of an error |
| `backtrace` | `string` |  |  | Details of an error (e.g. a backtrace) |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/engines/v1alpha1/TransformResponse.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="transformresponse-invalidquery-schema"></a>
### TransformResponse::InvalidQuery
Query did not pass validation

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `message` | `string` | V |  | Explanation of an error |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/engines/v1alpha1/TransformResponse.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="transformresponse-progress-schema"></a>
### TransformResponse::Progress
Reports query progress

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/engines/v1alpha1/TransformResponse.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="transformresponse-success-schema"></a>
### TransformResponse::Success
Query executed successfully

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `newOffsetInterval` | [OffsetInterval](#offsetinterval-schema) |  |  | Data slice produced by the transaction, if any. |
| `newWatermark` | `string` |  | [date-time](https://json-schema.org/draft/2019-09/json-schema-validation.html#rfc.section.7.3.1) | Watermark advanced by the transaction, if any. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/engines/v1alpha1/TransformResponse.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="reference-events"></a>
## events

<a name="eventfilter-schema"></a>
### EventFilter
Filters that work on domain event types and fields.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/events/v1alpha1/EventFilter.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="known-extra-attrs"></a>
#### Known Extensions
| Extension | Description |
| --- | --- |
| `opendatafabric.net/description` | Used for human readable schema field descriptions |
| `opendatafabric.net/type` | An extended set of logical types that ODF recommends but does not require every implementation to support |
| `opendatafabric.org/linkedObjects` | When attached to `AddData` event contains a summary of how many external objects were associated with a certain transaction as well as their size |
| `arrow.apache.org/bufferEncoding` | Used to accurately represent buffer encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/dateEncoding` | Used to accurately represent date encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/decimalEncoding` | Used to accurately represent decimal encoding type when converting Arrow schema to ODF schema |


<a name="known-extra-types"></a>
#### Known Extended Types
| Extended Type | Core Type | Description |
| --- | --- | --- |
| `Did` | `String` | Decentralized identifier `did:<method>:<id>` |
| `Multihash` | `String` | Hash in self-describing [multihash](https://github.com/multiformats/multihash) format |
| `ObjectLink` | `String` | Signifies that the value references an external object. The mandatory `linkType` property defines the type of the link (e.g. `Multihash`). |



<a name="reference-flows"></a>
## flows

<a name="flow-schema"></a>
### Flow
Defines a sequence of tasks to be executed upon certain trigger conditions.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `$schema` | `string` | V |  | Identifies this resource type. |
| `headers` | [ResourceHeadersInput](#resourceheadersinput-schema) | V |  | Container for identity and ownership information of a resource. |
| `spec` | [FlowSpecInput](#flowspecinput-schema) | V |  | Specifies the desired state of the flow. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/Flow.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowrun-schema"></a>
### FlowRun
Defines a set of tasks to be executed in a sequence.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `$schema` | `string` | V |  | Identifies this resource type. |
| `headers` | [ResourceHeadersInput](#resourceheadersinput-schema) | V |  | Container for identity and ownership information of a resource. |
| `spec` | [FlowRunSpecInput](#flowrunspecinput-schema) | V |  | Specifies the desired state of the flow run. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowRun.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowrunactivationcause-schema"></a>
### FlowRunActivationCause
Cause of the flow run activation

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `activationTime` | `string` | V | [date-time](https://json-schema.org/draft/2019-09/json-schema-validation.html#rfc.section.7.3.1) | Time at which the trigger fired. |
| `initiator` | [PrincipalHandle](#principalhandle-schema) |  |  | Principal that initiated the run, if applicable. |
| `trigger` | [FlowTrigger](#flowtrigger-schema) | V |  | Copy of the trigger configuration from the parent Flow that fired. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowRunActivationCause.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowrunactivationcauses-schema"></a>
### FlowRunActivationCauses
Condition capturing what caused this FlowRun to be scheduled. Set by the controller at creation time; never written by users. In case of a retry, the causes of the original run are preserved.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `activationCauses` | array([FlowRunActivationCause](#flowrunactivationcause-schema)) | V |  | Triggers that caused this run to be scheduled. |
| `lateActivationCauses` | array([FlowRunActivationCause](#flowrunactivationcause-schema)) |  |  | Additional triggers that fired while this run was already queued or executing. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowRunActivationCauses.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowrunretry-schema"></a>
### FlowRunRetry
Condition linking this FlowRun to the previous FlowRun it is retrying. Set by the controller; never written by users.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `retryOf` | [ResourceHandle](#resourcehandle-schema) | V |  | Reference to the FlowRun this run is retrying. |
| `retryNumber` | `integer` | V | `uint32` | Retry attempt number (1 = first retry, i.e. second run total). |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowRunRetry.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowrunspec-schema"></a>
### FlowRunSpec
Defines a set of tasks to be executed in a sequence.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `target` | [ResourceHandle](#resourcehandle-schema) |  |  | Defines the default target resources on which tasks will be performed. |
| `serviceAccount` | [PrincipalHandle](#principalhandle-schema) |  |  | Service account whose permissions the tasks of this run execute with. Must be an account of service account type. A run created by the flow controller copies it from the `Flow`. |
| `tasks` | array([TaskSpec](#taskspec-schema)) | V |  | List of tasks to run consecutively. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowRunSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowrunspecinput-schema"></a>
### FlowRunSpecInput
Defines a set of tasks to be executed in a sequence.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `target` | [ResourceRef](#resourceref-schema) |  |  | Defines the default target resources on which tasks will be performed. |
| `serviceAccount` | [PrincipalRef](#principalref-schema) |  |  | Service account whose permissions the tasks of this run execute with. Must be an account of service account type. A run created by the flow controller copies it from the `Flow`. |
| `tasks` | array([TaskSpecInput](#taskspecinput-schema)) | V |  | List of tasks to run consecutively. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowRunSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowrunstatus-schema"></a>
### FlowRunStatus
Condition tracking the overall execution status of a FlowRun and its spawned tasks.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `status` | [FlowRunStatus::Value](#flowrunstatus-value-schema) | V |  | Overall execution status of the FlowRun. |
| `tasks` | array([FlowRunStatus::TaskEntry](#flowrunstatus-taskentry-schema)) |  |  | Tasks spawned by this FlowRun, in execution order. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowRunStatus.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowrunstatus-taskentry-schema"></a>
### FlowRunStatus::TaskEntry
Describes a task spawned by this FlowRun.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` | V |  | Corresponds to the task name in spec.tasks. |
| `task` | [ResourceHandle](#resourcehandle-schema) | V |  | Reference to the spawned Task resource. |
| `status` | [TaskStatus](#taskstatus-schema) | V |  | Current execution phase of this task. |
| `outcome` | [TaskOutcome](#taskoutcome-schema) |  |  | Outcome kind once the task has finished. |
| `lastUpdatedAt` | `string` | V | [date-time](https://json-schema.org/draft/2019-09/json-schema-validation.html#rfc.section.7.3.1) | Last time at which this task's status has been updated. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowRunStatus.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowrunstatus-value-schema"></a>
### FlowRunStatus::Value
Overall execution status of the FlowRun.

| Enum Value |
| :---: |
| Waiting |
| Running |
| Finished |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowRunStatus.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowspec-schema"></a>
### FlowSpec
Defines a sequence of tasks to be executed upon certain trigger conditions.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `target` | [ResourceSelector](#resourceselector-schema) | V |  | Defines resources for which this flow will be instantiated. |
| `serviceAccount` | [PrincipalHandle](#principalhandle-schema) |  |  | Service account whose permissions the tasks of this flow execute with. Must be an account of service account type. Required for flows owned by an organization. When omitted, tasks execute with the permissions of the user account that owns the flow. |
| `triggers` | array([FlowTrigger](#flowtrigger-schema)) | V |  | Conditions that cause this flow to execute. |
| `tasks` | array([TaskSpec](#taskspec-schema)) | V |  | List of tasks to run consecutively. |
| `retryPolicy` | [RetryPolicy](#retrypolicy-schema) |  |  | Defines how a flow should react to failures. |
| `recentBindingsRetention` | [RecentBindingsRetention](#recentbindingsretention-schema) |  |  | Controls retention of `FlowStatus.recentBindings`. |
| `recentRunsRetention` | [RecentRunsRetention](#recentrunsretention-schema) |  |  | Controls retention of `FlowStatus.recentRuns`. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowspecinput-schema"></a>
### FlowSpecInput
Defines a sequence of tasks to be executed upon certain trigger conditions.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `target` | [ResourceSelector](#resourceselector-schema) | V |  | Defines resources for which this flow will be instantiated. |
| `serviceAccount` | [PrincipalRef](#principalref-schema) |  |  | Service account whose permissions the tasks of this flow execute with. Must be an account of service account type. Required for flows owned by an organization. When omitted, tasks execute with the permissions of the user account that owns the flow. |
| `triggers` | array([FlowTriggerInput](#flowtriggerinput-schema)) | V |  | Conditions that cause this flow to execute. |
| `tasks` | array([TaskSpecInput](#taskspecinput-schema)) | V |  | List of tasks to run consecutively. |
| `retryPolicy` | [RetryPolicy](#retrypolicy-schema) |  |  | Defines how a flow should react to failures. |
| `recentBindingsRetention` | [RecentBindingsRetention](#recentbindingsretention-schema) |  |  | Controls retention of `FlowStatus.recentBindings`. |
| `recentRunsRetention` | [RecentRunsRetention](#recentrunsretention-schema) |  |  | Controls retention of `FlowStatus.recentRuns`. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowstatus-schema"></a>
### FlowStatus
Condition tracking the overall status of a Flow and a summary of recent runs.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `status` | [FlowStatus::Value](#flowstatus-value-schema) | V |  | Overall operational status of the Flow. |
| `recentBindings` | array([FlowStatus::BindingEntry](#flowstatus-bindingentry-schema)) |  |  | Last N resources matched by the flow's target selector. Bounded by spec.recentBindingsRetention. |
| `bindingsTotal` | `integer` |  | `uint64` | Total number of resources currently matched by the flow's target selector. |
| `recentRuns` | array([FlowStatus::RunEntry](#flowstatus-runentry-schema)) |  |  | Summaries of recent FlowRuns. Bounded by spec.recentRunsRetention. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowStatus.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowstatus-bindingentry-schema"></a>
### FlowStatus::BindingEntry
A resource currently matched by the flow's target selector.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `target` | [ResourceHandle](#resourcehandle-schema) | V |  | The matched resource. |
| `boundAt` | `string` | V | [date-time](https://json-schema.org/draft/2019-09/json-schema-validation.html#rfc.section.7.3.1) | Time at which this binding was established. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowStatus.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowstatus-runentry-schema"></a>
### FlowStatus::RunEntry
Summary of a recent FlowRun.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `flowRun` | [ResourceHandle](#resourcehandle-schema) | V |  | Reference to the FlowRun resource. |
| `status` | `string` | V |  | Execution status of this run. |
| `outcome` | `string` |  |  | Outcome of this run, if finished. |
| `finishedAt` | `string` |  | [date-time](https://json-schema.org/draft/2019-09/json-schema-validation.html#rfc.section.7.3.1) | Time at which this run finished. |
| `retryOf` | [ResourceHandle](#resourcehandle-schema) |  |  | Reference to the FlowRun this run is retrying, if applicable. |
| `retryNumber` | `integer` |  | `uint32` | Retry attempt number (1 = first retry, i.e. second run total). |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowStatus.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowstatus-value-schema"></a>
### FlowStatus::Value
Overall operational status of the Flow.

| Enum Value |
| :---: |
| Active |
| Paused |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowStatus.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowtrigger-schema"></a>
### FlowTrigger
Condition that causes a flow to be executed.

| Union Type | Description |
| :---: | --- |
| [FlowTrigger::Manual](#flowtrigger-manual-schema) | Triggers the flow via an API call or UI action. |
| [FlowTrigger::Schedule](#flowtrigger-schedule-schema) | Triggers the flow on a cron schedule. |
| [FlowTrigger::Interval](#flowtrigger-interval-schema) | Triggers the flow at regular intervals. |
| [FlowTrigger::Event](#flowtrigger-event-schema) | Triggers the flow when an event bus event matching one of the filters is observed. |
| [FlowTrigger::Source](#flowtrigger-source-schema) | Triggers the flow when a source receives new data, with optional batching controls. |
| [FlowTrigger::InputsUpdated](#flowtrigger-inputsupdated-schema) | Triggers the flow when inputs of a derivative dataset receive new data, with optional batching controls. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowTrigger.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowtrigger-event-schema"></a>
### FlowTrigger::Event
Triggers the flow when an event bus event matching one of the filters is observed.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `enabled` | `boolean` |  |  | Allows to pause an individual trigger.<br/><br/>Default: true |
| `events` | [EventFilter](#eventfilter-schema) | V |  | Filters the event by type and fields. |
| `cooldown` | `string` |  | [duration](https://docs.rs/duration-string/latest/duration_string/) | Minimum interval between activations of the trigger. If the trigger fires again within the interval, the activation is postponed until the interval ends, batching multiple activations into one flow run. |
| `cooldownMaxBatch` | `integer` |  | `uint64` | If the trigger is activated this many times during the `cooldown` interval, it fires without waiting for the interval to end. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowTrigger.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowtrigger-inputsupdated-schema"></a>
### FlowTrigger::InputsUpdated
Triggers the flow when inputs of a derivative dataset receive new data, with optional batching controls.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `enabled` | `boolean` |  |  | Allows to pause an individual trigger.<br/><br/>Default: true |
| `cooldown` | `string` |  | [duration](https://docs.rs/duration-string/latest/duration_string/) | Minimum interval between activations of the trigger. If the trigger fires again within the interval, the activation is postponed until the interval ends, batching multiple activations into one flow run. |
| `cooldownMaxBatch` | `integer` |  | `uint64` | If the trigger is activated this many times during the `cooldown` interval, it fires without waiting for the interval to end. |
| `target` | [DatasetSelector](#datasetselector-schema) |  |  | Selector of derivative datasets whose inputs are watched. Defaults to the flow-level target when omitted. |
| `minRecordsToAwait` | `integer` |  | `uint64` | Minimum number of new input records to accumulate before triggering. |
| `maxAwaitInterval` | `string` |  | [duration](https://docs.rs/duration-string/latest/duration_string/) | Maximum time to wait for `minRecordsToAwait` before triggering anyway e.g. `1h`. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowTrigger.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowtrigger-interval-schema"></a>
### FlowTrigger::Interval
Triggers the flow at regular intervals.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `enabled` | `boolean` |  |  | Allows to pause an individual trigger.<br/><br/>Default: true |
| `cooldown` | `string` |  | [duration](https://docs.rs/duration-string/latest/duration_string/) | Minimum interval between activations of the trigger. If the trigger fires again within the interval, the activation is postponed until the interval ends, batching multiple activations into one flow run. |
| `cooldownMaxBatch` | `integer` |  | `uint64` | If the trigger is activated this many times during the `cooldown` interval, it fires without waiting for the interval to end. |
| `interval` | `string` | V | [duration](https://docs.rs/duration-string/latest/duration_string/) | Interval between activations e.g. `15m`. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowTrigger.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowtrigger-manual-schema"></a>
### FlowTrigger::Manual
Triggers the flow via an API call or UI action.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `enabled` | `boolean` |  |  | Allows to pause an individual trigger.<br/><br/>Default: true |
| `cooldown` | `string` |  | [duration](https://docs.rs/duration-string/latest/duration_string/) | Minimum interval between activations of the trigger. If the trigger fires again within the interval, the activation is postponed until the interval ends, batching multiple activations into one flow run. |
| `cooldownMaxBatch` | `integer` |  | `uint64` | If the trigger is activated this many times during the `cooldown` interval, it fires without waiting for the interval to end. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowTrigger.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowtrigger-schedule-schema"></a>
### FlowTrigger::Schedule
Triggers the flow on a cron schedule.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `enabled` | `boolean` |  |  | Allows to pause an individual trigger.<br/><br/>Default: true |
| `cooldown` | `string` |  | [duration](https://docs.rs/duration-string/latest/duration_string/) | Minimum interval between activations of the trigger. If the trigger fires again within the interval, the activation is postponed until the interval ends, batching multiple activations into one flow run. |
| `cooldownMaxBatch` | `integer` |  | `uint64` | If the trigger is activated this many times during the `cooldown` interval, it fires without waiting for the interval to end. |
| `cron` | `string` | V |  | Cron5 expression defining the schedule e.g. `@daily` or `*/30 * * * *`. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowTrigger.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowtrigger-source-schema"></a>
### FlowTrigger::Source
Triggers the flow when a source receives new data, with optional batching controls.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `enabled` | `boolean` |  |  | Allows to pause an individual trigger.<br/><br/>Default: true |
| `cooldown` | `string` |  | [duration](https://docs.rs/duration-string/latest/duration_string/) | Minimum interval between activations of the trigger. If the trigger fires again within the interval, the activation is postponed until the interval ends, batching multiple activations into one flow run. |
| `cooldownMaxBatch` | `integer` |  | `uint64` | If the trigger is activated this many times during the `cooldown` interval, it fires without waiting for the interval to end. |
| `source` | [ResourceHandle](#resourcehandle-schema) | V |  | Reference to the source resource that drives this trigger. |
| `minRecordsToAwait` | `integer` |  | `uint64` | Minimum number of new records to accumulate before triggering. |
| `maxAwaitInterval` | `string` |  | [duration](https://docs.rs/duration-string/latest/duration_string/) | Maximum time to wait for `minRecordsToAwait` before triggering anyway e.g. `1h`. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowTrigger.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowtriggerinput-schema"></a>
### FlowTriggerInput
Condition that causes a flow to be executed.

| Union Type | Description |
| :---: | --- |
| [FlowTriggerInput::Manual](#flowtriggerinput-manual-schema) | Triggers the flow via an API call or UI action. |
| [FlowTriggerInput::Schedule](#flowtriggerinput-schedule-schema) | Triggers the flow on a cron schedule. |
| [FlowTriggerInput::Interval](#flowtriggerinput-interval-schema) | Triggers the flow at regular intervals. |
| [FlowTriggerInput::Event](#flowtriggerinput-event-schema) | Triggers the flow when an event bus event matching one of the filters is observed. |
| [FlowTriggerInput::Source](#flowtriggerinput-source-schema) | Triggers the flow when a source receives new data, with optional batching controls. |
| [FlowTriggerInput::InputsUpdated](#flowtriggerinput-inputsupdated-schema) | Triggers the flow when inputs of a derivative dataset receive new data, with optional batching controls. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowTriggerInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowtriggerinput-event-schema"></a>
### FlowTriggerInput::Event
Triggers the flow when an event bus event matching one of the filters is observed.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `enabled` | `boolean` |  |  | Allows to pause an individual trigger.<br/><br/>Default: true |
| `events` | [EventFilter](#eventfilter-schema) | V |  | Filters the event by type and fields. |
| `cooldown` | `string` |  | [duration](https://docs.rs/duration-string/latest/duration_string/) | Minimum interval between activations of the trigger. If the trigger fires again within the interval, the activation is postponed until the interval ends, batching multiple activations into one flow run. |
| `cooldownMaxBatch` | `integer` |  | `uint64` | If the trigger is activated this many times during the `cooldown` interval, it fires without waiting for the interval to end. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowTriggerInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowtriggerinput-inputsupdated-schema"></a>
### FlowTriggerInput::InputsUpdated
Triggers the flow when inputs of a derivative dataset receive new data, with optional batching controls.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `enabled` | `boolean` |  |  | Allows to pause an individual trigger.<br/><br/>Default: true |
| `cooldown` | `string` |  | [duration](https://docs.rs/duration-string/latest/duration_string/) | Minimum interval between activations of the trigger. If the trigger fires again within the interval, the activation is postponed until the interval ends, batching multiple activations into one flow run. |
| `cooldownMaxBatch` | `integer` |  | `uint64` | If the trigger is activated this many times during the `cooldown` interval, it fires without waiting for the interval to end. |
| `target` | [DatasetSelector](#datasetselector-schema) |  |  | Selector of derivative datasets whose inputs are watched. Defaults to the flow-level target when omitted. |
| `minRecordsToAwait` | `integer` |  | `uint64` | Minimum number of new input records to accumulate before triggering. |
| `maxAwaitInterval` | `string` |  | [duration](https://docs.rs/duration-string/latest/duration_string/) | Maximum time to wait for `minRecordsToAwait` before triggering anyway e.g. `1h`. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowTriggerInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowtriggerinput-interval-schema"></a>
### FlowTriggerInput::Interval
Triggers the flow at regular intervals.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `enabled` | `boolean` |  |  | Allows to pause an individual trigger.<br/><br/>Default: true |
| `cooldown` | `string` |  | [duration](https://docs.rs/duration-string/latest/duration_string/) | Minimum interval between activations of the trigger. If the trigger fires again within the interval, the activation is postponed until the interval ends, batching multiple activations into one flow run. |
| `cooldownMaxBatch` | `integer` |  | `uint64` | If the trigger is activated this many times during the `cooldown` interval, it fires without waiting for the interval to end. |
| `interval` | `string` | V | [duration](https://docs.rs/duration-string/latest/duration_string/) | Interval between activations e.g. `15m`. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowTriggerInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowtriggerinput-manual-schema"></a>
### FlowTriggerInput::Manual
Triggers the flow via an API call or UI action.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `enabled` | `boolean` |  |  | Allows to pause an individual trigger.<br/><br/>Default: true |
| `cooldown` | `string` |  | [duration](https://docs.rs/duration-string/latest/duration_string/) | Minimum interval between activations of the trigger. If the trigger fires again within the interval, the activation is postponed until the interval ends, batching multiple activations into one flow run. |
| `cooldownMaxBatch` | `integer` |  | `uint64` | If the trigger is activated this many times during the `cooldown` interval, it fires without waiting for the interval to end. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowTriggerInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowtriggerinput-schedule-schema"></a>
### FlowTriggerInput::Schedule
Triggers the flow on a cron schedule.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `enabled` | `boolean` |  |  | Allows to pause an individual trigger.<br/><br/>Default: true |
| `cooldown` | `string` |  | [duration](https://docs.rs/duration-string/latest/duration_string/) | Minimum interval between activations of the trigger. If the trigger fires again within the interval, the activation is postponed until the interval ends, batching multiple activations into one flow run. |
| `cooldownMaxBatch` | `integer` |  | `uint64` | If the trigger is activated this many times during the `cooldown` interval, it fires without waiting for the interval to end. |
| `cron` | `string` | V |  | Cron5 expression defining the schedule e.g. `@daily` or `*/30 * * * *`. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowTriggerInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="flowtriggerinput-source-schema"></a>
### FlowTriggerInput::Source
Triggers the flow when a source receives new data, with optional batching controls.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `enabled` | `boolean` |  |  | Allows to pause an individual trigger.<br/><br/>Default: true |
| `cooldown` | `string` |  | [duration](https://docs.rs/duration-string/latest/duration_string/) | Minimum interval between activations of the trigger. If the trigger fires again within the interval, the activation is postponed until the interval ends, batching multiple activations into one flow run. |
| `cooldownMaxBatch` | `integer` |  | `uint64` | If the trigger is activated this many times during the `cooldown` interval, it fires without waiting for the interval to end. |
| `source` | [ResourceRef](#resourceref-schema) | V |  | Reference to the source resource that drives this trigger. |
| `minRecordsToAwait` | `integer` |  | `uint64` | Minimum number of new records to accumulate before triggering. |
| `maxAwaitInterval` | `string` |  | [duration](https://docs.rs/duration-string/latest/duration_string/) | Maximum time to wait for `minRecordsToAwait` before triggering anyway e.g. `1h`. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/FlowTriggerInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="recentbindingsretention-schema"></a>
### RecentBindingsRetention
Controls how many recent bindings are reported in `FlowStatus.recentBindings`.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `maxBindings` | `integer` |  | `uint32` | Maximum number of recent bindings to report. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/RecentBindingsRetention.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="recentrunsretention-schema"></a>
### RecentRunsRetention
Controls how many recent runs are reported in `FlowStatus.recentRuns`. Implementations may additionally cap these values.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `maxRuns` | `integer` |  | `uint32` | Maximum number of recent runs to report. |
| `maxAge` | `string` |  | [duration](https://docs.rs/duration-string/latest/duration_string/) | Maximum age of a finished run to be reported e.g. `30d`. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/RecentRunsRetention.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="retrybackoff-schema"></a>
### RetryBackoff
Type of the backoff scaling.

| Enum Value |
| :---: |
| Linear |
| Exponential |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/RetryBackoff.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="retrypolicy-schema"></a>
### RetryPolicy
Defines how a flow should react to failures.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `maxAttempts` | `integer` |  | `uint32` | Number of attempts before flow auto-scheduling will be disabled. |
| `minDelay` | `string` |  | [duration](https://docs.rs/duration-string/latest/duration_string/) | How long to wait until the first retry. |
| `backoff` | [RetryBackoff](#retrybackoff-schema) |  |  | Type of the backoff scaling. When omitted, retries use a constant `minDelay`. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/flows/v1alpha1/RetryPolicy.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="reference-legacy"></a>
## legacy

<a name="addpushsource-schema"></a>
### AddPushSource
Describes how to ingest data into a root dataset from a certain logical source.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `sourceName` | `string` | V |  | Identifies the source within this dataset. |
| `read` | [ReadStep](#readstep-schema) | V |  | Defines how data is read into structured format. |
| `preprocess` | [Transform](#transform-schema) |  |  | Pre-processing query that shapes the data. |
| `merge` | [MergeStrategy](#mergestrategy-schema) | V |  | Determines how newly-ingested data should be merged with existing history. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/legacy/v0/AddPushSource.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="datasetsnapshot-schema"></a>
### DatasetSnapshot
Represents a projection of the dataset metadata at a single point in time.
This type is typically used for defining new datasets and changing the existing ones.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` | V | [dataset-alias](datasets/identity.md#grammar) | Alias of the dataset. |
| `kind` | [DatasetKind](#datasetkind-schema) | V |  | Type of the dataset. |
| `metadata` | array([MetadataEvent](#metadataevent-schema)) | V |  | An array of metadata events that will be used to populate the chain. Here you can define polling and push sources, set licenses, add attachments etc. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/legacy/v0/DatasetSnapshot.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="disablepollingsource-schema"></a>
### DisablePollingSource
Disables the previously defined polling source.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/legacy/v0/DisablePollingSource.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="disablepushsource-schema"></a>
### DisablePushSource
Disables the previously defined source.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `sourceName` | `string` | V |  | Identifies the source to be disabled. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/legacy/v0/DisablePushSource.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="fetchstep-schema"></a>
### FetchStep
Defines the external source of data.

| Union Type | Description |
| :---: | --- |
| [FetchStep::Url](#fetchstep-url-schema) | Pulls data from one of the supported sources by its URL. |
| [FetchStep::FilesGlob](#fetchstep-filesglob-schema) | Uses glob operator to match files on the local file system. |
| [FetchStep::Container](#fetchstep-container-schema) | Runs the specified OCI container to fetch data from an arbitrary source. |
| [FetchStep::Mqtt](#fetchstep-mqtt-schema) | Connects to an MQTT broker to fetch events from the specified topic. |
| [FetchStep::EthereumLogs](#fetchstep-ethereumlogs-schema) | Connects to an Ethereum node to stream transaction logs. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/legacy/v0/FetchStep.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="fetchstep-container-schema"></a>
### FetchStep::Container
Runs the specified OCI container to fetch data from an arbitrary source.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `image` | `string` | V |  | Image name and and an optional tag. |
| `command` | array(`string`) |  |  | Specifies the entrypoint. Not executed within a shell. The default OCI image's ENTRYPOINT is used if this is not provided. |
| `args` | array(`string`) |  |  | Arguments to the entrypoint. The OCI image's CMD is used if this is not provided. |
| `env` | array([EnvVar](#envvar-schema)) |  |  | Environment variables to propagate into or set in the container. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/legacy/v0/FetchStep.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="fetchstep-ethereumlogs-schema"></a>
### FetchStep::EthereumLogs
Connects to an Ethereum node to stream transaction logs.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `chainId` | `integer` |  | `uint64` | Identifier of the chain to scan logs from. This parameter may be used for RPC endpoint lookup as well as asserting that provided `nodeUrl` corresponds to the expected chain. |
| `nodeUrl` | `string` |  | `url` | Url of the node. |
| `filter` | `string` |  |  | An SQL WHERE clause that can be used to pre-filter the logs before fetching them from the ETH node. |
| `signature` | `string` |  |  | Solidity log event signature to use for decoding. Using this field adds `event` to the output containing decoded log as JSON. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/legacy/v0/FetchStep.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="fetchstep-filesglob-schema"></a>
### FetchStep::FilesGlob
Uses glob operator to match files on the local file system.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `path` | `string` | V |  | Path with a glob pattern. |
| `eventTime` | [EventTimeSource](#eventtimesource-schema) |  |  | Describes how event time is extracted from the source metadata. |
| `cache` | [SourceCaching](#sourcecaching-schema) |  |  | Describes the caching settings used for this source. |
| `order` | [SourceOrdering](#sourceordering-schema) |  |  | Specifies how input files should be ordered before ingestion.<br/>Order is important as every file will be processed individually<br/>and will advance the dataset's watermark. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/legacy/v0/FetchStep.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="fetchstep-mqtt-schema"></a>
### FetchStep::Mqtt
Connects to an MQTT broker to fetch events from the specified topic.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `host` | `string` | V |  | Hostname of the MQTT broker. |
| `port` | `integer` | V | `int32` | Port of the MQTT broker. |
| `username` | `string` |  |  | Username to use for auth with the broker. |
| `password` | `string` |  |  | Password to use for auth with the broker (can be templated). |
| `topics` | array([MqttTopicSubscription](#mqtttopicsubscription-schema)) | V |  | List of topic subscription parameters. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/legacy/v0/FetchStep.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="fetchstep-url-schema"></a>
### FetchStep::Url
Pulls data from one of the supported sources by its URL.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `url` | `string` | V | `url` | URL of the data source |
| `eventTime` | [EventTimeSource](#eventtimesource-schema) |  |  | Describes how event time is extracted from the source metadata. |
| `cache` | [SourceCaching](#sourcecaching-schema) |  |  | Describes the caching settings used for this source. |
| `headers` | array([RequestHeader](#requestheader-schema)) |  |  | Headers to pass during the request (e.g. HTTP Authorization) |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/legacy/v0/FetchStep.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="manifest-schema"></a>
### Manifest
An object that wraps the metadata resources providing versioning and type identification. All root-level resources are wrapped with a manifest when serialized to disk.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `kind` | `string` | V | [multicodec](https://github.com/multiformats/multicodec) | Type of the resource. |
| `version` | `integer` | V | `int32` | Major version number of the resource contained in this manifest. It provides the mechanism for introducing compatibility breaking changes. |
| `content` | `object` | V | `generic` | Resource data. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/legacy/v0/Manifest.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="setpollingsource-schema"></a>
### SetPollingSource
Contains information on how externally-hosted data can be ingested into the root dataset.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `fetch` | [FetchStep](#fetchstep-schema) | V |  | Determines where data is sourced from. |
| `prepare` | array([PrepStep](#prepstep-schema)) |  |  | Defines how raw data is prepared before reading. |
| `read` | [ReadStep](#readstep-schema) | V |  | Defines how data is read into structured format. |
| `preprocess` | [Transform](#transform-schema) |  |  | Pre-processing query that shapes the data. |
| `merge` | [MergeStrategy](#mergestrategy-schema) | V |  | Determines how newly-ingested data should be merged with existing history. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/legacy/v0/SetPollingSource.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="reference-resources"></a>
## resources

<a name="addlabel-schema"></a>
### AddLabel
Apply a specific label to the resource.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `labelType` | `string` | V |  | Schema URI of the label being added. |
| `labelValue` | `any` | V |  | Value of the label being added. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/resources/v1alpha1/actions/AddLabel.json)
[^](#table-of-contents)


<a name="create-schema"></a>
### Create
Create a new resource instance of a type (object is a type URI) under an account.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `targetOwner` | [ResourceOwnerHandle](#resourceownerhandle-schema) | V |  | Account under which the resource is being created. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/resources/v1alpha1/actions/Create.json)
[^](#table-of-contents)


<a name="delete-schema"></a>
### Delete
Delete the resource instance.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/resources/v1alpha1/actions/Delete.json)
[^](#table-of-contents)


<a name="edit-schema"></a>
### Edit
Modify the resource manifest.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/resources/v1alpha1/actions/Edit.json)
[^](#table-of-contents)


<a name="labelfilter-schema"></a>
### LabelFilter
Filters that work on resource labels and identity headers.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/resources/v1alpha1/LabelFilter.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="known-extra-attrs"></a>
#### Known Extensions
| Extension | Description |
| --- | --- |
| `opendatafabric.net/description` | Used for human readable schema field descriptions |
| `opendatafabric.net/type` | An extended set of logical types that ODF recommends but does not require every implementation to support |
| `opendatafabric.org/linkedObjects` | When attached to `AddData` event contains a summary of how many external objects were associated with a certain transaction as well as their size |
| `arrow.apache.org/bufferEncoding` | Used to accurately represent buffer encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/dateEncoding` | Used to accurately represent date encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/decimalEncoding` | Used to accurately represent decimal encoding type when converting Arrow schema to ODF schema |


<a name="known-extra-types"></a>
#### Known Extended Types
| Extended Type | Core Type | Description |
| --- | --- | --- |
| `Did` | `String` | Decentralized identifier `did:<method>:<id>` |
| `Multihash` | `String` | Hash in self-describing [multihash](https://github.com/multiformats/multihash) format |
| `ObjectLink` | `String` | Signifies that the value references an external object. The mandatory `linkType` property defines the type of the link (e.g. `Multihash`). |



<a name="list-schema"></a>
### List
List resource instances of a type (object is a type URI) under an account.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `targetOwner` | [ResourceOwnerHandle](#resourceownerhandle-schema) | V |  | Account under which resources are being listed. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/resources/v1alpha1/actions/List.json)
[^](#table-of-contents)


<a name="reconciliationerror-schema"></a>
### ReconciliationError
Condition describing why the controller failed to reconcile the latest generation of a resource. Set by the controller; never written by users.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `code` | `string` | V |  | Machine-readable code identifying the type of the error. |
| `message` | `string` | V |  | Human-readable description of the error. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/resources/v1alpha1/ReconciliationError.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="removelabel-schema"></a>
### RemoveLabel
Remove a specific label from the resource.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `labelType` | `string` | V |  | Schema URI of the label being removed. |
| `labelValue` | `any` | V |  | Existing value of the label being removed. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/resources/v1alpha1/actions/RemoveLabel.json)
[^](#table-of-contents)


<a name="resource-schema"></a>
### Resource
Top-level container for canonical representation of a resource that specifies the type and version of the resource, carries identity, ownership, and status information.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `$schema` | `string` | V |  | Identifies the controlling entity, a bounded context that this resource belongs to, and the version. Url should follow the pattern `{base-url}/{context}/{version}/{name}` e.g. `https://opendatafabric.org/schemas/datasets/v1/Dataset`. |
| `headers` | [ResourceHeaders](#resourceheaders-schema) | V |  | Container for identity and ownership information of a resource. |
| `spec` | `object` | V | `generic` | Specifies the desired state of a resource. |
| `status` | [ResourceStatus](#resourcestatus-schema) | V |  | Resource lifecycle and reconciliation information. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/resources/v1alpha1/Resource.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="resourceannotations-schema"></a>
### ResourceAnnotations
Annotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. Unlike labels, annotations are not indexed and cannot be queried by.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/resources/v1alpha1/ResourceAnnotations.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="known-extra-attrs"></a>
#### Known Extensions
| Extension | Description |
| --- | --- |
| `opendatafabric.net/description` | Used for human readable schema field descriptions |
| `opendatafabric.net/type` | An extended set of logical types that ODF recommends but does not require every implementation to support |
| `opendatafabric.org/linkedObjects` | When attached to `AddData` event contains a summary of how many external objects were associated with a certain transaction as well as their size |
| `arrow.apache.org/bufferEncoding` | Used to accurately represent buffer encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/dateEncoding` | Used to accurately represent date encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/decimalEncoding` | Used to accurately represent decimal encoding type when converting Arrow schema to ODF schema |


<a name="known-extra-types"></a>
#### Known Extended Types
| Extended Type | Core Type | Description |
| --- | --- | --- |
| `Did` | `String` | Decentralized identifier `did:<method>:<id>` |
| `Multihash` | `String` | Hash in self-describing [multihash](https://github.com/multiformats/multihash) format |
| `ObjectLink` | `String` | Signifies that the value references an external object. The mandatory `linkType` property defines the type of the link (e.g. `Multihash`). |



<a name="resourceconditions-schema"></a>
### ResourceConditions
Container of generic contditions that can be added by contollers to provide additional information about the state of a resource. Keys uniquely identify the condition and should be in the form of a schema URL describing this condition, e.g. `https://opendatafabric.org/schemas/tasks/v1/TaskStatus`.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/resources/v1alpha1/ResourceConditions.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="known-extra-attrs"></a>
#### Known Extensions
| Extension | Description |
| --- | --- |
| `opendatafabric.net/description` | Used for human readable schema field descriptions |
| `opendatafabric.net/type` | An extended set of logical types that ODF recommends but does not require every implementation to support |
| `opendatafabric.org/linkedObjects` | When attached to `AddData` event contains a summary of how many external objects were associated with a certain transaction as well as their size |
| `arrow.apache.org/bufferEncoding` | Used to accurately represent buffer encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/dateEncoding` | Used to accurately represent date encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/decimalEncoding` | Used to accurately represent decimal encoding type when converting Arrow schema to ODF schema |


<a name="known-extra-types"></a>
#### Known Extended Types
| Extended Type | Core Type | Description |
| --- | --- | --- |
| `Did` | `String` | Decentralized identifier `did:<method>:<id>` |
| `Multihash` | `String` | Hash in self-describing [multihash](https://github.com/multiformats/multihash) format |
| `ObjectLink` | `String` | Signifies that the value references an external object. The mandatory `linkType` property defines the type of the link (e.g. `Multihash`). |



<a name="resourcehandle-schema"></a>
### ResourceHandle
Link to another resolved resource.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `account` | [ResourceOwnerHandle](#resourceownerhandle-schema) | V |  | Account that owns the target resource. |
| `type` | `string` | V |  | Type URI of the target resource. |
| `id` | `string` | V |  | ID of the resource within a node. |
| `did` | `string` |  | `did` | DID of the resource, if applicable. |
| `name` | `string` | V |  | Name of a resource. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/resources/v1alpha1/ResourceHandle.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="resourceheaders-schema"></a>
### ResourceHeaders
Container for identity and ownership information of a resource.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `id` | `string` | V |  | Unique identifier of a resource within entire ODF node. Automatically assigned upon resource creation. |
| `name` | `string` | V |  | Symbolic name of a resource that identifies it within a scope of an onwing account. |
| `account` | [ResourceOwnerHandle](#resourceownerhandle-schema) | V |  | Link to the account that owns the resource. |
| `labels` | [ResourceLabels](#resourcelabels-schema) | V |  | Map of string keys and values that can be used to organize, categorize, and query resources. |
| `annotations` | [ResourceAnnotations](#resourceannotations-schema) | V |  | Annotations is a key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. Unlike labels, annotations are not indexed and cannot be queried by. |
| `ownerReferences` | array([ResourceHandle](#resourcehandle-schema)) |  |  | References to resources that created this resource. Used for lineage tracking and cascading cleanup. |
| `generation` | `integer` | V | `uint64` | A sequential number that changes every time the resource header and spec are updated. Does not increment on status changes, thus signifying changes to the desired state. Populated by the system. Starts with `1`. |
| `statusGeneration` | `integer` | V | `uint64` | A sequential number that changes every time the resource status is updated. Can be passed back as a precondition of a status update to make sure the status was not modified since it was read. Populated by the system. Starts with `1`. |
| `createdAt` | `string` | V | [date-time](https://json-schema.org/draft/2019-09/json-schema-validation.html#rfc.section.7.3.1) | Time when the resource was first applied and assigned an identity. |
| `updatedAt` | `string` | V | [date-time](https://json-schema.org/draft/2019-09/json-schema-validation.html#rfc.section.7.3.1) | Time when the resource was last updated, including header, spec, and status updates. |
| `deletionRequestedAt` | `string` |  | [date-time](https://json-schema.org/draft/2019-09/json-schema-validation.html#rfc.section.7.3.1) | Time when deletion of the resource was requested. The resource remains visible until its controllers finish cleaning up and the deletion completes. |
| `deletedAt` | `string` |  | [date-time](https://json-schema.org/draft/2019-09/json-schema-validation.html#rfc.section.7.3.1) | Time when the resource was deleted. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/resources/v1alpha1/ResourceHeaders.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="resourceheadersinput-schema"></a>
### ResourceHeadersInput
Container for identity and ownership information of a resource.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `id` | `string` |  |  | Unique identifier of a resource within entire ODF node. Automatically assigned upon resource creation. |
| `name` | `string` | V |  | Symbolic name of a resource that identifies it within a scope of an onwing account. |
| `account` | [ResourceOwnerRef](#resourceownerref-schema) |  |  | Reference to the account that owns the resource. |
| `labels` | [ResourceLabels](#resourcelabels-schema) |  |  | Map of string keys and values that can be used to organize, categorize, and query resources. |
| `annotations` | [ResourceAnnotations](#resourceannotations-schema) |  |  | Annotations is a key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. Unlike labels, annotations are not indexed and cannot be queried by. |
| `ownerReferences` | array([ResourceRef](#resourceref-schema)) |  |  | References to resources that created this resource. Used for lineage tracking and cascading cleanup. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/resources/v1alpha1/ResourceHeadersInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="resourceinput-schema"></a>
### ResourceInput
Top-level container for user-authored representation of a resource that specifies the type and version of the resource and its desired state.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `$schema` | `string` | V |  | Identifies the controlling entity, a bounded context that this resource belongs to, and the version. Url should follow the pattern `{base-url}/{context}/{version}/{name}` e.g. `https://opendatafabric.org/schemas/datasets/v1/Dataset`. |
| `headers` | [ResourceHeadersInput](#resourceheadersinput-schema) | V |  | Container for identity and ownership information of a resource. |
| `spec` | `object` | V | `generic` | Specifies the desired state of a resource. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/resources/v1alpha1/ResourceInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="resourcelabels-schema"></a>
### ResourceLabels
Map of string keys and values that can be used to organize, categorize, and query resources.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/resources/v1alpha1/ResourceLabels.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="known-extra-attrs"></a>
#### Known Extensions
| Extension | Description |
| --- | --- |
| `opendatafabric.net/description` | Used for human readable schema field descriptions |
| `opendatafabric.net/type` | An extended set of logical types that ODF recommends but does not require every implementation to support |
| `opendatafabric.org/linkedObjects` | When attached to `AddData` event contains a summary of how many external objects were associated with a certain transaction as well as their size |
| `arrow.apache.org/bufferEncoding` | Used to accurately represent buffer encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/dateEncoding` | Used to accurately represent date encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/decimalEncoding` | Used to accurately represent decimal encoding type when converting Arrow schema to ODF schema |


<a name="known-extra-types"></a>
#### Known Extended Types
| Extended Type | Core Type | Description |
| --- | --- | --- |
| `Did` | `String` | Decentralized identifier `did:<method>:<id>` |
| `Multihash` | `String` | Hash in self-describing [multihash](https://github.com/multiformats/multihash) format |
| `ObjectLink` | `String` | Signifies that the value references an external object. The mandatory `linkType` property defines the type of the link (e.g. `Multihash`). |



<a name="resourcephase-schema"></a>
### ResourcePhase
Represents the reconciliation or deletion phase of a resource.

| Enum Value |
| :---: |
| Pending |
| Reconciling |
| Ready |
| Degraded |
| Failed |
| Deleting |
| Deleted |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/resources/v1alpha1/ResourcePhase.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="resourceref-schema"></a>
### ResourceRef
Reference to another resource.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `account` | [ResourceOwnerRef](#resourceownerref-schema) |  |  | Reference to an account that owns the target resource. |
| `id` | `string` |  |  | ID of the resource within a node. |
| `did` | `string` |  | `did` | DID of the resource. |
| `type` | `string` |  |  | Short type name or full type URI of the target resource. |
| `name` | `string` |  |  | Name of a resource. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/resources/v1alpha1/ResourceRef.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="resourceselector-schema"></a>
### ResourceSelector
Matches zero or many resources using identity and label filters.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `account` | [ResourceOwnerRef](#resourceownerref-schema) |  |  | Reference to an account that owns the target resources. |
| `id` | `string` |  |  | ID of the singular resource. |
| `did` | `string` |  | `did` | DID of the resource. |
| `type` | `string` |  |  | Short type name of the target resource e.g. `SecretSet` or a full schema URI e.g. `https://opendatafabric.org/schemas/config/v1alpha1/SecretSet`. |
| `name` | `string` |  |  | Name pattern in SQL `LIKE` format. |
| `labels` | [LabelFilter](#labelfilter-schema) |  |  | Filter by resource labels. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/resources/v1alpha1/ResourceSelector.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="resourcestatus-schema"></a>
### ResourceStatus
Resource lifecycle and reconciliation information.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `phase` | [ResourcePhase](#resourcephase-schema) | V |  | Represents the reconciliation phase of a resource as seen by the main resource controller. |
| `observedGeneration` | `integer` |  | `uint64` | Resource generation that was last seen by the main resource controller. |
| `observedAt` | `string` |  | [date-time](https://json-schema.org/draft/2019-09/json-schema-validation.html#rfc.section.7.3.1) | Time when the controller seen the resource state as defined in `observedGeneration`. |
| `reconciledGeneration` | `integer` |  | `uint64` | Resource generation that was last successfully reconciled by the main resource controller. |
| `reconciledAt` | `string` |  | [date-time](https://json-schema.org/draft/2019-09/json-schema-validation.html#rfc.section.7.3.1) | Time when the controller last reconciled the desired resource state as defined in `reconciledGeneration`. |
| `conditions` | [ResourceConditions](#resourceconditions-schema) | V |  | Detailed conditions describing the state of the resource that are added by controllers. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/resources/v1alpha1/ResourceStatus.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="view-schema"></a>
### View
Read the resource manifest.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/resources/v1alpha1/actions/View.json)
[^](#table-of-contents)


<a name="reference-sinks"></a>
## sinks

<a name="webhookendpoint-schema"></a>
### WebhookEndpoint
Defines a webhook endpoint that can receive event notifications and data.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `$schema` | `string` | V |  | Identifies this resource type. |
| `headers` | [ResourceHeadersInput](#resourceheadersinput-schema) | V |  | Container for identity and ownership information of a resource. |
| `spec` | [WebhookEndpointSpecInput](#webhookendpointspecinput-schema) | V |  | Specifies the desired state of the resource. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sinks/v1alpha1/WebhookEndpoint.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="webhookendpointspec-schema"></a>
### WebhookEndpointSpec
Defines a webhook endpoint that can receive event notifications and data.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `url` | `string` | V | `url` | URL of the webhook endpoint. |
| `secret` | [Secret](#secret-schema) |  |  | Shared secret used for HMAC signature of the request payload for authentication. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sinks/v1alpha1/WebhookEndpointSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="webhookendpointspecinput-schema"></a>
### WebhookEndpointSpecInput
Defines a webhook endpoint that can receive event notifications and data.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `url` | `string` | V | `url` | URL of the webhook endpoint. |
| `secret` | [Secret](#secret-schema) |  |  | Shared secret used for HMAC signature of the request payload for authentication. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sinks/v1alpha1/WebhookEndpointSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="reference-sources"></a>
## sources

<a name="compressionformat-schema"></a>
### CompressionFormat
Defines a compression algorithm.

| Enum Value |
| :---: |
| Gzip |
| Zip |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/CompressionFormat.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="envvar-schema"></a>
### EnvVar
Defines an environment variable passed into some job.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` | V |  | Name of the variable. |
| `value` | `string` |  |  | Value of the variable. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/EnvVar.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="eventtimesource-schema"></a>
### EventTimeSource
Defines the external source of data.

| Union Type | Description |
| :---: | --- |
| [EventTimeSource::FromMetadata](#eventtimesource-frommetadata-schema) | Extracts event time from the source's metadata. |
| [EventTimeSource::FromPath](#eventtimesource-frompath-schema) | Extracts event time from the path component of the source. |
| [EventTimeSource::FromSystemTime](#eventtimesource-fromsystemtime-schema) | Assigns event time from the system time source. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/EventTimeSource.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="eventtimesource-frommetadata-schema"></a>
### EventTimeSource::FromMetadata
Extracts event time from the source's metadata.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/EventTimeSource.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="eventtimesource-frompath-schema"></a>
### EventTimeSource::FromPath
Extracts event time from the path component of the source.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `pattern` | `string` | V | `regex` | Regular expression where first group contains the timestamp string. |
| `timestampFormat` | `string` |  |  | Format of the expected timestamp in java.text.SimpleDateFormat form. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/EventTimeSource.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="eventtimesource-fromsystemtime-schema"></a>
### EventTimeSource::FromSystemTime
Assigns event time from the system time source.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/EventTimeSource.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="ingress-schema"></a>
### Ingress
Defines the point where data enters the system.

| Union Type | Description |
| :---: | --- |
| [Ingress::Url](#ingress-url-schema) | Pulls data from one of the supported sources by its URL. |
| [Ingress::FilesGlob](#ingress-filesglob-schema) | Uses glob operator to match files on the local file system. |
| [Ingress::Container](#ingress-container-schema) | Runs the specified OCI container to fetch data from an arbitrary source. |
| [Ingress::Mqtt](#ingress-mqtt-schema) | Connects to an MQTT broker to fetch events from the specified topic. |
| [Ingress::EvmLogs](#ingress-evmlogs-schema) | Connects to an EVM (Ethereum) node to stream transaction logs. |
| [Ingress::RestEndpoint](#ingress-restendpoint-schema) | Exposes a REST HTTP endpoint that accepts pushed data records. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/Ingress.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="ingress-container-schema"></a>
### Ingress::Container
Runs the specified OCI container to fetch data from an arbitrary source.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `image` | `string` | V |  | Image name and and an optional tag. |
| `command` | array(`string`) |  |  | Specifies the entrypoint. Not executed within a shell. The default OCI image's ENTRYPOINT is used if this is not provided. |
| `args` | array(`string`) |  |  | Arguments to the entrypoint. The OCI image's CMD is used if this is not provided. |
| `env` | array([EnvVar](#envvar-schema)) |  |  | Environment variables to propagate into or set in the container. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/Ingress.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="ingress-evmlogs-schema"></a>
### Ingress::EvmLogs
Connects to an EVM (Ethereum) node to stream transaction logs.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `chainId` | `integer` |  | `uint64` | Identifier of the chain to scan logs from. This parameter may be used for RPC endpoint lookup as well as asserting that provided `nodeUrl` corresponds to the expected chain. |
| `nodeUrl` | `string` |  | `url` | Url of the node. |
| `filter` | `string` |  |  | An SQL WHERE clause that can be used to pre-filter the logs before fetching them from the ETH node. |
| `signature` | `string` |  |  | Solidity log event signature to use for decoding. Using this field adds `event` to the output containing decoded log as JSON. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/Ingress.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="ingress-filesglob-schema"></a>
### Ingress::FilesGlob
Uses glob operator to match files on the local file system.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `path` | `string` | V |  | Path with a glob pattern. |
| `eventTime` | [EventTimeSource](#eventtimesource-schema) |  |  | Describes how event time is extracted from the source metadata. |
| `cache` | [SourceCaching](#sourcecaching-schema) |  |  | Describes the caching settings used for this source. |
| `order` | [SourceOrdering](#sourceordering-schema) |  |  | Specifies how input files should be ordered before ingestion.<br/>Order is important as every file will be processed individually<br/>and will advance the dataset's watermark. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/Ingress.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="ingress-mqtt-schema"></a>
### Ingress::Mqtt
Connects to an MQTT broker to fetch events from the specified topic.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `host` | `string` | V |  | Hostname of the MQTT broker. |
| `port` | `integer` | V | `int32` | Port of the MQTT broker. |
| `username` | `string` |  |  | Username to use for auth with the broker. |
| `password` | `string` |  |  | Password to use for auth with the broker (can be templated). |
| `topics` | array([MqttTopicSubscription](#mqtttopicsubscription-schema)) | V |  | List of topic subscription parameters. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/Ingress.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="ingress-restendpoint-schema"></a>
### Ingress::RestEndpoint
Exposes a REST HTTP endpoint that accepts pushed data records.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `buffer` | [IngressBuffer](#ingressbuffer-schema) |  |  | Buffer configuration for holding records until they are ingested. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/Ingress.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="ingress-url-schema"></a>
### Ingress::Url
Pulls data from one of the supported sources by its URL.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `url` | `string` | V | `url` | URL of the data source |
| `eventTime` | [EventTimeSource](#eventtimesource-schema) |  |  | Describes how event time is extracted from the source metadata. |
| `cache` | [SourceCaching](#sourcecaching-schema) |  |  | Describes the caching settings used for this source. |
| `headers` | array([RequestHeader](#requestheader-schema)) |  |  | Headers to pass during the request (e.g. HTTP Authorization) |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/Ingress.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="ingressbuffer-schema"></a>
### IngressBuffer
Buffer configuration for holding pushed records until they are ingested.

| Union Type | Description |
| :---: | --- |
| [IngressBuffer::Memory](#ingressbuffer-memory-schema) | An in-memory buffer. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/IngressBuffer.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="ingressbuffer-memory-schema"></a>
### IngressBuffer::Memory
An in-memory buffer.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `bufferSize` | `integer` |  | `uint64` | Maximum number of records to hold in the buffer. |
| `overflowPolicy` | `string` |  |  | Policy applied when the buffer is full. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/IngressBuffer.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="mergestrategy-schema"></a>
### MergeStrategy
Merge strategy determines how newly ingested data should be combined with the data that already exists in the dataset.

| Union Type | Description |
| :---: | --- |
| [MergeStrategy::Append](#mergestrategy-append-schema) | Append merge strategy.<br/><br/>Under this strategy new data will be appended to the dataset in its entirety, without any deduplication. |
| [MergeStrategy::Ledger](#mergestrategy-ledger-schema) | Ledger merge strategy.<br/><br/>This strategy should be used for data sources containing ledgers of events. Currently this strategy will only perform deduplication of events using user-specified primary key columns. This means that the source data can contain partially overlapping set of records and only those records that were not previously seen will be appended. |
| [MergeStrategy::Snapshot](#mergestrategy-snapshot-schema) | Snapshot merge strategy.<br/><br/>This strategy can be used for data state snapshots that are taken periodically and contain only the latest state of the observed entity or system. Over time such snapshots can have new rows added, and old rows either removed or modified.<br/><br/>This strategy transforms snapshot data into an append-only event stream where data already added is immutable. It does so by performing Change Data Capture - essentially diffing the current state of data against the reconstructed previous state and recording differences as retractions or corrections. The Operation Type "op" column will contain:<br/>  - append (`+A`) when a row appears for the first time<br/>  - retraction (`-R`) when row disappears<br/>  - correction (`-C`, `+C`) when row data has changed, with `-C` event carrying the old value of the row and `+C` carrying the new value.<br/><br/>To correctly associate rows between old and new snapshots this strategy relies on user-specified primary key columns.<br/><br/>To identify whether a row has changed this strategy will compare all other columns one by one. If the data contains a column that is guaranteed to change whenever any of the data columns changes (for example a last modification timestamp, an incremental version, or a data hash), then it can be specified in `compareColumns` property to speed up the detection of modified rows. |
| [MergeStrategy::ChangelogStream](#mergestrategy-changelogstream-schema) | Changelog stream merge strategy.<br/><br/>This is the native stream format for ODF that accurately describes the evolution of all event records including appends, retractions, and corrections as per RFC-015. No pre-processing except for format validation is done. |
| [MergeStrategy::UpsertStream](#mergestrategy-upsertstream-schema) | Upsert stream merge strategy.<br/><br/>This strategy should be used for data sources containing ledgers of insert-or-update and delete events. Unlike ChangelogStream the insert-or-update events only carry the new values, so this strategy will use primary key to re-classify the events into an append or a correction from/to pair, looking up the previous values. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/MergeStrategy.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="mergestrategy-append-schema"></a>
### MergeStrategy::Append
Append merge strategy.

Under this strategy new data will be appended to the dataset in its entirety, without any deduplication.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/MergeStrategy.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="mergestrategy-changelogstream-schema"></a>
### MergeStrategy::ChangelogStream
Changelog stream merge strategy.

This is the native stream format for ODF that accurately describes the evolution of all event records including appends, retractions, and corrections as per RFC-015. No pre-processing except for format validation is done.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `primaryKey` | array(`string`) | V |  | Names of the columns that uniquely identify the record throughout its lifetime |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/MergeStrategy.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="mergestrategy-ledger-schema"></a>
### MergeStrategy::Ledger
Ledger merge strategy.

This strategy should be used for data sources containing ledgers of events. Currently this strategy will only perform deduplication of events using user-specified primary key columns. This means that the source data can contain partially overlapping set of records and only those records that were not previously seen will be appended.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `primaryKey` | array(`string`) | V |  | Names of the columns that uniquely identify the record throughout its lifetime |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/MergeStrategy.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="mergestrategy-snapshot-schema"></a>
### MergeStrategy::Snapshot
Snapshot merge strategy.

This strategy can be used for data state snapshots that are taken periodically and contain only the latest state of the observed entity or system. Over time such snapshots can have new rows added, and old rows either removed or modified.

This strategy transforms snapshot data into an append-only event stream where data already added is immutable. It does so by performing Change Data Capture - essentially diffing the current state of data against the reconstructed previous state and recording differences as retractions or corrections. The Operation Type "op" column will contain:
  - append (`+A`) when a row appears for the first time
  - retraction (`-R`) when row disappears
  - correction (`-C`, `+C`) when row data has changed, with `-C` event carrying the old value of the row and `+C` carrying the new value.

To correctly associate rows between old and new snapshots this strategy relies on user-specified primary key columns.

To identify whether a row has changed this strategy will compare all other columns one by one. If the data contains a column that is guaranteed to change whenever any of the data columns changes (for example a last modification timestamp, an incremental version, or a data hash), then it can be specified in `compareColumns` property to speed up the detection of modified rows.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `primaryKey` | array(`string`) | V |  | Names of the columns that uniquely identify the record throughout its lifetime. |
| `compareColumns` | array(`string`) |  |  | Names of the columns to compared to determine if a row has changed between two snapshots. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/MergeStrategy.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="mergestrategy-upsertstream-schema"></a>
### MergeStrategy::UpsertStream
Upsert stream merge strategy.

This strategy should be used for data sources containing ledgers of insert-or-update and delete events. Unlike ChangelogStream the insert-or-update events only carry the new values, so this strategy will use primary key to re-classify the events into an append or a correction from/to pair, looking up the previous values.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `primaryKey` | array(`string`) | V |  | Names of the columns that uniquely identify the record throughout its lifetime |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/MergeStrategy.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="mqttqos-schema"></a>
### MqttQos
MQTT quality of service class.

| Enum Value |
| :---: |
| AtMostOnce |
| AtLeastOnce |
| ExactlyOnce |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/MqttQos.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="mqtttopicsubscription-schema"></a>
### MqttTopicSubscription
MQTT topic subscription parameters.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `path` | `string` | V |  | Name of the topic (may include patterns). |
| `qos` | [MqttQos](#mqttqos-schema) |  |  | Quality of service class.<br/><br/>Default: "AtMostOnce" |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/MqttTopicSubscription.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="prepstep-schema"></a>
### PrepStep
Defines the steps to prepare raw data for ingestion.

| Union Type | Description |
| :---: | --- |
| [PrepStep::Decompress](#prepstep-decompress-schema) | Pulls data from one of the supported sources by its URL. |
| [PrepStep::Pipe](#prepstep-pipe-schema) | Executes external command to process the data using piped input/output. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/PrepStep.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="prepstep-decompress-schema"></a>
### PrepStep::Decompress
Pulls data from one of the supported sources by its URL.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `format` | [CompressionFormat](#compressionformat-schema) | V |  | Name of a compression algorithm used on data. |
| `subPath` | `string` |  |  | Path to a data file within a multi-file archive. Can contain glob patterns. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/PrepStep.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="prepstep-pipe-schema"></a>
### PrepStep::Pipe
Executes external command to process the data using piped input/output.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `command` | array(`string`) | V |  | Command to execute and its arguments. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/PrepStep.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="readstep-schema"></a>
### ReadStep
Defines how raw data should be read into the structured form.

| Union Type | Description |
| :---: | --- |
| [ReadStep::Csv](#readstep-csv-schema) | Reader for comma-separated files. |
| [ReadStep::GeoJson](#readstep-geojson-schema) | Reader for GeoJSON files. It expects one `FeatureCollection` object in the root and will create a record per each `Feature` inside it extracting the properties into individual columns and leaving the feature geometry in its own column. |
| [ReadStep::EsriShapefile](#readstep-esrishapefile-schema) | Reader for ESRI Shapefile format. |
| [ReadStep::Parquet](#readstep-parquet-schema) | Reader for Apache Parquet format. |
| [ReadStep::Json](#readstep-json-schema) | Reader for JSON files that contain an array of objects within them. |
| [ReadStep::NdJson](#readstep-ndjson-schema) | Reader for files containing multiple newline-delimited JSON objects with the same schema. |
| [ReadStep::NdGeoJson](#readstep-ndgeojson-schema) | Reader for Newline-delimited GeoJSON files. It is similar to `GeoJson` format but instead of `FeatureCollection` object in the root it expects every individual feature object to appear on its own line. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/ReadStep.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="readstep-csv-schema"></a>
### ReadStep::Csv
Reader for comma-separated files.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `ddlSchema` | array(`string`) |  |  | DEPRECATED: A DDL-formatted schema. Schema can be used to coerce values into more appropriate data types. |
| `separator` | `string` |  |  | Sets a single character as a separator for each field and value.<br/><br/>Default: "," |
| `encoding` | `string` |  |  | Decodes the CSV files by the given encoding type.<br/><br/>Default: "utf8" |
| `quote` | `string` |  |  | Sets a single character used for escaping quoted values where the separator can be part of the value. Set an empty string to turn off quotations.<br/><br/>Default: "\"" |
| `escape` | `string` |  |  | Sets a single character used for escaping quotes inside an already quoted value.<br/><br/>Default: "\\" |
| `header` | `boolean` |  |  | Use the first line as names of columns.<br/><br/>Default: false |
| `inferSchema` | `boolean` |  |  | Infers the input schema automatically from data. It requires one extra pass over the data.<br/><br/>Default: false |
| `nullValue` | `string` |  |  | Sets the string representation of a null value.<br/><br/>Default: "" |
| `dateFormat` | `string` |  |  | Sets the string that indicates a date format. The `rfc3339` is the only required format, the other format strings are implementation-specific.<br/><br/>Default: "rfc3339" |
| `timestampFormat` | `string` |  |  | Sets the string that indicates a timestamp format. The `rfc3339` is the only required format, the other format strings are implementation-specific.<br/><br/>Default: "rfc3339" |
| `schema` | [DataSchema](#dataschema-schema) |  |  | Schema used to coerce values into more appropriate data types. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/ReadStep.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="readstep-esrishapefile-schema"></a>
### ReadStep::EsriShapefile
Reader for ESRI Shapefile format.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `ddlSchema` | array(`string`) |  |  | DEPRECATED: A DDL-formatted schema. Schema can be used to coerce values into more appropriate data types. |
| `subPath` | `string` |  |  | If the ZIP archive contains multiple shapefiles use this field to specify a sub-path to the desired `.shp` file. Can contain glob patterns to act as a filter. |
| `schema` | [DataSchema](#dataschema-schema) |  |  | Schema used to coerce values into more appropriate data types. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/ReadStep.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="readstep-geojson-schema"></a>
### ReadStep::GeoJson
Reader for GeoJSON files. It expects one `FeatureCollection` object in the root and will create a record per each `Feature` inside it extracting the properties into individual columns and leaving the feature geometry in its own column.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `ddlSchema` | array(`string`) |  |  | DEPRECATED: A DDL-formatted schema. Schema can be used to coerce values into more appropriate data types. |
| `schema` | [DataSchema](#dataschema-schema) |  |  | Schema used to coerce values into more appropriate data types. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/ReadStep.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="readstep-json-schema"></a>
### ReadStep::Json
Reader for JSON files that contain an array of objects within them.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `subPath` | `string` |  |  | Path in the form of `a.b.c` to a sub-element of the root JSON object that is an array or objects. If not specified it is assumed that the root element is an array. |
| `ddlSchema` | array(`string`) |  |  | DEPRECATED: A DDL-formatted schema. Schema can be used to coerce values into more appropriate data types. |
| `dateFormat` | `string` |  |  | Sets the string that indicates a date format. The `rfc3339` is the only required format, the other format strings are implementation-specific.<br/><br/>Default: "rfc3339" |
| `encoding` | `string` |  |  | Allows to forcibly set one of standard basic or extended encodings.<br/><br/>Default: "utf8" |
| `timestampFormat` | `string` |  |  | Sets the string that indicates a timestamp format. The `rfc3339` is the only required format, the other format strings are implementation-specific.<br/><br/>Default: "rfc3339" |
| `schema` | [DataSchema](#dataschema-schema) |  |  | Schema used to coerce values into more appropriate data types. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/ReadStep.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="readstep-ndgeojson-schema"></a>
### ReadStep::NdGeoJson
Reader for Newline-delimited GeoJSON files. It is similar to `GeoJson` format but instead of `FeatureCollection` object in the root it expects every individual feature object to appear on its own line.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `ddlSchema` | array(`string`) |  |  | DEPRECATED: A DDL-formatted schema. Schema can be used to coerce values into more appropriate data types. |
| `schema` | [DataSchema](#dataschema-schema) |  |  | Schema used to coerce values into more appropriate data types. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/ReadStep.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="readstep-ndjson-schema"></a>
### ReadStep::NdJson
Reader for files containing multiple newline-delimited JSON objects with the same schema.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `ddlSchema` | array(`string`) |  |  | DEPRECATED: A DDL-formatted schema. Schema can be used to coerce values into more appropriate data types. |
| `dateFormat` | `string` |  |  | Sets the string that indicates a date format. The `rfc3339` is the only required format, the other format strings are implementation-specific.<br/><br/>Default: "rfc3339" |
| `encoding` | `string` |  |  | Allows to forcibly set one of standard basic or extended encodings.<br/><br/>Default: "utf8" |
| `timestampFormat` | `string` |  |  | Sets the string that indicates a timestamp format. The `rfc3339` is the only required format, the other format strings are implementation-specific.<br/><br/>Default: "rfc3339" |
| `schema` | [DataSchema](#dataschema-schema) |  |  | Schema used to coerce values into more appropriate data types. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/ReadStep.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="readstep-parquet-schema"></a>
### ReadStep::Parquet
Reader for Apache Parquet format.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `ddlSchema` | array(`string`) |  |  | DEPRECATED: A DDL-formatted schema. Schema can be used to coerce values into more appropriate data types. |
| `schema` | [DataSchema](#dataschema-schema) |  |  | Schema used to coerce values into more appropriate data types. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/ReadStep.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="requestheader-schema"></a>
### RequestHeader
Defines a header (e.g. HTTP) to be passed into some request.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` | V |  | Name of the header. |
| `value` | `string` | V |  | Value of the header. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/RequestHeader.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="source-schema"></a>
### Source
Defines an external source of data for ingestion.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `$schema` | `string` | V |  | Identifies this resource type. |
| `headers` | [ResourceHeadersInput](#resourceheadersinput-schema) | V |  | Container for identity and ownership information of a resource. |
| `spec` | [SourceSpecInput](#sourcespecinput-schema) | V |  | Specifies the desired state of the resource. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/Source.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="sourcecaching-schema"></a>
### SourceCaching
Defines how external data should be cached.

| Union Type | Description |
| :---: | --- |
| [SourceCaching::Forever](#sourcecaching-forever-schema) | After source was processed once it will never be ingested again. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/SourceCaching.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="sourcecaching-forever-schema"></a>
### SourceCaching::Forever
After source was processed once it will never be ingested again.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/SourceCaching.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="sourceordering-schema"></a>
### SourceOrdering
Specifies how input files should be ordered before ingestion.

| Enum Value |
| :---: |
| ByEventTime |
| ByName |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/SourceOrdering.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="sourcespec-schema"></a>
### SourceSpec
Specifies an external source of data for ingestion.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `config` | [ValueRefs](#valuerefs-schema) |  |  | Brings the configuration values into the local `config` context. |
| `ingress` | [Ingress](#ingress-schema) |  |  | Determines where data is sourced from. |
| `prepare` | array([PrepStep](#prepstep-schema)) |  |  | Defines how raw data is prepared before reading. |
| `read` | [ReadStep](#readstep-schema) | V |  | Defines how data is read into structured format. |
| `preprocess` | [Transform](#transform-schema) |  |  | Pre-processing query that shapes the data. |
| `merge` | [MergeStrategy](#mergestrategy-schema) |  |  | Determines how newly-ingested data should be merged with existing history. |
| `vocab` | [DatasetVocabulary](#datasetvocabulary-schema) |  |  | Defines the mapping of system fields to dataset column names. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/SourceSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="sourcespecinput-schema"></a>
### SourceSpecInput
Specifies an external source of data for ingestion.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `config` | [ValueRefs](#valuerefs-schema) |  |  | Brings the configuration values into the local `config` context. |
| `ingress` | [Ingress](#ingress-schema) |  |  | Determines where data is sourced from. |
| `prepare` | array([PrepStep](#prepstep-schema)) |  |  | Defines how raw data is prepared before reading. |
| `read` | [ReadStep](#readstep-schema) | V |  | Defines how data is read into structured format. |
| `preprocess` | [Transform](#transform-schema) |  |  | Pre-processing query that shapes the data. |
| `merge` | [MergeStrategy](#mergestrategy-schema) |  |  | Determines how newly-ingested data should be merged with existing history. |
| `vocab` | [DatasetVocabulary](#datasetvocabulary-schema) |  |  | Defines the mapping of system fields to dataset column names. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/SourceSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="sourcestate-schema"></a>
### SourceState
The state of the source the data was added from to allow fast resuming.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `sourceName` | `string` | V |  | Identifies the source that the state corresponds to. |
| `kind` | `string` | V |  | Identifies the type of the state. Standard types include: `odf/etag`, `odf/last-modified`. |
| `value` | `string` | V |  | Opaque value representing the state. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/sources/v1alpha1/SourceState.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="reference-storage"></a>
## storage

<a name="awscredentials-schema"></a>
### AwsCredentials
Access credentials for AWS or an AWS-compatible service.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `accessKey` | [ValueHandle](#valuehandle-schema) |  |  | Reference to a secret containing the AWS access key ID. |
| `secretKey` | [ValueHandle](#valuehandle-schema) |  |  | Reference to a secret containing the AWS secret access key. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/storage/v1alpha1/AwsCredentials.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="awscredentialsinput-schema"></a>
### AwsCredentialsInput
Access credentials for AWS or an AWS-compatible service.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `accessKey` | [ValueRef](#valueref-schema) |  |  | Reference to a secret containing the AWS access key ID. |
| `secretKey` | [ValueRef](#valueref-schema) |  |  | Reference to a secret containing the AWS secret access key. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/storage/v1alpha1/AwsCredentialsInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="persistentvolume-schema"></a>
### PersistentVolume
Defines a storage volume where data can be stored and its access credentials.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `$schema` | `string` | V |  | Identifies this resource type. |
| `headers` | [ResourceHeadersInput](#resourceheadersinput-schema) | V |  | Container for identity and ownership information of a resource. |
| `spec` | [PersistentVolumeSpecInput](#persistentvolumespecinput-schema) | V |  | Specifies the desired state of the resource. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/storage/v1alpha1/PersistentVolume.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="persistentvolumeref-schema"></a>
### PersistentVolumeRef
Reference to a `PersistentVolume`.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `account` | [ResourceOwnerRef](#resourceownerref-schema) |  |  | Reference to an account that owns the `PersistentVolume`. |
| `id` | `string` |  |  | ID of the resource. |
| `name` | `string` |  |  | Name of the resource. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/storage/v1alpha1/PersistentVolumeRef.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="persistentvolumespec-schema"></a>
### PersistentVolumeSpec
Defines a storage volume where data can be stored and its access credentials.

| Union Type | Description |
| :---: | --- |
| [PersistentVolumeSpec::S3](#persistentvolumespec-s3-schema) | An Amazon S3 or S3-compatible object storage bucket. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/storage/v1alpha1/PersistentVolumeSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="persistentvolumespec-s3-schema"></a>
### PersistentVolumeSpec::S3
An Amazon S3 or S3-compatible object storage bucket.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `endpoint` | `string` |  | `url` | S3 endpoint URL. If omitted, defaults to AWS S3. Use for S3-compatible stores e.g. `https://s3.amazonaws.com`. |
| `region` | `string` |  |  | AWS region where the bucket is located e.g. `us-west-2`. |
| `bucket` | `string` | V |  | Name of the S3 bucket. |
| `prefix` | `string` |  |  | Optional path prefix within the bucket. |
| `capacity` | [VolumeCapacity](#volumecapacity-schema) |  |  | Storage capacity allocation. |
| `credentials` | [AwsCredentials](#awscredentials-schema) |  |  | Access credentials for the bucket. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/storage/v1alpha1/PersistentVolumeSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="persistentvolumespecinput-schema"></a>
### PersistentVolumeSpecInput
Defines a storage volume where data can be stored and its access credentials.

| Union Type | Description |
| :---: | --- |
| [PersistentVolumeSpecInput::S3](#persistentvolumespecinput-s3-schema) | An Amazon S3 or S3-compatible object storage bucket. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/storage/v1alpha1/PersistentVolumeSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="persistentvolumespecinput-s3-schema"></a>
### PersistentVolumeSpecInput::S3
An Amazon S3 or S3-compatible object storage bucket.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `endpoint` | `string` |  | `url` | S3 endpoint URL. If omitted, defaults to AWS S3. Use for S3-compatible stores e.g. `https://s3.amazonaws.com`. |
| `region` | `string` |  |  | AWS region where the bucket is located e.g. `us-west-2`. |
| `bucket` | `string` | V |  | Name of the S3 bucket. |
| `prefix` | `string` |  |  | Optional path prefix within the bucket. |
| `capacity` | [VolumeCapacity](#volumecapacity-schema) |  |  | Storage capacity allocation. |
| `credentials` | [AwsCredentialsInput](#awscredentialsinput-schema) |  |  | Access credentials for the bucket. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/storage/v1alpha1/PersistentVolumeSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="volumecapacity-schema"></a>
### VolumeCapacity
Storage capacity allocation.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `storage` | `string` |  | [byte-size](https://www.thierry-lequeu.fr/data/PELS/Comm/Publications/Newsletter/9704/STORY18.HTML) | Maximum storage size e.g. `10Gi`. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/storage/v1alpha1/VolumeCapacity.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="reference-tasks"></a>
## tasks

<a name="failurebehavior-schema"></a>
### FailureBehavior
Determines how a flow run proceeds when a task finishes with a `Failed` outcome.

| Enum Value |
| :---: |
| Fail |
| Continue |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/FailureBehavior.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="noopbehavior-schema"></a>
### NoOpBehavior
Determines how a flow run proceeds when a task finishes with a `NoOp` outcome.

| Enum Value |
| :---: |
| Continue |
| Break |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/NoOpBehavior.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="syncauth-schema"></a>
### SyncAuth
Credentials used to access a remote location during a sync.

| Union Type | Description |
| :---: | --- |
| [SyncAuth::Bearer](#syncauth-bearer-schema) | Authenticates with a bearer token, e.g. an access token of a remote ODF node. |
| [SyncAuth::Aws](#syncauth-aws-schema) | Authenticates with AWS or an AWS-compatible object storage. |
| [SyncAuth::Headers](#syncauth-headers-schema) | Authenticates by passing custom request headers. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/SyncAuth.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="syncauth-aws-schema"></a>
### SyncAuth::Aws
Authenticates with AWS or an AWS-compatible object storage.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `endpoint` | `string` |  | `url` | S3 endpoint URL. If omitted, defaults to AWS S3. Use for S3-compatible stores. |
| `region` | `string` |  |  | AWS region where the bucket is located e.g. `us-west-2`. |
| `accessKey` | [ValueHandle](#valuehandle-schema) |  |  | Reference to a secret containing the AWS access key ID. |
| `secretKey` | [ValueHandle](#valuehandle-schema) |  |  | Reference to a secret containing the AWS secret access key. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/SyncAuth.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="syncauth-bearer-schema"></a>
### SyncAuth::Bearer
Authenticates with a bearer token, e.g. an access token of a remote ODF node.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `token` | [ValueHandle](#valuehandle-schema) | V |  | Reference to a secret containing the token. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/SyncAuth.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="syncauth-headers-schema"></a>
### SyncAuth::Headers
Authenticates by passing custom request headers.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `headers` | array([SyncAuthHeader](#syncauthheader-schema)) | V |  | Headers to pass with every request. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/SyncAuth.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="syncauthheader-schema"></a>
### SyncAuthHeader
A request header whose value is taken from a `VariableSet` or a `SecretSet`.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` | V |  | Name of the header. |
| `value` | [ValueHandle](#valuehandle-schema) | V |  | Reference to the value of the header. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/SyncAuthHeader.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="syncauthheaderinput-schema"></a>
### SyncAuthHeaderInput
A request header whose value is taken from a `VariableSet` or a `SecretSet`.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` | V |  | Name of the header. |
| `value` | [ValueRef](#valueref-schema) | V |  | Reference to the value of the header. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/SyncAuthHeaderInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="syncauthinput-schema"></a>
### SyncAuthInput
Credentials used to access a remote location during a sync.

| Union Type | Description |
| :---: | --- |
| [SyncAuthInput::Bearer](#syncauthinput-bearer-schema) | Authenticates with a bearer token, e.g. an access token of a remote ODF node. |
| [SyncAuthInput::Aws](#syncauthinput-aws-schema) | Authenticates with AWS or an AWS-compatible object storage. |
| [SyncAuthInput::Headers](#syncauthinput-headers-schema) | Authenticates by passing custom request headers. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/SyncAuthInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="syncauthinput-aws-schema"></a>
### SyncAuthInput::Aws
Authenticates with AWS or an AWS-compatible object storage.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `endpoint` | `string` |  | `url` | S3 endpoint URL. If omitted, defaults to AWS S3. Use for S3-compatible stores. |
| `region` | `string` |  |  | AWS region where the bucket is located e.g. `us-west-2`. |
| `accessKey` | [ValueRef](#valueref-schema) |  |  | Reference to a secret containing the AWS access key ID. |
| `secretKey` | [ValueRef](#valueref-schema) |  |  | Reference to a secret containing the AWS secret access key. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/SyncAuthInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="syncauthinput-bearer-schema"></a>
### SyncAuthInput::Bearer
Authenticates with a bearer token, e.g. an access token of a remote ODF node.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `token` | [ValueRef](#valueref-schema) | V |  | Reference to a secret containing the token. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/SyncAuthInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="syncauthinput-headers-schema"></a>
### SyncAuthInput::Headers
Authenticates by passing custom request headers.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `headers` | array([SyncAuthHeaderInput](#syncauthheaderinput-schema)) | V |  | Headers to pass with every request. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/SyncAuthInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="syncendpoint-schema"></a>
### SyncEndpoint
A remote location a dataset is synced with.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `url` | `string` | V | `url` | URL of the remote dataset. The scheme selects the transfer protocol e.g. `odf+https://`, `s3://`, `file://`. |
| `auth` | [SyncAuth](#syncauth-schema) |  |  | Credentials used to access the remote location. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/SyncEndpoint.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="syncendpointinput-schema"></a>
### SyncEndpointInput
A remote location a dataset is synced with.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `url` | `string` | V | `url` | URL of the remote dataset. The scheme selects the transfer protocol e.g. `odf+https://`, `s3://`, `file://`. |
| `auth` | [SyncAuthInput](#syncauthinput-schema) |  |  | Credentials used to access the remote location. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/SyncEndpointInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="task-schema"></a>
### Task
An individual work item to be executed.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `$schema` | `string` | V |  | Identifies this resource type. |
| `headers` | [ResourceHeadersInput](#resourceheadersinput-schema) | V |  | Container for identity and ownership information of a resource. |
| `spec` | [TaskSpecInput](#taskspecinput-schema) |  |  | Specifies the desired state of the task. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/Task.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskerror-schema"></a>
### TaskError
Type-specific details of a task failure. The `kind` property identifies the error, e.g. `InputDatasetCompacted`, and the remaining properties depend on it.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskError.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="known-extra-attrs"></a>
#### Known Extensions
| Extension | Description |
| --- | --- |
| `opendatafabric.net/description` | Used for human readable schema field descriptions |
| `opendatafabric.net/type` | An extended set of logical types that ODF recommends but does not require every implementation to support |
| `opendatafabric.org/linkedObjects` | When attached to `AddData` event contains a summary of how many external objects were associated with a certain transaction as well as their size |
| `arrow.apache.org/bufferEncoding` | Used to accurately represent buffer encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/dateEncoding` | Used to accurately represent date encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/decimalEncoding` | Used to accurately represent decimal encoding type when converting Arrow schema to ODF schema |


<a name="known-extra-types"></a>
#### Known Extended Types
| Extended Type | Core Type | Description |
| --- | --- | --- |
| `Did` | `String` | Decentralized identifier `did:<method>:<id>` |
| `Multihash` | `String` | Hash in self-describing [multihash](https://github.com/multiformats/multihash) format |
| `ObjectLink` | `String` | Signifies that the value references an external object. The mandatory `linkType` property defines the type of the link (e.g. `Multihash`). |



<a name="taskoutcome-schema"></a>
### TaskOutcome
Result of the execution of a task.

| Union Type | Description |
| :---: | --- |
| [TaskOutcome::Success](#taskoutcome-success-schema) | Task completed successfully. |
| [TaskOutcome::Failed](#taskoutcome-failed-schema) | Task failed. |
| [TaskOutcome::NoOp](#taskoutcome-noop-schema) | Task completed with no work done (e.g. no new data to process). |
| [TaskOutcome::Cancelled](#taskoutcome-cancelled-schema) | Task was cancelled before completion. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskOutcome.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskoutcome-cancelled-schema"></a>
### TaskOutcome::Cancelled
Task was cancelled before completion.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskOutcome.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskoutcome-failed-schema"></a>
### TaskOutcome::Failed
Task failed.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `message` | `string` | V |  | Human-readable description of the failure. |
| `recoverable` | `boolean` | V |  | Whether retrying the task could succeed, e.g. after a network error. An unrecoverable failure, such as an invalid query, would fail the same way again. |
| `error` | [TaskError](#taskerror-schema) |  |  | Type-specific details of the failure. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskOutcome.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskoutcome-noop-schema"></a>
### TaskOutcome::NoOp
Task completed with no work done (e.g. no new data to process).

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskOutcome.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskoutcome-success-schema"></a>
### TaskOutcome::Success
Task completed successfully.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskOutcome.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="known-extra-attrs"></a>
#### Known Extensions
| Extension | Description |
| --- | --- |
| `opendatafabric.net/description` | Used for human readable schema field descriptions |
| `opendatafabric.net/type` | An extended set of logical types that ODF recommends but does not require every implementation to support |
| `opendatafabric.org/linkedObjects` | When attached to `AddData` event contains a summary of how many external objects were associated with a certain transaction as well as their size |
| `arrow.apache.org/bufferEncoding` | Used to accurately represent buffer encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/dateEncoding` | Used to accurately represent date encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/decimalEncoding` | Used to accurately represent decimal encoding type when converting Arrow schema to ODF schema |


<a name="known-extra-types"></a>
#### Known Extended Types
| Extended Type | Core Type | Description |
| --- | --- | --- |
| `Did` | `String` | Decentralized identifier `did:<method>:<id>` |
| `Multihash` | `String` | Hash in self-describing [multihash](https://github.com/multiformats/multihash) format |
| `ObjectLink` | `String` | Signifies that the value references an external object. The mandatory `linkType` property defines the type of the link (e.g. `Multihash`). |



<a name="taskplan-schema"></a>
### TaskPlan
A self-contained logical execution plan of a task.

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskPlan.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="known-extra-attrs"></a>
#### Known Extensions
| Extension | Description |
| --- | --- |
| `opendatafabric.net/description` | Used for human readable schema field descriptions |
| `opendatafabric.net/type` | An extended set of logical types that ODF recommends but does not require every implementation to support |
| `opendatafabric.org/linkedObjects` | When attached to `AddData` event contains a summary of how many external objects were associated with a certain transaction as well as their size |
| `arrow.apache.org/bufferEncoding` | Used to accurately represent buffer encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/dateEncoding` | Used to accurately represent date encoding type when converting Arrow schema to ODF schema |
| `arrow.apache.org/decimalEncoding` | Used to accurately represent decimal encoding type when converting Arrow schema to ODF schema |


<a name="known-extra-types"></a>
#### Known Extended Types
| Extended Type | Core Type | Description |
| --- | --- | --- |
| `Did` | `String` | Decentralized identifier `did:<method>:<id>` |
| `Multihash` | `String` | Hash in self-describing [multihash](https://github.com/multiformats/multihash) format |
| `ObjectLink` | `String` | Signifies that the value references an external object. The mandatory `linkType` property defines the type of the link (e.g. `Multihash`). |



<a name="taskspec-schema"></a>
### TaskSpec
An individual work item to be executed as part of a flow.

| Union Type | Description |
| :---: | --- |
| [TaskSpec::Ingest](#taskspec-ingest-schema) | Fetches data from a source and appends it to a dataset. |
| [TaskSpec::Transform](#taskspec-transform-schema) | Executes transformation of data defined in a derivative dataset. |
| [TaskSpec::SyncFrom](#taskspec-syncfrom-schema) | Pulls new blocks from a remote dataset into a local dataset. |
| [TaskSpec::SyncTo](#taskspec-syncto-schema) | Pushes new blocks of a local dataset to a remote dataset. |
| [TaskSpec::Compaction](#taskspec-compaction-schema) | Compacts data files in matching datasets to improve query performance. |
| [TaskSpec::Reset](#taskspec-reset-schema) | Moves a block reference of a dataset to an earlier block, discarding the history that follows it. |
| [TaskSpec::GarbageCollection](#taskspec-garbagecollection-schema) | Removes unreferenced data files from matching datasets. |
| [TaskSpec::Verify](#taskspec-verify-schema) | Checks dataset metadata for integrity. |
| [TaskSpec::WebhookCall](#taskspec-webhookcall-schema) | Dispatches a certain payload to a specific `WebhookEndpoint`. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskspec-compaction-schema"></a>
### TaskSpec::Compaction
Compacts data files in matching datasets to improve query performance.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` |  |  | An alias for the task used to refer to it in flows and access the results |
| `target` | [DatasetHandle](#datasethandle-schema) |  |  | Reference to the dataset to compact. Defaults to the flow-level target when omitted. |
| `serviceAccount` | [PrincipalHandle](#principalhandle-schema) |  |  | Service account whose permissions the task executes with. Must be an account of service account type. Defaults to the service account of the run when omitted. |
| `onNoOp` | [NoOpBehavior](#noopbehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `NoOp` outcome: `Continue` executes the next task, while `Break` finishes the flow run early without an error.<br/><br/>Default: "Continue" |
| `onFailure` | [FailureBehavior](#failurebehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `Failed` outcome: `Fail` finishes the flow run with an error, while `Continue` executes the next task.<br/><br/>Default: "Fail" |
| `maxSliceSize` | `string` |  | [byte-size](https://www.thierry-lequeu.fr/data/PELS/Comm/Publications/Newsletter/9704/STORY18.HTML) | Target maximum size of each compacted data slice e.g. `100MiB`. |
| `maxSliceRecords` | `integer` |  | `uint64` | Target maximum number of records per compacted data slice. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskspec-garbagecollection-schema"></a>
### TaskSpec::GarbageCollection
Removes unreferenced data files from matching datasets.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` |  |  | An alias for the task used to refer to it in flows and access the results |
| `target` | [DatasetHandle](#datasethandle-schema) |  |  | Reference to the dataset to collect garbage from. Defaults to the flow-level target when omitted. |
| `serviceAccount` | [PrincipalHandle](#principalhandle-schema) |  |  | Service account whose permissions the task executes with. Must be an account of service account type. Defaults to the service account of the run when omitted. |
| `onNoOp` | [NoOpBehavior](#noopbehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `NoOp` outcome: `Continue` executes the next task, while `Break` finishes the flow run early without an error.<br/><br/>Default: "Continue" |
| `onFailure` | [FailureBehavior](#failurebehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `Failed` outcome: `Fail` finishes the flow run with an error, while `Continue` executes the next task.<br/><br/>Default: "Fail" |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskspec-ingest-schema"></a>
### TaskSpec::Ingest
Fetches data from a source and appends it to a dataset.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` |  |  | An alias for the task used to refer to it in flows and access the results |
| `target` | [DatasetHandle](#datasethandle-schema) |  |  | Reference to the dataset to ingest into. Defaults to the flow-level target when omitted. |
| `serviceAccount` | [PrincipalHandle](#principalhandle-schema) |  |  | Service account whose permissions the task executes with. Must be an account of service account type. Defaults to the service account of the run when omitted. |
| `onNoOp` | [NoOpBehavior](#noopbehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `NoOp` outcome: `Continue` executes the next task, while `Break` finishes the flow run early without an error.<br/><br/>Default: "Continue" |
| `onFailure` | [FailureBehavior](#failurebehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `Failed` outcome: `Fail` finishes the flow run with an error, while `Continue` executes the next task.<br/><br/>Default: "Fail" |
| `source` | [ResourceHandle](#resourcehandle-schema) | V |  | Reference to the source resource that defines how to fetch data. |
| `targetRecordsPerSlice` | `integer` |  | `uint64` | Target number of records to ingest per data slice. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskspec-reset-schema"></a>
### TaskSpec::Reset
Moves a block reference of a dataset to an earlier block, discarding the history that follows it.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` |  |  | An alias for the task used to refer to it in flows and access the results |
| `target` | [DatasetHandle](#datasethandle-schema) |  |  | Reference to the dataset to reset. Defaults to the flow-level target when omitted. |
| `serviceAccount` | [PrincipalHandle](#principalhandle-schema) |  |  | Service account whose permissions the task executes with. Must be an account of service account type. Defaults to the service account of the run when omitted. |
| `onNoOp` | [NoOpBehavior](#noopbehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `NoOp` outcome: `Continue` executes the next task, while `Break` finishes the flow run early without an error.<br/><br/>Default: "Continue" |
| `onFailure` | [FailureBehavior](#failurebehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `Failed` outcome: `Fail` finishes the flow run with an error, while `Continue` executes the next task.<br/><br/>Default: "Fail" |
| `ref` | `string` |  |  | Name of the block reference to reset.<br/><br/>Default: "head" |
| `newBlockHash` | `string` |  | [multihash](https://github.com/multiformats/multihash) | Hash of the block the reference will point to. Defaults to the `Seed` block when omitted. |
| `oldBlockHash` | `string` |  | [multihash](https://github.com/multiformats/multihash) | Expected hash of the block the reference currently points to. When specified, the task fails if the reference has moved, protecting against concurrent updates. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskspec-syncfrom-schema"></a>
### TaskSpec::SyncFrom
Pulls new blocks from a remote dataset into a local dataset.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` |  |  | An alias for the task used to refer to it in flows and access the results |
| `target` | [DatasetHandle](#datasethandle-schema) |  |  | Reference to the local dataset to pull into. Defaults to the flow-level target when omitted. |
| `serviceAccount` | [PrincipalHandle](#principalhandle-schema) |  |  | Service account whose permissions the task executes with. Must be an account of service account type. Defaults to the service account of the run when omitted. |
| `onNoOp` | [NoOpBehavior](#noopbehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `NoOp` outcome: `Continue` executes the next task, while `Break` finishes the flow run early without an error.<br/><br/>Default: "Continue" |
| `onFailure` | [FailureBehavior](#failurebehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `Failed` outcome: `Fail` finishes the flow run with an error, while `Continue` executes the next task.<br/><br/>Default: "Fail" |
| `source` | [SyncEndpoint](#syncendpoint-schema) | V |  | Remote dataset to pull from. |
| `force` | `boolean` |  |  | Overwrite the local dataset even if its history has diverged from the source.<br/><br/>Default: false |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskspec-syncto-schema"></a>
### TaskSpec::SyncTo
Pushes new blocks of a local dataset to a remote dataset.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` |  |  | An alias for the task used to refer to it in flows and access the results |
| `target` | [DatasetHandle](#datasethandle-schema) |  |  | Reference to the local dataset to push. Defaults to the flow-level target when omitted. |
| `serviceAccount` | [PrincipalHandle](#principalhandle-schema) |  |  | Service account whose permissions the task executes with. Must be an account of service account type. Defaults to the service account of the run when omitted. |
| `onNoOp` | [NoOpBehavior](#noopbehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `NoOp` outcome: `Continue` executes the next task, while `Break` finishes the flow run early without an error.<br/><br/>Default: "Continue" |
| `onFailure` | [FailureBehavior](#failurebehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `Failed` outcome: `Fail` finishes the flow run with an error, while `Continue` executes the next task.<br/><br/>Default: "Fail" |
| `destination` | [SyncEndpoint](#syncendpoint-schema) | V |  | Remote dataset to push to. |
| `force` | `boolean` |  |  | Overwrite the remote dataset even if its history has diverged from the local one.<br/><br/>Default: false |
| `createIfNotExists` | `boolean` |  |  | Create the remote dataset if it does not exist.<br/><br/>Default: true |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskspec-transform-schema"></a>
### TaskSpec::Transform
Executes transformation of data defined in a derivative dataset.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` |  |  | An alias for the task used to refer to it in flows and access the results |
| `target` | [DatasetHandle](#datasethandle-schema) |  |  | Reference to the derivative dataset that defines how to transform data. |
| `serviceAccount` | [PrincipalHandle](#principalhandle-schema) |  |  | Service account whose permissions the task executes with. Must be an account of service account type. Defaults to the service account of the run when omitted. |
| `onNoOp` | [NoOpBehavior](#noopbehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `NoOp` outcome: `Continue` executes the next task, while `Break` finishes the flow run early without an error.<br/><br/>Default: "Continue" |
| `onFailure` | [FailureBehavior](#failurebehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `Failed` outcome: `Fail` finishes the flow run with an error, while `Continue` executes the next task.<br/><br/>Default: "Fail" |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskspec-verify-schema"></a>
### TaskSpec::Verify
Checks dataset metadata for integrity.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` |  |  | An alias for the task used to refer to it in flows and access the results |
| `target` | [DatasetHandle](#datasethandle-schema) |  |  | Reference to the dataset to verify. Defaults to the flow-level target when omitted. |
| `serviceAccount` | [PrincipalHandle](#principalhandle-schema) |  |  | Service account whose permissions the task executes with. Must be an account of service account type. Defaults to the service account of the run when omitted. |
| `onNoOp` | [NoOpBehavior](#noopbehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `NoOp` outcome: `Continue` executes the next task, while `Break` finishes the flow run early without an error.<br/><br/>Default: "Continue" |
| `onFailure` | [FailureBehavior](#failurebehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `Failed` outcome: `Fail` finishes the flow run with an error, while `Continue` executes the next task.<br/><br/>Default: "Fail" |
| `replayTransform` | `boolean` |  |  | If true, re-executes transformations on derivative datasets to verify reproducibility. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskspec-webhookcall-schema"></a>
### TaskSpec::WebhookCall
Dispatches a certain payload to a specific `WebhookEndpoint`.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` |  |  | An alias for the task used to refer to it in flows and access the results |
| `serviceAccount` | [PrincipalHandle](#principalhandle-schema) |  |  | Service account whose permissions the task executes with. Must be an account of service account type. Defaults to the service account of the run when omitted. |
| `onNoOp` | [NoOpBehavior](#noopbehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `NoOp` outcome: `Continue` executes the next task, while `Break` finishes the flow run early without an error.<br/><br/>Default: "Continue" |
| `onFailure` | [FailureBehavior](#failurebehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `Failed` outcome: `Fail` finishes the flow run with an error, while `Continue` executes the next task.<br/><br/>Default: "Fail" |
| `endpoint` | [ResourceHandle](#resourcehandle-schema) | V |  | Reference to the `WebhookEndpoint`. |
| `payload` | `string` |  |  | The payload to send. May include templating. |
| `retryPolicy` | [RetryPolicy](#retrypolicy-schema) |  |  | Defines how a webhook should react to failures. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskSpec.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskspecinput-schema"></a>
### TaskSpecInput
An individual work item to be executed as part of a flow.

| Union Type | Description |
| :---: | --- |
| [TaskSpecInput::Ingest](#taskspecinput-ingest-schema) | Fetches data from a source and appends it to a dataset. |
| [TaskSpecInput::Transform](#taskspecinput-transform-schema) | Executes transformation of data defined in a derivative dataset. |
| [TaskSpecInput::SyncFrom](#taskspecinput-syncfrom-schema) | Pulls new blocks from a remote dataset into a local dataset. |
| [TaskSpecInput::SyncTo](#taskspecinput-syncto-schema) | Pushes new blocks of a local dataset to a remote dataset. |
| [TaskSpecInput::Compaction](#taskspecinput-compaction-schema) | Compacts data files in matching datasets to improve query performance. |
| [TaskSpecInput::Reset](#taskspecinput-reset-schema) | Moves a block reference of a dataset to an earlier block, discarding the history that follows it. |
| [TaskSpecInput::GarbageCollection](#taskspecinput-garbagecollection-schema) | Removes unreferenced data files from matching datasets. |
| [TaskSpecInput::Verify](#taskspecinput-verify-schema) | Checks dataset metadata for integrity. |
| [TaskSpecInput::WebhookCall](#taskspecinput-webhookcall-schema) | Dispatches a certain payload to a specific `WebhookEndpoint`. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskspecinput-compaction-schema"></a>
### TaskSpecInput::Compaction
Compacts data files in matching datasets to improve query performance.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` |  |  | An alias for the task used to refer to it in flows and access the results |
| `target` | [DatasetRef](#datasetref-schema) |  |  | Reference to the dataset to compact. Defaults to the flow-level target when omitted. |
| `serviceAccount` | [PrincipalRef](#principalref-schema) |  |  | Service account whose permissions the task executes with. Must be an account of service account type. Defaults to the service account of the run when omitted. |
| `onNoOp` | [NoOpBehavior](#noopbehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `NoOp` outcome: `Continue` executes the next task, while `Break` finishes the flow run early without an error.<br/><br/>Default: "Continue" |
| `onFailure` | [FailureBehavior](#failurebehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `Failed` outcome: `Fail` finishes the flow run with an error, while `Continue` executes the next task.<br/><br/>Default: "Fail" |
| `maxSliceSize` | `string` |  | [byte-size](https://www.thierry-lequeu.fr/data/PELS/Comm/Publications/Newsletter/9704/STORY18.HTML) | Target maximum size of each compacted data slice e.g. `100MiB`. |
| `maxSliceRecords` | `integer` |  | `uint64` | Target maximum number of records per compacted data slice. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskspecinput-garbagecollection-schema"></a>
### TaskSpecInput::GarbageCollection
Removes unreferenced data files from matching datasets.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` |  |  | An alias for the task used to refer to it in flows and access the results |
| `target` | [DatasetRef](#datasetref-schema) |  |  | Reference to the dataset to collect garbage from. Defaults to the flow-level target when omitted. |
| `serviceAccount` | [PrincipalRef](#principalref-schema) |  |  | Service account whose permissions the task executes with. Must be an account of service account type. Defaults to the service account of the run when omitted. |
| `onNoOp` | [NoOpBehavior](#noopbehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `NoOp` outcome: `Continue` executes the next task, while `Break` finishes the flow run early without an error.<br/><br/>Default: "Continue" |
| `onFailure` | [FailureBehavior](#failurebehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `Failed` outcome: `Fail` finishes the flow run with an error, while `Continue` executes the next task.<br/><br/>Default: "Fail" |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskspecinput-ingest-schema"></a>
### TaskSpecInput::Ingest
Fetches data from a source and appends it to a dataset.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` |  |  | An alias for the task used to refer to it in flows and access the results |
| `target` | [DatasetRef](#datasetref-schema) |  |  | Reference to the dataset to ingest into. Defaults to the flow-level target when omitted. |
| `serviceAccount` | [PrincipalRef](#principalref-schema) |  |  | Service account whose permissions the task executes with. Must be an account of service account type. Defaults to the service account of the run when omitted. |
| `onNoOp` | [NoOpBehavior](#noopbehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `NoOp` outcome: `Continue` executes the next task, while `Break` finishes the flow run early without an error.<br/><br/>Default: "Continue" |
| `onFailure` | [FailureBehavior](#failurebehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `Failed` outcome: `Fail` finishes the flow run with an error, while `Continue` executes the next task.<br/><br/>Default: "Fail" |
| `source` | [ResourceRef](#resourceref-schema) | V |  | Reference to the source resource that defines how to fetch data. |
| `targetRecordsPerSlice` | `integer` |  | `uint64` | Target number of records to ingest per data slice. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskspecinput-reset-schema"></a>
### TaskSpecInput::Reset
Moves a block reference of a dataset to an earlier block, discarding the history that follows it.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` |  |  | An alias for the task used to refer to it in flows and access the results |
| `target` | [DatasetRef](#datasetref-schema) |  |  | Reference to the dataset to reset. Defaults to the flow-level target when omitted. |
| `serviceAccount` | [PrincipalRef](#principalref-schema) |  |  | Service account whose permissions the task executes with. Must be an account of service account type. Defaults to the service account of the run when omitted. |
| `onNoOp` | [NoOpBehavior](#noopbehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `NoOp` outcome: `Continue` executes the next task, while `Break` finishes the flow run early without an error.<br/><br/>Default: "Continue" |
| `onFailure` | [FailureBehavior](#failurebehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `Failed` outcome: `Fail` finishes the flow run with an error, while `Continue` executes the next task.<br/><br/>Default: "Fail" |
| `ref` | `string` |  |  | Name of the block reference to reset.<br/><br/>Default: "head" |
| `newBlockHash` | `string` |  | [multihash](https://github.com/multiformats/multihash) | Hash of the block the reference will point to. Defaults to the `Seed` block when omitted. |
| `oldBlockHash` | `string` |  | [multihash](https://github.com/multiformats/multihash) | Expected hash of the block the reference currently points to. When specified, the task fails if the reference has moved, protecting against concurrent updates. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskspecinput-syncfrom-schema"></a>
### TaskSpecInput::SyncFrom
Pulls new blocks from a remote dataset into a local dataset.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` |  |  | An alias for the task used to refer to it in flows and access the results |
| `target` | [DatasetRef](#datasetref-schema) |  |  | Reference to the local dataset to pull into. Defaults to the flow-level target when omitted. |
| `serviceAccount` | [PrincipalRef](#principalref-schema) |  |  | Service account whose permissions the task executes with. Must be an account of service account type. Defaults to the service account of the run when omitted. |
| `onNoOp` | [NoOpBehavior](#noopbehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `NoOp` outcome: `Continue` executes the next task, while `Break` finishes the flow run early without an error.<br/><br/>Default: "Continue" |
| `onFailure` | [FailureBehavior](#failurebehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `Failed` outcome: `Fail` finishes the flow run with an error, while `Continue` executes the next task.<br/><br/>Default: "Fail" |
| `source` | [SyncEndpointInput](#syncendpointinput-schema) | V |  | Remote dataset to pull from. |
| `force` | `boolean` |  |  | Overwrite the local dataset even if its history has diverged from the source.<br/><br/>Default: false |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskspecinput-syncto-schema"></a>
### TaskSpecInput::SyncTo
Pushes new blocks of a local dataset to a remote dataset.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` |  |  | An alias for the task used to refer to it in flows and access the results |
| `target` | [DatasetRef](#datasetref-schema) |  |  | Reference to the local dataset to push. Defaults to the flow-level target when omitted. |
| `serviceAccount` | [PrincipalRef](#principalref-schema) |  |  | Service account whose permissions the task executes with. Must be an account of service account type. Defaults to the service account of the run when omitted. |
| `onNoOp` | [NoOpBehavior](#noopbehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `NoOp` outcome: `Continue` executes the next task, while `Break` finishes the flow run early without an error.<br/><br/>Default: "Continue" |
| `onFailure` | [FailureBehavior](#failurebehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `Failed` outcome: `Fail` finishes the flow run with an error, while `Continue` executes the next task.<br/><br/>Default: "Fail" |
| `destination` | [SyncEndpointInput](#syncendpointinput-schema) | V |  | Remote dataset to push to. |
| `force` | `boolean` |  |  | Overwrite the remote dataset even if its history has diverged from the local one.<br/><br/>Default: false |
| `createIfNotExists` | `boolean` |  |  | Create the remote dataset if it does not exist.<br/><br/>Default: true |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskspecinput-transform-schema"></a>
### TaskSpecInput::Transform
Executes transformation of data defined in a derivative dataset.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` |  |  | An alias for the task used to refer to it in flows and access the results |
| `target` | [DatasetRef](#datasetref-schema) |  |  | Reference to the derivative dataset that defines how to transform data. |
| `serviceAccount` | [PrincipalRef](#principalref-schema) |  |  | Service account whose permissions the task executes with. Must be an account of service account type. Defaults to the service account of the run when omitted. |
| `onNoOp` | [NoOpBehavior](#noopbehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `NoOp` outcome: `Continue` executes the next task, while `Break` finishes the flow run early without an error.<br/><br/>Default: "Continue" |
| `onFailure` | [FailureBehavior](#failurebehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `Failed` outcome: `Fail` finishes the flow run with an error, while `Continue` executes the next task.<br/><br/>Default: "Fail" |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskspecinput-verify-schema"></a>
### TaskSpecInput::Verify
Checks dataset metadata for integrity.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` |  |  | An alias for the task used to refer to it in flows and access the results |
| `target` | [DatasetRef](#datasetref-schema) |  |  | Reference to the dataset to verify. Defaults to the flow-level target when omitted. |
| `serviceAccount` | [PrincipalRef](#principalref-schema) |  |  | Service account whose permissions the task executes with. Must be an account of service account type. Defaults to the service account of the run when omitted. |
| `onNoOp` | [NoOpBehavior](#noopbehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `NoOp` outcome: `Continue` executes the next task, while `Break` finishes the flow run early without an error.<br/><br/>Default: "Continue" |
| `onFailure` | [FailureBehavior](#failurebehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `Failed` outcome: `Fail` finishes the flow run with an error, while `Continue` executes the next task.<br/><br/>Default: "Fail" |
| `replayTransform` | `boolean` |  |  | If true, re-executes transformations on derivative datasets to verify reproducibility. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskspecinput-webhookcall-schema"></a>
### TaskSpecInput::WebhookCall
Dispatches a certain payload to a specific `WebhookEndpoint`.

| Property | Type | Required | Format | Description |
| :---: | :---: | :---: | :---: | --- |
| `name` | `string` |  |  | An alias for the task used to refer to it in flows and access the results |
| `serviceAccount` | [PrincipalRef](#principalref-schema) |  |  | Service account whose permissions the task executes with. Must be an account of service account type. Defaults to the service account of the run when omitted. |
| `onNoOp` | [NoOpBehavior](#noopbehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `NoOp` outcome: `Continue` executes the next task, while `Break` finishes the flow run early without an error.<br/><br/>Default: "Continue" |
| `onFailure` | [FailureBehavior](#failurebehavior-schema) |  |  | How the flow run proceeds when this task finishes with a `Failed` outcome: `Fail` finishes the flow run with an error, while `Continue` executes the next task.<br/><br/>Default: "Fail" |
| `endpoint` | [ResourceRef](#resourceref-schema) | V |  | Reference to the `WebhookEndpoint`. |
| `payload` | `string` |  |  | The payload to send. May include templating. |
| `retryPolicy` | [RetryPolicy](#retrypolicy-schema) |  |  | Defines how a webhook should react to failures. |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskSpecInput.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)


<a name="taskstatus-schema"></a>
### TaskStatus
Execution phase of a task.

| Enum Value |
| :---: |
| Pending |
| Queued |
| Running |
| Finished |

[![JSON Schema](https://img.shields.io/badge/schema-JSON-orange)](../schemas/tasks/v1alpha1/TaskStatus.json)
[![Flatbuffers Schema](https://img.shields.io/badge/schema-flatbuffers-blue)](../contracts/metadata/opendatafabric.fbs)
[^](#table-of-contents)

