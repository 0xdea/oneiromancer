#![doc = env!("CARGO_PKG_DESCRIPTION")]
#![doc = ""]
#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![doc(html_logo_url = "https://raw.githubusercontent.com/0xdea/oneiromancer/master/.img/logo.png")]

use std::fs::File;
use std::io::{BufReader, BufWriter, Read as _, Write as _};
use std::path::Path;

use anyhow::Context as _;
use regex::Regex;
use spinners::{Spinner, Spinners};

#[expect(
    clippy::pub_use,
    reason = "`pub use` is the idiomatic way to flatten a public API"
)]
#[expect(
    clippy::useless_attribute,
    reason = "the `expect` attribute is actually useful here..."
)]
pub use crate::oneiromancer::{Oneiromancer, OneiromancerError, OneiromancerResults, Variable};

mod ollama;
mod oneiromancer;

/// Submits pseudocode in the `filepath` file to the local LLM for analysis. Outputs analysis results to
/// terminal and saves improved pseudocode in `filepath` with an `out.c` extension.
///
/// # Errors
///
/// Returns [`anyhow::Error`] in case something goes wrong with file I/O or analysis.
pub fn run(filepath: impl AsRef<Path>) -> anyhow::Result<()> {
    // Open the target pseudocode file for reading.
    println!(
        "[*] Analyzing pseudocode in `{}`",
        filepath.as_ref().display()
    );
    let file = File::open(&filepath)
        .with_context(|| format!("Failed to open `{}`", filepath.as_ref().display()))?;
    let mut pseudocode = String::new();
    BufReader::new(file)
        .read_to_string(&mut pseudocode)
        .with_context(|| format!("Failed to read from `{}`", filepath.as_ref().display()))?;

    // Submit pseudocode to the local LLM for analysis.
    let mut sp = Spinner::new(
        Spinners::SimpleDotsScrolling,
        "Querying the Oneiromancer".into(),
    );
    let analysis_results = Oneiromancer::new()
        .analyze_code(&pseudocode)
        .context("Failed to analyze pseudocode")?;
    sp.stop_with_message("[+] Successfully analyzed pseudocode".into());
    println!();

    // Create a function description.
    let function_description = format_description(&analysis_results);
    print!("{function_description}");

    // Apply variable renaming suggestions.
    println!("[-] Variable renaming suggestions:");
    for variable in analysis_results.variables() {
        println!(
            "    {}\t-> {}",
            variable.original_name(),
            variable.new_name()
        );
    }
    pseudocode = apply_renames(&pseudocode, analysis_results.variables())
        .context("Failed to apply variable renames")?;

    // Save the improved pseudocode to an output file.
    let outfilepath = filepath.as_ref().with_extension("out.c");
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

/// Formats `results` as a Phrack-style block comment, wrapping to 76 columns.
fn format_description(results: &OneiromancerResults) -> String {
    let options = textwrap::Options::new(76)
        .initial_indent(" * ")
        .subsequent_indent(" * ");
    format!(
        "/*\n * {}()\n *\n{}\n */\n\n",
        results.function_name(),
        textwrap::fill(results.comment(), &options)
    )
}

/// Applies variable renaming suggestions to `pseudocode` using whole-word regex substitution.
///
/// Assumes LLM-suggested names are collision-safe so renaming order cannot corrupt later replacements.
fn apply_renames(pseudocode: &str, variables: &[Variable]) -> anyhow::Result<String> {
    let mut result = pseudocode.to_owned();
    for variable in variables {
        let re = Regex::new(&format!(r"\b{}\b", regex::escape(variable.original_name())))
            .context("Failed to compile regex")?;
        result = re.replace_all(&result, variable.new_name()).into();
    }
    Ok(result)
}

#[cfg(test)]
#[expect(clippy::panic_in_result_fn, reason = "panics are allowed in test code")]
#[expect(clippy::expect_used, reason = "tests can use `expect`")]
mod tests {
    /// Pure logic tests.
    mod helpers {
        use super::super::*;
        use crate::{OneiromancerResults, Variable};

        #[test]
        fn format_description_produces_phrack_style_header() -> anyhow::Result<()> {
            let results: OneiromancerResults = serde_json::from_str(
                r#"{"function_name":"main","comment":"Entry point of the program.","variables":[]}"#,
            )?;

            let desc = format_description(&results);

            assert!(desc.starts_with("/*\n * main()\n *\n"), "unexpected header");
            assert!(desc.ends_with(" */\n\n"), "unexpected footer");
            assert!(desc.contains("Entry point"), "comment missing from output");

            Ok(())
        }

        #[test]
        fn format_description_wraps_long_comment() -> anyhow::Result<()> {
            let long_comment = "This is a very long function description that must be wrapped \
                because it far exceeds the seventy-six column limit imposed by the Phrack-style \
                formatting, so textwrap should split it across multiple lines.";
            let json =
                format!(r#"{{"function_name":"foo","comment":"{long_comment}","variables":[]}}"#);
            let results: OneiromancerResults = serde_json::from_str(&json)?;

            let desc = format_description(&results);

            for line in desc.lines() {
                assert!(line.len() <= 76, "line exceeds 76 columns: {line:?}");
            }
            let comment_lines = desc.lines().filter(|l| l.starts_with(" * ")).count();
            assert!(
                comment_lines > 2,
                "comment was not wrapped into multiple lines"
            );

            Ok(())
        }

        #[test]
        fn apply_renames_substitutes_whole_words() -> anyhow::Result<()> {
            let variables: Vec<Variable> =
                serde_json::from_str(r#"[{"original_name":"v1","new_name":"counter"}]"#)?;

            let result = apply_renames("int v1 = 0; v1++;", &variables)?;

            assert_eq!(result, "int counter = 0; counter++;");

            Ok(())
        }

        #[test]
        fn apply_renames_does_not_match_substrings() -> anyhow::Result<()> {
            let variables: Vec<Variable> =
                serde_json::from_str(r#"[{"original_name":"len","new_name":"length"}]"#)?;

            let result = apply_renames("strlen(len)", &variables)?;

            assert_eq!(result, "strlen(length)");

            Ok(())
        }

        #[test]
        fn apply_renames_with_no_variables_is_identity() -> anyhow::Result<()> {
            let pseudocode = "int v1 = 0;";

            let result = apply_renames(pseudocode, &[])?;

            assert_eq!(result, pseudocode);

            Ok(())
        }

        #[test]
        fn apply_renames_applies_multiple_variables_independently() -> anyhow::Result<()> {
            let variables: Vec<Variable> = serde_json::from_str(
                r#"[{"original_name":"v1","new_name":"index"},{"original_name":"v2","new_name":"count"}]"#,
            )?;

            let result = apply_renames("int v1 = 0; int v2 = 0; v1 += v2;", &variables)?;

            assert_eq!(result, "int index = 0; int count = 0; index += count;");

            Ok(())
        }
    }

    /// Tests for the Ollama-dependent public API.
    mod api {
        use std::env;
        use std::fs::{self, File};

        use crate::oneiromancer::{OLLAMA_BASEURL, OLLAMA_MODEL};
        use crate::{Oneiromancer, OneiromancerError, run};

        const VALID_PSEUDOCODE: &str = r#"int main() { int v1 = 0; printf("Hello, world!"); }"#;
        const VALID_PSEUDOCODE_FILEPATH: &str = "./tests/data/hello.c";
        const EMPTY_PSEUDOCODE_FILEPATH: &str = "./tests/data/empty.c";

        #[test]
        #[ignore = "requires a live Ollama instance"]
        fn analyze_code_works() -> anyhow::Result<()> {
            let baseurl = env::var("OLLAMA_BASEURL").unwrap_or_else(|_| OLLAMA_BASEURL.to_owned());
            let model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| OLLAMA_MODEL.to_owned());
            let pseudocode = VALID_PSEUDOCODE;

            let results = Oneiromancer::new()
                .baseurl(baseurl)
                .model(model)
                .analyze_code(pseudocode)?;

            assert!(!results.comment().is_empty(), "description is empty");

            Ok(())
        }

        #[test]
        #[ignore = "requires a live Ollama instance"]
        fn analyze_code_with_default_parameters_works() -> anyhow::Result<()> {
            let pseudocode = VALID_PSEUDOCODE;

            let results = Oneiromancer::new().analyze_code(pseudocode)?;

            assert!(!results.comment().is_empty(), "description is empty");

            Ok(())
        }

        #[test]
        #[ignore = "requires a live Ollama instance"]
        fn analyze_code_with_empty_pseudocode_string_fails() {
            let pseudocode = "";

            let result = Oneiromancer::new().analyze_code(pseudocode);

            assert!(result.is_err(), "analysis succeeded unexpectedly");
            assert!(
                matches!(result, Err(OneiromancerError::ResponseParseFailed(_))),
                "wrong error type returned: {result:?}"
            );
        }

        #[test]
        #[ignore = "requires a live Ollama instance"]
        fn analyze_file_works() -> anyhow::Result<()> {
            let baseurl = env::var("OLLAMA_BASEURL").unwrap_or_else(|_| OLLAMA_BASEURL.to_owned());
            let model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| OLLAMA_MODEL.to_owned());
            let filepath = VALID_PSEUDOCODE_FILEPATH;

            let results = Oneiromancer::new()
                .baseurl(baseurl)
                .model(model)
                .analyze_file(filepath)?;

            assert!(!results.comment().is_empty(), "description is empty");

            Ok(())
        }

        #[test]
        #[ignore = "requires a live Ollama instance"]
        fn analyze_file_with_default_parameters_works() -> anyhow::Result<()> {
            let filepath = VALID_PSEUDOCODE_FILEPATH;

            let results = Oneiromancer::new().analyze_file(filepath)?;

            assert!(!results.comment().is_empty(), "description is empty");

            Ok(())
        }

        #[test]
        #[ignore = "requires a live Ollama instance"]
        fn analyze_file_with_empty_input_file_fails() {
            let filepath = EMPTY_PSEUDOCODE_FILEPATH;

            let result = Oneiromancer::new().analyze_file(filepath);

            assert!(result.is_err(), "analysis succeeded unexpectedly");
            assert!(
                matches!(result, Err(OneiromancerError::ResponseParseFailed(_))),
                "wrong error type returned: {result:?}"
            );
        }

        #[test]
        fn analyze_file_with_invalid_input_filepath_fails() {
            let filepath = "./tests/data/invalid.c";

            let result = Oneiromancer::new().analyze_file(filepath);

            assert!(result.is_err(), "analysis succeeded unexpectedly");
            assert!(
                matches!(result, Err(OneiromancerError::FileReadFailed(_))),
                "wrong error type returned: {result:?}"
            );
        }

        #[test]
        #[ignore = "requires a live Ollama instance"]
        fn run_works() -> anyhow::Result<()> {
            let tmpdir = tempfile::tempdir()?;
            let filepath = tmpdir.path().join("test.c");
            fs::copy(VALID_PSEUDOCODE_FILEPATH, &filepath)?;
            let outfile = tmpdir.path().join("test.out.c");

            run(&filepath)?;

            assert!(outfile.exists(), "output file {outfile:?} does not exist");
            assert!(
                outfile.metadata()?.len() > 0,
                "output file {outfile:?} is empty"
            );

            Ok(())
        }

        #[test]
        #[ignore = "requires a live Ollama instance"]
        fn run_fails_if_output_file_already_exists() -> anyhow::Result<()> {
            let tmpdir = tempfile::tempdir()?;
            let filepath = tmpdir.path().join("test.c");
            fs::copy(VALID_PSEUDOCODE_FILEPATH, &filepath)?;
            let outfile = tmpdir.path().join("test.out.c");
            File::create(&outfile)?;

            let result = run(&filepath);

            assert!(result.is_err(), "run succeeded unexpectedly");
            assert!(
                outfile.metadata()?.len() == 0,
                "output file was overwritten"
            );

            Ok(())
        }

        #[test]
        #[ignore = "requires a live Ollama instance"]
        fn run_with_empty_file_fails() {
            let tmpdir = tempfile::tempdir().expect("failed to create temporary directory");
            let filepath = tmpdir.path().join("test.c");
            File::create(&filepath).expect("failed to create test file");
            let outfile = tmpdir.path().join("test.out.c");

            let result = run(&filepath);

            assert!(result.is_err(), "run succeeded unexpectedly");
            assert!(!outfile.exists(), "output file {outfile:?} exists");
        }

        #[test]
        fn run_with_invalid_input_filepath_fails() {
            let tmpdir = tempfile::tempdir().expect("failed to create temporary directory");
            let filepath = tmpdir.path().join("test.c");
            let outfile = tmpdir.path().join("test.out.c");

            let result = run(&filepath);

            assert!(result.is_err(), "run succeeded unexpectedly");
            assert!(!outfile.exists(), "output file {outfile:?} exists");
        }
    }
}
