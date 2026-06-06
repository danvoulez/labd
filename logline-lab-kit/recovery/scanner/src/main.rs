//! `recovery-scan` — scan files or directories for false-authority and
//! storage-as-truth language before promoting source material.

use std::path::Path;
use std::process::ExitCode;

use logline_lab_recovery_scanner::{scan_text, Finding};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: recovery-scan <path> [<path>...]");
        return ExitCode::from(2);
    }

    let mut total: Vec<(String, Finding)> = Vec::new();
    for arg in &args {
        scan_path(Path::new(arg), &mut total);
    }

    if total.is_empty() {
        println!("recovery-scan: clean ({} path(s))", args.len());
        ExitCode::SUCCESS
    } else {
        for (file, f) in &total {
            println!(
                "{}:{}: [{:?}] {} -> {}",
                file, f.line, f.category, f.pattern, f.excerpt
            );
        }
        println!("recovery-scan: {} finding(s)", total.len());
        ExitCode::FAILURE
    }
}

fn scan_path(path: &Path, out: &mut Vec<(String, Finding)>) {
    if path.is_dir() {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                scan_path(&entry.path(), out);
            }
        }
    } else if path.is_file() {
        let scannable = matches!(
            path.extension().and_then(|e| e.to_str()),
            Some("md" | "txt" | "rs" | "sql" | "json" | "toml" | "logline")
        );
        if !scannable {
            return;
        }
        if let Ok(text) = std::fs::read_to_string(path) {
            for f in scan_text(&text) {
                out.push((path.display().to_string(), f));
            }
        }
    }
}
