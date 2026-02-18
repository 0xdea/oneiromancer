//! main.rs

use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::process::ExitCode;

/// Package name
const PROGRAM: &str = env!("CARGO_PKG_NAME");
/// Package version
const VERSION: &str = env!("CARGO_PKG_VERSION");
/// Package authors
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() -> ExitCode {
    eprintln!("{PROGRAM} {VERSION} - GenAI assistant for pseudocode analysis");
    eprintln!("Copyright (c) 2025-2026 {AUTHORS}");
    eprintln!();

    // Parse command line arguments
    let mut args = env::args_os();
    let argv0 = args.next().unwrap_or_else(|| PROGRAM.into());
    let is_help = |a: &OsStr| a == OsStr::new("-h") || a == OsStr::new("--help");

    let prog = Path::new(&argv0)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(PROGRAM);

    let filename = match (args.next(), args.next()) {
        (Some(arg), None) if !is_help(&arg) => arg,
        _ => return usage(prog),
    };

    // Let's do it
    match oneiromancer::run(Path::new(&filename)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("\n[!] Error: {err:#}");
            ExitCode::FAILURE
        }
    }
}

/// Print usage information and exit
fn usage(prog: &str) -> ExitCode {
    eprintln!("Usage:");
    eprintln!("{prog} <target_file>.c");

    ExitCode::FAILURE
}
