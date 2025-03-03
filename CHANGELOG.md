# Changelog for oneiromancer

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

* Update dependencies.

## [0.3.1] - 2025-02-28

### Changed

* Bump Rust edition to 2024 and update dependencies and CI.

## [0.3.0] - 2025-02-26

### Added

* Add `semver-checks` in CI.

### Changed

* Replace `OLLAMA_URL` with `OLLAMA_BASEURL`.
* Improve documentation.
* Improve CI speed by removing redundant tasks.

## [0.2.0] - 2025-02-24

### Added

* Add support for `OLLAMA_URL` and `OLLAMA_MODEL` environment variables.

### Changed

* Implement `OllamaResponse` parsing into an `OneiromancerResults` struct.
* Improve documentation.

## [0.1.0] - 2025-02-22

* First release to be published to [crates.io](https://crates.io/).

[unreleased]: https://github.com/0xdea/oneiromancer/compare/v0.3.1...HEAD

[0.3.1]: https://github.com/0xdea/oneiromancer/compare/v0.3.0...v0.3.1

[0.3.0]: https://github.com/0xdea/oneiromancer/compare/v0.2.0...v0.3.0

[0.2.0]: https://github.com/0xdea/oneiromancer/compare/v0.1.0...v0.2.0

[0.1.0]: https://github.com/0xdea/oneiromancer/releases/tag/v0.1.0
