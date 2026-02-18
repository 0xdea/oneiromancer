#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/0xdea/oneiromancer/master/.img/logo.png")]

use std::fs::File;
use std::io::{BufReader, BufWriter, Read as _, Write as _};
use std::path::Path;

use anyhow::Context as _;
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
    let mut pseudocode = String::new();
    BufReader::new(file)
        .read_to_string(&mut pseudocode)
        .with_context(|| format!("Failed to read from `{}`", filepath.display()))?;

    // Submit pseudocode to the local LLM for analysis
    let mut sp = Spinner::new(
        Spinners::SimpleDotsScrolling,
        "Querying the Oneiromancer".into(),
    );
    let analysis_results = analyze_code(&pseudocode, &OneiromancerConfig::default())
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

    // Apply variable renaming suggestions (this assumes LLM-suggested names are collision-safe
    // and thus the renaming order will not corrupt later replacements)
    println!("[-] Variable renaming suggestions:");
    for variable in analysis_results.variables() {
        let original_name = variable.original_name();
        let new_name = variable.new_name();
        println!("    {original_name}\t-> {new_name}");

        let re = Regex::new(&format!(r"\b{}\b", regex::escape(original_name)))
            .context("Failed to compile regex")?;
        pseudocode = re.replace_all(&pseudocode, new_name).into();
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
    writer
        .write_all(function_description.as_bytes())
        .context("Failed to write to file")?;
    writer
        .write_all(pseudocode.as_bytes())
        .context("Failed to write to file")?;
    writer
        .flush()
        .context("Failed to flush the output stream")?;

    println!("[+] Done analyzing pseudocode");
    Ok(())
}

/// Submit `pseudocode` to the local LLM via the Ollama API using the specified
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
/// let pseudocode = r#"int main() { printf("Hello, world!"); }"#;
///
/// let results = analyze_code(&pseudocode, &OneiromancerConfig::default())?;
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
/// let pseudocode = r#"int main() { printf("Hello, world!"); }"#;
///
/// let config = OneiromancerConfig::new()
///     .with_baseurl("http://127.0.0.1:11434")
///     .with_model("aidapal");
/// let results = analyze_code(&pseudocode, &config)?;
///
/// dbg!(results.function_name());
/// dbg!(results.comment());
/// dbg!(results.variables());
/// # Ok(())
/// # }
/// ```
///
pub fn analyze_code(
    pseudocode: impl AsRef<str>,
    config: &OneiromancerConfig,
) -> Result<OneiromancerResults, OneiromancerError> {
    // Send Ollama API request and parse response
    let request = OllamaRequest::new(config.model(), pseudocode.as_ref());
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
    let mut pseudocode = String::new();
    BufReader::new(file).read_to_string(&mut pseudocode)?;

    // Analyze `pseudocode`
    analyze_code(&pseudocode, config)
}

#[cfg(test)]
mod tests {
    use std::{env, fs};

    use super::*;
    use crate::oneiromancer::{OLLAMA_BASEURL, OLLAMA_MODEL};

    const VALID_PSEUDOCODE: &str = r#"int main() { printf("Hello, world!"); }"#;
    const VALID_PSEUDOCODE_FILEPATH: &str = "./tests/data/hello.c";
    const EMPTY_PSEUDOCODE_FILEPATH: &str = "./tests/data/empty.c";

    #[test]
    fn ollama_request_works() -> anyhow::Result<()> {
        // Arrange
        let baseurl = env::var("OLLAMA_BASEURL").unwrap_or_else(|_| OLLAMA_BASEURL.to_owned());
        let model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| OLLAMA_MODEL.to_owned());
        let pseudocode = VALID_PSEUDOCODE;

        // Act
        let request = OllamaRequest::new(&model, pseudocode);
        let response = request.send(&baseurl)?;

        // Assert
        assert!(!response.response.is_empty(), "response is empty");

        Ok(())
    }

    #[test]
    fn ollama_request_with_wrong_url_fails() {
        // Arrange
        let baseurl = "http://127.0.0.1:6666";
        let model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| OLLAMA_MODEL.to_owned());
        let pseudocode = VALID_PSEUDOCODE;

        // Act
        let request = OllamaRequest::new(&model, pseudocode);
        let result = request.send(baseurl);

        // Assert
        assert!(result.is_err(), "request succeeded unexpectedly");
        assert!(
            matches!(result, Err(OneiromancerError::OllamaQueryFailed(_))),
            "wrong error type returned: {result:?}"
        );
    }

    #[test]
    fn ollama_request_with_empty_url_fails() {
        // Arrange
        let baseurl = "";
        let model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| OLLAMA_MODEL.to_owned());
        let pseudocode = VALID_PSEUDOCODE;

        // Act
        let request = OllamaRequest::new(&model, pseudocode);
        let result = request.send(baseurl);

        // Assert
        assert!(result.is_err(), "request succeeded unexpectedly");
        assert!(
            matches!(result, Err(OneiromancerError::OllamaQueryFailed(_))),
            "wrong error type returned: {result:?}"
        );
    }

    #[test]
    fn ollama_request_with_wrong_model_fails() {
        // Arrange
        let baseurl = env::var("OLLAMA_BASEURL").unwrap_or_else(|_| OLLAMA_BASEURL.to_owned());
        let model = "doesntexist";
        let pseudocode = VALID_PSEUDOCODE;

        // Act
        let request = OllamaRequest::new(model, pseudocode);
        let result = request.send(&baseurl);

        // Assert
        assert!(result.is_err(), "request succeeded unexpectedly");
        assert!(
            matches!(result, Err(OneiromancerError::OllamaQueryFailed(_))),
            "wrong error type returned: {result:?}"
        );
    }

    #[test]
    fn ollama_request_with_empty_model_fails() {
        // Arrange
        let baseurl = env::var("OLLAMA_BASEURL").unwrap_or_else(|_| OLLAMA_BASEURL.to_owned());
        let model = "";
        let pseudocode = VALID_PSEUDOCODE;

        // Act
        let request = OllamaRequest::new(model, pseudocode);
        let result = request.send(&baseurl);

        // Assert
        assert!(result.is_err(), "request succeeded unexpectedly");
        assert!(
            matches!(result, Err(OneiromancerError::OllamaQueryFailed(_))),
            "wrong error type returned: {result:?}"
        );
    }

    #[test]
    fn ollama_request_with_empty_prompt_returns_an_empty_response() -> anyhow::Result<()> {
        // Arrange
        let baseurl = env::var("OLLAMA_BASEURL").unwrap_or_else(|_| OLLAMA_BASEURL.to_owned());
        let model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| OLLAMA_MODEL.to_owned());
        let pseudocode = "";

        // Act
        let request = OllamaRequest::new(&model, pseudocode);
        let response = request.send(&baseurl)?;

        // Assert
        assert!(response.response.is_empty(), "response is not empty");

        Ok(())
    }

    #[test]
    fn analyze_code_works() -> anyhow::Result<()> {
        // Arrange
        let baseurl = env::var("OLLAMA_BASEURL").unwrap_or_else(|_| OLLAMA_BASEURL.to_owned());
        let model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| OLLAMA_MODEL.to_owned());
        let config = OneiromancerConfig::new()
            .with_baseurl(baseurl)
            .with_model(model);
        let pseudocode = VALID_PSEUDOCODE;

        // Act
        let results = analyze_code(pseudocode, &config)?;

        // Assert
        assert!(!results.comment().is_empty(), "description is empty");

        Ok(())
    }

    #[test]
    fn analyze_code_with_default_parameters_works() -> anyhow::Result<()> {
        // Arrange
        let pseudocode = VALID_PSEUDOCODE;

        // Act
        let results = analyze_code(pseudocode, &OneiromancerConfig::default())?;

        // Assert
        assert!(!results.comment().is_empty(), "description is empty");

        Ok(())
    }

    #[test]
    fn analyze_code_with_empty_pseudocode_string_fails() {
        // Arrange
        let pseudocode = "";

        // Act
        let result = analyze_code(pseudocode, &OneiromancerConfig::default());

        // Assert
        assert!(result.is_err(), "analysis succeeded unexpectedly");
        assert!(
            matches!(result, Err(OneiromancerError::ResponseParseFailed(_))),
            "wrong error type returned: {result:?}"
        );
    }

    #[test]
    fn analyze_file_works() -> anyhow::Result<()> {
        // Arrange
        let baseurl = env::var("OLLAMA_BASEURL").unwrap_or_else(|_| OLLAMA_BASEURL.to_owned());
        let model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| OLLAMA_MODEL.to_owned());
        let config = OneiromancerConfig::new()
            .with_baseurl(baseurl)
            .with_model(model);
        let filepath = VALID_PSEUDOCODE_FILEPATH;

        // Act
        let results = analyze_file(filepath, &config)?;

        // Assert
        assert!(!results.comment().is_empty(), "description is empty");

        Ok(())
    }

    #[test]
    fn analyze_file_with_default_parameters_works() -> anyhow::Result<()> {
        // Arrange
        let filepath = VALID_PSEUDOCODE_FILEPATH;

        // Act
        let results = analyze_file(filepath, &OneiromancerConfig::default())?;

        // Assert
        assert!(!results.comment().is_empty(), "description is empty");

        Ok(())
    }

    #[test]
    fn analyze_file_with_empty_input_file_fails() {
        // Arrange
        let filepath = EMPTY_PSEUDOCODE_FILEPATH;

        // Act
        let result = analyze_file(filepath, &OneiromancerConfig::default());

        // Assert
        assert!(result.is_err(), "analysis succeeded unexpectedly");
        assert!(
            matches!(result, Err(OneiromancerError::ResponseParseFailed(_))),
            "wrong error type returned: {result:?}"
        );
    }

    #[test]
    fn analyze_file_with_invalid_input_filepath_fails() {
        // Arrange
        let filepath = "./tests/data/invalid.c";

        // Act
        let result = analyze_file(filepath, &OneiromancerConfig::default());

        // Assert
        assert!(result.is_err(), "analysis succeeded unexpectedly");
        assert!(
            matches!(result, Err(OneiromancerError::FileReadFailed(_))),
            "wrong error type returned: {result:?}"
        );
    }

    #[test]
    fn run_works() -> anyhow::Result<()> {
        // Arrange
        let tmpdir = tempfile::tempdir()?;
        let filepath = tmpdir.path().join("test.c");
        fs::copy(VALID_PSEUDOCODE_FILEPATH, &filepath)?;
        let outfile = tmpdir.path().join("test.out.c");

        // Act
        run(&filepath)?;

        // Assert
        assert!(outfile.exists(), "output file {outfile:?} does not exist");
        assert!(
            outfile.metadata()?.len() > 0,
            "output file {outfile:?} is empty"
        );

        Ok(())
    }

    #[test]
    #[expect(clippy::expect_used, reason = "tests can use `expect`")]
    fn run_with_empty_file_fails() {
        // Arrange
        let tmpdir = tempfile::tempdir().expect("failed to create temporary directory");
        let filepath = tmpdir.path().join("test.c");
        File::create(&filepath).expect("failed to create test file");
        let outfile = tmpdir.path().join("test.out.c");

        // Act
        let result = run(&filepath);

        // Assert
        assert!(result.is_err(), "run succeeded unexpectedly");
        assert!(!outfile.exists(), "output file {outfile:?} exists");
    }

    #[test]
    #[expect(clippy::expect_used, reason = "tests can use `expect`")]
    fn run_with_invalid_input_filepath_fails() {
        // Arrange
        let tmpdir = tempfile::tempdir().expect("failed to create temporary directory");
        let filepath = tmpdir.path().join("test.c");
        let outfile = tmpdir.path().join("test.out.c");

        // Act
        let result = run(&filepath);

        // Assert
        assert!(result.is_err(), "run succeeded unexpectedly");
        assert!(!outfile.exists(), "output file {outfile:?} exists");
    }
}
