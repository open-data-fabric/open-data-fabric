# Tasks and Flows <!-- omit in toc -->

**Domain**: Automation\
**Depends on**: [Resource Framework](resource-framework.md), [Authorization](../auth-and-privacy/authorization.md)\
**RFCs**: [RFC-019](../../rfcs/019-iac-task-flow-system.md)


## Abstract <!-- omit in toc -->

This spec defines the resources used to define, schedule, and execute workloads within ODF [Nodes](../glossary.md#node). A `Task` represents a single unit of work like ingesting or transforming data of a [Dataset](../glossary.md#dataset), a `FlowRun` executes a sequence of tasks, and a `Flow` acts as a template that instantiates flow runs for a set of target resources when its triggers fire.


## Table of Contents <!-- omit in toc -->

- [Overview](#overview)
- [Task](#task)
  - [Task Lifecycle](#task-lifecycle)
  - [Task Outcome](#task-outcome)
  - [Task Types](#task-types)
- [FlowRun](#flowrun)
- [Flow](#flow)
  - [Target Selector](#target-selector)
  - [Retry Policy](#retry-policy)
- [Triggers](#triggers)
- [Authorization](#authorization)
- [Schemas](#schemas)


## Overview
The system is built from three [Resource](../glossary.md#resource) types defined using the [Resource Framework](resource-framework.md):

| Resource | Description |
| --- | --- |
| [`Task`](#task) | A single unit of work, from intent through execution to its outcome |
| [`FlowRun`](#flowrun) | A sequence of tasks executed one after another |
| [`Flow`](#flow) | A template for instantiating flow runs, with triggers that decide when to instantiate them |

Tasks and flow runs are short-lived - they are deleted as soon as they finish, so the existing resources of these types are exactly the work that is pending or in progress. Deleted resources stay readable for a configurable retention period (e.g. 10 days) before being purged, allowing operators to inspect recent runs.


## Task
A `Task` resource represents a single unit of work. Its `spec` captures the **intent**: the kind of operation, the target resource, and high-level parameters. This is what an operator or a `FlowRun` controller writes when creating a task. The spec is stable, human-readable, and never rewritten.

```yaml
$schema: https://opendatafabric.org/schemas/tasks/v1alpha1/Task
headers:
  name: task-123456
  ownerReferences:
    - FlowRun:c27331ce-ce88-4ff9-8c5a-4ce8107cc03f  # FlowRun that spawned this task
spec:
  kind: Transform
  target: Dataset:sergiimk/foo
```

Tasks are executed by **workers**. Every worker specializes in one task type and pulls the next task of that type from the node's **queue**, which schedules and prioritizes tasks. The worker advances the task through the steps its type requires - e.g. fetching data for an ingest, or elaborating the execution plan of a transform, requesting an [Engine](../glossary.md#engine) and dispatching the plan to it - and commits the result.

As it progresses, the worker records the state of the task in type-specific status [conditions](resource-framework.md#conditions):
- [`TaskStatus`](../../schemas/tasks/v1alpha1/TaskStatus.json) - execution phase of the task (see [Task Lifecycle](#task-lifecycle))
- [`TaskPlan`](../../schemas/tasks/v1alpha1/TaskPlan.json) - a self-contained logical execution plan with fully resolved paths, offsets, schemas, and all other inputs
- [`TaskOutcome`](../../schemas/tasks/v1alpha1/TaskOutcome.json) - result of the execution (see [Task Outcome](#task-outcome))

Example of a task after planning and execution:

```yaml
$schema: https://opendatafabric.org/schemas/tasks/v1alpha1/Task
headers:
  name: task-123456
  ownerReferences:
    - FlowRun:c27331ce-ce88-4ff9-8c5a-4ce8107cc03f
  deletedAt: 2026-09-11T02:23:41Z  # Deleted upon completion, readable until the retention period ends
spec:
  kind: Transform
  target: Dataset:sergiimk/foo
status:
  phase: Deleted
  observedGeneration: 1
  conditions:
    https://opendatafabric.org/schemas/tasks/v1alpha1/TaskStatus: Finished
    https://opendatafabric.org/schemas/tasks/v1alpha1/TaskPlan:
      kind: TransformPlan
      datasetId: did:odf:fed0..17bf
      systemTime: 2026-09-11T02:22:52Z
      queryInputs:
        - datasetId: did:odf:fed0..685b
          offsetInterval:
            start: 0
            end: 399493
          dataPaths:
            - s3://repo/f162..8a9f/data/0a..fa
      transform:
        kind: Sql
        query: SELECT ... FROM ...
      newDataPath: s3://repo/f162..8a9f/data/...
      newCheckpointPath: s3://repo/f162..8a9f/checkpoint/...
    https://opendatafabric.org/schemas/tasks/v1alpha1/TaskOutcome:
      kind: Success
      result:
        kind: TransformResult
        newBlockHash: f162..f008
        newOffsetInterval:
          start: 0
          end: 1050
        newWatermark: 2023-04-15T00:00:00Z
```

The structure of plans and results is specific to the task type.


### Task Lifecycle
The `TaskStatus` condition proceeds through the following phases:

| Status | Description |
| --- | --- |
| `Pending` | Task was created, and the node is validating its inputs and resolving what is needed to schedule it (e.g. the engine a transform uses) |
| `Queued` | Task is waiting in the queue for a worker |
| `Running` | A worker has claimed the task and is executing it, including the commit |
| `Finished` | Terminal - the `TaskOutcome` condition is set, and the task resource is deleted |

The resource [phase](resource-framework.md#phases) reflects the work of the **task controller**, whose job is to admit the task: validate its inputs, resolve what is needed to schedule it, and put it into the queue. `phase: Ready` means the task was admitted, while `phase: Failed` means it was rejected - in which case the controller also sets an unrecoverable `Failed` outcome. Execution is reported separately by workers in the `TaskStatus` condition, so an admitted task stays in `phase: Ready` regardless of its `TaskStatus` until it is deleted.

Deleting a task that has not finished yet **cancels** it when possible. Until its outcome is recorded, a task being deleted remains visible in `phase: Deleting`.


### Task Outcome
The `TaskOutcome` condition is one of:

| Outcome | Description |
| --- | --- |
| `Success` | Task completed successfully, with a type-specific `result` |
| `Failed` | Task failed |
| `NoOp` | Task completed with no work done (e.g. no new data to process). Such a task may have no `TaskPlan` if the worker realized there is nothing to do |
| `Cancelled` | Task was cancelled before completion |

A `Failed` outcome carries a human-readable `message`, and tells whether the failure is `recoverable`, i.e. whether retrying the task could succeed. Recoverable failures (e.g. a network error) can be retried according to the [retry policy](#retry-policy), while unrecoverable ones (e.g. an invalid query) would fail the same way again. Type-specific details of the failure are carried in `error` ([`TaskError`](../../schemas/tasks/v1alpha1/TaskError.json)):

```yaml
https://opendatafabric.org/schemas/tasks/v1alpha1/TaskOutcome:
  kind: Failed
  message: Input dataset was compacted since the last transformation
  recoverable: false
  error:
    kind: InputDatasetCompacted
    inputDataset: Dataset:sergiimk/sensor-temp
```


### Task Types
The `kind` field of the task spec ([`TaskSpec`](../../schemas/tasks/v1alpha1/TaskSpec.json)) selects one of the following task types. All types accept:
- `name` - an alias used to refer to the task in a flow and access its results
- `target` - reference to the target resource, which defaults to the target of the flow run when omitted
- `serviceAccount` - see [Authorization](#authorization)
- `onNoOp` - how the flow run proceeds when the task finishes with a `NoOp` outcome: `Continue` (default) executes the next task, while `Break` finishes the flow run early without an error
- `onFailure` - how the flow run proceeds when the task finishes with a `Failed` outcome: `Fail` (default) finishes the flow run with an error, while `Continue` executes the next task

`Ingest` - fetches data from a `Source` and appends it to a [Root Dataset](../glossary.md#root-dataset) (see [Ingestion](../datasets/ingestion.md)):

```yaml
kind: Ingest
target: Dataset:sergiimk/sensor-temp
source: Source:sergiimk/sensor-temp-http
targetRecordsPerSlice: 10000  # optional: target number of records per data slice
```

`Transform` - executes the transformation of data defined in a [Derivative Dataset](../glossary.md#derivative-dataset):

```yaml
kind: Transform
target: Dataset:sergiimk/sensor-temp-daily
```

`SyncFrom` - pulls new blocks from a remote dataset into a local dataset (see [Transfer Protocols](../exchange/transfer-protocols.md)):

```yaml
kind: SyncFrom
target: Dataset:sergiimk/sensor-temp
source:
  url: odf+https://node.example.com/acme/sensor-temp  # scheme selects the transfer protocol
  auth:
    kind: Bearer
    token: SecretSet:acme-node#accessToken
force: false  # optional: overwrite the local dataset even if histories have diverged
```

`SyncTo` - pushes new blocks of a local dataset to a remote dataset:

```yaml
kind: SyncTo
target: Dataset:sergiimk/sensor-temp
destination:
  url: s3://my-bucket/datasets/sensor-temp/
  auth:
    kind: Aws
    region: us-west-2
    accessKey: SecretSet:my-aws-secrets#accessKey
    secretKey: SecretSet:my-aws-secrets#secretKey
force: false  # optional: overwrite the remote dataset even if histories have diverged
createIfNotExists: true  # optional: create the remote dataset if it does not exist
```

Sync endpoints ([`SyncEndpoint`](../../schemas/tasks/v1alpha1/SyncEndpoint.json)) specify the `url` of the remote dataset, where the scheme selects the transfer protocol (e.g. `odf+https://`, `s3://`, `file://`), and optional credentials ([`SyncAuth`](../../schemas/tasks/v1alpha1/SyncAuth.json)):
- `Bearer` - passes a token, e.g. an access token of a remote ODF node
- `Aws` - credentials for AWS or an AWS-compatible object storage
- `Headers` - custom request headers with values referencing variables and secrets

The execution plan of a sync task carries references to secrets and never their values - these are resolved only during execution.

`Compaction` - compacts data files of a dataset to improve query performance:

```yaml
kind: Compaction
target: Dataset:sergiimk/sensor-temp
maxSliceSize: 100MiB  # optional: target maximum size of each compacted data slice
maxSliceRecords: 10000  # optional: target maximum number of records per data slice
```

`Reset` - moves a block reference of a dataset to an earlier block, discarding the history that follows it:

```yaml
kind: Reset
target: Dataset:sergiimk/sensor-temp
ref: head  # optional: block reference to reset
newBlockHash: f162..8a9f  # optional: block the reference will point to, defaults to the `Seed` block
oldBlockHash: f162..f008  # optional: expected current block, task fails if the reference has moved
```

`GarbageCollection` - removes unreferenced data files of a dataset:

```yaml
kind: GarbageCollection
target: Dataset:sergiimk/sensor-temp
```

`Verify` - checks dataset metadata for integrity, optionally replaying transformations to verify their reproducibility:

```yaml
kind: Verify
target: Dataset:sergiimk/sensor-temp
replayTransform: true  # optional
```

`WebhookCall` - dispatches a payload to a [`WebhookEndpoint`](../../schemas/sinks/v1alpha1/WebhookEndpoint.json) resource:

```yaml
kind: WebhookCall
endpoint: WebhookEndpoint:sergiimk/notify-slack
payload: '{"event": "ingest-complete", "dataset": "{{task.source}}"}'  # optional, supports templating
retryPolicy:  # optional: overrides the retry policy of the flow for this task
  maxAttempts: 5
  minDelay: 30s
  backoff: Exponential
```


## FlowRun
A `FlowRun` resource ([`FlowRunSpec`](../../schemas/flows/v1alpha1/FlowRunSpec.json)) is a set of tasks to be executed in a sequence:

```yaml
$schema: https://opendatafabric.org/schemas/flows/v1alpha1/FlowRun
headers:
  name: flow-run-123456
  ownerReferences:
    - Flow:c27331ce-ce88-4ff9-8c5a-4ce8107cc03f  # Flow that spawned this run
spec:
  target: Dataset:sergiimk/foo
  tasks:
    - kind: Transform  # Implicit name: task-0-transform
    - kind: GarbageCollection
      name: gc
```

The `target` of the flow run is used as the default `target` of its tasks to avoid duplication.

The run controller spawns `Task` resources one after another and tracks the progress in the following conditions:
- [`FlowRunStatus`](../../schemas/flows/v1alpha1/FlowRunStatus.json) - overall status of the run (`Waiting`, `Running`, or `Finished`) and the list of spawned tasks with their names, references, statuses, and outcomes
- [`FlowRunActivationCauses`](../../schemas/flows/v1alpha1/FlowRunActivationCauses.json) - what led to the execution of the run: the time, the initiating [Principal](../glossary.md#principal), and a copy of the trigger configuration. Triggers that fired while the run was already queued or executing are recorded as `lateActivationCauses`. In case of a retry, the causes of the original run are preserved.
- [`FlowRunRetry`](../../schemas/flows/v1alpha1/FlowRunRetry.json) - reference to the previous run and the retry number, if this run is a retry

```yaml
status:
  phase: Ready
  conditions:
    https://opendatafabric.org/schemas/flows/v1alpha1/FlowRunStatus:
      status: Running
      tasks:
        - name: task-0-transform
          task: Task:a1b2c3d4-1111-2222-3333-444444444444
          status: Finished
          outcome:
            kind: Success
          lastUpdatedAt: 2026-09-11T02:00:00Z
        - name: gc
          task: Task:e5f6a7b8-5555-6666-7777-888888888888
          status: Running
          lastUpdatedAt: 2026-09-11T02:02:00Z
    https://opendatafabric.org/schemas/flows/v1alpha1/FlowRunActivationCauses:
      activationCauses:
        - activationTime: 2026-09-11T02:00:00Z
          initiator: User:alice
          trigger:
            kind: Manual
      lateActivationCauses: []
    https://opendatafabric.org/schemas/flows/v1alpha1/FlowRunRetry:
      retryOf: FlowRun:9b2e4f1a-3c7d-4e8b-a1f2-6d5e7c8b9a0d
      retryNumber: 2  # Second retry, i.e. third run total
```

Activation causes and retry information are known when the run is created, but they are part of `status` rather than `spec`, because they can only be reliably set by the controller, not by a user.

Like tasks, flow runs are **deleted as soon as the run finishes** and stay readable for the retention period. Deleting a run that has not finished yet cancels it, together with its unfinished tasks, which reference the run in `ownerReferences`.

A run created by a flow copies the `serviceAccount` of the `Flow` into its own spec, so changes to the flow affect only future runs, never runs in progress.


## Flow
A `Flow` resource ([`FlowSpec`](../../schemas/flows/v1alpha1/FlowSpec.json)) acts as a template for instantiating flow runs, and defines the triggers that decide when to instantiate them:

```yaml
$schema: https://opendatafabric.org/schemas/flows/v1alpha1/Flow
headers:
  name: compact-and-gc-roots
spec:
  target:  # Selector matching all root datasets under `sergiimk`
    type: Dataset
    account: sergiimk
    name: '%'
    labels:
      DatasetKind: Root
  triggers:
    - kind: Schedule
      cron: "@daily"
  tasks:
    - kind: Compaction
      maxSliceSize: 100MiB
      maxSliceRecords: 10000
      onNoOp: Break  # Skip garbage collection if there was nothing to compact
    - kind: GarbageCollection
  retryPolicy:
    maxAttempts: 3
    minDelay: 1min
    backoff: Exponential
  recentBindingsRetention:
    maxBindings: 10
  recentRunsRetention:
    maxRuns: 100
    maxAge: 30d
```

The `target` of the flow is used as the default target of its triggers and tasks, to avoid duplication while still allowing overrides (e.g. an `InputsUpdated` trigger watching a different dataset than the one the tasks operate on).

The flow controller reports the state of the flow in the [`FlowStatus`](../../schemas/flows/v1alpha1/FlowStatus.json) condition:
- `status` - `Active` or `Paused`
- `recentBindings` and `bindingsTotal` - the last resources matched by the [target selector](#target-selector) and their total number
- `recentRuns` - summaries of recent flow runs, including their status, outcome, and retry information

The number of reported bindings and runs is controlled by `recentBindingsRetention` ([`RecentBindingsRetention`](../../schemas/flows/v1alpha1/RecentBindingsRetention.json)) and `recentRunsRetention` ([`RecentRunsRetention`](../../schemas/flows/v1alpha1/RecentRunsRetention.json)). Implementations may additionally cap these values.


### Target Selector
The `target` of a flow is a [selector](resource-framework.md#selectors). When it matches a resource, the flow controller sets up triggers for that resource automatically. As resources are created or deleted, the controller updates its set of active targets accordingly.

The association between a flow and its matched resources is a derived state maintained by the controller. Implementations may provide a dedicated API for querying this index, e.g. to list flows by target or targets by flow.


### Retry Policy
The `retryPolicy` ([`RetryPolicy`](../../schemas/flows/v1alpha1/RetryPolicy.json)) defines how a flow reacts to recoverable failures:
- `maxAttempts` - number of attempts before automatic scheduling of the flow is disabled
- `minDelay` - how long to wait until the first retry
- `backoff` - how the delay is scaled with subsequent attempts: `Linear` or `Exponential`. When omitted, retries use a constant delay

Tasks like `WebhookCall` may override the retry policy of the flow.

A retry creates a new `FlowRun` that references the failed one in its `FlowRunRetry` condition.


## Triggers
A flow specifies a set of triggers ([`FlowTrigger`](../../schemas/flows/v1alpha1/FlowTrigger.json)) that decide when to instantiate a `FlowRun`:

| Kind | Description |
| --- | --- |
| `Manual` | Fires on an API call or a UI action |
| `Schedule` | Fires on a [cron](https://en.wikipedia.org/wiki/Cron) schedule, e.g. `@daily` or `*/30 * * * *` |
| `Interval` | Fires at regular intervals, e.g. every `15m` |
| `Source` | Fires when a [source](../datasets/ingestion.md#sources) receives new data |
| `InputsUpdated` | Fires when inputs of the derivative datasets matching the `target` selector (defaulting to the flow target) receive new data |
| `Event` | Fires when an event bus event matching the filters is observed |

All triggers support the following common properties:
- `enabled` - allows to pause an individual trigger by setting it to `false`
- `cooldown` - minimum interval between activations of the trigger. The trigger fires upon the first activation, but if it fires again within the `cooldown` interval, the activation is postponed until the interval ends, batching multiple activations into one flow run
- `cooldownMaxBatch` - if the trigger is activated this many times during the `cooldown` interval, it fires without waiting for the interval to end

`Schedule` and `Interval` triggers are guaranteed to fire if the node was down when the next tick was supposed to happen, but fire only once for all missed ticks.

`InputsUpdated` triggers allow batching of new input data, e.g. to transform a derivative dataset only once enough new records accumulate in its inputs:

```yaml
kind: InputsUpdated
minRecordsToAwait: 100  # wait until this many new input records accumulate
maxAwaitInterval: 1h  # but run at least once an hour
```

`Source` triggers similarly allow batching of pushed data:

```yaml
kind: Source
source: Source:sensor.temp.http
minRecordsToAwait: 100  # wait until this many new records accumulate
maxAwaitInterval: 1h  # but run at least once an hour
```

`Event` triggers are a low-level mechanism that should be used sparingly, e.g. to run a flow on dataset `A` whenever dataset `B` is updated. They specify the event filters ([`EventFilter`](../../schemas/events/v1alpha1/EventFilter.json)), and are commonly combined with `cooldown` to batch multiple events into one flow run.

```yaml
kind: Event
events:
  type: dataset.ref.updated
cooldown: 10min
```


## Authorization
A task executes with the permissions of the user [Principal](../glossary.md#principal) who created it.

Tasks, flow runs, and flows can specify a `serviceAccount` - a non-human principal that gets permissions only through explicit policy bindings (see [Authorization](../auth-and-privacy/authorization.md#service-account)):
- A flow owned by an [Organization](../auth-and-privacy/authorization.md#organization) must specify a service account. When omitted for flows owned by users, tasks execute with the permissions of the user account that owns the flow.
- A flow run inherits the service account of its flow, and a task inherits the service account of its run, unless overridden.

```yaml
$schema: https://opendatafabric.org/schemas/flows/v1alpha1/Flow
headers:
  account: acme
  name: ingest-sensors
spec:
  serviceAccount: ServiceAccount:acme/ingest-bot
  target: Dataset:acme/sensor-temp
  # ...
```


## Schemas

| Schema | Description |
| --- | --- |
| [`Task`](../../schemas/tasks/v1alpha1/Task.json) | Task resource |
| [`TaskSpec`](../../schemas/tasks/v1alpha1/TaskSpec.json) | Task types and their parameters |
| [`TaskStatus`](../../schemas/tasks/v1alpha1/TaskStatus.json) | Execution phase of a task |
| [`TaskPlan`](../../schemas/tasks/v1alpha1/TaskPlan.json) | Execution plan of a task |
| [`TaskOutcome`](../../schemas/tasks/v1alpha1/TaskOutcome.json) | Result of a task |
| [`TaskError`](../../schemas/tasks/v1alpha1/TaskError.json) | Type-specific details of a task failure |
| [`NoOpBehavior`](../../schemas/tasks/v1alpha1/NoOpBehavior.json) | How a flow run proceeds after a `NoOp` task |
| [`FailureBehavior`](../../schemas/tasks/v1alpha1/FailureBehavior.json) | How a flow run proceeds after a failed task |
| [`SyncEndpoint`](../../schemas/tasks/v1alpha1/SyncEndpoint.json) | Remote location of a sync task |
| [`SyncAuth`](../../schemas/tasks/v1alpha1/SyncAuth.json) | Credentials of a sync task |
| [`FlowRun`](../../schemas/flows/v1alpha1/FlowRun.json) | Flow run resource |
| [`FlowRunSpec`](../../schemas/flows/v1alpha1/FlowRunSpec.json) | Tasks of a flow run |
| [`FlowRunStatus`](../../schemas/flows/v1alpha1/FlowRunStatus.json) | Execution status of a flow run |
| [`FlowRunActivationCauses`](../../schemas/flows/v1alpha1/FlowRunActivationCauses.json) | What caused a flow run |
| [`FlowRunRetry`](../../schemas/flows/v1alpha1/FlowRunRetry.json) | Link to the run being retried |
| [`Flow`](../../schemas/flows/v1alpha1/Flow.json) | Flow resource |
| [`FlowSpec`](../../schemas/flows/v1alpha1/FlowSpec.json) | Target, triggers, tasks, and retry policy of a flow |
| [`FlowStatus`](../../schemas/flows/v1alpha1/FlowStatus.json) | Status and recent runs of a flow |
| [`FlowTrigger`](../../schemas/flows/v1alpha1/FlowTrigger.json) | Conditions that cause a flow to execute |
| [`RetryPolicy`](../../schemas/flows/v1alpha1/RetryPolicy.json) | Reaction to failures |
| [`RecentBindingsRetention`](../../schemas/flows/v1alpha1/RecentBindingsRetention.json) | Retention of recent bindings in flow status |
| [`RecentRunsRetention`](../../schemas/flows/v1alpha1/RecentRunsRetention.json) | Retention of recent runs in flow status |
