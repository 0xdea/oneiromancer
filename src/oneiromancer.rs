//! Analyze pseudocode and handle results and errors.

use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read as _};
use std::path::Path;

use serde::Deserialize;
use thiserror::Error;

use crate::ollama::OllamaRequest;

/// Default Ollama URL.
pub const OLLAMA_BASEURL: &str = "http://127.0.0.1:11434";
/// Default Ollama model.
pub const OLLAMA_MODEL: &str = "aidapal";

/// Oneiromancer client for analyzing pseudocode via the Ollama API.
#[derive(Debug, Clone)]
pub struct Oneiromancer {
    /// Ollama API base URL.
    baseurl: String,
    /// Ollama model to use for analysis.
    model: String,
}

impl Oneiromancer {
    /// Creates a new [`Oneiromancer`] with default values.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds an [`Oneiromancer`] with a custom `baseurl`.
    #[must_use]
    pub fn with_baseurl(mut self, baseurl: impl Into<String>) -> Self {
        self.baseurl = baseurl.into();
        self
    }

    /// Builds an [`Oneiromancer`] with a custom `model`.
    #[must_use]
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    /// Gets the configured `baseurl`.
    #[must_use]
    pub fn baseurl(&self) -> &str {
        &self.baseurl
    }

    /// Gets the configured `model`.
    #[must_use]
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Submits `pseudocode` to the local LLM via the Ollama API.
    ///
    /// Returns [`OneiromancerResults`] which contains the parsed LLM response.
    ///
    /// # Errors
    ///
    /// Returns the appropriate [`OneiromancerError`] in case something goes wrong with the analysis.
    ///
    /// # Examples
    ///
    /// Basic usage (default Ollama base URL and model):
    /// ```
    /// # fn main() -> anyhow::Result<()> {
    /// use oneiromancer::Oneiromancer;
    ///
    /// let pseudocode = r#"int main() { printf("Hello, world!"); }"#;
    ///
    /// let results = Oneiromancer::new().analyze_code(pseudocode)?;
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
    /// use oneiromancer::Oneiromancer;
    ///
    /// let pseudocode = r#"int main() { printf("Hello, world!"); }"#;
    ///
    /// let results = Oneiromancer::new()
    ///     .with_baseurl("http://127.0.0.1:11434")
    ///     .with_model("aidapal")
    ///     .analyze_code(pseudocode)?;
    ///
    /// dbg!(results.function_name());
    /// dbg!(results.comment());
    /// dbg!(results.variables());
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub fn analyze_code(
        &self,
        pseudocode: impl AsRef<str>,
    ) -> Result<OneiromancerResults, OneiromancerError> {
        let request = OllamaRequest::new(&self.model, pseudocode.as_ref());
        request.send(&self.baseurl)?.parse()
    }

    /// Submits pseudocode in the `filepath` file to the local LLM via the Ollama API.
    ///
    /// Returns [`OneiromancerResults`] which contains the parsed LLM response.
    ///
    /// # Errors
    ///
    /// Returns the appropriate [`OneiromancerError`] in case something goes wrong with file I/O or analysis.
    ///
    /// # Examples
    ///
    /// Basic usage (default Ollama base URL and model):
    /// ```
    /// # fn main() -> anyhow::Result<()> {
    /// use oneiromancer::Oneiromancer;
    ///
    /// let filepath = "./tests/data/hello.c";
    ///
    /// let results = Oneiromancer::new().analyze_file(filepath)?;
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
    /// use oneiromancer::Oneiromancer;
    ///
    /// let filepath = "./tests/data/hello.c";
    ///
    /// let results = Oneiromancer::new()
    ///     .with_baseurl("http://127.0.0.1:11434")
    ///     .with_model("aidapal")
    ///     .analyze_file(filepath)?;
    ///
    /// dbg!(results.function_name());
    /// dbg!(results.comment());
    /// dbg!(results.variables());
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub fn analyze_file(
        &self,
        filepath: impl AsRef<Path>,
    ) -> Result<OneiromancerResults, OneiromancerError> {
        let file = File::open(&filepath)?;
        let mut pseudocode = String::new();
        BufReader::new(file).read_to_string(&mut pseudocode)?;
        self.analyze_code(&pseudocode)
    }
}

/// Sets `baseurl` and `model` to the value of `OLLAMA_BASEURL` and `OLLAMA_MODEL`
/// environment variables, if any, or falls back to hardcoded default values.
impl Default for Oneiromancer {
    fn default() -> Self {
        Self {
            baseurl: env::var("OLLAMA_BASEURL").unwrap_or_else(|_| OLLAMA_BASEURL.to_owned()),
            model: env::var("OLLAMA_MODEL").unwrap_or_else(|_| OLLAMA_MODEL.to_owned()),
        }
    }
}

/// Oneiromancer error type.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum OneiromancerError {
    /// Failure in reading the input file.
    #[error(transparent)]
    FileReadFailed(#[from] io::Error),
    /// Failure in querying the Ollama API.
    #[error(transparent)]
    OllamaQueryFailed(#[from] ureq::Error),
    /// Failure in parsing the Ollama response.
    #[error(transparent)]
    ResponseParseFailed(#[from] serde_json::Error),
}

/// Pseudocode analysis results.
#[derive(Deserialize, Debug, Clone)]
pub struct OneiromancerResults {
    /// Recommended function name.
    function_name: String,
    /// Function description.
    comment: String,
    /// Variable renaming suggestions.
    variables: Vec<Variable>,
}

impl OneiromancerResults {
    /// Gets the recommended function name.
    #[must_use]
    pub fn function_name(&self) -> &str {
        &self.function_name
    }

    /// Gets function description.
    #[must_use]
    pub fn comment(&self) -> &str {
        &self.comment
    }

    /// Gets variable renaming suggestions.
    #[must_use]
    pub fn variables(&self) -> &[Variable] {
        &self.variables
    }
}

/// Variable renaming suggestion.
#[derive(Deserialize, Debug, Clone)]
pub struct Variable {
    /// Original name of the variable.
    original_name: String,
    /// Suggested name for the variable.
    new_name: String,
}

impl Variable {
    /// Gets the original name of the variable.
    #[must_use]
    pub fn original_name(&self) -> &str {
        &self.original_name
    }

    /// Gets the suggested name for the variable.
    #[must_use]
    pub fn new_name(&self) -> &str {
        &self.new_name
    }
}
