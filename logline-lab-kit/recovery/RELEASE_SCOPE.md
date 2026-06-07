# RELEASE_SCOPE.md — what "LogLine Lab Kit v0 (generic)" means

This file is the **v0 cut**. It overrides ambiguous done-language elsewhere
(including `build-pack/final-real-project-doc.md` §21) where they disagree. It
exists so the next builder does not confuse the generic kit with optional
packs/profiles/deployments.

## Two governing principles

1. **Storage is ontology, not a detail.** A local file / outbox / cache is
   *candidate capture, transport, scratchpad, or dev fallback* — never the
   protocol-grade place where **admitted** LogLine Acts live. Admitted Acts
   require a **declared Spine Profile**. Onboarding forces that decision.
   `LogLine Act is the internal truth grammar.`
2. **International standards are projections of the LogLine Act graph, not
   competitors.** Scientific rigor enters as projection packs (PROV, RO-Crate,
   DataCite, CSL, ISA-Tab, JATS), never as core semantics.
   `Scientific standards are external projection grammars.`

## In scope for generic v0 (BLOCKERS)

```txt
Act core (nine slots, candidate-generous, promotion-strict, canonical hash)
no-pack basics path (a Lab forms on identity + profile, no pack required)
storage/spine onboarding contract + honest status matrix
candidate-only and dev-ephemeral modes (clearly non-publication-grade)
offline conformance + exportable examples
one study bench
headless surfaces (Start/Today/Timeline/Write/Schedule/Workbench/Proof/Learn/Settings)
labkit tick (the Lab confronts time, materialized as Acts)
proof / evidence / ghost / receipt discipline
CLI + MCP read/draft boundary (same map for human and LLM)
clean build/test/doctor evidence + one no-pack end-to-end fixture
```

## Done definition (v0 generic)

The previous done-line "the demo pack completes first session" is **replaced**:

```txt
The NO-PACK BASICS path completes a first session:
init/open a Lab (identity + profile) -> write candidate -> admit a valid Act
(under the configured spine grade) -> run conformance -> run a study bench
-> schedule + tick -> inspect proof -> learn the next Act -> export examples.
No demo pack. No Supabase. No Manhattan. No mandatory pack.
```

## Explicitly SOON / optional (NOT v0 blockers)

```txt
Supabase live ingest            (optional storage profile, feature-gated)
Postgres / Neon real spines     (SOON storage profiles)
Cloudflare / D1                 (SOON)
Santo Andre high-frequency lab  (local SQL + outbox + queue) — pack/deployment, LATER
Manhattan physical fleet        (pack + runtime + deployment)
LAB_8GB / LAB_512 / LAB_256     (deployments)
ChatGPT bridge / model middleware (apps)
Hermes / OpenClaw runtimes
science-core projections beyond PROV skeleton (Etapas 7-8)
Homebrew / GitHub Actions distribution
resident labd daemon loop (v0 ships labd as embeddable host lib + CLI-driven host)
```

## Storage grades (Etapa 1)

```txt
candidate-only   capture candidates only; cannot claim protocol-grade admission
dev-ephemeral    local append-only Act-log spine; DEV-ONLY / unregistered /
                 non-publication-grade
publication      a declared external Spine Profile (postgres/neon/supabase/byo) —
                 SOON in v0; required for publication-grade admitted Acts
```

Acts admitted without a publication-grade spine are labelled `dev-only`.

## Santo André note (the source of the storage noise)

Santo André is Dan's own Lab, derived from the generic kit. It legitimately needs
high-frequency local SQL + an outbox + a Supabase queue + async ingestion into
registered Acts. **That is Santo-André-specific and belongs LATER as a
pack/profile/deployment. It must not define generic Lab Kit storage.**
