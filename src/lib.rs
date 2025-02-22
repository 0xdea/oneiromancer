//!
//! oneiromancer - GenAI assistant for C code analysis
//! Copyright (c) 2025 Marco Ivaldi <raptor@0xdeadbeef.info>
//!
//! > "A large fraction of the flaws in software development are due to programmers not fully
//! > understanding all the possible states their code may execute in." -- John Carmack
//!
//! > "Can it run Doom?" -- https://www.canitrundoom.org/
//!
//! Oneiromancer is a reverse engineering assistant that uses a locally running LLM that has been
//! fine-tuned for Hex-Rays pseudo-code, to aid with code analysis. It can analyze a function or a
//! smaller code snippet, returning a high-level description of what the code does, a recommended
//! name for the function, and variable renaming suggestions, based on the results of the analysis.
//!
//! ## Features
//! * Support for the fine-tuned LLM [aidapal](https://huggingface.co/AverageBusinessUser/aidapal) based on `mistral-7b-instruct`.
//! * Easy integration with the pseudo-code extractor [haruspex](https://github.com/0xdea/haruspex) and popular IDEs.
//! * Code description, recommended function name, and variable renaming suggestions are printed to the terminal.
//! * Improved pseudo-code of each analyzed function is saved in a separated file for easy inspection.
//! * External crates can invoke `analyze_code` or `analyze_file` to analyze pseudo-code and then process analysis results.
//!
//! ## Blog post
//! * <https://security.humanativaspa.it/aiding-reverse-engineering-with-rust-and-a-local-llm> (*coming soon*)
//!
//! ## See also
//! * <https://www.atredis.com/blog/2024/6/3/how-to-train-your-large-language-model>
//! * <https://huggingface.co/AverageBusinessUser/aidapal>
//! * <https://github.com/atredispartners/aidapal>
//!
//! ## Installing
//! The easiest way to get the latest release is via [crates.io](https://crates.io/crates/oneiromancer):
//! ```sh
//! $ cargo install oneiromancer
//! ```
//!
//! To install as a library, run the following command in your project directory:
//! ```sh
//! $ cargo add oneiromancer
//! ```
//!
//! ## Compiling
//! Alternatively, you can build from [source](https://github.com/0xdea/oneiromancer):
//! ```sh
//! $ git clone https://github.com/0xdea/oneiromancer
//! $ cd oneiromancer
//! $ cargo build --release
//! ```
//!
//! ## Configuration
//! 1. Download and install [ollama](https://ollama.com/).
//! 2. Download the fine-tuned weights and Ollama modelfile from [huggingface](https://huggingface.co/):
//!     ```sh
//!     $ wget https://huggingface.co/AverageBusinessUser/aidapal/resolve/main/aidapal-8k.Q4_K_M.gguf
//!     $ wget https://huggingface.co/AverageBusinessUser/aidapal/resolve/main/aidapal.modelfile
//!     ```
//! 3. Configure Ollama by running the following commands within the directory in which you downloaded the files:
//!     ```sh
//!     $ ollama create aidapal -f aidapal.modelfile
//!     $ ollama list
//!     ```
//!
//! ## Usage
//! 1. Run oneiromancer as follows:
//!     ```sh
//!     $ oneiromancer <source_file>.c
//!     ```
//! 2. Find the extracted pseudo-code of each decompiled function in `source_file.out.c`:
//!     ```sh
//!     $ vim <source_file>.out.c
//!     $ code <source_file>.out.c
//!     ```
//! *Note: for best results, you shouldn't submit for analysis to the LLM more than one function at a time.*
//!
//! ## Tested on
//! * Apple macOS Sequoia 15.2 with ollama 0.5.11
//!
//! ## Changelog
//! * <https://github.com/0xdea/oneiromancer/blob/master/CHANGELOG.md>
//!
//! ## Credits
//! * Chris (@AverageBusinessUser) at Atredis Partners for his fine-tuned LLM `aidapal` <3
//!
//! ## TODO
//! * Improve output file handling with versioning and/or an output directory.
//! * Extensive testing on the `windows` target family to confirm that it works properly even in edge cases.
//! * Implement other features of the IDAPython `aidapal` IDA Pro plugin (e.g., context).
//! * Implement a "minority report" protocol (i.e., make three queries and select the best responses).
//! * Integrate with [haruspex](https://github.com/0xdea/haruspex) and [idalib](https://github.com/binarly-io/idalib).
//! * Investigate other use cases for the `aidapal` LLM and implement a modular LLM architecture to plug in custom LLMs.
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
#[derive(Serialize, Debug, Clone)]
struct OllamaRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
    format: &'a str,
}

impl<'a> OllamaRequest<'a> {
    /// Create a new `OllamaRequest`
    const fn new(model: &'a str, prompt: &'a str) -> Self {
        Self {
            model,
            prompt,
            stream: false,
            format: "json",
        }
    }

    /// Send an `OllamaRequest`.
    ///
    /// Return an `OllamaResponse` or the appropriate `OneiromancerError` in case something goes wrong.
    fn send(&self, url: &str) -> Result<OllamaResponse, OneiromancerError> {
        Ok(ureq::post(url)
            .send_json(self)?
            .body_mut()
            .read_json::<OllamaResponse>()?)
    }
}

/// Ollama API response content
#[derive(Deserialize, Debug, Clone)]
pub struct OllamaResponse {
    pub response: String,
}

/// Code analysis results
#[derive(Deserialize, Debug, Clone)]
pub struct AnalysisResults {
    /// Recommended function name
    pub function_name: String,
    /// Function description
    pub comment: String,
    /// Variable renaming suggestions
    pub variables: Vec<Variable>,
}

/// Variable renaming suggestion
#[derive(Deserialize, Debug, Clone)]
pub struct Variable {
    /// Original name of the variable
    pub original_name: String,
    /// Suggested name for the variable
    pub new_name: String,
}

#[derive(Error, Debug)]
pub enum OneiromancerError {
    #[error(transparent)]
    FileReadFailed(#[from] std::io::Error),
    #[error(transparent)]
    OllamaQueryFailed(#[from] ureq::Error),
}

/// Submit code in `filepath` file to local LLM for analysis. Output analysis results to terminal
/// and save improved pseudo-code in `filepath` with a modified `out.c` extension.
///
/// Return success or an error in case something goes wrong.
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
    let function_description = format!(
        "/*\n * {}()\n *\n{}\n */\n\n",
        &analysis_results.function_name,
        textwrap::fill(&analysis_results.comment, &options)
    );
    print!("{function_description}");

    // Apply variable renaming suggestions
    println!("[-] Variable renaming suggestions:");
    for variable in &analysis_results.variables {
        println!("    {}\t-> {}", variable.original_name, variable.new_name);
        let re = Regex::new(&format!(r"\b{}\b", variable.original_name))?;
        source_code = re
            .replace_all(&source_code, variable.new_name.as_str())
            .into();
    }

    // Save improved source code to output file
    let outfilepath = filepath.with_extension("out.c");
    println!();
    println!("[*] Saving improved source code in {outfilepath:?}");

    let mut writer = BufWriter::new(File::create_new(&outfilepath)?);
    writer.write_all(function_description.as_bytes())?;
    writer.write_all(source_code.as_bytes())?;
    writer.flush()?;

    println!("[+] Done analyzing source code");
    Ok(())
}

/// Submit code in `filepath` file to the local LLM via the Ollama API using the specified `url` and `model`.
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
    analyze_code(&source_code, url, model)
}

/// Submit `source_code` to the local LLM via the Ollama API using the specified `url` and `model`.
///
/// Return an `OllamaResponse` or the appropriate `OneiromancerError` in case something goes wrong.
pub fn analyze_code(
    source_code: &str,
    url: Option<&str>,
    model: Option<&str>,
) -> Result<OllamaResponse, OneiromancerError> {
    // Send Ollama API request
    let request = OllamaRequest::new(model.unwrap_or(OLLAMA_MODEL), source_code);
    request.send(url.unwrap_or(OLLAMA_URL))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ollama_request_works() {
        let url = OLLAMA_URL;
        let model = OLLAMA_MODEL;
        let source_code = r#"int main() { printf("Hello, world!"); }"#;

        let request = OllamaRequest::new(model, source_code);
        let result = request.send(url);

        assert!(result.is_ok());
        assert!(!result.unwrap().response.is_empty());
    }

    #[test]
    fn ollama_request_with_wrong_url_fails() {
        let url = "http://127.0.0.1:6666";
        let model = OLLAMA_MODEL;
        let source_code = r#"int main() { printf("Hello, world!"); }"#;

        let request = OllamaRequest::new(model, source_code);
        let result = request.send(url);

        assert!(result.is_err());
    }

    #[test]
    fn ollama_request_with_wrong_model_fails() {
        let url = OLLAMA_URL;
        let model = "doesntexist";
        let source_code = r#"int main() { printf("Hello, world!"); }"#;

        let request = OllamaRequest::new(model, source_code);
        let result = request.send(url);

        assert!(result.is_err());
    }

    #[test]
    fn ollama_request_with_empty_prompt_returns_an_empty_response() {
        let url = OLLAMA_URL;
        let model = OLLAMA_MODEL;
        let source_code = "";

        let request = OllamaRequest::new(model, source_code);
        let result = request.send(url);

        assert!(result.is_ok());
        assert!(result.unwrap().response.is_empty());
    }

    #[test]
    fn analyze_code_works() {
        let url = OLLAMA_URL;
        let model = OLLAMA_MODEL;
        let source_code = r#"int main() { printf("Hello, world!"); }"#;

        let result = analyze_code(source_code, Some(url), Some(model));

        assert!(result.is_ok());
        assert!(!result.unwrap().response.is_empty());
    }

    #[test]
    fn analyze_code_with_default_parameters_works() {
        let source_code = r#"int main() { printf("Hello, world!"); }"#;

        let result = analyze_code(source_code, None, None);

        assert!(result.is_ok());
        assert!(!result.unwrap().response.is_empty());
    }

    #[test]
    fn analyze_file_works() {
        let url = OLLAMA_URL;
        let model = OLLAMA_MODEL;
        let source_code = r#"int main() { printf("Hello, world!"); }"#;

        let tmpdir = tempfile::tempdir().unwrap();
        let filepath = tmpdir.path().join("test.c");
        let mut tmpfile = File::create(&filepath).unwrap();
        writeln!(tmpfile, "{source_code}").unwrap();

        let result = analyze_file(&filepath, Some(url), Some(model));

        assert!(result.is_ok());
        assert!(!result.unwrap().response.is_empty());
    }

    #[test]
    fn analyze_file_with_default_parameters_works() {
        let source_code = r#"int main() { printf("Hello, world!"); }"#;

        let tmpdir = tempfile::tempdir().unwrap();
        let filepath = tmpdir.path().join("test.c");
        let mut tmpfile = File::create(&filepath).unwrap();
        writeln!(tmpfile, "{source_code}").unwrap();

        let result = analyze_file(&filepath, None, None);

        assert!(result.is_ok());
        assert!(!result.unwrap().response.is_empty());
    }

    #[test]
    fn run_works() {
        let source_code = r#"int main() { printf("Hello, world!"); }"#;

        let tmpdir = tempfile::tempdir().unwrap();
        let filepath = tmpdir.path().join("test.c");
        let mut tmpfile = File::create(&filepath).unwrap();
        writeln!(tmpfile, "{source_code}").unwrap();

        let result = run(&filepath);
        let outfile = tmpdir.path().join("test.out.c");

        assert!(result.is_ok());
        assert!(outfile.exists());
        assert_ne!(outfile.metadata().unwrap().len(), 0);
    }
}
