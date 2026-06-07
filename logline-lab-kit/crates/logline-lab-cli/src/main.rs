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
use logline_lab_labd::Lab;

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
    Session {
        #[command(flatten)]
        lab: LabArgs,
        #[arg(long = "act")]
        acts: Vec<PathBuf>,
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
    /// Surface: Settings — configuration (authority always locked).
    Settings {
        #[command(flatten)]
        lab: LabArgs,
    },
    /// Scan paths for false-authority / storage-as-truth language.
    Scan { paths: Vec<PathBuf> },
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
        Command::Session { lab, acts } => {
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
        Command::Settings { lab } => print_json(&load(&lab)?.settings())?,
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
