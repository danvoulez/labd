# 05 — Acceptance Tests

A right v0 is complete when these pass:

```txt
A1   Act with exactly nine slots validates.
A2   Act with a missing slot fails.
A3   Act with a tenth canonical slot fails.
A4   Same Act produces same canonical hash.
A5   Ugly candidate can be preserved.
A6   Local emit stores Act in local cache/outbox.
A7   Re-emitting same Act is idempotent.
A8   Sync writes Act to configured spine.
A9   Projection reads Act from spine.
A10  Blocked Act appears when evidence or permission is missing.
A11  Evidence can be attached to scope.
A12  Receipt candidate names exact scope.
A13  Report does not pretend to be receipt.
A14  Pack loads without changing core.
A15  Profile loads without changing core.
A16  App/MCP call becomes draft Act.
A17  Unauthorized app action is blocked.
A18  Worker cannot execute without allow.
A19  Worker returns evidence, not closure.
A20  Clock tick emits or evaluates due Acts.
A21  Due Act resolves or creates reschedule Act.
A22  Lab report renders state from projections.
A23  Demo pack completes first session.
A24  Santo André pack loads without becoming core.
A25  Manhattan pack loads without becoming core.
A26  Manhattan L-06 emits evidence.
A27  L-06 receipt closes only L-06.
A28  Projection shows L-06 health from Acts.
A29  Recovery scan catches false authority.
A30  Storage scan rejects file/SQLite truth language.
```

## v0 Done

```txt
A user installs LogLine Lab Kit.
The user initializes a Lab with a profile and pack.
The user emits an Act.
The Act validates as exactly nine canonical slots.
The Act is canonicalized and hashed.
The Act is stored in the local outbox.
The Act syncs to the configured spine.
The Act is read through projections.
A blocked Act is opened or carried.
Evidence is attached.
A scoped receipt candidate is prepared.
A Lab report is generated.
One worker dry-run runs.
One real worker command runs.
One MCP app is registered.
One grant is issued.
One unauthorized app action is rejected.
One clock tick runs.
One due Act is evaluated.
One demo pack loads.
The Santo André pack loads without changing core.
The Manhattan pack loads without changing core.
Manhattan L-06 runs as the serious proof:
  Act → gate → worker ping → evidence → scoped receipt → projection.
```
