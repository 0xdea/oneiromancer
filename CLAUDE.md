# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**oneiromancer** is a reverse engineering assistant (CLI + library) that analyzes Hex-Rays pseudocode using the fine-tuned `aidapal` LLM running locally via Ollama. It produces function descriptions, suggested function names, variable renames, and improved pseudocode.

## Commands

```bash
# Build
cargo build
cargo build --release

# Lint and format (must pass CI)
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings

# Tests (require a running Ollama instance; see below)
cargo test
```

## Development Requirements

Tests and the binary itself require a running [Ollama](https://ollama.com) instance (v0.21.2+) with the `aidapal` model loaded.

Configuration via environment variables:
- `OLLAMA_BASEURL` — Ollama server URL (default: `http://127.0.0.1:11434`)
- `OLLAMA_MODEL` — model name (default: `aidapal`)

Most tests require a live Ollama instance and are marked `#[ignore = "requires a live Ollama instance"]`; run `cargo test` to execute only non-Ollama tests. Test fixtures live in `tests/data/` (`hello.c`, `empty.c`).

Tests are organised into three locations:
- `src/lib.rs` `mod tests::helpers` — pure logic tests for `format_description` and `apply_renames` (no Ollama)
- `src/lib.rs` `mod tests::api` — `analyze_*` and `run_*` tests (most need Ollama)
- `src/ollama.rs` `mod tests` — `ollama_request_*` tests (most need Ollama)
- `tests/integration.rs` — mock-server integration tests using `httpmock` (no Ollama)

## Architecture

Single Rust crate (edition 2024) that exposes both a binary and a public library API.

**Entry points:**
- `src/main.rs` — CLI: reads one `.c` file argument, calls `oneiromancer::run()`
- `src/lib.rs` — public API: re-exports `Oneiromancer`, `OneiromancerError`, `OneiromancerResults`, `Variable`; defines `run()` and private helpers

**Module responsibilities:**
- `src/oneiromancer.rs` — `Oneiromancer` struct (builder pattern: `with_baseurl`, `with_model`; methods: `analyze_code`, `analyze_file`), `OneiromancerResults`, `OneiromancerError`, `Variable`; reads `OLLAMA_BASEURL`/`OLLAMA_MODEL` env vars in `Default` impl
- `src/ollama.rs` — `OllamaRequest`/`OllamaResponse`: serializes the prompt, POSTs to `/api/generate` with `stream: false, format: "json"`, parses response back to `OneiromancerResults`

**Private helpers in `src/lib.rs`:**
- `format_description(results)` — formats a Phrack-style `/* ... */` block comment, wrapping to 76 columns
- `apply_renames(pseudocode, variables)` — applies whole-word regex substitutions; assumes LLM-suggested names are collision-safe so order cannot corrupt later replacements

**Data flow:**
```
CLI arg (.c file)
  → lib::run()
    → Oneiromancer::new().analyze_code(pseudocode)
      → OllamaRequest::send() → POST /api/generate
      → OllamaResponse::parse() → OneiromancerResults
    → format_description()   (Phrack-style, 76-col wrap)
    → apply_renames()        (whole-word regex substitution)
    → write improved pseudocode to <filename>.out.c
```

## Workspace Lint Policy

`Cargo.toml` enables strict workspace lints including `missing_docs`, plus clippy restriction lints. `unwrap()`, `expect()`, and `panic!()` in library code will generate warnings — use `?` and `thiserror`/`anyhow` instead.

## Release Profile

Release builds use LTO, opt-level 3, `panic = "abort"`, and stripped binaries. Do not assume backtraces are available in release mode.
