# Provenance — vendored LogLine canon conformance suite

This directory is a **vendored copy** of the authoritative conformance suite. It is the
binding contract labd is tested against. Do **not** hand-edit these files — they are
regenerated only by deliberate re-vendoring at a new pinned commit.

| Field | Value |
|---|---|
| Upstream repo | `LogLine-Foundation/conformance` |
| Branch | `main` |
| Pinned commit | `389a6b676af30bf5e344f9287ef51472b7f7a53f` |
| Upstream pushed | 2026-05-21 |
| Vendored on | 2026-06-07 |
| Files | 57 (excludes upstream `.gitignore`) |

## Verification

Dan's local snapshot was confirmed byte-identical to upstream@389a6b6 before vendoring
(receipt schema, canon schema, jcs-rfc8785 profile, verify-receipt.mjs, and the envelope
vector all SHA-256 MATCH; 58/58-file tree).

To re-verify against upstream at any time:

```sh
bash foundation/conformance/check-drift.sh
```

## Re-vendoring (deliberate act only)

1. Pick the new upstream commit SHA.
2. Re-copy the tree from that SHA into this directory.
3. Update the table above and re-run `check-drift.sh`.
4. Re-run the conformance harness; review which vectors changed expectation.

Re-vendoring is a reviewed change, never automatic — the canon is delicate and every
stored hash depends on it.
