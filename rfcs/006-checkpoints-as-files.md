# RFC-006: Store Checkpoints as Files

**Start Date**: 2022-04-19

[![RFC Status](https://img.shields.io/github/issues/detail/state/kamu-data/open-data-fabric/24?label=RFC%20Status)](https://github.com/kamu-data/open-data-fabric/issues/24)

[![Spec PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/25?label=Spec%20PR)](https://github.com/kamu-data/open-data-fabric/pull/25)

## Summary

This RFC proposes to no longer allow arbitrary files and directory structures as engine checkpoints and require that a checkpoint was always a single file.

## Motivation

Currently engine can produce a checkpoint that includes arbitrary structure of files and directories. This presents few problems:

1) Metadata blocks need to refer to checkpoints by hash, but there is no standard approach to computing a hash of a directory. We would need to create a stable directory hashing algorithm ourselves and this complexity will spread to all implementations.

2) Allowing arbitrary file structures is also a security concern, e.g. need to make sure engines don't create weird symlinks.

3) When downloading a dataset from repository, many transfer protocols don't have a standard way to list a directory. One such example is HTTP - there is no standard content format for `GET` on a directory - most web servers will return a stylyzed HTML. Similarly to how Git can clone a repo using the ["dumb protocol"](https://git-scm.com/book/en/v2/Git-Internals-Transfer-Protocols) we'd like to be able to walk and download entire dataset, and this means having a fixed directory structure and only referencing files.

## Guide-level explanation

## Reference-level explanation

Specification will be updated to no nonger refer to checkpoints as opaque directories.

The temporary `ExecuteQueryRequest` schema that relies on file mounting will be updated.

## Drawbacks

- Engines that produce multiple files as checkpoints will need extra `tar` / `untar` steps

## Rationale and alternatives

- Since ODF implements its own content-addressable storage we could support "tree" structures in it just like git does. This would however come at a high cost in complexity and is not justified at the current stage.

## Prior art

- Git's bare repository format and  ["dumb sync protocol"](https://git-scm.com/book/en/v2/Git-Internals-Transfer-Protocols) don't rely on directory listing

## Unresolved questions

- What's the most efficient checkpoint management scheme for long-term solution?
  - Can it be mmap'ed files as in our goal for data slices?

## Future possibilities

- This is a necessary step for implementing a sync protocol for pulling datasets from IPFS and other storage systems that provide HTTP gateway.
