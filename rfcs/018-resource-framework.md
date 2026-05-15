# RFC-018: Resource Framework

[![Issue](https://img.shields.io/github/issues/detail/state/kamu-data/open-data-fabric/108?label=Issue)](https://github.com/kamu-data/open-data-fabric/issues/108)
[![PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/116?label=PR)](https://github.com/kamu-data/open-data-fabric/pull/116)

**Start Date**: 2025-06-14

**Published Date**: 2025-08-25

**Authors**:
- [Sergii Mikhtoniuk](mailto:smikhtoniuk@kamu.dev), [Kamu](https://kamu.dev)
- [Sergiy Zaychenko](mailto:sergiy.zaychenko@kamu.dev), [Kamu](https://kamu.dev)


**Compatibility**:
- [X] Backwards-compatible
- [X] Forwards-compatible

<!--
Backwards-compatible means:
- whether software updated to this RFC will be able to operate on old data
- in case of protocol updates - whether older clients will be able to communicate with newer servers

Forwards-compatible means:
- whether data written by new software still can be used by older software
- if newer clients will be able to communicate with older servers
-->


## Summary
<!--
One paragraph explanation of the feature.
-->


## Motivation
<!--
Why are we doing this? What use cases does it support? What is the expected outcome?
-->


## Guide-level explanation
<!--
Explain the proposal as if it was already included in the spec, and you were teaching it to another ODF user. That generally means:

- Introducing new named concepts.
- Explaining the feature largely in terms of examples.
- Explaining how one should *think* about the feature, and how it should impact the way they use ODF. It should explain the impact as concretely as possible.
- If applicable, provide sample error messages, deprecation warnings, or migration guidance.
- If applicable, describe the differences between teaching this to existing ODF users and new ODF users.

For implementation-oriented RFCs (e.g. for coordinator internals), this section should focus on how coordinator programmers should think about the change, and give examples of its concrete impact. For policy RFCs, this section should provide an example-driven introduction to the policy, and explain its impact in concrete terms.
-->


## Reference-level explanation
<!--
This is the technical portion of the RFC. Explain the design in sufficient detail that:

- Its interaction with other features is clear.
- It is reasonably clear how the feature would be implemented.
- Corner cases are dissected by example.

The section should return to the examples given in the previous section, and explain more fully how the detailed proposal makes those examples work.
-->


## Compatibility
<!--
Details on compatibility of these changes.
-->


## Drawbacks
<!--
Why should we *not* do this?
-->


## Rationale and alternatives
<!--
- Why is this design the best in the space of possible designs?
- What other designs have been considered and what is the rationale for not choosing them?
- What is the impact of not doing this?
-->


## Prior art
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

### Design Parallels with Kubernetes

#### Resource manifests

In Kubernetes the manifests are structured as:
```yaml
apiVersion: external-secrets.io/v1beta1
kind: SecretStore
metadata: {}
spec: {}
status: {}
```

In ODF we propose:
```yaml
context: opendatafabric.org/v1
kind: SecretSet
header: {}
spec: {}
status: {}
```

```yaml
"@context": https://opendatafabric.org/core/v1/context.jsonld
"@type": odf:SecretSet
"@id": https://cluster.example.com/resources/8f3b2c1a-4d5e-6f7a-8b9c-0d1e2f3a4b5c
header: {}
spec: {}
status: {}
```

Thoughts:
- `apiVersion` in K8s is a REST-level concept which is too narrow compared to what we're trying to represent
- `context` is used to represent the resource domain to make it similar to JSON-LD `@context`
- `kind` is used for consistency with k8s and the current ODF enums, although in future we can consider migrating to JSON-LD `@type`
- `header` is used instead of `metadata` because latter is already an overloaded term in ODF and is generic to the point of being meaningless


## Unresolved questions
<!--
- What parts of the design do you expect to resolve through the RFC process before this gets merged?
- What parts of the design do you expect to resolve through the implementation of this feature before stabilization?
- What related issues do you consider out of scope for this RFC that could be addressed in the future independently of the solution that comes out of this RFC?
-->


## Future possibilities
<!--
Think about what the natural extension and evolution of your proposal would be and how it would affect ODF as a whole. Try to use this section as a tool to more fully consider all possible interactions with the ODF in your proposal.

This is also a good place to "dump ideas", if they are out of scope for the RFC you are writing but otherwise related.

If you have tried and cannot think of any future possibilities, you may simply state that you cannot think of anything.

Note that having something written down in the future-possibilities section is not a reason to accept the current or a future RFC; such notes should be in the section on motivation or rationale in this or subsequent RFCs. The section merely provides additional information.
-->
