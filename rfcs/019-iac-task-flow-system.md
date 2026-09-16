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
Building in a bottom-up order, we define the `Task` resource. `Task` represents a single unit of work, from intent through planning and execution to commit.

`Task` resource `spec` captures the **intent**: the target resource and the operation kind with high-level parameters. This is what a human operator or a `FlowRun` controller writes when creating a task. Spec is stable, human-readable, and never rewritten.

The **execution plan** - with fully resolved paths, offsets, schemas, and all other inputs needed by the worker - is computed by the node's planner and written into `TaskPlan` status condition.

The `TaskStatus` lifecycle proceeds through the following phases:
- **Pending** — task created, waiting for the planner
- **Planning** — node planner is resolving the full execution plan into `TaskPlan`
- **Ready** — `TaskPlan` is populated; task is queued for a worker
- **Running** — worker is executing the plan
- **Committing** — worker reported its result; node is validating output and writing the metadata block
- **Finished** — terminal; `TaskOutcome` condition is either `Success`, `Failed`, `NoOp` or `Cancelled`; the resource enters a TTL period before deletion.

Task resources are **retained for a configurable TTL period after completion** (e.g. 10 days) before being deleted. This allows the UI and operators to inspect recent runs directly from the resource store without querying the event store. After TTL expires, the resource is deleted, but the full history of any task remains recoverable from the event sourcing store by resource ID.

Note that the **separation of planning, execution, and commit phase** allows to run certain parts of the task on the node (with access to metadata and storage state), and heavy computational tasks on a separate worker.

Example of a task as created by a `FlowRun` controller (spec describes only the intent, no plan yet):
```yaml
$schema: https://opendatafabric.org/schemas/flow/v1alpha1/Task
headers:
  ownerReferences:
    - FlowRun:c27331ce-ce88-4ff9-8c5a-4ce8107cc03f  # ResourceRef of the FlowRun that spawned this task
spec:
  kind: Transform
  target: Dataset:sergiimk/foo  # ResourceRef
status:
  phase: Pending
```

Example of the same task after planning and execution:
```yaml
$schema: https://opendatafabric.org/schemas/flow/v1alpha1/Task
headers:
  ownerReferences:
    - FlowRun:c27331ce-ce88-4ff9-8c5a-4ce8107cc03f  # ResourceRef of the FlowRun that spawned this task
spec:
  kind: Transform
  target: Dataset:sergiimk/foo  # ResourceRef
status:
  phase: Ready  # NOTE: Task resource is retained for a TTL period, then deleted
  observedGeneration: 1
  conditions:
    https://opendatafabric.org/schemas/task/v1alpha1/TaskStatus: Finished  # Pending / Planning / Ready / Running / Committing / Finished
    https://opendatafabric.org/schemas/task/v1alpha1/TaskPlan:
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
    https://opendatafabric.org/schemas/task/v1alpha1/TaskOutcome:
      kind: Success  # Success / Failed / NoOp / Cancelled
      result:
        kind: TransformResult
        newBlockHash: f162..f008
        newOffsetInterval:
          start: 0
          end: 1050
        newWatermark: 2023-04-15T00:00:00Z
```

Note that a task may finish with a `NoOp` outcome and no `TaskPlan` if planner realizes there is nothing to do (e.g. no new data to process).


### `FlowRun`
`FlowRun` resources are a set of tasks to be executed in a sequence.

Example:
```yaml
$schema: https://opendatafabric.org/schemas/flow/v1alpha1/FlowRun
headers:
  ownerReferences:
    - Flow:f47ac10b-58cc-4372-a567-0e02b2c3d479  # ResourceRef of the Flow that spawned this run
spec:
  target: Dataset:sergiimk/foo
  tasks:
    - kind: Transform   # Implicit name: task-0-transform
    - kind: GarbageCollection  # Implicit name: task-1-garbage-collect
status:
  phase: Ready  # NOTE: FlowRun resource is retained for a TTL period, then deleted
  conditions:
    # Tracks the overall status and the tasks that were spawned during the execution
    https://opendatafabric.org/schemas/flow/v1alpha1/FlowRunStatus:
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
    https://opendatafabric.org/schemas/flow/v1alpha1/FlowRunActivationCauses:
      activationCauses:
        - activationTime: 2026-09-11T02:00:00Z
          initiator: system  # AccountHandle
          trigger:  # Copy of the trigger's configuration in the parent Flow
            kind: Manual
      lateActivationCauses: []
    # Links to the previous FlowRun, if this is a retry
    https://opendatafabric.org/schemas/flow/v1alpha1/FlowRunRetry:
      retryOf: FlowRun:9b2e4f1a-3c7d-4e8b-a1f2-6d5e7c8b9a0d  # ResourceHandle
```

The `spec.target` on the `FlowRun` level is used as the default `target` for tasks in `spec.tasks` list to avoid duplication.

> Note: Although `retryOf` and `activationCauses` are immutable and known at `FlowRun` creation, they are part of `status` rather than `spec` because they carry information that can only be reliably set by the controller, not by a user.


### `Flow`
`Flow` resources act as templates for instantiating `FlowRun`s and define triggers that decide when to instantiate them.

Example:
```yaml
$schema: https://opendatafabric.org/schemas/flow/v1alpha1/Flow
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
      params:
        minUncompactedRows: 100  # TODO: Take a look at compaction RFC
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
    https://opendatafabric.org/schemas/flow/v1alpha1/FlowStatus:
      status: Active  # Active / Paused
      recentBindings:
        - target: Dataset:sergiimk/foo
          boundAt: 2026-01-01T00:00:00Z
      bindingsTotal: 1
      recentRuns:
        - flowRun: FlowRun:f47ac10b-1111-2222-3333-444444444444  # ResourceRef
          status: Finished
          outcome: Success
          finishedAt: 2026-09-11T02:00:00Z
        - flowRun: FlowRun:9b2e4f1a-5555-6666-7777-888888888888  # ResourceRef
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

Examples:
```yaml
# Reacts to an API call or a UI button press
kind: Manual

# Fires on specified Cron schedule
# Guaranteed to fire if node was down when the next tick was supposed to happen
# Fires only once for all missed ticks
kind: Schedule
cron: "@daily"

# Fires at regular intervals
# Guaranteed to fire if node was down when the next tick was supposed to happen
# Fires only once for all missed ticks
kind: Interval
interval: 15m

# Reacts to events on the event bus
# This is a very low-level trigger that should be used sparingly
kind: Event
target: Dataset:sergiimk/foo
events:
  type: dataset.ref.updated  # Domain event filter (TODO: Spec for event types)

# Fires when inputs of a derivative dataset have updates
kind: InputsUpdated
target: Dataset:sergiimk/bar
minRecordsToAwait: 100  # optional batching
maxAwaitInterval: 1h  # run at least once an hour if minNewRecords have not been reached

# Fires when specified source is updated
kind: SourceUpdated
source: Source:sensor.temp.http
minRecordsToAwait: 100  # optional batching
maxAwaitInterval: 1h  # run at least once an hour if minNewRecords have not been reached
```

The following properties are common across all trigger types:

```yaml
enabled: true  # Allows to pause an individual trigger
cooldown: 10m  # Don't fire more often than every 10 minutes (batches multiple activations into one)
```


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
