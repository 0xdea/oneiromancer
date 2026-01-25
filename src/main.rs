//! main.rs

use std::path::Path;
use std::{env, process};

const PROGRAM: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() {
    println!("{PROGRAM} {VERSION} - GenAI assistant for pseudocode analysis");
    println!("Copyright (c) 2025-2026 {AUTHORS}");
    println!();

    // Parse command line arguments
    let args = env::args().collect::<Vec<_>>();

    let prog = Path::new(&args[0])
        .file_name()
        .unwrap()
        .to_str()
        .unwrap_or(PROGRAM);

    let filename = match args.len() {
        2 => &args[1],
        _ => usage(prog),
    };
    if filename.starts_with('-') {
        usage(prog);
    }

    // Let's do it
    match oneiromancer::run(Path::new(filename)) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("\n[!] Error: {err:#}");
            process::exit(1);
        }
    }
}

/// Print usage information and exit
fn usage(prog: &str) -> ! {
    println!("Usage:");
    println!("{prog} <target_file>.c");

    process::exit(0);
}
