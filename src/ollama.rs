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

#[cfg(test)]
#[expect(clippy::panic_in_result_fn, reason = "panics are allowed in test code")]
mod tests {
    use std::env;

    use super::OllamaRequest;
    use crate::OneiromancerError;
    use crate::oneiromancer::{OLLAMA_BASEURL, OLLAMA_MODEL};

    const VALID_PSEUDOCODE: &str = r#"int main() { int v1 = 0; printf("Hello, world!"); }"#;

    #[test]
    #[ignore = "requires a live Ollama instance"]
    fn ollama_request_works() -> anyhow::Result<()> {
        let baseurl = env::var("OLLAMA_BASEURL").unwrap_or_else(|_| OLLAMA_BASEURL.to_owned());
        let model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| OLLAMA_MODEL.to_owned());
        let pseudocode = VALID_PSEUDOCODE;

        let request = OllamaRequest::new(&model, pseudocode);
        let response = request.send(&baseurl)?;

        assert!(!response.response().is_empty(), "response is empty");

        Ok(())
    }

    #[test]
    fn ollama_request_with_wrong_url_fails() {
        let baseurl = "http://127.0.0.1:6666";
        let model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| OLLAMA_MODEL.to_owned());
        let pseudocode = VALID_PSEUDOCODE;

        let request = OllamaRequest::new(&model, pseudocode);
        let result = request.send(baseurl);

        assert!(result.is_err(), "request succeeded unexpectedly");
        assert!(
            matches!(result, Err(OneiromancerError::OllamaQueryFailed(_))),
            "wrong error type returned: {result:?}"
        );
    }

    #[test]
    fn ollama_request_with_empty_url_fails() {
        let baseurl = "";
        let model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| OLLAMA_MODEL.to_owned());
        let pseudocode = VALID_PSEUDOCODE;

        let request = OllamaRequest::new(&model, pseudocode);
        let result = request.send(baseurl);

        assert!(result.is_err(), "request succeeded unexpectedly");
        assert!(
            matches!(result, Err(OneiromancerError::OllamaQueryFailed(_))),
            "wrong error type returned: {result:?}"
        );
    }

    #[test]
    #[ignore = "requires a live Ollama instance"]
    fn ollama_request_with_wrong_model_fails() {
        let baseurl = env::var("OLLAMA_BASEURL").unwrap_or_else(|_| OLLAMA_BASEURL.to_owned());
        let model = "doesntexist";
        let pseudocode = VALID_PSEUDOCODE;

        let request = OllamaRequest::new(model, pseudocode);
        let result = request.send(&baseurl);

        assert!(result.is_err(), "request succeeded unexpectedly");
        assert!(
            matches!(result, Err(OneiromancerError::OllamaQueryFailed(_))),
            "wrong error type returned: {result:?}"
        );
    }

    #[test]
    #[ignore = "requires a live Ollama instance"]
    fn ollama_request_with_empty_model_fails() {
        let baseurl = env::var("OLLAMA_BASEURL").unwrap_or_else(|_| OLLAMA_BASEURL.to_owned());
        let model = "";
        let pseudocode = VALID_PSEUDOCODE;

        let request = OllamaRequest::new(model, pseudocode);
        let result = request.send(&baseurl);

        assert!(result.is_err(), "request succeeded unexpectedly");
        assert!(
            matches!(result, Err(OneiromancerError::OllamaQueryFailed(_))),
            "wrong error type returned: {result:?}"
        );
    }

    #[test]
    #[ignore = "requires a live Ollama instance"]
    fn ollama_request_with_empty_prompt_returns_an_empty_response() -> anyhow::Result<()> {
        let baseurl = env::var("OLLAMA_BASEURL").unwrap_or_else(|_| OLLAMA_BASEURL.to_owned());
        let model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| OLLAMA_MODEL.to_owned());
        let pseudocode = "";

        let request = OllamaRequest::new(&model, pseudocode);
        let response = request.send(&baseurl)?;

        assert!(response.response().is_empty(), "response is not empty");

        Ok(())
    }
}
