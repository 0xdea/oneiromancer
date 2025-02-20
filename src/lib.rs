//!
//! oneiromancer - Reverse engineering assistant that uses a locally running LLM to assist with code analysis.
//! Copyright (c) 2025 Marco Ivaldi <raptor@0xdeadbeef.info>
//!
//! > "A large fraction of the flaws in software development are due to programmers not fully
//! > understanding all the possible states their code may execute in."
//! >
//! > -- John Carmack
//!
//! Oneiromancer is a research engineering assistant that uses a locally running LLM that has been
//! fine-tuned for Hex-Rays pseudo-code, to aid with code analysis.
//!
//! ## Features
//! * TODO
//!
//! ## Blog post
//! * TODO
//!
//! ## See also
//! * <https://www.atredis.com/blog/2024/6/3/how-to-train-your-large-language-model>
//! * <https://huggingface.co/AverageBusinessUser/aidapal>
//! * <https://github.com/atredispartners/aidapal>
//!
//! ## Installing
//! The easiest way to get the latest release is via [crates.io](https://crates.io/crates/oneiromancer):
//! ```sh
//! TODO
//! ```
//!
//! ## Compiling
//! Alternatively, you can build from [source](https://github.com/0xdea/oneiromancer):
//! ```sh
//! TODO
//! ```
//!
//! ## Usage
//! ```sh
//! TODO
//! ```
//!
//! ## Examples
//! TODO:
//! ```sh
//! TODO
//! ```
//!
//! TODO:
//! ```sh
//! TODO
//! ```
//!
//! ## Tested on/with
//! * TODO
//!
//! ## Changelog
//! * <https://github.com/0xdea/oneiromancer/blob/master/CHANGELOG.md>
//!
//! ## Credits
//! * TODO
//!
//! ## TODO
//! * TODO
//!

#![doc(html_logo_url = "https://raw.githubusercontent.com/0xdea/oneiromancer/master/.img/logo.png")]

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use serde::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use thiserror::Error;

/// Ollama API request content
// TODO - implement better types for url, model, etc.?
#[derive(Debug, Serialize)]
struct OllamaRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
    format: &'a str,
}

/// Ollama API response content
// TODO - use a reference instead of an owned type?
#[derive(Debug, Deserialize)]
pub struct OllamaResponse {
    response: String,
}

#[derive(Debug, Error)]
pub enum OneiromancerError {
    #[error(transparent)]
    FileReadFailed(#[from] std::io::Error),
    #[error(transparent)]
    OllamaQueryFailed(#[from] ureq::Error),
}

/// TODO
pub fn run(filepath: &Path) -> anyhow::Result<()> {
    // Check target source code file
    println!("[*] Analyzing source code file {filepath:?}");
    if !filepath.is_file() {
        return Err(anyhow::anyhow!("invalid file path"));
    }

    // Analyze the target source code file
    let mut sp = Spinner::new(
        Spinners::SimpleDotsScrolling,
        "Querying the Oneiromancer".into(),
    );
    let result = analyze_this(filepath, None, None)?;
    sp.stop_with_message("[+] Done".into());
    println!("[+] Successfully analyzed source code file");

    dbg!(result);

    // TODO - parse LLM output
    // TODO - terminal output
    // TODO - file output (version? other solution?)

    Ok(())
}

/// Submit code in `filepath` to the local LLM via the Ollama API using the specified `url` and `model`.
///
/// Return an `OllamaResponse` or the appropriate `OneiromancerError` in case something goes wrong.
pub fn analyze_this(
    filepath: &Path,
    url: Option<&str>,
    model: Option<&str>,
) -> Result<OllamaResponse, OneiromancerError> {
    // Default Ollama URL and model
    const OLLAMA_URL: &str = "http://127.0.0.1:11434/api/generate";
    const OLLAMA_MODEL: &str = "aidapal";

    // Open target source code file for reading
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);
    let mut source_code = String::new();
    reader.read_to_string(&mut source_code)?;

    // Build Ollama API request
    let send_body = OllamaRequest {
        model: model.unwrap_or(OLLAMA_MODEL),
        prompt: &source_code,
        stream: false,
        format: "json",
    };

    // Send Ollama API request
    Ok(ureq::post(url.unwrap_or(OLLAMA_URL))
        .send_json(&send_body)?
        .body_mut()
        .read_json::<OllamaResponse>()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
