# 07 — Archive and Debris Policy

## Preserve

```txt
current reduced docs
ATLAS
Foundation refs
conformance vectors
working code
recovery notes
receipts
evidence
current build-pack
```

## Demote

```txt
transcripts
assistant-generated ADRs
old zips
false authority files
stale architecture drafts
duplicated docs
old primitive/artifact/ledger language
```

## Remove

```txt
build outputs
generated junk
.DS_Store
__MACOSX/
target/
dead manifests
fake checksums
files pretending to be source of truth
```

## Semantic rule

Never call disposable junk archive.

An archive must preserve meaningful historical or institutional learning.
Fresh mistakes, duplicate generated files, poisoned ADRs, stale build outputs, and false authority files are debris, not archive.
