use serde::Deserialize;
use thiserror::Error;

/// Default Ollama URL
pub const OLLAMA_BASEURL: &str = "http://127.0.0.1:11434";
/// Default Ollama model
pub const OLLAMA_MODEL: &str = "aidapal";

#[derive(Error, Debug)]
pub enum OneiromancerError {
    #[error(transparent)]
    FileReadFailed(#[from] std::io::Error),
    #[error(transparent)]
    OllamaQueryFailed(#[from] ureq::Error),
    #[error(transparent)]
    ResponseParseFailed(#[from] serde_json::Error),
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

impl OneiromancerResults {
    /// Get recommended function name
    pub fn function_name(&self) -> &str {
        &self.function_name
    }

    /// Get function description
    pub fn comment(&self) -> &str {
        &self.comment
    }

    /// Get variable renaming suggestions
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

impl Variable {
    /// Get original name of the variable
    pub fn original_name(&self) -> &str {
        &self.original_name
    }

    /// Get suggested name for the variable
    pub fn new_name(&self) -> &str {
        &self.new_name
    }
}
