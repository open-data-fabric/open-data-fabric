# RFC-013: Enum representation in YAML encoding

[![Issue](https://img.shields.io/github/issues/detail/state/kamu-data/open-data-fabric/62?label=Issue)](https://github.com/kamu-data/open-data-fabric/issues/70)
[![Spec PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/63?label=PR)](https://github.com/kamu-data/open-data-fabric/pull/63)

**Start Date**: 2023-12-14

**Authors**:
- [Sergii Mikhtoniuk](mailto:sergii.mikhtoniuk@kamu.dev), [Kamu](https://kamu.dev)

**Compatibility**:
- [ ] Backwards-compatible
- [ ] Forwards-compatible

## Summary
Proposes to use `kind: PascalCaseVariantName` convention to standardize how enum variants are differentiated in YAML / JSON serialization format.

## Motivation
Currently the spec does not specify how enum variants must be differentiated between when ODF structures are serialized in YAML / JSON form.

This part was left to implementations and Kamu just ended up using `kind: camelCaseVariantName`.

## Reference-level explanation
The proposal is to use internally-tagged representation:

```yaml
enumProperty:
  kind: PascalCaseVariantName
  foo: bar
```

The internal tagging provides the most concise representation, while remaining non-ambiguous. The slight drawback is that it gives `kind` property a special function and we should be mindful of name conflicts.

Using the `PascalCase` for variant names makes names appear identical in YAML / JSON to how they appears in ODF schemas.

## Compatibility
This change will be executed as part of the backwards compatibility breaking changes.

## Drawbacks
N/A

## Alternatives
- [Supported enum representations in `serde`](https://serde.rs/enum-representations.html) (a defacto Rust's standard serialization library)

## Prior art
N/A

## Unresolved questions
N/A

## Future possibilities
N/A
