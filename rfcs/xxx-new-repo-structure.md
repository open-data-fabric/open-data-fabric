# RFC-XXX: New Repository Structure

**Start Date**: 2022-04-22

[![RFC Status](https://img.shields.io/github/issues/detail/state/kamu-data/open-data-fabric/26?label=RFC%20Status)](https://github.com/kamu-data/open-data-fabric/issues/26)

[![Spec PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/27?label=Spec%20PR)](https://github.com/kamu-data/open-data-fabric/pull/27)

## Summary

This RFC proposes ... .

## Motivation

## Guide-level explanation

## Reference-level explanation

**Dataset Layout**:
```
refs/
  <ref name>
blocks/
  <multihash>  
data/
  <multihash>
checkpoints/
  <multihash>
info (summary + refs)
```

Info should contain a dataset layout version of sorts.

**Registry + Repo Layout**:
```
<cid|name>/
  <dataset>
datasets.yaml
datasets.yaml.gz ??? Need self-describing versioned file format
```

**Workspace Layout**:
```
.kamu/
  datastes/
    <name>/
      <dataset>
  repos/
    <name>
  run/
    ...
  .kamuconfig.yaml
```

## Drawbacks

## Rationale and alternatives

## Prior art

## Unresolved questions
- Should we allow having two datasets with same CID in one workspace?
  - But with different set of refs / info
- Where to locate `cache`?

## Future possibilities

