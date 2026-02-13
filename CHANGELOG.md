# Changelog for oneiromancer

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

* Optimize release profile options.
* Update dependencies.

## [0.6.7] - 2026-02-05

### Added

* Introduce the `AUTHORS` constant.
* Add context to errors in file writing.
* Add two tests for `OllamaRequest`.

### Changed

* Improve buffered reader usage.
* Improve command line parsing, error handling, and usage messages.
* Improve tests and documentation.
* Update copyright notice.
* Update dependencies.

### Fixed

* Escape regex patterns used for variable renaming.

## [0.6.6] - 2025-12-06

### Changed

* Improve tests.
* Include `README.md` as the crate documentation to avoid writing it twice.
* Update dependencies.

## [0.6.5] - 2025-11-30

### Changed

* Improve documentation.
* Update dependencies.

## [0.6.4] - 2025-10-23

### Changed

* Improve documentation.
* Update dependencies.

## [0.6.3] - 2025-09-18

### Changed

* Update dependencies.

## [0.6.2] - 2025-06-13

### Changed

* Disable debug info to improve compile time.
* Update dependencies.

## [0.6.1] - 2025-05-23

### Added

* Add contents read permission to build CI.

### Changed

* Update dependencies.

### Fixed

* Address new clippy lints.

## [0.6.0] - 2025-04-17

### Changed

* Create an `OneiromancerConfig` struct to simplify the public API in `analyze_code` and `analyze_file`.
* Use `impl AsRef<str>` as `pseudocode` type in `analyze_code`.
* Improve documentation.

### Fixed

* Update `sccache-action` version.

## [0.5.5] - 2025-04-15

### Changed

* Improve documentation.
* Update dependencies.

### Fixed

* Allow `missing_const_for_fn` clippy lint where appropriate.

## [0.5.4] - 2025-03-29

### Added

* Add `security` category to Cargo.toml.

### Changed

* Improve documentation.
* Update dependencies.

## [0.5.3] - 2025-03-27

### Added

* Add back the `tempfile` dependency to make `run` testing more robust.

### Changed

* Use `impl AsRef<Path>` as `filepath` type in `analyze_file` to simplify the public API.
* Simplify unit tests that deal with the filesystem.
* Update dependencies.

### Removed

* Remove useless `assert!` calls in tests.

## [0.5.2] - 2025-03-26

### Added

* Add some filesystem unit tests that explicitly test for failures.

### Changed

* Refactor unit tests to remove the `tempfile` dependency.
* Update dependencies.

## [0.5.1] - 2025-03-20

### Changed

* Improve documentation.
* Update dependencies.

## [0.5.0] - 2025-03-10

### Changed

* Update dependencies.
* Add doc-tests for functions `analyze_code` and `analyze_file`.
* Add `missing_docs` lint and improve documentation.
* Avoid generating documentation for private items.
* Improve CI effectiveness and performance.

## [0.4.0] - 2025-03-06

### Changed

* Refactor code in separate `ollama` and `oneiromancer` modules.
* Improve code style.
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

[unreleased]: https://github.com/0xdea/oneiromancer/compare/v0.6.7...HEAD

[0.6.7]: https://github.com/0xdea/oneiromancer/compare/v0.6.6...v0.6.7

[0.6.6]: https://github.com/0xdea/oneiromancer/compare/v0.6.5...v0.6.6

[0.6.5]: https://github.com/0xdea/oneiromancer/compare/v0.6.4...v0.6.5

[0.6.4]: https://github.com/0xdea/oneiromancer/compare/v0.6.3...v0.6.4

[0.6.3]: https://github.com/0xdea/oneiromancer/compare/v0.6.2...v0.6.3

[0.6.2]: https://github.com/0xdea/oneiromancer/compare/v0.6.1...v0.6.2

[0.6.1]: https://github.com/0xdea/oneiromancer/compare/v0.6.0...v0.6.1

[0.6.0]: https://github.com/0xdea/oneiromancer/compare/v0.5.5...v0.6.0

[0.5.5]: https://github.com/0xdea/oneiromancer/compare/v0.5.4...v0.5.5

[0.5.4]: https://github.com/0xdea/oneiromancer/compare/v0.5.3...v0.5.4

[0.5.3]: https://github.com/0xdea/oneiromancer/compare/v0.5.2...v0.5.3

[0.5.2]: https://github.com/0xdea/oneiromancer/compare/v0.5.1...v0.5.2

[0.5.1]: https://github.com/0xdea/oneiromancer/compare/v0.5.0...v0.5.1

[0.5.0]: https://github.com/0xdea/oneiromancer/compare/v0.3.1...v0.5.0

[0.4.0]: https://github.com/0xdea/oneiromancer/compare/v0.3.1...v0.4.0

[0.3.1]: https://github.com/0xdea/oneiromancer/compare/v0.3.0...v0.3.1

[0.3.0]: https://github.com/0xdea/oneiromancer/compare/v0.2.0...v0.3.0

[0.2.0]: https://github.com/0xdea/oneiromancer/compare/v0.1.0...v0.2.0

[0.1.0]: https://github.com/0xdea/oneiromancer/releases/tag/v0.1.0
