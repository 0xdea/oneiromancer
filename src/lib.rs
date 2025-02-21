//!
//! oneiromancer - Reverse engineering assistant that uses a locally running LLM to aid with code analysis.
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
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use regex::Regex;
use serde::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use thiserror::Error;

/// Default Ollama URL
pub const OLLAMA_URL: &str = "http://127.0.0.1:11434/api/generate";
/// Default Ollama model
pub const OLLAMA_MODEL: &str = "aidapal";

/// Ollama API request content
#[derive(Serialize, Debug)]
struct OllamaRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
    format: &'a str,
}

impl<'a> OllamaRequest<'a> {
    const fn new(model: &'a str, prompt: &'a str) -> Self {
        Self {
            model,
            prompt,
            stream: false,
            format: "json",
        }
    }
}

/// Ollama API response content
#[derive(Deserialize, Debug)]
pub struct OllamaResponse {
    pub response: String,
}

/// Code analysis results
// TODO - What happens if we have less or more than one function?
#[derive(Deserialize, Debug)]
pub struct AnalysisResults {
    /// Suggested function name
    pub function_name: String,
    /// Function description
    pub comment: String,
    /// Variable renaming suggestions
    pub variables: Vec<Variable>,
}

/// Variable renaming suggestion
#[derive(Deserialize, Debug)]
pub struct Variable {
    /// Original name of the variable
    pub original_name: String,
    /// Suggested new name of the variable
    pub new_name: String,
}

#[derive(Error, Debug)]
pub enum OneiromancerError {
    #[error(transparent)]
    FileReadFailed(#[from] std::io::Error),
    #[error(transparent)]
    OllamaQueryFailed(#[from] ureq::Error),
}

/// TODO
pub fn run(filepath: &Path) -> anyhow::Result<()> {
    // Open target source code file for reading
    println!("[*] Analyzing source code in {filepath:?}");
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);
    let mut source_code = String::new();
    reader.read_to_string(&mut source_code)?;

    // Submit source code to local LLM for analysis
    let mut sp = Spinner::new(
        Spinners::SimpleDotsScrolling,
        "Querying the Oneiromancer".into(),
    );
    let ollama_response = analyze_code(&source_code, None, None)?;

    // Parse Ollama response
    let analysis_results: AnalysisResults = serde_json::from_str(&ollama_response.response)?;
    sp.stop_with_message("[+] Successfully analyzed source code".into());
    println!();

    // Create function description in Phrack-style, wrapping to 76 columns
    let options = textwrap::Options::new(76)
        .initial_indent(" * ")
        .subsequent_indent(" * ");
    let comment = format!(
        "/*\n * {}()\n *\n{}\n */\n\n",
        &analysis_results.function_name,
        textwrap::fill(&analysis_results.comment, &options)
    );
    print!("{comment}");

    // Apply variable renaming suggestions
    println!("[-] Variable renaming suggestions:");
    for variable in &analysis_results.variables {
        println!("    {}\t-> {}", variable.original_name, variable.new_name);
        let re = Regex::new(&format!(r"\b{}\b", variable.original_name))?;
        source_code = re
            .replace_all(&source_code, variable.new_name.as_str())
            .into();
    }

    // Write modified source code to output file
    // TODO - add version number for better scalability?
    let outfilepath = filepath.with_extension("out.c");
    println!();
    println!("[*] Applying suggestions into {outfilepath:?}...");

    let mut writer = BufWriter::new(File::create_new(&outfilepath)?);
    writer.write_all(comment.as_bytes())?;
    writer.write_all(source_code.as_bytes())?;
    writer.flush()?;

    println!("[+] Done analyzing source code");

    Ok(())
}

/// Submit code in `filepath` to the local LLM via the Ollama API using the specified `url` and `model`.
///
/// Return an `OllamaResponse` or the appropriate `OneiromancerError` in case something goes wrong.
pub fn analyze_file(
    filepath: &Path,
    url: Option<&str>,
    model: Option<&str>,
) -> Result<OllamaResponse, OneiromancerError> {
    // Open target source code file for reading
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);
    let mut source_code = String::new();
    reader.read_to_string(&mut source_code)?;

    // Analyze `source_code`
    analyze_code(&source_code, model, url)
}

/// Submit `source_code` to the local LLM via the Ollama API using the specified `url` and `model`.
///
/// Return an `OllamaResponse` or the appropriate `OneiromancerError` in case something goes wrong.
pub fn analyze_code(
    source_code: &str,
    url: Option<&str>,
    model: Option<&str>,
) -> Result<OllamaResponse, OneiromancerError> {
    // Build Ollama API request
    let send_body = OllamaRequest::new(model.unwrap_or(OLLAMA_MODEL), source_code);

    // Send Ollama API request
    query_ollama(url.unwrap_or(OLLAMA_URL), &send_body)
}

/// Send an `OllamaRequest`.
///
/// Return an `OllamaResponse` or the appropriate `OneiromancerError` in case something goes wrong.
fn query_ollama(url: &str, send_body: &OllamaRequest) -> Result<OllamaResponse, OneiromancerError> {
    Ok(ureq::post(url)
        .send_json(send_body)?
        .body_mut()
        .read_json::<OllamaResponse>()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_ollama_works() {
        let url = OLLAMA_URL;
        let model = OLLAMA_MODEL;
        let source_code = r#"int main() { printf("Hello, world!"); }"#;

        let send_body = OllamaRequest::new(model, source_code);
        let result = query_ollama(url, &send_body);

        assert!(result.is_ok());
        assert!(!result.unwrap().response.is_empty());
    }

    #[test]
    fn query_ollama_with_wrong_url_fails() {
        let url = "http://127.0.0.1:6666";
        let model = OLLAMA_MODEL;
        let source_code = r#"int main() { printf("Hello, world!"); }"#;

        let send_body = OllamaRequest::new(model, source_code);
        let result = query_ollama(url, &send_body);

        assert!(result.is_err());
    }

    #[test]
    fn query_ollama_with_wrong_model_fails() {
        let url = OLLAMA_URL;
        let model = "doesntexist";
        let source_code = r#"int main() { printf("Hello, world!"); }"#;

        let send_body = OllamaRequest::new(model, source_code);
        let result = query_ollama(url, &send_body);

        assert!(result.is_err());
    }

    #[test]
    fn query_ollama_with_empty_prompt_returns_an_empty_response() {
        let url = OLLAMA_URL;
        let model = OLLAMA_MODEL;
        let source_code = "";

        let send_body = OllamaRequest::new(model, source_code);
        let result = query_ollama(url, &send_body);

        assert!(result.is_ok());
        assert!(result.unwrap().response.is_empty());
    }

    // TODO - add other tests (e.g. analyze_code, analyze_file, run, file i/o, see other tools)
}
