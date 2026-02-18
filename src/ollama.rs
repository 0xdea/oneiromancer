//! Handle interactions with the Ollama API

use serde::{Deserialize, Serialize};

use crate::{OneiromancerError, OneiromancerResults};

/// Ollama API request content
#[derive(Serialize, Debug, Clone)]
pub struct OllamaRequest<'a> {
    /// Name of the model to use for the analysis
    model: &'a str,
    /// Input prompt to send to the model
    prompt: &'a str,
    /// Whether to stream the response or not (should be `false` for our purposes)
    stream: bool,
    /// Response format to use (should be `json` for our purposes)
    format: &'a str,
}

impl<'a> OllamaRequest<'a> {
    /// Create a new `OllamaRequest`
    pub(crate) const fn new(model: &'a str, prompt: &'a str) -> Self {
        Self {
            model,
            prompt,
            stream: false,
            format: "json",
        }
    }

    /// Send an `OllamaRequest` to the `/api/generate` endpoint at `baseurl`.
    ///
    /// Return an `OllamaResponse` or the appropriate `OneiromancerError` in case something goes wrong.
    pub(crate) fn send(&self, baseurl: &str) -> Result<OllamaResponse, OneiromancerError> {
        let url = format!("{}{}", baseurl.trim_end_matches('/'), "/api/generate");
        Ok(ureq::post(url)
            .send_json(self)?
            .body_mut()
            .read_json::<OllamaResponse>()?)
    }
}

/// Ollama API response content
#[derive(Deserialize, Debug, Clone)]
pub struct OllamaResponse {
    /// Ollama API response content
    pub(crate) response: String,
}

impl OllamaResponse {
    /// Parse an `OllamaResponse` into an `OneiromancerResults` struct.
    ///
    /// Return `OneiromancerResults` or the appropriate `OneiromancerError` in case something goes wrong.
    pub(crate) fn parse(&self) -> Result<OneiromancerResults, OneiromancerError> {
        Ok(serde_json::from_str(&self.response)?)
    }
}
