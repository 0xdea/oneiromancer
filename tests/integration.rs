//! Integration tests using a mock Ollama API server.

#![expect(clippy::panic_in_result_fn, reason = "panics are allowed in test code")]
#![expect(
    clippy::tests_outside_test_module,
    reason = "no need to have a test module for integration tests in `/tests`"
)]
#![expect(
    clippy::default_numeric_fallback,
    reason = "numeric literals in test code don't have to be annotated with a type suffix"
)]

use httpmock::prelude::*;
use oneiromancer::{Oneiromancer, OneiromancerError};

const VALID_PSEUDOCODE: &str = r#"int main() { int v1 = 0; printf("Hello, world!"); }"#;

// The `response` field is an escaped JSON string that deserializes to [`OneiromancerResults`].
const MOCK_VALID_RESPONSE: &str = r#"{"response":"{\"function_name\":\"main\",\"comment\":\"Entry point of the program.\",\"variables\":[{\"original_name\":\"v1\",\"new_name\":\"counter\"}]}"}"#;
const MOCK_MALFORMED_RESPONSE: &str = r#"{"response":"not valid json"}"#;

/// Returns an [`Oneiromancer`] instance configured to use the given mock server.
fn client(server: &MockServer) -> Oneiromancer {
    Oneiromancer::new()
        .with_baseurl(server.base_url())
        .with_model("test-model")
}

#[test]
fn analyze_code_sends_correct_request() -> anyhow::Result<()> {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/api/generate")
            .body_includes("test-model")
            .body_includes("int main()");
        then.status(200)
            .header("content-type", "application/json")
            .body(MOCK_VALID_RESPONSE);
    });

    client(&server).analyze_code(VALID_PSEUDOCODE)?;

    mock.assert();

    Ok(())
}

#[test]
fn analyze_code_returns_parsed_results() -> anyhow::Result<()> {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST).path("/api/generate");
        then.status(200)
            .header("content-type", "application/json")
            .body(MOCK_VALID_RESPONSE);
    });

    let results = client(&server).analyze_code(VALID_PSEUDOCODE)?;

    mock.assert();
    assert_eq!(results.function_name(), "main", "wrong function name");
    assert_eq!(
        results.comment(),
        "Entry point of the program.",
        "wrong comment"
    );
    assert_eq!(results.variables().len(), 1, "wrong number of variables");
    if let Some(var) = results.variables().first() {
        assert_eq!(var.original_name(), "v1", "wrong original name");
        assert_eq!(var.new_name(), "counter", "wrong new name");
    }

    Ok(())
}

#[test]
fn analyze_code_with_server_error_returns_query_failed() {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(POST).path("/api/generate");
        then.status(500);
    });

    let result = client(&server).analyze_code(VALID_PSEUDOCODE);

    assert!(
        matches!(result, Err(OneiromancerError::OllamaQueryFailed(_))),
        "expected OllamaQueryFailed, got: {result:?}"
    );
}

#[test]
fn analyze_code_with_malformed_response_returns_parse_failed() {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(POST).path("/api/generate");
        then.status(200)
            .header("content-type", "application/json")
            .body(MOCK_MALFORMED_RESPONSE);
    });

    let result = client(&server).analyze_code(VALID_PSEUDOCODE);

    assert!(
        matches!(result, Err(OneiromancerError::ResponseParseFailed(_))),
        "expected ResponseParseFailed, got: {result:?}"
    );
}
