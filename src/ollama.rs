//! Handle interactions with the Ollama API

use serde::{Deserialize, Serialize};

use crate::{OneiromancerError, OneiromancerResults};

/// Ollama API request content
#[derive(Serialize, Debug, Clone)]
pub struct OllamaRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
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
