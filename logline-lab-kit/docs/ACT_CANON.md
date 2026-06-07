# Act Canon

The canonical LogLine Act has **exactly nine slots** — there is no tenth slot:

```
who  did  this  when  confirmed_by  if_ok  if_doubt  if_not  status
```

Slot meanings (FINAL §7.1): `who` actor; `did` verb/claim; `this` subject;
`when` time; `confirmed_by` proof source or absence; `if_ok`/`if_doubt`/`if_not`
consequence routes; `status` state under the declared practice.

Hashes, signatures, runtime data, selected branch, storage timestamps, and
envelopes live **around** the Act, never inside it. Files, JSON, SQLite rows, UI
state, reports, and provider responses are not semantic truth.

- Implementation: `crates/logline-act`.
- Schema: `schemas/act.schema.json`.
- Canonical JSON + content/tuple hashing are deterministic (A05).
- Candidate generosity, promotion strictness (A03/A04): ugly capture is allowed;
  promotion requires validation.
