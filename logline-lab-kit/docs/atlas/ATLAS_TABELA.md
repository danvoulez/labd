> **Historical.** Superseded for generic v0 by `recovery/RELEASE_SCOPE.md` and `recovery/ACCEPTANCE_STATUS.md`. Kept for the record; figures like "43 tests", "A1–A30", "demo pack", and any "outbox is the record" / Supabase-as-default language reflect an earlier state.

# ATLAS TABELA — LogLine / Lab Kit / Minilab ecosystem

Tabela operacional derivada do `MAP.md`, do transcript e dos pacotes lidos nesta conversa.

**Regra de preenchimento:** Dan decide status. Campos desconhecidos ficam vazios ou `⬜`. Este arquivo não é receipt, não é fonte de verdade e não afirma implementação.

## Colunas

- `ID`: identificador estável para referência rápida.
- `Camada`: área do ecossistema.
- `Item`: peça, subpeça, obrigação, app, profile, pack, comando, prova ou decisão.
- `Função`: para que serve.
- `Onde`: repo, path, Lab, máquina, superfície ou fonte esperada.
- `Linguagem / SDK`: stack provável ou exigida.
- `Código pronto?`: deixado vazio/`⬜` quando incerto; `parcial` quando há material conhecido mas não fechado.
- `Status`: marcador que Dan preenche.
- `Como montar`: encaixe operacional mínimo.
- `Vazio / próxima prova`: o buraco que precisa aparecer.

## Legenda sugerida

```txt
⬜  vazio / status não preenchido
✅  existe e foi verificado por prova escopada
🟡  parcial / existe mas precisa alinhamento
🔴  necessário e ainda não feito
❓  desconhecido — verificar antes de confiar
⛔  proibido / removido / não reconstruir
```

## Tabela

| ID | Camada | Item | Função | Onde | Linguagem / SDK | Código pronto? | Status | Como montar | Vazio / próxima prova |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0.0 | Mapa | Ecosystem Atlas | Índice operacional do ecossistema inteiro; ver estrutura, vazios, dependências e provas | MAP.md / ATLAS_TABELA.md | Markdown | sim | ⬜ | Manter este arquivo como tabela mestre; status preenchido manualmente por Dan | Não usar como receipt; é tracker |
| 0.1 | Mapa | Diagrama raiz | Mostrar fluxo Foundation → Lab Kit → Labs → Federation → Surfaces | MAP.md | Markdown diagram | sim | ⬜ | Manter diagrama curto e usar tabela para detalhe | Atualizar quando camada nova entrar |
| 0.2 | Mapa | Legend/status | Padronizar status sem fingir prova | MAP.md | Markdown | sim | ⬜ | Dan troca marcadores; assistente deixa vazio quando incerto | Evitar preencher status por entusiasmo |
| 1.0 | LogLine Foundation | Foundation repos | Publicar protocolo mínimo; não operar Labs | GitHub Foundation repos / _archive/logline-foundation-canonical-repos 2 | Markdown · JSON Schema · Rust | parcial | ⬜ | Reconciliar repo público com cópia vendored; congelar versão usada | Confirmação de rev/commit |
| 1.1 | LogLine Foundation | Canonical Act form | Forma única canônica: nove slots | canon / docs / schemas | JSON Schema / Markdown | sim | ⬜ | Garantir schema com who/did/this/when/confirmed_by/if_ok/if_doubt/if_not/status | Nenhum décimo slot |
| 1.2 | LogLine Foundation | Slot: who | Identidade declarada de quem origina o ato | Act schema | JSON | ⬜ | ⬜ | Definir constraints mínimas sem domain-lock | Vocabulário de identidade |
| 1.3 | LogLine Foundation | Slot: did | Verbo/ação do movimento semântico | Act schema | JSON | ⬜ | ⬜ | Validar presença; verbo domain-specific vem de pack/convention | Classificação de verbos |
| 1.4 | LogLine Foundation | Slot: this | Payload/claim/objeto tocado pelo Act | Act schema | JSON | ⬜ | ⬜ | Permitir objeto flexível; meaning específico via projection/pack | Limites de tamanho e tipos |
| 1.5 | LogLine Foundation | Slot: when | Momento do Act; base do Lab-as-clock | Act schema | ISO 8601 / time discipline | ⬜ | ⬜ | Padronizar timestamp e clock tolerance | Clock/source policy |
| 1.6 | LogLine Foundation | Slot: confirmed_by | Pivô de evidência/testemunha que seleciona rota | Act schema / engine | JSON / Rust | parcial | ⬜ | Engine deve avaliar evidência sem executar consequência | Vetor de conformance para witness |
| 1.7 | LogLine Foundation | Slot: if_ok | Rota quando há suporte suficiente | Act schema | JSON | ⬜ | ⬜ | Obrigatório; não pode ser blank para action consequente | Semântica mínima universal |
| 1.8 | LogLine Foundation | Slot: if_doubt | Rota de falta de prova/simulação/plano/bloqueio | Act schema | JSON | ⬜ | ⬜ | Obrigatório; não executa efeito externo por si | Vetor ambiguous |
| 1.9 | LogLine Foundation | Slot: if_not | Rota de negação/falha/contradição | Act schema | JSON | ⬜ | ⬜ | Obrigatório; não confundir falha técnica com falsificação | Taxonomia técnica vs semântica |
| 1.10 | LogLine Foundation | Slot: status | Estado do Act sem virar ontologia universal | Act schema | JSON | ⬜ | ⬜ | Status vocabulary definido por profile/pack/conformance/practice | Lista mínima por pack |
| 1.11 | LogLine Foundation | Canonicalization | Same meaning → same bytes → same hash | canon / conformance | JCS RFC 8785 | parcial | ⬜ | Implementar/citar RFC8785; testar roundtrip | Byte vectors finais |
| 1.12 | LogLine Foundation | tuple_hash | Identidade do core de nove slots | engine / conformance | Rust / SHA-256 or profile | parcial | ⬜ | Definir profile exato; rodar vectors | Evitar divergência length-prefixed vs JCS |
| 1.13 | LogLine Foundation | content_hash | Identidade do conteúdo/record sem auto-referência | engine / receipt profile | Rust / SHA-256 | parcial | ⬜ | Definir exclusion de id/hashes; testar | Profile final |
| 1.14 | LogLine Foundation | envelope_hash | Hash de transporte/proveniência fora do Act/receipt core | envelope spec | Rust / JSON | ⬜ | ⬜ | Se usado, manter fora do molde canônico | Confirmar se permanece Convention |
| 1.15 | LogLine Foundation | Receipt profile | Fechamento escopado; receipts beat stories | conformance / schemas | JSON Schema | parcial | ⬜ | Receipt candidate só vira receipt com evidence e scope | Finalizar mold logline.receipt.v0 |
| 1.16 | LogLine Foundation | Conformance valid | Casos que devem passar | conformance/valid | JSON fixtures | parcial | ⬜ | Organizar fixtures por Act/receipt/adapter | Cobertura |
| 1.17 | LogLine Foundation | Conformance invalid | Casos que devem falhar | conformance/invalid | JSON fixtures | parcial | ⬜ | Incluir tenth-slot, missing route, bad hash, fake receipt | Cobertura |
| 1.18 | LogLine Foundation | Conformance ambiguous | Casos que devem ir para doubt/blocked, não pass/fail falso | conformance/ambiguous | JSON fixtures | parcial | ⬜ | Formalizar ambiguous como classe própria | Runner semantics |
| 1.19 | LogLine Foundation | Governance / LIP | Mudar canon raramente e publicamente | governance / lips | Markdown | parcial | ⬜ | Definir estados Draft/Proposed/Accepted/Superseded/Rejected | Foundation anchor/signature |
| 1.20 | LogLine Foundation | Foundation anchor | Assinar origem/versionamento do protocolo | templates/anchor.md / GitHub | Ed25519 did:key | ⬜ | ⬜ | Gerar genesis Act self-confirmed e publicar assinatura | Chave real e rotation policy |
| 2.0 | Engine | Rust logline engine | Juiz canônico local: parse, validate, branch, hash, receipt | _archive/.../engine-main | Rust / cargo | sim/parcial | ⬜ | Lift out of archive; cargo build; expose LOGLINE_RUNTIME_BIN | Re-run current vectors |
| 2.1 | Engine | Nine slot crates | Implementar comportamento por slot | engine-main/crates | Rust | sim/parcial | ⬜ | Auditar contra 01_the_act.md e conformance | Desalinhamentos |
| 2.2 | Engine | confirmed_by evaluator | Selecionar ok/doubt/not por witness/evidence | engine-main | Rust | sim/parcial | ⬜ | Testar mesma Act com e sem evidence | Vetor independente |
| 2.3 | Engine | did classifier | Classificar verbos sem inventar autorização | engine-main | Rust | parcial | ⬜ | Unknown verb deve ir para doubt, não guess | Pack conventions |
| 2.4 | Engine | logline CLI | Interface local para parse/check/run | engine-main/crates/logline-cli | Rust CLI | sim/parcial | ⬜ | Comandos version/status/check-canon/parse/run | Empacotamento |
| 2.5 | Engine | Adapter protocol | Seam para runtimes externos chamarem engine | spec/logline.adapter.v0 | Spec / Rust / JSON | parcial | ⬜ | Ler em detalhe antes de criar bindings TS | Compat com AI SDK/MCP |
| 2.6 | Engine | Core vs admission split | Separar canon/hashing de política/admissão | engine + runtime | Rust | ⬜ | ⬜ | Definir crates ou módulos distintos | Evitar engine virar Tower |
| 2.7 | Admission | constitutional-runtime | Admissão semântica/policy/IR/lowering/evidence | _archive/.../constitutional-runtime-main | Rust | parcial | ⬜ | Verificar embalagem; decidir uso ou simplificação | Passa if-doubt vectors? |
| 2.8 | Admission | Gate predecessor | Preparar allow/require/deny sem executar | constitutional-runtime / labd | Rust | ⬜ | ⬜ | Boundary entre candidate Act e worker | Policy JSON final |
| 3.0 | Lab Kit | Product root | Kit operacional instalável e opinon-free | logline-lab-kit/ | Rust + Markdown + SQL | parcial | ⬜ | Unificar árvore final com crates/docs/templates/examples | Repo vivo |
| 3.1 | Lab Kit | labd host | Host binário: engine + gate + clock + adapters + API | crates/logline-lab-labd or labd | Rust / axum? | parcial | ⬜ | Marrar engine/runtime, expor HTTP/MCP-friendly endpoints | Canon alignment |
| 3.2 | Lab Kit | CLI | Comando humano para emitir/ler Acts e operar Lab | crates/logline-lab-cli | Rust clap or oclif TS | parcial | ⬜ | Escolher Rust CLI vs oclif plugin model; manter same emit path | Decisão de stack |
| 3.3 | Lab Kit | Act core crate | Model, validate, canonicalize, hash | crates/logline-act | Rust | parcial | ⬜ | Extrair do engine ou importar foundation | No duplication |
| 3.4 | Lab Kit | Core crate | Manifest, admission, errors, domain-agnostic models | crates/logline-lab-core | Rust | ⬜ | ⬜ | Criar API interna pequena | Contratos |
| 3.5 | Lab Kit | Local cache/outbox | Buffer provisório; retry; nunca truth | crates/logline-lab-local | Rust + SQLite | parcial | ⬜ | SQLite como queue/outbox; claim limits explícitos | No ledger language |
| 3.6 | Lab Kit | Supabase/Postgres adapter | Profile v0 spine adapter | crates/logline-lab-supabase | Rust/TS + SQL | parcial | ⬜ | Insert/upsert idempotente em ops.logline_acts | RLS/ingest function |
| 3.7 | Lab Kit | Postgres generic profile | Spine adapter sem Supabase-specifics | profiles/postgres | SQL / Rust | ⬜ | ⬜ | Generalizar do Supabase profile | Connection model |
| 3.8 | Lab Kit | Filesystem manual profile | Export/debug/manual mode, não official spine | profiles/filesystem-manual | Files/JSON | ⬜ | ⬜ | Deixar claro que é export/fixture | No file as truth |
| 3.9 | Lab Kit | Manifest | Declarar Lab instance, practice, profile, projectors, policies | manifests/ / schemas/lab-manifest | YAML/JSON Schema | parcial | ⬜ | Exemplo + validator + evolution rules | Required fields final |
| 3.10 | Lab Kit | Bundle / pack manifest | Transportar conventions, assets, policies, projectors | templates/bundle / pack.json | JSON/YAML | ⬜ | ⬜ | Schema para pack import | Bundle identity hash |
| 3.11 | Lab Kit | Convention Table | Nomear receipt/grant/blocked/check/report sem virar canon | templates/conventions.md | Markdown/JSON | parcial | ⬜ | Criar table machine-readable opcional | Versionamento |
| 3.12 | Lab Kit | Envelope | Carregar provenance/run metadata ao redor do Act | schemas/runtime-envelope | JSON Schema | parcial | ⬜ | Manter fora dos nove slots | Fields finais |
| 3.13 | Lab Kit | Projection | Read models derivados dos Acts | crates/logline-lab-projectors | Rust/SQL | parcial | ⬜ | Project registry/evidence/receipt/blocked/status | Rebuild guarantees |
| 3.14 | Lab Kit | Clock | Emitir timed Acts; due checks; não cron invisível | crates/logline-lab-clock | Rust / scheduler | ⬜ | ⬜ | Scheduler só dispara heartbeat; tick vira Act | Due semantics |
| 3.15 | Lab Kit | Hooks | Extensão em pontos declarados sem core edits | hooks/default | Scripts/JSON | ⬜ | ⬜ | Definir hook contract e allowed effects | Security model |
| 3.16 | Lab Kit | Worker boundary | Transformar admitted work em execução e evidence | crates/logline-lab-hermes or worker | Rust/TS/MCP | ⬜ | ⬜ | Workorder narrow, evidence return, no closure | Hermes/OpenClaw decision |
| 3.17 | Lab Kit | Reports | Daily/expedition/conformance/audit/learning views | reports/templates | Markdown templates | parcial | ⬜ | Render from projections/evidence/receipts | No report as receipt |
| 3.18 | Lab Kit | Study benches | Ambientes de estudo de Acts/runtimes/receipts/etc | benches/ | Markdown/fixtures/tests | parcial | ⬜ | Each bench: README/examples/fixtures/invalid/exercises | Coverage |
| 3.19 | Lab Kit | Docs surfaces | Start/Practice/Study/Build/Reference | docs/00..14 | Markdown | parcial | ⬜ | Gerar docs consistentes com product surfaces | Avoid old jargon |
| 4.0 | Spine | ops.logline_acts | Official semantic spine for Supabase profile | Supabase/Postgres | SQL | parcial | ⬜ | Table with 9 slots + hashes/envelope refs + created_at | Migration 0001 final |
| 4.1 | Spine | Remote ingest function | Validar/idempotent upsert de Acts | Supabase SQL/RPC | SQL/PLpgSQL | parcial | ⬜ | ops.ingest_logline_act; count 0→1 repeat stays 1 | RLS/security |
| 4.2 | Spine | Local outbox sync | Mover Acts do cache para spine | local + supabase adapter | Rust/SQLite/Postgres | ⬜ | ⬜ | Retry finite; no silent drop; receipts not claimed until accepted | Backoff/dead letter |
| 4.3 | Projections | registry.entities | What exists now, derived from Acts | registry schema | SQL | ⬜ | ⬜ | Project register/rename/retire Acts | Stable IDs |
| 4.4 | Projections | registry.runtimes | Named machines/runtimes and state | registry schema | SQL | ⬜ | ⬜ | Project runtime registration/heartbeat Acts | Staleness |
| 4.5 | Projections | registry.links | Relations between entities | registry schema | SQL | ⬜ | ⬜ | Project link/grant/contract Acts | Relationship semantics |
| 4.6 | Projections | evidence_records | Referenceable observations | evidence schema | SQL/Storage refs | ⬜ | ⬜ | Store scope, producer, redaction, payload_ref/hash | Evidence shape final |
| 4.7 | Projections | receipt_index | Scoped closure index | receipts schema | SQL | ⬜ | ⬜ | Receipt candidate review then index closed scope | Conformance link |
| 4.8 | Projections | blocked/ghost view | What is missing/stuck and why | audit.open_blocked / registry.blocked | SQL | ⬜ | ⬜ | Project blocked Acts / carry / close / reject | Name ghost vs blocked Act decision |
| 4.9 | Projections | audit.v_mobile_today | Phone-friendly current status | audit schema | SQL | ⬜ | ⬜ | Read from Acts/projections; no direct truth | Fields |
| 4.10 | Projections | report views | Daily/learning reports | audit/report schemas | SQL + templates | ⬜ | ⬜ | Render summaries with evidence state | Stale-data warnings |
| 4.11 | Projections | projection rebuild | Recompute all read models from spine | projector functions | SQL/Rust | ⬜ | ⬜ | Rebuild on pack/proof-rule change | Determinism tests |
| 4.12 | Projections | projection staleness | Mark derived views stale after rules/source changes | projection metadata | SQL | ⬜ | ⬜ | Store derives_from refs and last_projected_at | Staleness policy |
| 5.0 | Layering | Canon | Small/stable Foundation rules | Foundation | Spec | sim/parcial | ⬜ | Do not amend for local Lab | Promotion process |
| 5.1 | Layering | Pack | Opinionated conventions, policies, data, projectors | packs/ | JSON/YAML/assets | ⬜ | ⬜ | Pack manifest + content hash + import Act | Pack schema |
| 5.2 | Layering | Profile | Backend/capability/runtime/storage choices | profiles/ | YAML/JSON | ⬜ | ⬜ | Profile declares storage/auth/runtime/adapters | No doctrine in profile |
| 5.3 | Layering | Lab instance | Kit + pack + profile + anchor + spine | Labs | Acts + config | ⬜ | ⬜ | Declared by Lab genesis/register Act | Lab manifest |
| 5.4 | Layering | Overlay | Private deployment identity/policy/taste | dan.minilab.work etc | Config/secrets/UI | ⬜ | ⬜ | Keep outside universal product | Boundary docs |
| 5.5 | Layering | Frontend | Optional human surface | CLI/web/phone/ChatGPT | TS/Rust/etc | ⬜ | ⬜ | Consume projections; never govern | Surface inventory |
| 5.6 | Pack | Santo André Pack | Recommended/reference pack by Dan | packs/santo-andre | Acts/JSON/assets | ⬜ | ⬜ | Declare pack_id; include dispatchers/proof rules/projectors | Contents list |
| 5.7 | Pack | Manhattan Pack | Fleet health/repair conventions | packs/manhattan | Acts/JSON/Rust/Python | parcial | ⬜ | L-01..L-30 as conventions/probes/workorders | Pack manifest |
| 5.8 | Pack | Personal Offline Pack | Private longitudinal record/proof-of-human-life practice | packs/personal-offline | Acts/adapters | ⬜ | ⬜ | Local/private profile + batch signing | Adapter priority |
| 5.9 | Pack | Classroom/Contract/Runtime Observatory packs | Future example packs | packs/future | ⬜ | ⬜ | ⬜ | Leave empty until needed | Use cases |
| 6.0 | Santo André Lab | Lab identity | First/reference Lab instance | Santo André Lab | Act + did:key | parcial | ⬜ | Genesis Act + anchor + pack declaration | Real published anchor |
| 6.1 | Santo André Lab | Anchor | Self-attested genesis; verify remote Acts | templates/examples/santo-andre.genesis.json | Ed25519 did:key | sim/parcial | ⬜ | Persist key outside repo; publish triple | Key custody |
| 6.2 | Santo André Lab | Spine profile | Supabase/Postgres v0 spine | Supabase project | Postgres/Supabase | parcial | ⬜ | Connect profile; ingest Acts; projections | RLS and backup |
| 6.3 | Santo André Lab | minilab.work face | Digital surface/projection of Lab state | minilab.work | TS/Next/Cloudflare | parcial | ⬜ | Read projections; no direct semantic writes | Current deployment truth |
| 6.4 | Santo André Lab | Research method | Never let beauty outrun evidence | Santo André docs/benches | Markdown/Acts | parcial | ⬜ | Experiment registry, probes, receipts | First active track |
| 6.5 | Santo André Lab | First proof path | Prove LabKit end-to-end in reference Lab | LAB_8GB + spine + minilab.work | Rust/SQL/TS | ⬜ | ⬜ | Run admitted L-06 or tick-to-spine proof | Receipt |
| 6.6 | Manhattan Lab | Lab identity | Peer Lab for physical fleet | 3 Mac minis | Act + did:key | ⬜ | ⬜ | Own anchor, own/federated spine, Manhattan Pack | Decide co-locate vs separate spine |
| 6.7 | Manhattan Lab | LAB_8GB Capital | Supervisor / Capital / Hermes-clock-MCP-spine | LAB_8GB | macOS + launchd + MCP | parcial | ⬜ | Install Kit/Manhattan pack; run probes first | PH-00/PH-01 receipts |
| 6.8 | Manhattan Lab | LAB_512 engine room | Inference/model worker | LAB_512 | macOS + OpenAI-compatible model | parcial | ⬜ | Keep orchestration light; serve model over private bus | Runtime health |
| 6.9 | Manhattan Lab | LAB_256 workbench | Human bench / cockpit / hands-on work | LAB_256 | macOS + pitwall/vibe-codin | parcial | ⬜ | Bind pitwall/cockpit to Act spine | Actual install |
| 6.10 | Personal Offline Lab | Identity | Private longitudinal Lab | local/private | Act + local profile | ⬜ | ⬜ | Declare Lab with local/private pack/profile | Scope and storage |
| 6.11 | Personal Offline Lab | Adapters | Email/location/apps/personal signals into Acts | local adapters | MCP/OS APIs | ⬜ | ⬜ | Capture candidates, batch sign/checkpoint | Privacy boundaries |
| 6.12 | Community/Company Labs | Future instances | Third parties run their own Labs | external | varies | ⬜ | ⬜ | Kit + their pack/profile/anchor | None yet |
| 7.0 | Manhattan | Manhattan design | Fleet observe/repair/receipt system | _archive/manhattan 3/project-manhattan-v2 | Python/Rust? launchd/macOS | parcial | ⬜ | Treat as design awaiting proof; wrap as Lab Connection app | Install/validation |
| 7.1 | Manhattan | L-01 Identity | Machine identity gate; halt mutation on mismatch | Manhattan Pack | macOS probes | parcial | ⬜ | Probe ComputerName/HostName/LocalHostName; receipt | Identity receipt |
| 7.2 | Manhattan | L-02 Auto-login | Headless boot/session availability | Manhattan Pack | macOS settings | ⬜ | ⬜ | Probe and repair if approved | MDM ghost |
| 7.3 | Manhattan | L-03 FileVault off | Allow unattended boot | Manhattan Pack | fdesetup | ⬜ | ⬜ | Probe only unless human-approved | Security acceptance |
| 7.4 | Manhattan | L-04 Keychain usable | User-session credential availability | user_agent | macOS keychain | ⬜ | ⬜ | Requires user agent | Agent implementation |
| 7.5 | Manhattan | L-05 Wi-Fi route | Internet path via Wi-Fi | Manhattan Pack | networksetup/route | ⬜ | ⬜ | Wi-Fi has gateway/DNS; Ethernet not gateway | Actual iface |
| 7.6 | Manhattan | L-06 Ethernet peer /30 | Private inference bus 8GB↔512 | LAB_8GB/LAB_512 | macOS networksetup/ping | parcial | ⬜ | Probe static /30 no gateway/DNS; ping over cable | Done-v1 target |
| 7.7 | Manhattan | L-07 Peer witness | Machines witness each other / WOL planning | Manhattan Pack | ping/WOL | ⬜ | ⬜ | Probe peer; WOL only after MAC policy | G-02/G-03 |
| 7.8 | Manhattan | L-08 SSH port 22 | Remote shell standard surface | Manhattan Pack | sshd | ⬜ | ⬜ | Use port 22 only; no com.lab-sshd-2222 | Reachability |
| 7.9 | Manhattan | L-09 pmset no-sleep | Prevent sleep/headless downtime | Manhattan Pack | pmset | ⬜ | ⬜ | Include ttyskeepawake; receipt | BUG-07 |
| 7.10 | Manhattan | L-10 Firewall off | Chosen local accessibility posture | Manhattan Pack | socketfilterfw | ⬜ | ⬜ | Probe/repair if approved | Risk acceptance |
| 7.11 | Manhattan | L-11 pf disabled | No packet filter drift | Manhattan Pack | pfctl | ⬜ | ⬜ | Probe/repair if approved | Policy |
| 7.12 | Manhattan | L-12 Screensaver/lock disabled | Headless session usability | user_agent | macOS defaults | ⬜ | ⬜ | Requires user agent; no root fake | Agent |
| 7.13 | Manhattan | L-13 TeamViewer system | Remote vendor system service | launchd vendor | launchctl | ⬜ | ⬜ | Treat vendor plist as admitted service | Plist scan |
| 7.14 | Manhattan | L-14 TeamViewer user | User-session remote component | user_agent / LaunchAgents | launchctl gui | ⬜ | ⬜ | Scan /Library/LaunchAgents too | BUG-04 |
| 7.15 | Manhattan | L-15 cloudflared process | Tunnel process health | launchd vendor | cloudflared | ⬜ | ⬜ | Probe process/plist without corrupt parse | BUG-05 |
| 7.16 | Manhattan | L-16 Cloudflare tunnel health | Actual tunnel reachability, not just process | user_agent/system? | HTTP endpoint | ⬜ | ⬜ | Resolve endpoint + parse pattern | G-01 |
| 7.17 | Manhattan | L-17 Login survival | Prove user session survives reboot | user_agent | macOS/loginwindow | ⬜ | ⬜ | Post-reboot evidence | PH-10 |
| 7.18 | Manhattan | L-18 Daily rejuvenation phases | Pre-restart cleanup/refresh | daemon loop | shell/macOS | ⬜ | ⬜ | Implement as daemon loop phases | Receipt retention |
| 7.19 | Manhattan | L-19 Daily restart | Daily reboot by daemon loop | daemon loop | shutdown -r now | ⬜ | ⬜ | Stagger 06:40/06:50/06:59; no pmset schedule | Receipt before restart |
| 7.20 | Manhattan | L-20 Post-reboot survival | Prove Manhattan returns after reboot | daemon+agent | launchd/HTTP | ⬜ | ⬜ | Reboot then receipt after alive | PH-10 |
| 7.21 | Manhattan | L-21 Filesystem gate | Human/sudo boundary for mutation | gate paths | sudo/fs | ⬜ | ⬜ | Config-driven paths; open/close receipts; sudo -K | BUG-03 |
| 7.22 | Manhattan | L-22 No parallel authority | No helper plists/watchdogs/rogue services | launchd scan | macOS | ⬜ | ⬜ | Audit all domains incl LaunchAgents | Final rogue=0 |
| 7.23 | Manhattan | L-23 Receipts | Proof trail for checks/repairs | receipt store | JSON/LogLine Act | ⬜ | ⬜ | Retention: audit 7d, mutation/gate/restart forever | BUG-06 |
| 7.24 | Manhattan | L-24 Drift detection | Detect and report divergence | daemon/user_agent | probes | ⬜ | ⬜ | Each L-item has probe, expected, drift condition | Projection |
| 7.25 | Manhattan | L-25 Remote Management | Canonical Apple remote surface | macOS ARD | kickstart/systemsetup | ⬜ | ⬜ | Probe install/enable state; not Screen Sharing | G-07/MDM |
| 7.26 | Manhattan | L-26 Bluetooth | Apple continuity/local controls | macOS | blueutil? system_profiler? | ⬜ | ⬜ | Probe reliable CLI by macOS version | G-04 |
| 7.27 | Manhattan | L-27 AirDrop | User-session sharing surface | user_agent | macOS defaults | ⬜ | ⬜ | Requires user session proof | G-04 |
| 7.28 | Manhattan | L-28 Handoff/Universal Control | Apple stack continuity | user_agent | macOS defaults | ⬜ | ⬜ | No third-party mouse tool without approval | Reliability |
| 7.29 | Manhattan | L-29 TCC/manual MDM | Permission constraints/manual profile | macOS TCC/MDM | profiles | ⬜ | ⬜ | Document what cannot be automated | G-05 |
| 7.30 | Manhattan | L-30 LAB Runtime | Runtime process for Lab operations | LAB machines | launchd/MCP/labd | ⬜ | ⬜ | Define process and heartbeat contract | Runtime receipt |
| 7.31 | Manhattan | PH-00 baseline | Read-only v1 evidence | LAB_8GB first | probes | ⬜ | ⬜ | No mutation; capture facts | Baseline receipt |
| 7.32 | Manhattan | PH-01 ghost resolution | Resolve read-only ghosts G-01/G-02/G-05/G-07/G-08 | LAB_8GB | probes | ⬜ | ⬜ | Probe actual endpoints/interfaces/MDM/ARD/MACs | No inference |
| 7.33 | Manhattan | PH-02 bug patches | Fix BUG-01..BUG-07 safely | repo | code + gate | ⬜ | ⬜ | Open fs gate, patch, test, close gate | Patch receipts |
| 7.34 | Manhattan | PH-10 final audit | Reboot survival and clean audit | LAB_8GB | daemon/receipts | ⬜ | ⬜ | post-reboot OK + daily restart requested + rogue/stale/drift zero | Porting gate |
| 8.0 | Federation | Inter-Lab contract | Signed contract Acts between Labs | Santo André ↔ Manhattan | Acts + signatures | ⬜ | ⬜ | Both anchors sign same contract content_hash | Contract schema |
| 8.1 | Federation | Remote verification | Verify remote Acts without shared DB | federation adapter | Rust/TS crypto | ⬜ | ⬜ | Resolve anchor, recompute hash, verify signature | Anchor publication |
| 8.2 | Federation | Data minimization | Share verifiability without surveillance | projection policy | Policy JSON | ⬜ | ⬜ | Project only needed proof/status, not raw private state | Policy table |
| 8.3 | Federation | First federation demo | Manhattan health consumed by Santo André | Santo André spine/projection | Acts/HTTP/MCP | ⬜ | ⬜ | Manhattan emits signed health; Santo André verifies and renders | End-to-end receipt |
| 9.0 | Apps/MCP | Lab Connection app standard | Capabilities plug into Lab via MCP | MCP host-runtime | MCP TypeScript SDK | parcial | ⬜ | Register app as entity, issue grant, tools emit draft Acts | SDK selection |
| 9.1 | Apps/MCP | register_entity app | Identity for app/tool | spine | Act | ⬜ | ⬜ | Connect app → emit register_entity | Projection |
| 9.2 | Apps/MCP | issue_grant | Scoped/time-limited permissions | spine/gate | Act + policy | ⬜ | ⬜ | Grant verbs/resources/duration; revoke by later Act | Grant convention |
| 9.3 | Apps/MCP | Web search app | Research/citation capability | MCP app | TS/HTTP | ⬜ | ⬜ | Tool call becomes draft Act; evidence refs returned | Provider choice |
| 9.4 | Apps/MCP | Calendar app | Schedule/due Acts integration | MCP app | Google Calendar/MCP | ⬜ | ⬜ | Calendar events as observations/projections, not truth | Privacy |
| 9.5 | Apps/MCP | Computer control app | GUI/terminal hands on Macs | MCP host runtime | MCP/Apple/PTY | parcial | ⬜ | Protected commands through gate | Tool threat model |
| 9.6 | Apps/MCP | Manhattan app | Check/repair physical fleet | Manhattan endpoints/MCP wrapper | TS/Python/MCP | ⬜ | ⬜ | Wrap /health/items/repair as draft Acts/workorders | Install proof |
| 9.7 | CLI | logline status | Show current state projection | CLI | oclif TS or Rust | ⬜ | ⬜ | Read projections only | Stack decision |
| 9.8 | CLI | logline doctor | Check spine/projections/brakes/canon health | CLI | oclif/Rust | ⬜ | ⬜ | Run read-only probes; blocked if unknown | Doctor checklist |
| 9.9 | CLI | logline emit | Emit raw Act | CLI | oclif/Rust | ⬜ | ⬜ | Validate → gate/admit → outbox/spine | Claim limits |
| 9.10 | CLI | logline add entity | Friendly register_entity command | CLI plugin | oclif/Rust | ⬜ | ⬜ | Build Act with did=register_entity | Projection |
| 9.11 | CLI | logline add experiment | Open experiment via Act | CLI plugin | oclif/Rust | ⬜ | ⬜ | Build experiment candidate; route to research pack | Experiment schema |
| 9.12 | CLI | logline blocked | List blocked Acts | CLI | SQL/projection | ⬜ | ⬜ | Read blocked/ghost projection | Terminology decision |
| 9.13 | CLI | logline receipt prepare | Prepare scoped closure candidate | CLI | Rust/TS | ⬜ | ⬜ | Requires evidence refs; no fake receipt | Reviewer path |
| 9.14 | CLI | Plugin model | Packs add command groups | CLI | oclif plugins or Rust subcommands | ⬜ | ⬜ | Manhattan plugin, research plugin, app plugin | Stack decision |
| 10.0 | Bench | pitwall | Terminal/coding bench with event ledger and watchers | ~/pitwall | TypeScript/Electron/node-pty | sim/parcial | ⬜ | Bind PitwallEvent → Act; sync ledger to spine | Verify current build |
| 10.1 | Bench | Pitwall EventLedger | Append-only local events | pitwall/src/core/events | SQLite/TS | sim/parcial | ⬜ | Treat as local event cache, not truth; bridge to Acts | Mapping table |
| 10.2 | Bench | Claim tracking | Claim-vs-evidence state machine | pitwall/src/core/claims | TypeScript | sim/parcial | ⬜ | Map verified→ok, failed/contradicted→not, else doubt | Proof path |
| 10.3 | Bench | Watcher lanes | Scout/reviewer/test/risk monitors | pitwall | TS + model adapters | parcial | ⬜ | Each watcher outputs observations/candidate Acts | No authority |
| 10.4 | Cockpit | vibe-codin | Human-facing cockpit/code-lab surface | ~/vibe-codin-.but.-real | Next.js / AI SDK / shadcn | sim/parcial | ⬜ | Read spine projections; propose Acts; never write direct CRUD | Verify current wiring |
| 10.5 | Cockpit | Daily screen | What changed/alive/blocked/needs Dan | cockpit | Next.js | ⬜ | ⬜ | Render daily status projection | Status schema |
| 10.6 | Cockpit | Control screen | Machines/registry/policies | cockpit | Next.js | ⬜ | ⬜ | Read registry, policies, blocked Acts | No executor UI bypass |
| 10.7 | Cockpit | Work screen | Code bench + experiments | cockpit/pitwall | Next/Electron | ⬜ | ⬜ | Embed/bridge pitwall, receipts, evidence | Integration |
| 10.8 | Models | OpenAI-compatible local model | Standard model endpoint, no custom gateway | LAB_512 | OpenAI-compatible API | parcial | ⬜ | Call directly through AI SDK provider | Current endpoint |
| 10.9 | Models | Vercel AI SDK middleware | Redact, frame as Act, capture claim, canon verdict | middleware package | TypeScript / AI SDK | parcial | ⬜ | wrapLanguageModel with redaction/ledger/tool/canon middleware | Package in repo |
| 10.10 | Models | Secret redaction middleware | Strip secrets before model and before logs | middleware | TS | parcial | ⬜ | Pattern/secret manager refs; test planted token | Coverage |
| 10.11 | Models | Act framing middleware | Put request in Act context | middleware | TS | ⬜ | ⬜ | NL/model call produces candidate Act | Prompt contract |
| 10.12 | Models | Ledger/claim capture middleware | Record model output as claim, not truth | middleware | TS | parcial | ⬜ | evidence_captured not verified | Projection |
| 10.13 | Models | Tool parser middleware | Tool calls for non-native local models | @ai-sdk-tool/parser | TS | parcial | ⬜ | Use hermesToolMiddleware only for non-native models | Current quirks |
| 10.14 | Models | Cloud model adapters | High-quality reasoning via GPT/Claude where needed | pitwall/model adapters | OpenAI/Anthropic SDK | parcial | ⬜ | Same Act/evidence discipline | Provider policy |
| 11.0 | Worker | Hermes | Agent runtime / worker / cron / skills | ~/.local/bin/hermes / ~/.hermes | Nous Hermes runtime | sim/parcial | ⬜ | Decide if primary worker; make actions emit Acts | License/version reconfirm |
| 11.1 | Worker | Hermes cron | Clock heartbeat source candidate | ~/.hermes/cron | cron/hooks | empty | ⬜ | Cron emits tick Acts, not hidden jobs | First tick proof |
| 11.2 | Worker | Hermes skills | Standing work/tool execution | ~/.hermes/skills | skills | parcial | ⬜ | Wrap skill run as workorder/evidence | Scope grants |
| 11.3 | Worker | OpenClaw | Alternate agent runtime / device/task/gate | ~/.openclaw | OpenClaw | sim/parcial | ⬜ | Evaluate against Hermes; maybe secondary | Choose primary |
| 11.4 | Worker | OpenClaw exec approvals | Human-in-loop require route | ~/.openclaw/exec-approvals.json | JSON | parcial | ⬜ | Map to require decision | Approval semantics |
| 11.5 | Worker | MCP host-runtime | Hands/apps on LAB machines | lab512 port 8788 / minilab.work tunnel | Node/MCP | sim/parcial | ⬜ | Register as runtime/app; tool calls draft Acts | Runtime health receipt |
| 11.6 | Worker | Worker alternatives | Ansible/osquery/shell runners | future / Manhattan upstream replacement | Ansible/osquery | parcial | ⬜ | Use proven tools under Act/workorder boundary | Selection |
| 12.0 | Authority | Gate | allow / require / deny for pending Acts | labd / policy | Rust/Policy JSON | ⬜ | ⬜ | Fails closed; never executes itself | Gate 0/1 |
| 12.1 | Authority | Policy JSON | Rules for allowed/require/deny | profiles/policies | JSON | ⬜ | ⬜ | Define command classes, scopes, risk, grants | Policy schema |
| 12.2 | Authority | Brakes | Global safety switches default deny | Doppler/env/profile | env/config | ⬜ | ⬜ | SAFE_MODE, DRY_RUN, ALLOW_EXEC, etc | Config doctor |
| 12.3 | Authority | Protected commands | Destructive/sensitive actions require Dan/window | policy/gate/worker | Policy + OS | ⬜ | ⬜ | Only available behind gate; no voluntary audit | Command list |
| 12.4 | Authority | Passkey/window | Temporary broad execution window for protected actions | future gate | WebAuthn/passkey | ⬜ | ⬜ | Open by Dan, expires automatically | Implementation |
| 12.5 | Security | Doppler | Secrets/config material, not authority | Doppler project | Doppler CLI/API | parcial | ⬜ | Reference secret names, never values in Acts/logs | Rotate leaked token |
| 12.6 | Security | Keychain | Local key persistence for Lab anchor if chosen | macOS keychain | Security framework | ⬜ | ⬜ | Store secret key; public did/key in anchor | Threat model |
| 12.7 | Truth | Source of truth classes | Each truth class has governing source | source map | Markdown/Acts/DB | parcial | ⬜ | Keep source class on claims/reports | Plain-language mapping |
| 12.8 | Truth | Evidence states | unknown/planned/declared/observed/evidence_captured/verified/blocked/etc | status/report schemas | Enum/convention | parcial | ⬜ | Use in all reports/projections | Ghost vs blocked vocabulary |
| 12.9 | Current State | Current State projection | Current operational view; not canon/receipt | projection/report | SQL/Markdown | ⬜ | ⬜ | Rebuild from Acts/evidence/receipts/blocked | Initialize |
| 13.0 | Daily Ops | Daily status report | Lab wakes up honestly | cockpit/report | Markdown/SQL/TS | ⬜ | ⬜ | Sections: daily/control/work/blocked/needs Dan | Template |
| 13.1 | Daily Ops | LAB status | Machines alive/stale/offline/blocked | Manhattan/LAB projections | SQL/HTTP | ⬜ | ⬜ | Heartbeat + observations + blocked Acts | Heartbeat source |
| 13.2 | Daily Ops | Casa/home ops | Domestic continuity if included | home pack/profile | Acts/report | ⬜ | ⬜ | Plans not execution; confirmations as Acts | Dan scope |
| 13.3 | Daily Ops | Work/request ledger | What Dan asked, route, next act | cockpit/spine | Acts/projection | ⬜ | ⬜ | Request is not execution | Current request capture |
| 13.4 | Research | Santo André method | intuition→thesis→invariant→probe→observation→receipt | research bench | Acts/templates | parcial | ⬜ | Every research step as Act | Experiment registry |
| 13.5 | Research | Experiment registry | Track active research/probes/evidence | projection/bench | SQL/Markdown | ⬜ | ⬜ | Project experiment Acts | First track |
| 13.6 | Research | External recognition | Publication/citation/acceptance proof | external registry | URLs/DOI/Acts | ⬜ | ⬜ | Recognition requires external record | None yet |
| 13.7 | Science | LLA-001 agent | Reasoning conformance specialist | templates/traps.md / Agent-LogLine.md | Prompt/RAG/verifiers | parcial | ⬜ | Traps+rubric → prompt → RAG → typed graph → verifiers | Answer key |
| 13.8 | Science | Traps suite | Catch model/protocol reasoning failures | templates/traps.md | Markdown/tests | parcial | ⬜ | 25 traps + baseline generic LLM | Run results |
| 14.0 | App Park | App membership | App is admitted/bounded/observable/recoverable/receiptable | App Park pack | Acts/policy | ⬜ | ⬜ | Container running is not membership | Membership schema |
| 14.1 | App Park | Shared infra | Routes, DB, storage, secrets, jobs for admitted apps | LABs/profile | Cloudflare/Supabase/Doppler | ⬜ | ⬜ | All capabilities through grants and gates | Infra inventory |
| 14.2 | App Park | Intelligence App | Slow cognition/local mini-LLM reports | LAB apps | macOS binary/LLM | ⬜ | ⬜ | Cron beehive; advisory reports with confidence | Phase 2 |
| 14.3 | App Park | Constitutional Runtime platform | Platform app/capability set | future | Rust | ⬜ | ⬜ | After tower/gate/heartbeat foundation | Scope |
| 14.4 | App Park | HUGE Company platform | Commercial platform on LAB infra | future | varies | ⬜ | ⬜ | Runs after command physics exists | Business scope |
| 14.5 | App Park | Personal Engine platform | Personal workflows/continuity | future | varies | ⬜ | ⬜ | Act-shaped, low custom UI | Scope |
| 15.0 | Recovery | Archive recovery | Recover product from contaminated corpus | Archive(1).zip / recovery protocol | shell/Rust/Markdown | parcial | ⬜ | Extract, classify, remove debris, merge, correct authority/storage, mark ghosts | Actual run receipt |
| 15.1 | Recovery | Remove false authority | LLM docs/ADRs/transcripts not decisions | docs cleanup | Markdown | parcial | ⬜ | Demote assistant-generated ADRs/inventory/manifests | Review |
| 15.2 | Recovery | Storage language repair | SQLite/file not truth/spine | code/docs | grep/patch | parcial | ⬜ | Replace ledger/spine wording; rename LocalLedger if safe | Build after patch |
| 15.3 | Recovery | Artifact removal | Remove artifact as semantic category | crates/logline-lab-artifacts etc | Rust | parcial | ⬜ | Remove crate/imports/cleanup/TTL | Build proof |
| 15.4 | Generator | Product catalog | Input for future generator | product.yaml | YAML | draft | ⬜ | Generator follows, does not invent | Finalize catalog |
| 15.5 | Generator | Command catalog | CLI command spec | commands.yaml | YAML | draft | ⬜ | Generate commands/tests/docs | Stack decision |
| 15.6 | Generator | Schema catalog | JSON schema inventory | schemas.yaml | YAML/JSON Schema | draft | ⬜ | Generate schemas only from catalog | Field review |
| 15.7 | Generator | Migration catalog | Supabase/Postgres migration order | migrations.yaml | YAML/SQL | draft | ⬜ | Generate migrations; no extra schemas | SQL review |
| 15.8 | Generator | Acceptance catalog | Acceptance tests inventory | acceptance.yaml | YAML/shell | draft | ⬜ | Generate test templates | Runnable tests |
| 15.9 | Acceptance | Installable | Kit installs cleanly | tests | shell/cargo/npm | ⬜ | ⬜ | test_install.sh | Not run |
| 15.10 | Acceptance | First session | First Lab flow completes | tests | shell/CLI | ⬜ | ⬜ | init→emit→project→blocked→evidence→receipt candidate→report | Not run |
| 15.11 | Acceptance | Emit Act | Act reaches online spine idempotently | tests | Rust/SQL | parcial | ⬜ | count 0→1 repeat 1 | Need official test |
| 15.12 | Acceptance | No file spine | Files are not official semantic storage | tests | grep/test | ⬜ | ⬜ | Scan repo for official acts/*.json claims | Not run |
| 15.13 | Acceptance | No fake receipt | Receipt requires evidence/scope | tests | schema/fixtures | ⬜ | ⬜ | Invalid receipt fixtures fail | Not run |
| 16.0 | Build Order | Phase A — stabilize form | Freeze Act/hash/receipt/conformance basis | Foundation/engine | Rust/JSON | ⬜ | ⬜ | Re-run vectors; reconcile docs | Proof |
| 16.1 | Build Order | Phase B — live engine | Build and expose logline binary | engine | Rust | ⬜ | ⬜ | cargo build; LOGLINE_RUNTIME_BIN | Receipt |
| 16.2 | Build Order | Phase C — Act spine circuit | emit→outbox→spine→readback | Lab Kit + Supabase | Rust/SQL | ⬜ | ⬜ | First official circuit test | Receipt |
| 16.3 | Build Order | Phase D — labd/gate | Admission + allow/require/deny endpoint | labd | Rust/HTTP | ⬜ | ⬜ | No world-touch first | Boundary proof |
| 16.4 | Build Order | Phase E — worker boundary | Admitted workorder → execution report → evidence | worker | Hermes/OpenClaw/MCP | ⬜ | ⬜ | Read-only probe first | Evidence |
| 16.5 | Build Order | Phase F — Manhattan L-06 | Physical proof over Ethernet bus | LAB_8GB/LAB_512 | macOS/ping/Acts | ⬜ | ⬜ | Admit L-06, ping, receipt, spine, projection | Done-v1 |
| 16.6 | Build Order | Phase G — product experience | First session user journey | docs/CLI/bench | Markdown/CLI/UI | ⬜ | ⬜ | Load/Declare/Observe/Emit/Project/Learn | Report |
| 16.7 | Open Decision | Vocabulary public vs internal | Decide ghost vs blocked, Hermes vs worker, canon vs rules | docs/glossary | Markdown | ⬜ | ⬜ | Dan call; maybe maintain public/plain aliases | Decision |
| 16.8 | Open Decision | Primary worker runtime | Hermes vs OpenClaw vs both | runtime layer | runtime configs | ⬜ | ⬜ | Probe which emits Acts cleaner | Decision |
| 16.9 | Open Decision | CLI stack | Rust clap vs oclif TS | CLI | Rust/TS | ⬜ | ⬜ | Pick based on plugin growth + engine binding | Decision |
| 16.10 | Open Decision | Spine default for generic Kit | File default offline vs Supabase profile default | profiles | profile config | ⬜ | ⬜ | Do not universalize Supabase | Decision |
| 16.11 | Forbidden | No custom gateway | Do not rebuild llm-gateway; use standard API + middleware | model layer | AI SDK | ⛔ | ⬜ | Bypass poisoned gateway | Rotate any leaked secrets |
| 16.12 | Forbidden | No file as truth | Files are examples/exports/debug/reports | all repo | n/a | ⛔ | ⬜ | No official acts/*.json spine | Scans |
| 16.13 | Forbidden | No artifact semantic category | Artifact removed as native concept | code/docs | n/a | ⛔ | ⬜ | Use evidence refs/output refs, not artifact ontology | Scans |
| 16.14 | Forbidden | No direct CRUD semantic writes | Forms/tables do not govern | DB/UI | SQL/TS | ⛔ | ⬜ | All semantic change through Acts | Tests |
| 16.15 | Forbidden | No AI authority | Model output draft/candidate only | AI layer | n/a | ⛔ | ⬜ | Models propose; gate/human/rules decide | Policy |
| 16.16 | Final | Done v1 sentence | End-to-end protocol proof | LAB_8GB → LAB_512 → spine → minilab.work | labd/Manhattan/Supabase/TS | ⬜ | ⬜ | labd on LAB_8GB admits L-06, runs ping, closes signed receipt, writes spine, minilab.work shows healthy | The receipt |

## Conferência rápida do MAP.md

- O `MAP.md` atual está bom como diagrama/índice hierárquico.
- Esta tabela é melhor para enxergar vazios porque separa função, stack, existência de código e prova necessária.
- Mantive `Status` quase sempre como `⬜`; não forcei falso fechamento.
- Onde escrevi `parcial`, é porque o transcript ou os docs mencionam código/material existente, mas sem transformar isso em `done`.
- Recomendo editar status nesta tabela e manter `MAP.md` como mapa visual/árvore.