//!
//! oneiromancer - GenAI tool for pseudo-code analysis
//! Copyright (c) 2025 Marco Ivaldi <raptor@0xdeadbeef.info>
//!
//! > "A large fraction of the flaws in software development are due to programmers not fully
//! > understanding all the possible states their code may execute in." -- John Carmack
//!
//! > "Can it run Doom?" -- <https://canitrundoom.org/>
//!
//! Oneiromancer is a reverse engineering assistant that uses a locally running LLM that has been
//! fine-tuned for Hex-Rays pseudo-code to aid with code analysis. It can analyze a function or a
//! smaller code snippet, returning a high-level description of what the code does, a recommended
//! name for the function, and variable renaming suggestions, based on the results of the analysis.
//!
//! ## Features
//! * Cross-platform support for the fine-tuned LLM [aidapal](https://huggingface.co/AverageBusinessUser/aidapal) based on `mistral-7b-instruct`.
//! * Easy integration with the pseudo-code extractor [haruspex](https://github.com/0xdea/haruspex) and popular IDEs.
//! * Code description, recommended function name, and variable renaming suggestions are printed on the terminal.
//! * Improved pseudo-code of each analyzed function is saved in a separate file for easy inspection.
//! * External crates can invoke [`analyze_code`] or [`analyze_file`] to analyze pseudo-code and then process analysis results.
//!
//! ## Blog post
//! * <https://security.humanativaspa.it/aiding-reverse-engineering-with-rust-and-a-local-llm>
//!
//! ## See also
//! * <https://www.atredis.com/blog/2024/6/3/how-to-train-your-large-language-model>
//! * <https://huggingface.co/AverageBusinessUser/aidapal>
//! * <https://github.com/atredispartners/aidapal>
//! * <https://plugins.hex-rays.com/atredispartners/aidapal>
//!
//! ## Installing
//! The easiest way to get the latest release is via [crates.io](https://crates.io/crates/oneiromancer):
//! ```sh
//! cargo install oneiromancer
//! ```
//!
//! To install as a library, run the following command in your project directory:
//! ```sh
//! cargo add oneiromancer
//! ```
//!
//! ## Compiling
//! Alternatively, you can build from [source](https://github.com/0xdea/oneiromancer):
//! ```sh
//! git clone https://github.com/0xdea/oneiromancer
//! cd oneiromancer
//! cargo build --release
//! ```
//!
//! ## Configuration
//! 1. Download and install [Ollama](https://ollama.com/).
//! 2. Download the fine-tuned weights and the Ollama modelfile from [Hugging Face](https://huggingface.co/):
//!     ```sh
//!     wget https://huggingface.co/AverageBusinessUser/aidapal/resolve/main/aidapal-8k.Q4_K_M.gguf
//!     wget https://huggingface.co/AverageBusinessUser/aidapal/resolve/main/aidapal.modelfile
//!     ```
//! 3. Configure Ollama by running the following commands within the directory in which you downloaded the files:
//!     ```sh
//!     ollama create aidapal -f aidapal.modelfile
//!     ollama list
//!     ```
//!
//! ## Usage
//! 1. Run oneiromancer as follows:
//!     ```sh
//!     export OLLAMA_BASEURL=custom_baseurl # if not set, the default will be used
//!     export OLLAMA_MODEL=custom_model # if not set, the default will be used
//!     oneiromancer <target_file>.c
//!     ```
//! 2. Find the extracted pseudo-code of each decompiled function in `<target_file>.out.c`:
//!     ```sh
//!     vim <target_file>.out.c
//!     code <target_file>.out.c
//!     ```
//! *Note: for best results, you shouldn't submit for analysis to the LLM more than one function at a time.*
//!
//! ## Tested on
//! * Apple macOS Sequoia 15.2 with Ollama 0.5.11
//! * Ubuntu Linux 24.04.2 LTS with Ollama 0.5.11
//! * Microsoft Windows 11 23H2 with Ollama 0.5.11
//!
//! ## Changelog
//! * <https://github.com/0xdea/oneiromancer/blob/master/CHANGELOG.md>
//!
//! ## Credits
//! * Chris Bellows (@AverageBusinessUser) at Atredis Partners for his fine-tuned LLM `aidapal` <3
//!
//! ## TODO
//! * Improve output file handling with versioning and/or an output directory.
//! * Implement other features of the IDAPython `aidapal` IDA Pro plugin (e.g., context).
//! * Integrate with [haruspex](https://github.com/0xdea/haruspex) and [idalib](https://github.com/binarly-io/idalib).
//! * Use custom types in the public API and implement a provider abstraction.
//! * Implement a "minority report" protocol (i.e., make three queries and select the best responses).
//! * Investigate other use cases for the `aidapal` LLM and implement a modular architecture to plug in custom LLMs.
//!

#![doc(html_logo_url = "https://raw.githubusercontent.com/0xdea/oneiromancer/master/.img/logo.png")]

use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use anyhow::Context;
use regex::Regex;
use spinners::{Spinner, Spinners};

use crate::ollama::OllamaRequest;
pub use crate::oneiromancer::{
    OneiromancerConfig, OneiromancerError, OneiromancerResults, Variable,
};

mod ollama;
mod oneiromancer;

/// Submit pseudo-code in `filepath` file to local LLM for analysis. Output analysis results to
/// terminal and save improved pseudo-code in `filepath` with an `out.c` extension.
///
/// ## Errors
///
/// Returns success or a generic error in case something goes wrong.
pub fn run(filepath: &Path) -> anyhow::Result<()> {
    // Open the target pseudo-code file for reading
    println!("[*] Analyzing pseudo-code in `{}`", filepath.display());
    let file =
        File::open(filepath).with_context(|| format!("Failed to open `{}`", filepath.display()))?;
    let mut reader = BufReader::new(file);
    let mut pseudo_code = String::new();
    reader
        .read_to_string(&mut pseudo_code)
        .with_context(|| format!("Failed to read from `{}`", filepath.display()))?;

    // Submit pseudo-code to the local LLM for analysis
    let mut sp = Spinner::new(
        Spinners::SimpleDotsScrolling,
        "Querying the Oneiromancer".into(),
    );
    let analysis_results = analyze_code(&pseudo_code, &OneiromancerConfig::default())
        .context("Failed to analyze pseudo-code")?;
    sp.stop_with_message("[+] Successfully analyzed pseudo-code".into());
    println!();

    // Create a function description in Phrack-style, wrapping to 76 columns
    let options = textwrap::Options::new(76)
        .initial_indent(" * ")
        .subsequent_indent(" * ");
    let function_description = format!(
        "/*\n * {}()\n *\n{}\n */\n\n",
        analysis_results.function_name(),
        textwrap::fill(analysis_results.comment(), &options)
    );
    print!("{function_description}");

    // Apply variable renaming suggestions
    println!("[-] Variable renaming suggestions:");
    for variable in analysis_results.variables() {
        let original_name = variable.original_name();
        let new_name = variable.new_name();
        println!("    {original_name}\t-> {new_name}");

        let re = Regex::new(&format!(r"\b{original_name}\b")).context("Failed to compile regex")?;
        pseudo_code = re.replace_all(&pseudo_code, new_name).into();
    }

    // Save improved pseudo-code to an output file
    let outfilepath = filepath.with_extension("out.c");
    println!();
    println!(
        "[*] Saving improved pseudo-code in `{}`",
        outfilepath.display()
    );

    let mut writer = BufWriter::new(
        File::create_new(&outfilepath)
            .with_context(|| format!("Failed to create `{}`", outfilepath.display()))?,
    );
    writer.write_all(function_description.as_bytes())?;
    writer.write_all(pseudo_code.as_bytes())?;
    writer.flush()?;

    println!("[+] Done analyzing pseudo-code");
    Ok(())
}

/// Submit `pseudo_code` to the local LLM via the Ollama API using the specified
/// [`OneiromancerConfig`] (or [`OneiromancerConfig::default()`] to use default values).
///
/// ## Errors
///
/// Returns [`OneiromancerResults`] or the appropriate [`OneiromancerError`] in case something goes wrong.
///
/// ## Examples
///
/// Basic usage (default Ollama base URL and model):
/// ```
/// # fn main() -> anyhow::Result<()> {
/// use oneiromancer::{OneiromancerConfig, analyze_code};
///
/// let pseudo_code = r#"int main() { printf("Hello, world!"); }"#;
///
/// let results = analyze_code(&pseudo_code, &OneiromancerConfig::default())?;
///
/// dbg!(results.function_name());
/// dbg!(results.comment());
/// dbg!(results.variables());
/// # Ok(())
/// # }
/// ```
///
/// Advanced usage (explicit Ollama base URL and model):
/// ```
/// # fn main() -> anyhow::Result<()> {
/// use oneiromancer::{OneiromancerConfig, analyze_code};
///
/// let pseudo_code = r#"int main() { printf("Hello, world!"); }"#;
///
/// let config = OneiromancerConfig::new()
///     .with_baseurl("http://127.0.0.1:11434")
///     .with_model("aidapal");
/// let results = analyze_code(&pseudo_code, &config)?;
///
/// dbg!(results.function_name());
/// dbg!(results.comment());
/// dbg!(results.variables());
/// # Ok(())
/// # }
/// ```
///
pub fn analyze_code(
    pseudo_code: impl AsRef<str>,
    config: &OneiromancerConfig,
) -> Result<OneiromancerResults, OneiromancerError> {
    // Send Ollama API request and parse response
    let request = OllamaRequest::new(config.model(), pseudo_code.as_ref());
    request.send(config.baseurl())?.parse()
}

/// Submit pseudo-code in the `filepath` file to the local LLM via the Ollama API using the specified
/// [`OneiromancerConfig`] (or [`OneiromancerConfig::default()`] to use default values).
///
/// ## Errors
///
/// Returns [`OneiromancerResults`] or the appropriate [`OneiromancerError`] in case something goes wrong.
///
/// ## Examples
///
/// Basic usage (default Ollama base URL and model):
/// ```
/// # fn main() -> anyhow::Result<()> {
/// use oneiromancer::{OneiromancerConfig, analyze_file};
///
/// let filepath = "./tests/data/hello.c";
///
/// let results = analyze_file(&filepath, &OneiromancerConfig::default())?;
///
/// dbg!(results.function_name());
/// dbg!(results.comment());
/// dbg!(results.variables());
/// # Ok(())
/// # }
/// ```
///
/// Advanced usage (explicit Ollama base URL and model):
/// ```
/// # fn main() -> anyhow::Result<()> {
/// use oneiromancer::{OneiromancerConfig, analyze_file};
///
/// let filepath = "./tests/data/hello.c";
///
/// let config = OneiromancerConfig::new()
///     .with_baseurl("http://127.0.0.1:11434")
///     .with_model("aidapal");
/// let results = analyze_file(&filepath, &config)?;
///
/// dbg!(results.function_name());
/// dbg!(results.comment());
/// dbg!(results.variables());
/// # Ok(())
/// # }
/// ```
///
pub fn analyze_file(
    filepath: impl AsRef<Path>,
    config: &OneiromancerConfig,
) -> Result<OneiromancerResults, OneiromancerError> {
    // Open target pseudo-code file for reading
    // Note: for easier testing, we could use a generic function together with `std::io::Cursor`
    let file = File::open(&filepath)?;
    let mut reader = BufReader::new(file);
    let mut pseudo_code = String::new();
    reader.read_to_string(&mut pseudo_code)?;

    // Analyze `pseudo_code`
    analyze_code(&pseudo_code, config)
}

#[cfg(test)]
mod tests {
    use std::{env, fs};

    use super::*;
    use crate::oneiromancer::{OLLAMA_BASEURL, OLLAMA_MODEL};

    #[test]
    fn ollama_request_works() {
        let baseurl = env::var("OLLAMA_BASEURL");
        let model = env::var("OLLAMA_MODEL");
        let pseudo_code = r#"int main() { printf("Hello, world!"); }"#;

        let request = OllamaRequest::new(model.as_deref().unwrap_or(OLLAMA_MODEL), pseudo_code);
        let result = request.send(baseurl.as_deref().unwrap_or(OLLAMA_BASEURL));

        assert!(!result.unwrap().response.is_empty(), "response is empty");
    }

    #[test]
    fn ollama_request_with_wrong_url_fails() {
        let baseurl = "http://127.0.0.1:6666";
        let model = env::var("OLLAMA_MODEL");
        let pseudo_code = r#"int main() { printf("Hello, world!"); }"#;

        let request = OllamaRequest::new(model.as_deref().unwrap_or(OLLAMA_MODEL), pseudo_code);
        let result = request.send(baseurl);

        assert!(result.is_err());
    }

    #[test]
    fn ollama_request_with_wrong_model_fails() {
        let baseurl = env::var("OLLAMA_BASEURL");
        let model = "doesntexist";
        let pseudo_code = r#"int main() { printf("Hello, world!"); }"#;

        let request = OllamaRequest::new(model, pseudo_code);
        let result = request.send(baseurl.as_deref().unwrap_or(OLLAMA_BASEURL));

        assert!(result.is_err());
    }

    #[test]
    fn ollama_request_with_empty_prompt_returns_an_empty_response() {
        let baseurl = env::var("OLLAMA_BASEURL");
        let model = env::var("OLLAMA_MODEL");
        let pseudo_code = "";

        let request = OllamaRequest::new(model.as_deref().unwrap_or(OLLAMA_MODEL), pseudo_code);
        let result = request.send(baseurl.as_deref().unwrap_or(OLLAMA_BASEURL));

        assert!(result.unwrap().response.is_empty(), "response is not empty");
    }

    #[test]
    fn analyze_code_works() {
        let baseurl = env::var("OLLAMA_BASEURL");
        let model = env::var("OLLAMA_MODEL");
        let config = OneiromancerConfig::new()
            .with_baseurl(baseurl.as_deref().unwrap_or(OLLAMA_BASEURL))
            .with_model(model.as_deref().unwrap_or(OLLAMA_MODEL));
        let pseudo_code = r#"int main() { printf("Hello, world!"); }"#;

        let result = analyze_code(pseudo_code, &config);

        assert!(
            !result.unwrap().comment().is_empty(),
            "description is empty"
        );
    }

    #[test]
    fn analyze_code_with_default_parameters_works() {
        let pseudo_code = r#"int main() { printf("Hello, world!"); }"#;

        let result = analyze_code(pseudo_code, &OneiromancerConfig::default());

        assert!(
            !result.unwrap().comment().is_empty(),
            "description is empty"
        );
    }

    #[test]
    fn analyze_code_with_empty_pseudo_code_string_fails() {
        let pseudo_code = "";

        let result = analyze_code(pseudo_code, &OneiromancerConfig::default());

        assert!(result.is_err());
    }

    #[test]
    fn analyze_file_works() {
        let baseurl = env::var("OLLAMA_BASEURL");
        let model = env::var("OLLAMA_MODEL");
        let config = OneiromancerConfig::new()
            .with_baseurl(baseurl.as_deref().unwrap_or(OLLAMA_BASEURL))
            .with_model(model.as_deref().unwrap_or(OLLAMA_MODEL));
        let filepath = "./tests/data/hello.c";

        let result = analyze_file(filepath, &config);

        assert!(
            !result.unwrap().comment().is_empty(),
            "description is empty"
        );
    }

    #[test]
    fn analyze_file_with_default_parameters_works() {
        let filepath = "./tests/data/hello.c";

        let result = analyze_file(filepath, &OneiromancerConfig::default());

        assert!(
            !result.unwrap().comment().is_empty(),
            "description is empty"
        );
    }

    #[test]
    fn analyze_file_with_empty_input_file_fails() {
        let filepath = "./tests/data/empty.c";

        let result = analyze_file(filepath, &OneiromancerConfig::default());

        assert!(result.is_err());
    }

    #[test]
    fn analyze_file_with_invalid_input_filepath_fails() {
        let filepath = "./tests/data/invalid.c";

        let result = analyze_file(filepath, &OneiromancerConfig::default());

        assert!(result.is_err());
    }

    #[test]
    fn run_works() {
        let tmpdir = tempfile::tempdir().unwrap();
        let filepath = tmpdir.path().join("test.c");
        fs::copy("./tests/data/hello.c", &filepath).unwrap();

        let result = run(&filepath);
        let outfile = tmpdir.path().join("test.out.c");

        assert!(result.is_ok());
        assert!(outfile.exists(), "output file {outfile:?} does not exist");
        assert!(
            outfile.metadata().unwrap().len() > 0,
            "output file {outfile:?} is empty"
        );
    }

    #[test]
    fn run_with_empty_file_fails() {
        let tmpdir = tempfile::tempdir().unwrap();
        let filepath = tmpdir.path().join("test.c");
        File::create(&filepath).unwrap();

        let result = run(&filepath);
        let outfile = tmpdir.path().join("test.out.c");

        assert!(result.is_err());
        assert!(!outfile.exists());
    }

    #[test]
    fn run_with_invalid_input_filepath_fails() {
        let tmpdir = tempfile::tempdir().unwrap();
        let filepath = tmpdir.path().join("test.c");

        let result = run(&filepath);
        let outfile = tmpdir.path().join("test.out.c");

        assert!(result.is_err());
        assert!(!outfile.exists());
    }
}
