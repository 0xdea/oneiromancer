#![doc = include_str!("../README.md")]
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

/// Submit pseudocode in `filepath` file to local LLM for analysis. Output analysis results to
/// terminal and save improved pseudocode in `filepath` with an `out.c` extension.
///
/// ## Errors
///
/// Returns success or a generic error in case something goes wrong.
pub fn run(filepath: &Path) -> anyhow::Result<()> {
    // Open the target pseudocode file for reading
    println!("[*] Analyzing pseudocode in `{}`", filepath.display());
    let file =
        File::open(filepath).with_context(|| format!("Failed to open `{}`", filepath.display()))?;
    let mut reader = BufReader::new(file);
    let mut pseudo_code = String::new();
    reader
        .read_to_string(&mut pseudo_code)
        .with_context(|| format!("Failed to read from `{}`", filepath.display()))?;

    // Submit pseudocode to the local LLM for analysis
    let mut sp = Spinner::new(
        Spinners::SimpleDotsScrolling,
        "Querying the Oneiromancer".into(),
    );
    let analysis_results = analyze_code(&pseudo_code, &OneiromancerConfig::default())
        .context("Failed to analyze pseudocode")?;
    sp.stop_with_message("[+] Successfully analyzed pseudocode".into());
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

    // Save the improved pseudocode to an output file
    let outfilepath = filepath.with_extension("out.c");
    println!();
    println!(
        "[*] Saving improved pseudocode in `{}`",
        outfilepath.display()
    );

    let mut writer = BufWriter::new(
        File::create_new(&outfilepath)
            .with_context(|| format!("Failed to create `{}`", outfilepath.display()))?,
    );
    writer.write_all(function_description.as_bytes())?;
    writer.write_all(pseudo_code.as_bytes())?;
    writer.flush()?;

    println!("[+] Done analyzing pseudocode");
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

/// Submit pseudocode in the `filepath` file to the local LLM via the Ollama API using the specified
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
    // Open target pseudocode file for reading
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
        // Arrange
        let baseurl = env::var("OLLAMA_BASEURL");
        let model = env::var("OLLAMA_MODEL");
        let pseudo_code = r#"int main() { printf("Hello, world!"); }"#;

        // Act
        let request = OllamaRequest::new(model.as_deref().unwrap_or(OLLAMA_MODEL), pseudo_code);
        let result = request.send(baseurl.as_deref().unwrap_or(OLLAMA_BASEURL));

        // Assert
        assert!(!result.unwrap().response.is_empty(), "response is empty");
    }

    #[test]
    fn ollama_request_with_wrong_url_fails() {
        // Arrange
        let baseurl = "http://127.0.0.1:6666";
        let model = env::var("OLLAMA_MODEL");
        let pseudo_code = r#"int main() { printf("Hello, world!"); }"#;

        // Act
        let request = OllamaRequest::new(model.as_deref().unwrap_or(OLLAMA_MODEL), pseudo_code);
        let result = request.send(baseurl);

        // Assert
        assert!(result.is_err(), "request succeeded unexpectedly");
    }

    #[test]
    fn ollama_request_with_wrong_model_fails() {
        // Arrange
        let baseurl = env::var("OLLAMA_BASEURL");
        let model = "doesntexist";
        let pseudo_code = r#"int main() { printf("Hello, world!"); }"#;

        // Act
        let request = OllamaRequest::new(model, pseudo_code);
        let result = request.send(baseurl.as_deref().unwrap_or(OLLAMA_BASEURL));

        // Assert
        assert!(result.is_err(), "request succeeded unexpectedly");
    }

    #[test]
    fn ollama_request_with_empty_prompt_returns_an_empty_response() {
        // Arrange
        let baseurl = env::var("OLLAMA_BASEURL");
        let model = env::var("OLLAMA_MODEL");
        let pseudo_code = "";

        // Act
        let request = OllamaRequest::new(model.as_deref().unwrap_or(OLLAMA_MODEL), pseudo_code);
        let result = request.send(baseurl.as_deref().unwrap_or(OLLAMA_BASEURL));

        // Assert
        assert!(result.unwrap().response.is_empty(), "response is not empty");
    }

    #[test]
    fn analyze_code_works() {
        // Arrange
        let baseurl = env::var("OLLAMA_BASEURL");
        let model = env::var("OLLAMA_MODEL");
        let config = OneiromancerConfig::new()
            .with_baseurl(baseurl.as_deref().unwrap_or(OLLAMA_BASEURL))
            .with_model(model.as_deref().unwrap_or(OLLAMA_MODEL));
        let pseudo_code = r#"int main() { printf("Hello, world!"); }"#;

        // Act
        let result = analyze_code(pseudo_code, &config);

        // Assert
        assert!(
            !result.unwrap().comment().is_empty(),
            "description is empty"
        );
    }

    #[test]
    fn analyze_code_with_default_parameters_works() {
        // Arrange
        let pseudo_code = r#"int main() { printf("Hello, world!"); }"#;

        // Act
        let result = analyze_code(pseudo_code, &OneiromancerConfig::default());

        // Assert
        assert!(
            !result.unwrap().comment().is_empty(),
            "description is empty"
        );
    }

    #[test]
    fn analyze_code_with_empty_pseudo_code_string_fails() {
        // Arrange
        let pseudo_code = "";

        // Act
        let result = analyze_code(pseudo_code, &OneiromancerConfig::default());

        // Assert
        assert!(result.is_err(), "analysis succeeded unexpectedly");
    }

    #[test]
    fn analyze_file_works() {
        // Arrange
        let baseurl = env::var("OLLAMA_BASEURL");
        let model = env::var("OLLAMA_MODEL");
        let config = OneiromancerConfig::new()
            .with_baseurl(baseurl.as_deref().unwrap_or(OLLAMA_BASEURL))
            .with_model(model.as_deref().unwrap_or(OLLAMA_MODEL));
        let filepath = "./tests/data/hello.c";

        // Act
        let result = analyze_file(filepath, &config);

        // Assert
        assert!(
            !result.unwrap().comment().is_empty(),
            "description is empty"
        );
    }

    #[test]
    fn analyze_file_with_default_parameters_works() {
        // Arrange
        let filepath = "./tests/data/hello.c";

        // Act
        let result = analyze_file(filepath, &OneiromancerConfig::default());

        // Assert
        assert!(
            !result.unwrap().comment().is_empty(),
            "description is empty"
        );
    }

    #[test]
    fn analyze_file_with_empty_input_file_fails() {
        // Arrange
        let filepath = "./tests/data/empty.c";

        // Act
        let result = analyze_file(filepath, &OneiromancerConfig::default());

        // Assert
        assert!(result.is_err(), "analysis succeeded unexpectedly");
    }

    #[test]
    fn analyze_file_with_invalid_input_filepath_fails() {
        // Arrange
        let filepath = "./tests/data/invalid.c";

        // Act
        let result = analyze_file(filepath, &OneiromancerConfig::default());

        // Assert
        assert!(result.is_err(), "analysis succeeded unexpectedly");
    }

    #[test]
    fn run_works() {
        // Arrange
        let tmpdir = tempfile::tempdir().unwrap();
        let filepath = tmpdir.path().join("test.c");
        fs::copy("./tests/data/hello.c", &filepath).unwrap();

        // Act
        let result = run(&filepath);
        let outfile = tmpdir.path().join("test.out.c");

        // Assert
        assert!(result.is_ok(), "run failed");
        assert!(outfile.exists(), "output file {outfile:?} does not exist");
        assert!(
            outfile.metadata().unwrap().len() > 0,
            "output file {outfile:?} is empty"
        );
    }

    #[test]
    fn run_with_empty_file_fails() {
        // Arrange
        let tmpdir = tempfile::tempdir().unwrap();
        let filepath = tmpdir.path().join("test.c");
        File::create(&filepath).unwrap();

        // Act
        let result = run(&filepath);
        let outfile = tmpdir.path().join("test.out.c");

        // Assert
        assert!(result.is_err(), "run succeeded unexpectedly");
        assert!(!outfile.exists(), "output file {outfile:?} exists");
    }

    #[test]
    fn run_with_invalid_input_filepath_fails() {
        // Arrange
        let tmpdir = tempfile::tempdir().unwrap();
        let filepath = tmpdir.path().join("test.c");

        // Act
        let result = run(&filepath);
        let outfile = tmpdir.path().join("test.out.c");

        // Assert
        assert!(result.is_err(), "run succeeded unexpectedly");
        assert!(!outfile.exists(), "output file {outfile:?} exists");
    }
}
