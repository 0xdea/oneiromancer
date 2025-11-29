//! main.rs

use clap::Parser;
use oneiromancer::cli;
use std::{env, process};

const PROGRAM: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("{PROGRAM} {VERSION} - GenAI tool for pseudocode analysis");
    println!("Copyright (c) 2025 Marco Ivaldi <raptor@0xdeadbeef.info>");
    println!();

    // Parse command line arguments
    let args: cli::Args = cli::Args::parse();

    if !args.binary.exists() {
        eprintln!("\n[!] Error: the specified file does not exist");
        process::exit(-1);
    }

    // Let's do it
    match oneiromancer::run(&args.binary) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("\n[!] Error: {err:#}");
            process::exit(1);
        }
    }
}
