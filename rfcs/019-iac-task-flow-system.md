# RFC-019: IaC Task and Flow System <!-- omit in toc -->

[![PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/131?label=PR)](https://github.com/kamu-data/open-data-fabric/pull/131)

**Start Date**: 2026-10-10

**Published Date**: 2026-10-22

**Authors**:
- [Sergii Mikhtoniuk](mailto:smikhtoniuk@kamu.dev), [Kamu](https://kamu.dev)
- [Sergiy Zaychenko](mailto:sergiy.zaychenko@kamu.dev), [Kamu](https://kamu.dev)


**Compatibility**:
- [X] Backwards-compatible
- [ ] Forwards-compatible


## Summary <!-- omit in toc -->
This RFC proposes a new set of resources to define and control workload scheduling and execution within ODF nodes.

## Table of Contents <!-- omit in toc -->

- [Current Prototype Implementation in Kamu](#current-prototype-implementation-in-kamu)
- [Proposed IaC-based System](#proposed-iac-based-system)
  - [Task](#task)
    - [Task Types](#task-types)
  - [`FlowRun`](#flowrun)
  - [`Flow`](#flow)
    - [Target Selector](#target-selector)
  - [`FlowTrigger`](#flowtrigger)
- [Appendix A: Flow System Prototype in Kamu](#appendix-a-flow-system-prototype-in-kamu)

## Current Prototype Implementation in Kamu
Kamu has been the testing bed for the initial implementation of the task and flow systems. Current design is presented in [Appendix A](#appendix-a-flow-system-prototype-in-kamu).

The current prototype system has a few design issues:
- Flows are closely coupled with datasets - we'll need them to work with many other types of resources
- It exposes an ever growing GQL API surface that has to be updated when adding new flow types
- It is not extensible and would not allow custom plug-in flow types
- Flows consist of one workload only and cannot chain multiple steps (e.g. compact then GC)


## Proposed IaC-based System
We propose to define a new Tasks & Flows system on the core ODF level that builds on the [Resource Framework](./018-iac-resource-framework.md) and allows defining and scheduling workloads in a generic, extensible way.


### Task
Building in a bottom-up order, we define the `Task` resource. `Task` represents a single unit of work, from intent through execution to its outcome.

`Task` resource `spec` captures the **intent**: the target resource and the operation kind with high-level parameters. This is what a human operator or a `FlowRun` controller writes when creating a task. Spec is stable, human-readable, and never rewritten.

Tasks are executed by **workers**. Every worker specializes in one task type and pulls the next task of that type from the node's **queue**, which schedules and prioritizes tasks. The worker then advances the task through the steps its type requires - e.g. fetching data for an ingest, or elaborating the execution plan of a transform, requesting an engine and dispatching the plan to it - and commits the result. As it progresses, the worker records the state of the task in type-specific status conditions. For example, the **execution plan** - with fully resolved paths, offsets, schemas, and all other inputs - is written into the `TaskPlan` condition.

The `TaskStatus` lifecycle proceeds through the following phases:
- **Pending** — task created; node is validating its inputs and resolving what is needed to schedule it (e.g. the engine a transform uses)
- **Queued** — task is waiting in the queue for a worker
- **Running** — a worker has claimed the task and is executing it, including the commit
- **Finished** — terminal; `TaskOutcome` condition is either `Success`, `Failed`, `NoOp` or `Cancelled`, and the task resource is deleted (see below).

The resource `phase` reflects the work of the **task controller**, whose job is to admit the task: validate its inputs, resolve what is needed to schedule it, and put it into the queue. `phase: Ready` means the task was admitted, while `phase: Failed` means it was rejected - in which case the controller also sets an unrecoverable `Failed` outcome. Execution is reported separately by workers in the `TaskStatus` condition, so an admitted task stays in `phase: Ready` whatever its `TaskStatus` is, until it is deleted.

Example of a task as created by a `FlowRun` controller (spec describes only the intent, no plan yet):
```yaml
$schema: https://opendatafabric.org/schemas/flows/v1alpha1/Task
headers:
  ownerReferences:
    - FlowRun:c27331ce-ce88-4ff9-8c5a-4ce8107cc03f  # ResourceRef of the FlowRun that spawned this task
spec:
  kind: Transform
  target: Dataset:sergiimk/foo  # ResourceRef
status:
  phase: Pending
```

Task resources are **deleted as soon as they finish**, so the existing tasks are exactly the work that is pending, queued, or running. Deleted tasks **stay readable** for a configurable retention period (e.g. 10 days) before being purged. This allows the UI and operators to inspect recent runs.

Example of the same task after planning and execution:
```yaml
$schema: https://opendatafabric.org/schemas/flows/v1alpha1/Task
headers:
  ownerReferences:
    - FlowRun:c27331ce-ce88-4ff9-8c5a-4ce8107cc03f  # ResourceRef of the FlowRun that spawned this task
  deletedAt: 2026-09-11T02:23:41Z  # Deleted upon completion, readable until the retention period ends
spec:
  kind: Transform
  target: Dataset:sergiimk/foo  # ResourceRef
status:
  phase: Deleted  # Deleted upon completion; execution is reported in `TaskStatus`
  observedGeneration: 1
  conditions:
    https://opendatafabric.org/schemas/tasks/v1alpha1/TaskStatus: Finished  # Pending / Queued / Running / Finished
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
      kind: Success  # Success / Failed / NoOp / Cancelled
      result:
        kind: TransformResult
        newBlockHash: f162..f008
        newOffsetInterval:
          start: 0
          end: 1050
        newWatermark: 2023-04-15T00:00:00Z
```

Note that a task may finish with a `NoOp` outcome and no `TaskPlan` if the worker realizes there is nothing to do (e.g. no new data to process).

A `Failed` outcome describes the error and tells whether it is **recoverable**, i.e. whether retrying the task could succeed. Recoverable failures (e.g. a network error) can be retried by the flow according to its `retryPolicy`, while unrecoverable ones (e.g. an invalid query) would fail the same way again. Type-specific details of the error are carried in `error`:
```yaml
https://opendatafabric.org/schemas/tasks/v1alpha1/TaskOutcome:
  kind: Failed
  message: Input dataset was compacted since the last transformation
  recoverable: false
  error:
    kind: InputDatasetCompacted
    inputDataset: Dataset:sergiimk/sensor-temp
```

Deleting a task that has not finished yet **cancels** it when possible. Until its outcome is recorded, a task being deleted remains visible in `phase: Deleting`.



#### Task Types
The following task types will be initially supported.

`Ingest` - fetches data from a source and appends it to a dataset.

```yaml
kind: Ingest
target: Dataset:sergiimk/sensor-temp  # optional: defaults to the flow-level target
source: Source:sergiimk/sensor-temp-http  # ResourceRef to the Source resource
targetRecordsPerSlice: 10000  # optional: target number of records to ingest per slice
```

`Transform` - executes transformation of data defined in a derivative dataset.

```yaml
kind: Transform
target: Dataset:sergiimk/sensor-temp-daily  # ResourceRef to the derivative dataset
```

`SyncFrom` - pulls new blocks from a remote dataset into a local dataset.

```yaml
kind: SyncFrom
target: Dataset:sergiimk/sensor-temp  # optional: defaults to the flow-level target
source:
  url: odf+https://node.example.com/acme/sensor-temp  # scheme selects the transfer protocol
  auth:  # optional
    kind: Bearer
    token: SecretSet:acme-node#accessToken  # ValueRef
force: false  # optional: overwrite the local dataset even if histories have diverged
```

`SyncTo` - pushes new blocks of a local dataset to a remote dataset.

```yaml
kind: SyncTo
target: Dataset:sergiimk/sensor-temp  # optional: defaults to the flow-level target
destination:
  url: s3://my-bucket/datasets/sensor-temp/
  auth:
    kind: Aws
    region: us-west-2
    accessKey: SecretSet:my-aws-secrets#accessKey  # ValueRef
    secretKey: SecretSet:my-aws-secrets#secretKey  # ValueRef
force: false  # optional: overwrite the remote dataset even if histories have diverged
createIfNotExists: true  # optional: create the remote dataset if it does not exist
```

Sync endpoints support the following `auth` kinds:
- `Bearer` - passes a token, e.g. an access token of a remote ODF node
- `Aws` - credentials for AWS or an AWS-compatible object storage
- `Headers` - custom request headers with values referencing `VariableSet`s and `SecretSet`s

Note that the execution plan of a sync task carries references to secrets and never their values - these are resolved only during execution.

`Compaction` - compacts data files in matching datasets to improve query performance.

```yaml
kind: Compaction
target: Dataset:sergiimk/sensor-temp  # ResourceRef to the dataset to compact
maxSliceSize: 100MiB     # optional: target maximum size of each compacted data slice
maxSliceRecords: 10000   # optional: target maximum number of records per slice
```

`Reset` - moves a block reference of a dataset to an earlier block, discarding the history that follows it.

```yaml
kind: Reset
target: Dataset:sergiimk/sensor-temp  # ResourceRef to the dataset to reset
ref: head  # optional: block reference to reset, defaults to `head`
newBlockHash: f162..8a9f  # optional: block the reference will point to, defaults to the `Seed` block
oldBlockHash: f162..f008  # optional: expected current block, task fails if the reference has moved
```

`GarbageCollection` - removes unreferenced data files from matching datasets.

```yaml
kind: GarbageCollection
target: Dataset:sergiimk/sensor-temp  # ResourceRef to the dataset to collect garbage from
```

`Verify` - checks dataset metadata for integrity, optionally replaying transformations.

```yaml
kind: Verify
target: Dataset:sergiimk/sensor-temp  # optional: defaults to the flow-level target
replayTransform: true  # optional: re-executes transformations to verify reproducibility
```

`WebhookCall` - dispatches a payload to a `WebhookEndpoint` resource.

```yaml
kind: WebhookCall
endpoint: WebhookEndpoint:sergiimk/notify-slack  # ResourceRef to the WebhookEndpoint
payload: '{"event": "ingest-complete", "dataset": "{{task.source}}"}'  # optional, supports templating
retryPolicy:                     # optional: overrides the flow-level retry policy for this task
  maxAttempts: 5
  minDelay: 30s
  backoff: Exponential
```

Custom task types can also be specified using a full schema URI as the kind:

```yaml
kind: https://acme.com/schemas/tasks/v1/SomeTask
someCustomParam: value
```


### `FlowRun`
`FlowRun` resources are a set of tasks to be executed in a sequence.

Example:
```yaml
$schema: https://opendatafabric.org/schemas/flows/v1alpha1/FlowRun
headers:
  ownerReferences:
    - Flow:f47ac10b-58cc-4372-a567-0e02b2c3d479  # ResourceRef of the Flow that spawned this run
spec:
  target: Dataset:sergiimk/foo
  tasks:
    - kind: Transform   # Implicit name: task-0-transform
    - kind: GarbageCollection  # Implicit name: task-1-garbage-collect
status:
  phase: Ready  # NOTE: FlowRun resource is deleted when the run finishes, like a Task
  conditions:
    # Tracks the overall status and the tasks that were spawned during the execution
    https://opendatafabric.org/schemas/flows/v1alpha1/FlowRunStatus:
      status: Running  # Waiting / Running / Retrying / Finished
      tasks:
        - name: task-0-transform  # Corresponds to spec.tasks[0]
          task: Task:a1b2c3d4-1111-2222-3333-444444444444  # ResourceHandle
          status: Finished
          outcome:
            kind: Success
          lastUpdatedAt: 2026-09-11T02:00:00Z
        - name: task-1-garbage-collect   # Corresponds to spec.tasks[1]
          task: Task:e5f6a7b8-5555-6666-7777-888888888888  # ResourceHandle
          status: Finished
          outcome:
            kind: Success
          lastUpdatedAt: 2026-09-11T02:02:00Z
    # Explains what led to execution of this flow
    # In case of a retry - the causes of the original run are preserved
    https://opendatafabric.org/schemas/flows/v1alpha1/FlowRunActivationCauses:
      activationCauses:
        - activationTime: 2026-09-11T02:00:00Z
          initiator: system  # AccountHandle
          trigger:  # Copy of the trigger's configuration in the parent Flow
            kind: Manual
      lateActivationCauses: []
    # Links to the previous FlowRun, if this is a retry
    https://opendatafabric.org/schemas/flows/v1alpha1/FlowRunRetry:
      retryOf: FlowRun:9b2e4f1a-3c7d-4e8b-a1f2-6d5e7c8b9a0d  # ResourceHandle
      retryNumber: 2  # Second time retrying (i.e. third run total)
```

The `spec.target` on the `FlowRun` level is used as the default `target` for tasks in `spec.tasks` list to avoid duplication.

Like tasks, `FlowRun` resources are **deleted as soon as the run finishes** and stay readable for the retention period. Deleting a run that has not finished yet cancels it, together with its unfinished tasks, which reference the run in `ownerReferences`.

A run created by the flow controller copies the `serviceAccount` of its `Flow` into its own spec, so changes to the flow affect only future runs, never runs in progress.

> Note: Although `retryOf` and `activationCauses` are immutable and known at `FlowRun` creation, they are part of `status` rather than `spec` because they carry information that can only be reliably set by the controller, not by a user.


### `Flow`
`Flow` resources act as templates for instantiating `FlowRun`s and define triggers that decide when to instantiate them.

Example:
```yaml
$schema: https://opendatafabric.org/schemas/flows/v1alpha1/Flow
headers:
  name: compact-and-gc-roots
spec:
  target:  # Selector matching all Root datasets under `sergiimk`
    type: Dataset
    account: sergiimk
    name: %
    labels:
      datasetKind: Root
  triggers:
    - kind: Event
      events:
        type: dataset.ref.updated  # TODO: Spec for event types / groups
      cooldown: 10min
  tasks:
    - kind: Compaction
      maxSliceSize: 100MiB
      maxSliceRecords: 10000
      onNoOp: Break         # Break / Continue: Break finishes the flow run early without error
      onFailure: Fail       # Fail / Continue
    - kind: GarbageCollection
  retryPolicy:
    maxAttempts: 3
    minDelay: 1min
    backoff: Exponential    # None / Linear / Exponential
  recentBindingsRetention:  # Controls retention for `FlowStatus.recentBindings`
    maxBindings: 10
  recentRunsRetention:      # Controls retention for `FlowStatus.recentRuns`
    maxRuns: 100            # Also capped by the controller settings
    maxAge: 30d
status:
  phase: Ready
  conditions:
    https://opendatafabric.org/schemas/flows/v1alpha1/FlowStatus:
      status: Active  # Active / Paused
      recentBindings:
        - target: Dataset:sergiimk/foo
          boundAt: 2026-01-01T00:00:00Z
      bindingsTotal: 1
      recentRuns:
        - flowRun: FlowRun:f47ac10b-1111-2222-3333-444444444444  # ResourceHandle
          status: Finished
          outcome: Success
          finishedAt: 2026-09-11T02:00:00Z
          retryOf: FlowRun:9b2e4f1a-5555-6666-7777-888888888888  # ResourceHandle
          retryNumber: 1  # First time retrying (i.e. second run total)
        - flowRun: FlowRun:9b2e4f1a-5555-6666-7777-888888888888  # ResourceHandle
          status: Finished
          outcome: Failed
          finishedAt: 2026-09-10T02:00:00Z
```

The `spec.target` on the `Flow` level is used as the default `target` for triggers in `spec.triggers` list and tasks in `spec.tasks` list to avoid duplication while still allowing overrides (e.g. triggering a flow on dataset `A` based on events in dataset `B`).


#### Target Selector
When a `Flow`'s `target` selector matches a resource, the flow controller sets up triggers for that resource automatically. As resources are created or deleted, the controller subscribes to those events and updates its set of active targets accordingly.

The association between a `Flow` and its matched resources is purely a derived state that the controller maintains. The `FlowStatus.recentBindigs` may reflect last N bindings that were created for debugging purposes. To show the full paginated list of bindings, the flow controller may provide a dedicated API for querying the index, e.g. in REST:
- `GET /flow/v1alpha1/flow/_/inverseSearch?target={ResourceRef}` - list flows by target
- `GET /flow/v1alpha1/flow/{id}/targets` - list targets by flow


### `FlowTrigger`
Flows specify a set of triggers that decide when to instantiate a `FlowRun`.

`FlowRun` stores the trigger configuration that lead to its creation in `activationCauses`.

**Core trigger types**:

`Manual` trigger - reacts to an API call or a UI button press.

```yaml
kind: Manual
```

`Schedule` trigger - fires on specified Cron schedule. Guaranteed to fire if node was down when the next tick was supposed to happen. Fires only once for all missed ticks.

```yaml
kind: Schedule
cron: "@daily"
```

`Interval` trigger - fires at regular intervals. Guaranteed to fire if node was down when the next tick was supposed to happen. Fires only once for all missed ticks.

```yaml
kind: Interval
interval: 15m
```

`Event` trigger - reacts to events on the event bus. This is a very low-level trigger that should be used sparingly.

```yaml
kind: Event
target: Dataset:sergiimk/foo
events:
  type: dataset.ref.updated  # Domain event filter (TODO: Spec for event types)
```

`InputsUpdated` trigger - fires when inputs of a derivative dataset have updates.

```yaml
kind: InputsUpdated
target: Dataset:sergiimk/bar
minRecordsToAwait: 100  # optional batching
maxAwaitInterval: 1h  # run at least once an hour if minNewRecords have not been reached
```

`SourceUpdated` trigger - fires when specified source is updated.

```yaml
kind: SourceUpdated
source: Source:sensor.temp.http
minRecordsToAwait: 100  # optional batching
maxAwaitInterval: 1h  # run at least once an hour if minNewRecords have not been reached
```

Besides triggers defined in ODF spec custom triggers can also be specified.

```yaml
kind: https://acme.com/schemas/tasks/v1/SpecialTrigger
someCustomParam: value
```

The following properties are common across all trigger types:

```yaml
enabled: true  # Allows to pause an individual trigger
cooldown: 10m  # Don't fire more often than every 10 minutes (batches multiple activations into one)
```


### Authorization
A task executes with the permissions of a user **principal** who created it.

When a task, flow run, or a flow is defined for an organization they must specify `serviceAccount` property which defines a non-human principal that gets permissions only through explicit policy bindings.

A task created from a `FlowRun` inherits the `serviceAccount` of the run, and `FlowRun` inherits one from `Flow`.

Example:
```yaml
$schema: https://opendatafabric.org/schemas/flows/v1alpha1/Flow
headers:
  account: acme
  name: ingest-sensors
spec:
  serviceAccount: acme/ingest-bot  # AccountRef to an account of service account type
  target: Dataset:acme/sensor-temp
  # ...
```

Details of the authorization mechanism are outside of the scope of this RFC.


## Appendix A: Flow System Prototype in Kamu
The design is roughly captured by the following GQL types:
```rust
type Task {
	taskId: TaskID!
	status: TaskStatus!
	cancellationRequested: Boolean!
	outcome: TaskOutcome
	createdAt: DateTime!
	ranAt: DateTime
	cancellationRequestedAt: DateTime
	finishedAt: DateTime
}
enum TaskStatus {
	QUEUED
	RUNNING
	FINISHED
}
union TaskOutcome = TaskOutcomeSuccess | TaskOutcomeFailed | TaskOutcomeCancelled
union TaskFailureReason = TaskFailureReasonGeneral | TaskFailureReasonInputDatasetCompacted | TaskFailureReasonWebhookDeliveryProblem

// When to run the flow
type FlowTrigger {
	paused: Boolean!
	schedule: FlowTriggerScheduleRule
	reactive: FlowTriggerReactiveRule
	stopPolicy: FlowTriggerStopPolicy!
}
union FlowTriggerScheduleRule = TimeDelta | Cron5ComponentExpression
type FlowTriggerReactiveRule {
	forNewData: FlowTriggerBatchingRule!
	forBreakingChange: FlowTriggerBreakingChangeRule!
}

// NEW: Flow
type FlowConfiguration {
	rule: FlowConfigRule!
	retryPolicy: FlowRetryPolicy
}

// Parameters used to plan the task
// NEW: Flow.spec.tasks[].params
type FlowConfigRule = FlowConfigRuleIngest | FlowConfigRuleCompaction | FlowConfigRuleReset
type FlowConfigRuleCompaction {
	maxSliceSize: Int!
	maxSliceRecords: Int!
}
type FlowConfigRuleIngest {
	fetchUncacheable: Boolean!
	fetchNextIteration: Boolean!
}
type FlowConfigRuleReset {
	mode: FlowConfigResetPropagationMode!
	oldHeadHash: Multihash
}

// Whether to retry on failures
// NOTE: Old flows could only execute one thing
// NEW: Flow.spec.retryPolicy
type FlowRetryPolicy {
	maxAttempts: Int!
	minDelay: TimeDelta!
	backoffType: FlowRetryBackoffType!
}

// A single execution (run) of the flow
// NEW: FlowRun
type Flow {
  flowId: FlowID!
  datasetId: DatasetID
  description: FlowDescription!
  status: FlowStatus!
  outcome: FlowOutcome
  timing: FlowTimingRecords!
  taskIds: [TaskID!]!
  history: [FlowEvent!]!
  initiator: Account
  primaryActivationCause: FlowActivationCause!
  startCondition: FlowStartCondition
  configSnapshot: FlowConfigRule
  retryPolicy: FlowRetryPolicy
  relatedTrigger: FlowTrigger
}

enum FlowStatus {
	WAITING
	RUNNING
	RETRYING
	FINISHED
}

// What led to the execution of the flow
// NEW: Flow.spec.triggers
type FlowStartCondition = FlowStartConditionSchedule | 
                          FlowStartConditionThrottling | 
                          FlowStartConditionReactive | 
                          FlowStartConditionExecutor

union FlowActivationCause = FlowActivationCauseManual | 
                            FlowActivationCauseAutoPolling | 
                            FlowActivationCauseDatasetUpdate | 
                            FlowActivationCauseIterationFinished

type DatasetFlowProcess {
	flowType: DatasetFlowType!
	dataset: Dataset!
	summary: FlowProcessSummary!
}

// Summary of last flow runs (for UI cards)
type FlowProcessSummary {
	effectiveState: FlowProcessEffectiveState!
	consecutiveFailures: Int!
	stopPolicy: FlowTriggerStopPolicy!
	lastSuccessAt: DateTime
	lastAttemptAt: DateTime
	lastFailureAt: DateTime
	nextPlannedAt: DateTime
	runningSince: DateTime
	pausedAt: DateTime
	autoStoppedReason: FlowProcessAutoStopReason
	autoStoppedAt: DateTime
}
```
