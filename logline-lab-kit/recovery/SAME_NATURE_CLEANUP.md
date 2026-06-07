# After-D pass — same-nature cleanup: canon vs tooling vs projections

> Status: TODO (do NOT start during D). Queued by the operator after the D provider work.
> Do not delete the sealed conformance tooling — it just proved the gate. Relabel, don't remove.

## The principle

Everything semantically important is a **LogLine Act**. The canon is intentionally simple:
it is the mold `canon-main/logline.receipt.v0` (nine string slots + `id`=content_hash +
`hashes` + `json_canonicalization`) plus the JCS / hash / envelope discipline proven by
external tooling. You extend by **(a) not**, or **(b) a pack of rule-Acts** — never by
adding a meta-language to the core.

A tool can help **verify** canon without **becoming** canon.

## What the canon is NOT
- not YAML
- not JSON Schema
- not the `.logline` grammar vectors
- not the conformance harness / verifier
- not generated prose
- not the directory we happened to name `foundation/conformance/canon/`

## Audit targets (relabel as tooling/witness/projection; do not call them canon/authority)
- `foundation/conformance/**` — schemas (`*.schema.json`), vectors (incl. `.logline`),
  `cases/`, `tools/verify-receipt.mjs`, drift check → **external conformance tooling /
  reference witness**. Consider renaming the `…/canon/` subdir so "canon" stops naming a
  tooling folder.
- `schemas/*.json` (labd) — JSON Schema is foreign nature. Keep only as client-interop /
  projection validation, clearly labeled tooling — never as the source of Act meaning.
- The planned **P7 "validate every surface against JSON Schema"** — reconsider: validate
  surfaces against example Acts + the mold + the hash discipline (same nature), not a
  bolted-on schema language. If JSON Schema stays, it is interop tooling, not authority.
- Surface contracts, docs language — find every place tooling is called "canon" and fix it.
- Provider examples — DONE in D (nine-slot Act JSON, string slots; no YAML/JSON-Schema/`.logline`).

## Still-open: divergence #3
`labd Act` with `serde_json::Value` slots is **not** `logline.receipt.v0` with string
slots. Provider-decision Acts were made string-slot in D to stay canon-shape, but the
general Act↔receipt projection (string slots + `id`/`hashes`/`receipt_version`/AUX) is
unbuilt. Do not close it by pretending the two are identical.

## Goal
Everything semantically important is an Act. External tooling may exist, but it must be
labeled tooling / witness / projection — never authority.
