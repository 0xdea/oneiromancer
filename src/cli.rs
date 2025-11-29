//! Command-line argument parsing.
//!
//! This module contains the arguments definition for the command-line interface,
//! handled by [`clap`].

use crate::oneiromancer::{OLLAMA_BASEURL, OLLAMA_MODEL};
use clap::Parser;
use std::path::PathBuf;

/// Command-line arguments for the application.
#[derive(Parser, Debug)]
#[command(name = "Oneiromancer", about = "Reverse engineering assistant that uses a locally running LLM to aid with pseudocode analysis.", long_about = None, version)]
pub struct Args {
    /// Path to the file containing the pseudocode to analyze.
    pub binary: PathBuf,

    /// Base URL for the Ollama API.
    ///
    /// Defaults to the value of the `OLLAMA_BASEURL` environment variable, or
    /// a built-in default if not set.
    #[arg(short, long, env = "OLLAMA_BASEURL", default_value = OLLAMA_BASEURL)]
    pub base_url: String,

    /// Name of the LLM model to use.
    ///
    /// Defaults to the value of the `OLLAMA_MODEL` environment variable, or
    /// a built-in default if not set.
    #[arg(short, long, env = "OLLAMA_MODEL", default_value = OLLAMA_MODEL)]
    pub model: String,
}
