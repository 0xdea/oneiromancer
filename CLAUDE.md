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

Tests split into two categories: the `format_description` and `apply_renames` helpers are tested without Ollama (pure logic); everything else (`ollama_request_*`, `analyze_*`, `run_*`) requires a live Ollama instance. Test fixtures live in `tests/data/` (`hello.c`, `empty.c`).

## Architecture

Single Rust crate (edition 2024) that exposes both a binary and a public library API.

**Entry points:**
- `src/main.rs` — CLI: reads one `.c` file argument, calls `oneiromancer::run()`
- `src/lib.rs` — public API: `run()`, `analyze_code()`, `analyze_file()`

**Module responsibilities:**
- `src/oneiromancer.rs` — `OneiromancerConfig` (URL + model, reads env vars), `OneiromancerResults` (deserialized LLM output: function name, comment, variable list), `OneiromancerError`
- `src/ollama.rs` — `OllamaRequest`/`OllamaResponse`: serializes the prompt, POSTs to `/api/generate` with `stream: false, format: "json"`, parses response back to `OneiromancerResults`

**Private helpers in `src/lib.rs`:**
- `format_description(results)` — formats a Phrack-style `/* ... */` block comment, wrapping to 76 columns
- `apply_renames(pseudocode, variables)` — applies whole-word regex substitutions; assumes LLM-suggested names are collision-safe so order cannot corrupt later replacements

**Data flow:**
```
CLI arg (.c file)
  → lib::run()
    → analyze_code(pseudocode, config)
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
