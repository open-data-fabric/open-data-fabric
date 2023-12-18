# RFC-012: Recommend `base16` encoding for textual representation of hashes

[![Issue](https://img.shields.io/github/issues/detail/state/kamu-data/open-data-fabric/62?label=Issue)](https://github.com/kamu-data/open-data-fabric/issues/23)
[![Spec PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/63?label=PR)](https://github.com/kamu-data/open-data-fabric/pull/63)

**Start Date**: 2023-12-14

**Authors**:
- [Sergii Mikhtoniuk](mailto:sergii.mikhtoniuk@kamu.dev), [Kamu](https://kamu.dev)

**Compatibility**:
- [ ] Backwards-compatible
- [ ] Forwards-compatible

## Summary
Proposes to expand the spec's support to all `multibase` encodings but recommend `base16` as default choice.

## Motivation
Very early on ODF adopted `base58-btc` encoding, simply mimicing other projects like Bitcoin, IPFS, and `did:key` W3C spec. But this encoding is problematic:
- It can result in text that resembles words, potentially resulting in swear words in hashes and DIDs
- It's hard to write down or read out as many letters and numbers can look similar
- Its lexicographic order does not match the sort order of binary data
- It doesn't work with subdomains ([RFC1035](https://datatracker.ietf.org/doc/html/rfc1035), [RFC1123](https://datatracker.ietf.org/doc/html/rfc1123)) that are restrictied to: case-insensitive, `a-b0-9`` and less than 63 bytes.

The latter issue is especially important to allow exposing datasets and their attachments in a similar way to IPFS gateways, where subdamains are needed to ensure HTTP origin security.

## Reference-level explanation
The proposal is to:

1) Specify that ODF implementations must support all [multibase](https://github.com/multiformats/multibase) formats in `final` status of approval process, performing transcoding where needed.

2) Recommend new implementations to use `base16` encoding ([RFC4648](https://datatracker.ietf.org/doc/html/rfc4648)) where possible to avoid the pitfals described in motivation section.

3) For now, continue using `base58-btc` for DID encoding to maintain compatibility with `did:key` spec

## Compatibility
This change will be executed as part of the backwards compatibility breaking changes.

## Drawbacks
Declaring that we support multiple `multibase` encodings instead of just one may slightly complicate compatibility between implementations, but since multibase is a self-describing format these issues should be easily addressable. This approach also has additional benefit of upgradeability.

For an implementation chosing to stick with `base16` recommendation the main drawback is the **increase of length** of hashes in:
- file and directory names
- human-readable formats
- presentation layer

Because the binary representation is unaffected we don't see this as an issue.

## Alternatives
- `base32hex` encoding is case-insensitive and maintains sort order - dismissed as it can still form obscene words
- [Crackford's base32 encoding](https://www.crockford.com/base32.html) avoids letter/number similarity and excludes 'U' to reduce likelihood of accidental obscenity - dismissed as not yet supported by the [multibase](https://github.com/multiformats/multibase) spec

## Prior art
- Releted IPFS tickets:
  - https://github.com/ipfs/kubo/issues/4143
  - https://github.com/ipfs/specs/issues/247

## Unresolved questions
We continue using `base58-btc` for DIDs to maintain full compatibility with `did:key` spec. The spec, however, has an [open issue](https://github.com/w3c-ccg/did-method-key/issues/21) to reconsider the choice of `base58-btc` and potentially open up it up to support other `multibase` encodings.

## Future possibilities
N/A
