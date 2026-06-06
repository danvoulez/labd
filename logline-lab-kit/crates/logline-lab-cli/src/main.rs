//! `labkit` — the generic LogLine Lab Kit command surface.
//!
//! Generic machinery only: no pack-specific or Dan-specific behavior baked in
//! (Operator §13). A Lab is described by three manifests (lab/pack/profile); the
//! CLI loads them, then emits, syncs, projects, reports, and runs doctor/scan.

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use logline_act::Act;
use logline_lab_core::{LabManifest, PackManifest, ProfileManifest};
use logline_lab_labd::Lab;

#[derive(Parser)]
#[command(name = "labkit", version, about = "LogLine Lab Kit — installable Act machine")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Show the canonical nine Act slots.
    Slots,
    /// Validate an Act JSON file against the nine-slot rule.
    Validate {
        /// Path to an Act JSON file.
        path: PathBuf,
    },
    /// Run doctor on a Lab described by manifest files.
    Doctor {
        #[arg(long)]
        lab: PathBuf,
        #[arg(long)]
        pack: PathBuf,
        #[arg(long)]
        profile: PathBuf,
        /// Optional file-backed outbox.
        #[arg(long)]
        outbox: Option<PathBuf>,
    },
    /// Emit an Act (from a JSON file) into a Lab's outbox, then sync + report.
    Session {
        #[arg(long)]
        lab: PathBuf,
        #[arg(long)]
        pack: PathBuf,
        #[arg(long)]
        profile: PathBuf,
        /// One or more Act JSON files to emit this session.
        #[arg(long = "act")]
        acts: Vec<PathBuf>,
        #[arg(long)]
        outbox: Option<PathBuf>,
    },
    /// Scan paths for false-authority / storage-as-truth language.
    Scan {
        paths: Vec<PathBuf>,
    },
}

fn load_lab(lab: &PathBuf, pack: &PathBuf, profile: &PathBuf, outbox: &Option<PathBuf>) -> Result<Lab> {
    let lab_m = LabManifest::load(&read(lab)?).map_err(|e| anyhow::anyhow!("{e}"))?;
    let pack_m = PackManifest::load(&read(pack)?).map_err(|e| anyhow::anyhow!("{e}"))?;
    let profile_m = ProfileManifest::load(&read(profile)?).map_err(|e| anyhow::anyhow!("{e}"))?;
    let lab = match outbox {
        Some(path) => Lab::init_with_outbox(lab_m, pack_m, profile_m, path),
        None => Lab::init(lab_m, pack_m, profile_m),
    }
    .map_err(|e| anyhow::anyhow!("{e}"))?;
    Ok(lab)
}

fn read(path: &PathBuf) -> Result<String> {
    std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))
}

fn now() -> String {
    // No system-clock dependency in core; the CLI stamps a coarse marker.
    "1970-01-01T00:00:00Z".to_string()
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Slots => {
            for slot in logline_act::SLOTS {
                println!("{slot}");
            }
        }
        Command::Validate { path } => {
            let text = read(&path)?;
            match Act::from_json_strict(&text) {
                Ok(act) => {
                    println!("valid: content_hash={}", act.content_hash().unwrap());
                }
                Err(e) => {
                    eprintln!("invalid: {e}");
                    std::process::exit(1);
                }
            }
        }
        Command::Doctor { lab, pack, profile, outbox } => {
            let lab = load_lab(&lab, &pack, &profile, &outbox)?;
            let report = lab.doctor();
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::Session { lab, pack, profile, acts, outbox } => {
            let mut lab = load_lab(&lab, &pack, &profile, &outbox)?;
            for act_path in &acts {
                let act = Act::from_json_strict(&read(act_path)?)
                    .map_err(|e| anyhow::anyhow!("{}: {e}", act_path.display()))?;
                let outcome = lab.emit(&act).map_err(|e| anyhow::anyhow!("{e}"))?;
                println!("emit {} -> {}", act_path.display(), outcome.content_hash());
            }
            let sync = lab.sync().map_err(|e| anyhow::anyhow!("{e}"))?;
            println!("sync: ingested={} already_present={}", sync.ingested, sync.already_present);
            let report = lab.report(&now());
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
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
