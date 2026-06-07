//! `labkit` — the generic LogLine Lab Kit command surface.
//!
//! Headless-first: every experience surface is a command family that prints a
//! stable JSON read-model (each carries a `kind` contract tag). A Lab is a
//! directory on disk (`--store`); the same Lab is identical for the CLI, an MCP
//! client, or any future GUI/TUI. Generic machinery only (Operator §13): a Lab
//! needs identity + profile; packs are optional complements.

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use logline_act::Act;
use logline_lab_core::{bench::StudyBench, LabManifest, PackManifest, ProfileManifest};
use logline_lab_labd::{Lab, ResidentSession};

#[derive(Parser)]
#[command(name = "labkit", version, about = "LogLine Lab Kit — installable Lab formation kit")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Args, Clone)]
struct LabArgs {
    #[arg(long)]
    lab: PathBuf,
    #[arg(long)]
    profile: PathBuf,
    /// Zero or more pack manifests (optional — the basics need none).
    #[arg(long = "pack")]
    packs: Vec<PathBuf>,
    /// Lab directory on disk (durable Acts, evidence, ghosts, candidates).
    #[arg(long)]
    store: Option<PathBuf>,
}

#[derive(clap::Args, Clone)]
struct Now {
    #[arg(long, default_value = "1970-01-01T00:00:00Z")]
    now: String,
}

#[derive(Subcommand)]
enum Command {
    /// Show the canonical nine Act slots.
    Slots,
    /// Validate an Act JSON file against the nine-slot rule.
    Validate { path: PathBuf },
    /// Run the offline protocol conformance suite (or export comparable examples).
    Conformance {
        /// Print exportable examples (valid Acts + content hashes) instead of the report.
        #[arg(long, default_value_t = false)]
        export: bool,
    },
    /// Inspect a Lab's wiring (doctor).
    Doctor {
        #[command(flatten)]
        lab: LabArgs,
    },
    /// Emit Act file(s) into a Lab, then sync and report.
    Emit {
        #[command(flatten)]
        lab: LabArgs,
        #[arg(long = "act")]
        acts: Vec<PathBuf>,
    },
    /// Resident session (provider-free): presence over the Act graph.
    Session {
        #[command(subcommand)]
        cmd: SessionCmd,
    },
    /// Surface: Start — declare/open a Lab and show first next actions.
    Start {
        #[command(flatten)]
        lab: LabArgs,
    },
    /// Surface: Today — due/overdue/blocked/running/recent/ghosts/capacity.
    Today {
        #[command(flatten)]
        lab: LabArgs,
        #[command(flatten)]
        now: Now,
    },
    /// Surface: Timeline — past/present/future Acts.
    Timeline {
        #[command(flatten)]
        lab: LabArgs,
        #[command(flatten)]
        now: Now,
    },
    /// Surface: Write — capture a candidate Act (ugly capture allowed).
    Write {
        #[command(flatten)]
        lab: LabArgs,
        /// JSON file holding the candidate.
        #[arg(long)]
        json: PathBuf,
    },
    /// Surface: Schedule — show the schedule, or place an Act as a future obligation.
    Schedule {
        #[command(flatten)]
        lab: LabArgs,
        #[command(flatten)]
        now: Now,
        /// Act JSON file to schedule (optional — omit to just show the schedule).
        #[arg(long)]
        act: Option<PathBuf>,
        /// Due time for the scheduled Act (RFC3339).
        #[arg(long)]
        due: Option<String>,
    },
    /// Surface: Workbench — run a study bench and record evidence/ghost.
    Workbench {
        #[command(flatten)]
        lab: LabArgs,
        #[command(flatten)]
        now: Now,
        /// Study bench JSON file.
        #[arg(long)]
        bench: PathBuf,
        /// Whether the observation met expectation.
        #[arg(long, default_value_t = false)]
        met: bool,
        /// Observed payload JSON file (optional).
        #[arg(long)]
        observed: Option<PathBuf>,
    },
    /// Surface: Proof — claim/evidence/receipt/ghost separation for a scope.
    Proof {
        #[command(flatten)]
        lab: LabArgs,
        /// Claim Act JSON file.
        #[arg(long)]
        act: PathBuf,
        #[arg(long)]
        scope: String,
    },
    /// Surface: Tick — confront time; emit tick/disposition/reschedule Acts.
    Tick {
        #[command(flatten)]
        lab: LabArgs,
        #[command(flatten)]
        now: Now,
        /// Due time assigned to reschedule Acts for overdue work.
        #[arg(long, default_value = "2999-01-01T00:00:00Z")]
        next_due: String,
    },
    /// Show the storage/spine onboarding matrix (where admitted Acts register).
    Storage,
    /// Surface: Learn — learning report with a proposed next Act.
    Learn {
        #[command(flatten)]
        lab: LabArgs,
        #[command(flatten)]
        now: Now,
    },
    /// Settings — configuration view + provider registry (authority always locked).
    Settings {
        #[command(subcommand)]
        cmd: SettingsCmd,
    },
    /// Scan paths for false-authority / storage-as-truth language.
    Scan { paths: Vec<PathBuf> },
}

#[derive(Subcommand)]
enum SettingsCmd {
    /// Show the settings read-model (authority always locked).
    View {
        #[command(flatten)]
        lab: LabArgs,
    },
    /// Manage model provider profiles. Config only — never core semantic authority.
    Providers {
        #[command(subcommand)]
        cmd: ProvidersCmd,
    },
}

// Provider truth lives in the Lab's Acts; these commands EMIT provider-decision Acts and
// PROJECT the registry from them. There is no registry-of-record file. Each needs a Lab
// (`--lab/--profile/--store`) because provider config is part of a Lab's accountable history.
#[derive(Subcommand)]
enum ProvidersCmd {
    /// List providers (projected from the Lab's Acts) + the availability matrix.
    List {
        #[command(flatten)]
        lab: LabArgs,
    },
    /// Register a provider — emits a `register_provider` Act (available kind: openai-compatible).
    Add {
        /// Provider id (e.g. `ollama`, `openai`, `minilab`).
        id: String,
        #[arg(long, default_value = "openai-compatible")]
        kind: String,
        #[arg(long = "base-url")]
        base_url: String,
        #[arg(long)]
        model: String,
        /// Env var holding the API key/token (only the NAME is recorded; never the value).
        #[arg(long = "api-key-env")]
        api_key_env: Option<String>,
        #[arg(long, default_value_t = false)]
        dev_only: bool,
        #[command(flatten)]
        lab: LabArgs,
        #[command(flatten)]
        now: Now,
    },
    /// Disable a provider — emits a `disable_provider` Act.
    Disable {
        id: String,
        #[command(flatten)]
        lab: LabArgs,
        #[command(flatten)]
        now: Now,
    },
    /// Set the default provider — emits a `set_default_provider` Act.
    SetDefault {
        id: String,
        #[command(flatten)]
        lab: LabArgs,
        #[command(flatten)]
        now: Now,
    },
    /// Live-test a provider (one-line round-trip; needs the configured key env) and record
    /// a `test_provider` Act.
    Test {
        id: String,
        #[command(flatten)]
        lab: LabArgs,
        #[command(flatten)]
        now: Now,
    },
}

/// Resident session subcommands. All provider-free (step C). A provider attaches behind
/// the `ProviderAdapter` trait in a later step; it is never required.
#[derive(Subcommand)]
enum SessionCmd {
    /// Open (or resume) a resident session and show Start. Use `--store` for durability/resume.
    Start {
        #[command(flatten)]
        lab: LabArgs,
        #[command(flatten)]
        now: Now,
    },
    /// Read a Lab read-surface (start/today/timeline/schedule/learn/settings/storage).
    View {
        #[command(flatten)]
        lab: LabArgs,
        #[command(flatten)]
        now: Now,
        #[arg(long)]
        surface: String,
    },
    /// Capture material as a candidate (NOT admitted): `--text` or `--json`.
    Write {
        #[command(flatten)]
        lab: LabArgs,
        #[arg(long)]
        text: Option<String>,
        #[arg(long)]
        json: Option<PathBuf>,
    },
    /// Human approval: mint an authorization candidate; the Lab admits the target Act.
    Approve {
        #[command(flatten)]
        lab: LabArgs,
        #[command(flatten)]
        now: Now,
        /// Strict Act JSON file to promote.
        #[arg(long)]
        json: PathBuf,
    },
    /// Tick the Lab (confront time).
    Tick {
        #[command(flatten)]
        lab: LabArgs,
        #[command(flatten)]
        now: Now,
        #[arg(long, default_value = "2999-01-01T00:00:00Z")]
        next_due: String,
    },
    /// Show the session transcript (projection over Acts; rebuilt from the Lab).
    Transcript {
        #[command(flatten)]
        lab: LabArgs,
    },
    /// Close the session (state lives in the Lab; nothing else to persist).
    Close {
        #[command(flatten)]
        lab: LabArgs,
    },
    /// Ask a provider to suggest a candidate Act (candidate material only; not admitted).
    Suggest {
        #[command(flatten)]
        lab: LabArgs,
        #[command(flatten)]
        now: Now,
        /// Provider id (defaults to the Lab's default provider).
        #[arg(long)]
        provider: Option<String>,
        #[arg(long)]
        text: String,
    },
}

fn read(path: &PathBuf) -> Result<String> {
    std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))
}

fn load(args: &LabArgs) -> Result<Lab> {
    let lab_m = LabManifest::load(&read(&args.lab)?).map_err(|e| anyhow::anyhow!("{e}"))?;
    let profile_m = ProfileManifest::load(&read(&args.profile)?).map_err(|e| anyhow::anyhow!("{e}"))?;
    let mut packs = Vec::new();
    for p in &args.packs {
        packs.push(PackManifest::load(&read(p)?).map_err(|e| anyhow::anyhow!("{e}"))?);
    }
    let lab = match &args.store {
        Some(dir) => Lab::open(lab_m, packs, profile_m, dir),
        None => Lab::init(lab_m, packs, profile_m),
    }
    .map_err(|e| anyhow::anyhow!("{e}"))?;
    Ok(lab)
}

fn print_json<T: serde::Serialize>(v: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(v)?);
    Ok(())
}

fn resident(lab: &LabArgs) -> Result<ResidentSession> {
    // Provider-free resident session over a (preferably `--store`-backed) Lab.
    Ok(ResidentSession::open(load(lab)?, "resident", "operator"))
}

/// Project the provider registry from a Lab's admitted Acts. The Act graph is the truth;
/// this projection is rebuilt every read — there is no registry-of-record file.
fn project_registry(rs: &ResidentSession) -> logline_lab_providers::ProviderRegistry {
    let acts: Vec<Act> = rs.lab().spine().all().iter().map(|s| s.act.clone()).collect();
    logline_lab_providers::ProviderRegistry::project_from_acts(&acts)
}

/// Emit a provider-decision Act (the Lab admits it; the human running the command is the
/// `confirmed_by`). This is how provider truth enters the Lab — never a config file.
fn emit_decision(rs: &mut ResidentSession, act_value: &serde_json::Value) -> Result<()> {
    let act = Act::from_value_strict(act_value).map_err(|e| anyhow::anyhow!("{e}"))?;
    rs.lab_mut().emit(&act).map_err(|e| anyhow::anyhow!("{e}"))?;
    rs.lab_mut().sync().map_err(|e| anyhow::anyhow!("{e}"))?;
    Ok(())
}

fn run_settings(cmd: SettingsCmd) -> Result<()> {
    use logline_lab_providers::{
        availability, disable_provider_act, register_provider_act, set_default_provider_act,
        test_provider_act, suggest_blocking, AuthConfig, ProviderKind, ProviderProfile,
    };
    match cmd {
        SettingsCmd::View { lab } => print_json(&load(&lab)?.settings())?,
        SettingsCmd::Providers { cmd } => match cmd {
            ProvidersCmd::List { lab } => {
                let rs = resident(&lab)?;
                let reg = project_registry(&rs);
                print_json(&serde_json::json!({
                    "default": reg.default,
                    "providers": reg.list(),
                    "availability": availability()
                        .into_iter()
                        .map(|(k, s)| serde_json::json!({"kind": k, "status": s}))
                        .collect::<Vec<_>>(),
                    "source": "projected from the Lab's provider Acts (no registry-of-record file)",
                }))?;
            }
            ProvidersCmd::Add { id, kind, base_url, model, api_key_env, dev_only, lab, now } => {
                let kind = ProviderKind::parse(&kind)
                    .ok_or_else(|| anyhow::anyhow!("unknown provider kind `{kind}`"))?;
                if !kind.is_available() {
                    anyhow::bail!("provider kind `{}` is SOON; only openai-compatible is available", kind.as_str());
                }
                let profile = ProviderProfile {
                    id: id.clone(),
                    kind,
                    base_url,
                    model,
                    auth: AuthConfig { kind: "bearer_env".to_string(), env: api_key_env },
                    enabled: true,
                    dev_only,
                };
                let mut rs = resident(&lab)?;
                emit_decision(&mut rs, &register_provider_act("operator", &profile, &now.now))?;
                println!("register_provider `{id}` emitted as a LogLine Act");
            }
            ProvidersCmd::Disable { id, lab, now } => {
                let mut rs = resident(&lab)?;
                emit_decision(&mut rs, &disable_provider_act("operator", &id, &now.now))?;
                println!("disable_provider `{id}` emitted as a LogLine Act");
            }
            ProvidersCmd::SetDefault { id, lab, now } => {
                let mut rs = resident(&lab)?;
                emit_decision(&mut rs, &set_default_provider_act("operator", &id, "resident_session", &now.now))?;
                println!("set_default_provider `{id}` emitted as a LogLine Act");
            }
            ProvidersCmd::Test { id, lab, now } => {
                let mut rs = resident(&lab)?;
                let profile = project_registry(&rs)
                    .get_enabled(&id)
                    .ok_or_else(|| anyhow::anyhow!("no enabled provider `{id}` (register it first)"))?
                    .clone();
                let ctx = logline_lab_session::ProviderContext::default();
                let ok = match suggest_blocking(&profile, "Reply with exactly: ok", &ctx, &now.now) {
                    Ok(c) => {
                        print_json(&c)?;
                        true
                    }
                    Err(e) => {
                        eprintln!("provider `{id}` test failed: {e}");
                        false
                    }
                };
                emit_decision(&mut rs, &test_provider_act("operator", &id, ok, &now.now))?;
                if !ok {
                    std::process::exit(1);
                }
            }
        },
    }
    Ok(())
}

fn run_session(cmd: SessionCmd) -> Result<()> {
    let le = |e: logline_lab_labd::LabError| anyhow::anyhow!("{e}");
    match cmd {
        SessionCmd::Start { lab, now } => {
            let rs = resident(&lab)?;
            print_json(&rs.view("start", &now.now).map_err(le)?)?;
        }
        SessionCmd::View { lab, now, surface } => {
            let rs = resident(&lab)?;
            print_json(&rs.view(&surface, &now.now).map_err(le)?)?;
        }
        SessionCmd::Write { lab, text, json } => {
            let mut rs = resident(&lab)?;
            match (text, json) {
                (Some(t), _) => print_json(&rs.write_text(&t).map_err(le)?)?,
                (None, Some(p)) => {
                    let v: serde_json::Value = serde_json::from_str(&read(&p)?)?;
                    rs.write_candidate(v.clone()).map_err(le)?;
                    print_json(&v)?;
                }
                (None, None) => anyhow::bail!("session write needs --text or --json"),
            }
        }
        SessionCmd::Approve { lab, now, json } => {
            let mut rs = resident(&lab)?;
            let act = Act::from_json_strict(&read(&json)?).map_err(|e| anyhow::anyhow!("{e}"))?;
            print_json(&rs.approve(&act, &now.now).map_err(le)?)?;
        }
        SessionCmd::Tick { lab, now, next_due } => {
            let mut rs = resident(&lab)?;
            print_json(&rs.tick(&now.now, &next_due).map_err(le)?)?;
        }
        SessionCmd::Transcript { lab } => {
            let rs = resident(&lab)?;
            print_json(&rs.transcript())?;
        }
        SessionCmd::Close { lab } => {
            let rs = resident(&lab)?;
            let _ = rs.close();
            println!("session closed");
        }
        SessionCmd::Suggest { lab, now, provider, text } => {
            use logline_lab_providers as prov;
            let pe = |e: logline_lab_session::ProviderError| anyhow::anyhow!("{e}");
            let mut rs = resident(&lab)?;
            // Resolve the provider from the registry PROJECTED from the Lab's Acts.
            let reg = project_registry(&rs);
            let profile = match &provider {
                Some(id) => reg
                    .get_enabled(id)
                    .ok_or_else(|| anyhow::anyhow!("no enabled provider `{id}`"))?
                    .clone(),
                None => reg
                    .default_profile()
                    .ok_or_else(|| anyhow::anyhow!("no default provider; `labkit settings providers add` then `set-default`"))?
                    .clone(),
            };
            // Context: the surfaces the provider may read (same JSON a human sees).
            let today = rs.view("today", &now.now).map_err(le)?;
            let ctx = logline_lab_session::ProviderContext {
                surfaces: vec![("today".to_string(), today)],
                transcript: Vec::new(),
            };
            // The model drafts; nothing is admitted. Provenance recorded on the candidate.
            let candidate = prov::suggest_blocking(&profile, &text, &ctx, &now.now).map_err(pe)?;
            // Accountable record of the call (no secrets).
            if let Some(mp) = candidate.provenance.model.as_ref() {
                emit_decision(&mut rs, &prov::provider_call_act("operator", mp, &now.now))?;
            }
            // Capture the provider-suggested candidate (model provenance; NOT admitted).
            rs.write_candidate(candidate.candidate.clone()).map_err(le)?;
            print_json(&candidate)?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Slots => {
            for slot in logline_act::SLOTS {
                println!("{slot}");
            }
        }
        Command::Validate { path } => match Act::from_json_strict(&read(&path)?) {
            Ok(act) => println!("valid: content_hash={}", act.content_hash().unwrap()),
            Err(e) => {
                eprintln!("invalid: {e}");
                std::process::exit(1);
            }
        },
        Command::Conformance { export } => {
            let vectors = logline_lab_conformance::builtin_vectors();
            if export {
                print_json(&logline_lab_conformance::export_examples(&vectors))?;
            } else {
                let report = logline_lab_conformance::run(&vectors);
                print_json(&report)?;
                if !report.is_green() {
                    std::process::exit(1);
                }
            }
        }
        Command::Doctor { lab } => print_json(&load(&lab)?.doctor())?,
        Command::Emit { lab, acts } => {
            let mut lab = load(&lab)?;
            for act_path in &acts {
                let act = Act::from_json_strict(&read(act_path)?)
                    .map_err(|e| anyhow::anyhow!("{}: {e}", act_path.display()))?;
                let outcome = lab.emit(&act).map_err(|e| anyhow::anyhow!("{e}"))?;
                println!("emit {} -> {}", act_path.display(), outcome.content_hash());
            }
            let s = lab.sync().map_err(|e| anyhow::anyhow!("{e}"))?;
            println!("sync: ingested={} already_present={}", s.ingested, s.already_present);
            print_json(&lab.report("1970-01-01T00:00:00Z"))?;
        }
        Command::Session { cmd } => run_session(cmd)?,
        Command::Start { lab } => print_json(&load(&lab)?.start())?,
        Command::Today { lab, now } => print_json(&load(&lab)?.today(&now.now))?,
        Command::Timeline { lab, now } => print_json(&load(&lab)?.timeline(&now.now))?,
        Command::Write { lab, json } => {
            let mut lab = load(&lab)?;
            let value: serde_json::Value = serde_json::from_str(&read(&json)?)?;
            print_json(&lab.write(&value).map_err(|e| anyhow::anyhow!("{e}"))?)?;
        }
        Command::Schedule { lab, now, act, due } => {
            let mut lab = load(&lab)?;
            if let (Some(act_path), Some(due)) = (&act, &due) {
                let a = Act::from_json_strict(&read(act_path)?).map_err(|e| anyhow::anyhow!("{e}"))?;
                let scheduled = lab.schedule(&a, due).map_err(|e| anyhow::anyhow!("{e}"))?;
                lab.sync().map_err(|e| anyhow::anyhow!("{e}"))?;
                println!("scheduled {} due {}", scheduled.did.as_str().unwrap_or(""), due);
            }
            print_json(&lab.schedule_view(&now.now))?;
        }
        Command::Workbench { lab, now, bench, met, observed } => {
            let mut lab = load(&lab)?;
            let bench = StudyBench::load(&read(&bench)?).map_err(|e| anyhow::anyhow!("{e}"))?;
            let observed_value = match &observed {
                Some(p) => serde_json::from_str(&read(p)?)?,
                None => serde_json::Value::Null,
            };
            let run = lab
                .workbench(&bench, met, observed_value, "operator", &now.now)
                .map_err(|e| anyhow::anyhow!("{e}"))?;
            lab.sync().map_err(|e| anyhow::anyhow!("{e}"))?;
            print_json(&run)?;
        }
        Command::Proof { lab, act, scope } => {
            let lab = load(&lab)?;
            let a = Act::from_json_strict(&read(&act)?).map_err(|e| anyhow::anyhow!("{e}"))?;
            print_json(&lab.proof(&a, &scope))?;
        }
        Command::Tick { lab, now, next_due } => {
            let mut lab = load(&lab)?;
            let report = lab.tick(&now.now, &next_due).map_err(|e| anyhow::anyhow!("{e}"))?;
            lab.sync().map_err(|e| anyhow::anyhow!("{e}"))?;
            print_json(&report)?;
        }
        Command::Storage => print_json(&logline_lab_labd::storage_matrix())?,
        Command::Learn { lab, now } => print_json(&load(&lab)?.learn(&now.now))?,
        Command::Settings { cmd } => run_settings(cmd)?,
        Command::Scan { paths } => {
            let mut findings = 0usize;
            for p in &paths {
                if let Ok(text) = std::fs::read_to_string(p) {
                    for f in logline_lab_recovery_scanner::scan_text(&text) {
                        println!("{}:{}: [{:?}] {}", p.display(), f.line, f.category, f.pattern);
                        findings += 1;
                    }
                }
            }
            println!("scan: {findings} finding(s)");
            if findings > 0 {
                std::process::exit(1);
            }
        }
    }
    Ok(())
}
