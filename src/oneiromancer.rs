//! Collect code analysis results and handle errors

use std::env;

use serde::Deserialize;
use thiserror::Error;

/// Default Ollama URL
pub const OLLAMA_BASEURL: &str = "http://127.0.0.1:11434";
/// Default Ollama model
pub const OLLAMA_MODEL: &str = "aidapal";

/// Oneiromancer error type
#[derive(Error, Debug)]
pub enum OneiromancerError {
    /// Failure in reading input file
    #[error(transparent)]
    FileReadFailed(#[from] std::io::Error),
    /// Failure in querying Ollama API
    #[error(transparent)]
    OllamaQueryFailed(#[from] ureq::Error),
    /// Failure in parsing Ollama response
    #[error(transparent)]
    ResponseParseFailed(#[from] serde_json::Error),
}

/// Oneiromancer configuration
#[derive(Debug, Clone)]
pub struct OneiromancerConfig {
    baseurl: String,
    model: String,
}

#[allow(clippy::missing_const_for_fn)]
impl OneiromancerConfig {
    /// Create a new `OneiromancerConfig` with default values
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the configured `baseurl`
    #[must_use]
    pub fn baseurl(&self) -> &str {
        &self.baseurl
    }

    /// Get the configured `model`
    #[must_use]
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Build an `OneiromancerConfig` with a custom `baseurl`
    #[must_use]
    pub fn with_baseurl(mut self, baseurl: impl Into<String>) -> Self {
        self.baseurl = baseurl.into();
        self
    }

    /// Build an `OneiromancerConfig` with a custom `model`
    #[must_use]
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }
}

/// Set `baseurl` and `model` to the value of `OLLAMA_BASEURL` and `OLLAMA_MODEL`
/// environment variables, if any, or fall back to hardcoded default values.
impl Default for OneiromancerConfig {
    fn default() -> Self {
        Self {
            baseurl: env::var("OLLAMA_BASEURL").unwrap_or_else(|_| OLLAMA_BASEURL.to_string()),
            model: env::var("OLLAMA_MODEL").unwrap_or_else(|_| OLLAMA_MODEL.to_string()),
        }
    }
}

/// Code analysis results
#[derive(Deserialize, Debug, Clone)]
pub struct OneiromancerResults {
    /// Recommended function name
    function_name: String,
    /// Function description
    comment: String,
    /// Variable renaming suggestions
    variables: Vec<Variable>,
}

#[allow(clippy::missing_const_for_fn)]
impl OneiromancerResults {
    /// Get recommended function name
    #[must_use]
    pub fn function_name(&self) -> &str {
        &self.function_name
    }

    /// Get function description
    #[must_use]
    pub fn comment(&self) -> &str {
        &self.comment
    }

    /// Get variable renaming suggestions
    #[must_use]
    pub fn variables(&self) -> &[Variable] {
        &self.variables
    }
}

/// Variable renaming suggestion
#[derive(Deserialize, Debug, Clone)]
pub struct Variable {
    /// Original name of the variable
    original_name: String,
    /// Suggested name for the variable
    new_name: String,
}

#[allow(clippy::missing_const_for_fn)]
impl Variable {
    /// Get original name of the variable
    #[must_use]
    pub fn original_name(&self) -> &str {
        &self.original_name
    }

    /// Get suggested name for the variable
    #[must_use]
    pub fn new_name(&self) -> &str {
        &self.new_name
    }
}
