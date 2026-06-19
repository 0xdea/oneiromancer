//! Handle interactions with the Ollama API.

use serde::{Deserialize, Serialize};

use crate::{OneiromancerError, OneiromancerResults};

/// Ollama API request content.
#[derive(Serialize, Debug, Clone)]
pub struct OllamaRequest<'a> {
    /// Name of the model to use for the analysis.
    model: &'a str,
    /// Input prompt to send to the model.
    prompt: &'a str,
    /// Whether to stream the response or not (should be `false` for our purposes).
    stream: bool,
    /// Response format to use (should be `json` for our purposes).
    format: &'a str,
}

impl<'a> OllamaRequest<'a> {
    /// Creates a new [`OllamaRequest`].
    pub(crate) const fn new(model: &'a str, prompt: &'a str) -> Self {
        Self {
            model,
            prompt,
            stream: false,
            format: "json",
        }
    }

    /// Sends an [`OllamaRequest`] to the `/api/generate` endpoint at `baseurl`.
    ///
    /// Returns an [`OllamaResponse`] which contains the LLM response.
    ///
    /// # Errors
    ///
    /// Returns the appropriate [`OneiromancerError`] in case something goes wrong with the request.
    pub(crate) fn send(&self, baseurl: &str) -> Result<OllamaResponse, OneiromancerError> {
        let url = format!("{}{}", baseurl.trim_end_matches('/'), "/api/generate");
        Ok(ureq::post(url)
            .send_json(self)?
            .body_mut()
            .read_json::<OllamaResponse>()?)
    }
}

/// Ollama API response.
#[derive(Deserialize, Debug, Clone)]
pub struct OllamaResponse {
    /// Ollama API response content.
    response: String,
}

impl OllamaResponse {
    /// Returns the raw response string from the Ollama API.
    pub(crate) fn response(&self) -> &str {
        &self.response
    }

    /// Parses an [`OllamaResponse`] into an [`OneiromancerResults`] struct.
    ///
    /// Returns [`OneiromancerResults`] which contains the parsed LLM response.
    ///
    /// # Errors
    ///
    /// Returns the appropriate [`OneiromancerError`] in case something goes wrong with parsing.
    pub(crate) fn parse(&self) -> Result<OneiromancerResults, OneiromancerError> {
        Ok(serde_json::from_str(self.response())?)
    }
}
