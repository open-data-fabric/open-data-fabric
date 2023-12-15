# RFC-012: Use base32 encoding for hashes and DIDs

[![Issue](https://img.shields.io/github/issues/detail/state/kamu-data/open-data-fabric/62?label=Issue)](https://github.com/kamu-data/open-data-fabric/issues/62)
[![Spec PR](https://img.shields.io/github/pulls/detail/state/kamu-data/open-data-fabric/63?label=PR)](https://github.com/kamu-data/open-data-fabric/pull/63)

**Start Date**: 2023-12-14

**Authors**:
- [Sergii Mikhtoniuk](mailto:sergii.mikhtoniuk@kamu.dev), [Kamu](https://kamu.dev)

**Compatibility**:
- [ ] Backwards-compatible
- [ ] Forwards-compatible

## Summary
base32 (rfc4648 - no padding - highest letter)

- block hashes
- DIDs???

## Motivation
Very early on ODF started using `base58`, simply mimicing other projects like Bitcoin and IPFS, but this encoding is problematic:
- It can result in hashes that resemble words - we don't want users to ever encounter swear words
- It's hard to write down or read out as many letters and numbers can look similar
- It doesn't work with subdomains ([RFC1035](https://datatracker.ietf.org/doc/html/rfc1035), [RFC1123](https://datatracker.ietf.org/doc/html/rfc1123)) that are restrictied to: case-insensitive, `a-b0-9`` and less than 63 bytes

The latter issue is especially important if we ever end up doing something similar to IPFS gateways, e.g. allowing browsing dataset state or attachments at specific block

The ability to do proper security origins for the HTTP gateway with subdomains (cidv1abcde.dweb.link). This is very important if we want to handle reports with Google's safe browsing system (which is designed for origins). With the current design, all content is on the same browser origin, and a single phishing/malware report on any of the IPFS gateways (hosted by us or someone else) will make web browsers block every single thing on the origin with a giant red warning message until it's cleared up with Google (which from experience can take several days!)
Root paths are in the right place, which dramatically improves compatibility with existing web sites that tend to do a lot of this:
<img src="/rootimg.jpg">
Allows us to register dweb.link (and ipfs.io, etc.) to the Public Suffix List, which will prevent the sandboxed content from reading/manipulating cookies on the parent domain (and on other cidv1 subdomains).
Opens up the ability for go-ipfs to do HTTP Host Header parsing and automatic Let's Encrypt support (if we wanted to), so anyone can set up a public IPFS gateway without additional software. Once Let's Encrypt gets their wildcard cert domains shipped (Dec 2017), this could be a fully automated process. Otherwise something like nginx would be needed (I could write an example nginx.conf that people could use for it).
It should use lowercase base32 characters by default, so that it's consistent with subdomain usage (all the browsers will force lowercase). IIRC the RFC doesn't care if it's lowercased, I think people just default to upper case for legacy reasons.


## Guide-level explanation
The proposal is to replace `base58` encoding with `base32` ([RFC4648](https://datatracker.ietf.org/doc/html/rfc4648), no padding, highest letter ????????).

In block hashes
In DIDs

Note: Using `base32` in `did:key` method is [the W3C spec](https://w3c-ccg.github.io/did-method-key/). There is an [open issue](https://github.com/w3c-ccg/did-method-key/issues/21), however, about considering other encoding or oppening spec up to support any multibase format.

## Reference-level explanation

## Compatibility

## Drawbacks
- Slight increase in length of hashes in presentation layer and human-readable formats

## Alternatives
- Use [Crackford's base32 encoding](https://www.crockford.com/base32.html)

## Prior art
- Releted IPFS tickets:
  - https://github.com/ipfs/kubo/issues/4143
  - https://github.com/ipfs/specs/issues/247
- 

## Unresolved questions
- Non-trivial ingestion merge strategies (snapshot, ledger) currently require access to past stream data to perform CDC and deduplication. We were considering to avoid the need to read past data eventually by storing all necessary information in the checkpoints. However, allowing for multiple push sources would mean that we need separate checkpoints per source as sources can have different merge strategies. Given that it's still not clear if storing state in checkpoints for things like CDC is even practical for large datasets - we decided to not let this block this RFC, as the need for several push sources per dataset does seem like a practical necessity.

## Future possibilities
N/A

- is hash algo present in our block hashes multibase????