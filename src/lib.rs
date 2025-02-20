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
use thiserror::Error;

#[derive(Serialize, Debug)]
pub struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    format: String,
}

#[derive(Deserialize, Debug)]
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
    // Open target source code file - TODO not needed? Open error handling should be enough
    println!("[*] Analyzing source code file {filepath:?}");

    // TODO - spinners, see jiggy

    // TODO - better handling of default parameters?
    let result = analyze_code(filepath, "", "")?;

    dbg!(result);

    /*
    if !filepath.is_file() {
        return Err(anyhow::anyhow!("invalid file path"));
    }
    // TODO - file open logic (to be checked also on windows)
    println!("[+] Successfully opened source code file");
    println!();
    */

    // TODO - parse LLM output
    // TODO - terminal output
    // TODO - file output (version? other solution?)

    Ok(())
}

// TODO - pub library fn analyze_code
// TODO - implement better types for url, model, etc.
pub fn analyze_code(
    filepath: &Path,
    url: &str,
    model: &str,
) -> Result<OllamaResponse, OneiromancerError> {
    // Default ollama URL and model
    const URL: &str = "http://127.0.0.1:11434/api/generate";
    const MODEL: &str = "aidapal";

    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    let url = if url.is_empty() { URL } else { url };
    let model = if model.is_empty() { MODEL } else { model };

    // TODO add new() and maybe other methods to my type
    let send_body = OllamaRequest {
        model: model.into(),
        prompt: buffer,
        stream: false,
        format: "json".into(),
    };

    Ok(ureq::post(url)
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
