//! `labkit` — the generic LogLine Lab Kit command surface.
//!
//! Generic machinery only (Operator §13). A Lab needs only an identity and a
//! profile — packs are optional complements. State lives in the file-backed
//! outbox; the spine is rebuilt from it each run. The experience surfaces
//! (Start/Today/Timeline/Write/Schedule/Learn/Settings) are CLI wrappers over
//! the `labd` library grammar.

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use logline_act::Act;
use logline_lab_core::{LabManifest, PackManifest, ProfileManifest};
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
    /// File-backed outbox for durable state.
    #[arg(long)]
    outbox: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Command {
    /// Show the canonical nine Act slots.
    Slots,
    /// Validate an Act JSON file against the nine-slot rule.
    Validate { path: PathBuf },
    /// Run the offline protocol conformance suite.
    Conformance,
    /// Inspect a Lab's wiring.
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
    /// Start surface — declare/open a Lab.
    Start {
        #[command(flatten)]
        lab: LabArgs,
    },
    /// Today surface — due/overdue/blocked/running/recent/ghosts/capacity.
    Today {
        #[command(flatten)]
        lab: LabArgs,
        #[arg(long, default_value = "1970-01-01T00:00:00Z")]
        now: String,
    },
    /// Timeline surface — past/present/future Acts.
    Timeline {
        #[command(flatten)]
        lab: LabArgs,
        #[arg(long, default_value = "1970-01-01T00:00:00Z")]
        now: String,
    },
    /// Write surface — capture a candidate Act (ugly capture allowed).
    Write {
        #[command(flatten)]
        lab: LabArgs,
        /// JSON file holding the candidate.
        #[arg(long)]
        json: PathBuf,
    },
    /// Learn surface — learning report with a proposed next Act.
    Learn {
        #[command(flatten)]
        lab: LabArgs,
        #[arg(long, default_value = "1970-01-01T00:00:00Z")]
        now: String,
    },
    /// Settings surface — configuration (authority always locked).
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

fn load(args: &LabArgs, rehydrate: bool) -> Result<Lab> {
    let lab_m = LabManifest::load(&read(&args.lab)?).map_err(|e| anyhow::anyhow!("{e}"))?;
    let profile_m = ProfileManifest::load(&read(&args.profile)?).map_err(|e| anyhow::anyhow!("{e}"))?;
    let mut packs = Vec::new();
    for p in &args.packs {
        packs.push(PackManifest::load(&read(p)?).map_err(|e| anyhow::anyhow!("{e}"))?);
    }
    let mut lab = match &args.outbox {
        Some(path) => Lab::init_with_outbox(lab_m, packs, profile_m, path),
        None => Lab::init(lab_m, packs, profile_m),
    }
    .map_err(|e| anyhow::anyhow!("{e}"))?;
    if rehydrate {
        lab.rehydrate().map_err(|e| anyhow::anyhow!("{e}"))?;
    }
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
        Command::Conformance => {
            let report = logline_lab_conformance::run(&logline_lab_conformance::builtin_vectors());
            print_json(&report)?;
            if !report.is_green() {
                std::process::exit(1);
            }
        }
        Command::Doctor { lab } => {
            let lab = load(&lab, true)?;
            print_json(&lab.doctor())?;
        }
        Command::Session { lab, acts } => {
            let mut lab = load(&lab, true)?;
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
        Command::Start { lab } => print_json(&load(&lab, true)?.start())?,
        Command::Today { lab, now } => print_json(&load(&lab, true)?.today(&now))?,
        Command::Timeline { lab, now } => print_json(&load(&lab, true)?.timeline(&now))?,
        Command::Write { lab, json } => {
            let mut lab = load(&lab, true)?;
            let value: serde_json::Value = serde_json::from_str(&read(&json)?)?;
            let outcome = lab.write(&value).map_err(|e| anyhow::anyhow!("{e}"))?;
            print_json(&outcome)?;
        }
        Command::Learn { lab, now } => print_json(&load(&lab, true)?.learn(&now))?,
        Command::Settings { lab } => print_json(&load(&lab, false)?.settings())?,
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
