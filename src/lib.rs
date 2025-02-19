//!
//! oneiromancer - Reverse engineering assistant that uses a locally running LLM to assist with code analysis.
//! Copyright (c) 2025 Marco Ivaldi <raptor@0xdeadbeef.info>
//!
//! > "A large fraction of the flaws in software development are due to programmers not fully
//! > understanding all the possible states their code may execute in."
//! >
//! > -- John Carmack
//!
//! Oneiromancer is a research engineering assistant that uses a locally running LLM that has been
//! fine-tuned for Hex-Rays pseudo-code, to aid with code analysis.
//!
//! ## Features
//! * TODO
//!
//! ## Blog post
//! * TODO
//!
//! ## See also
//! * <https://www.atredis.com/blog/2024/6/3/how-to-train-your-large-language-model>
//! * <https://huggingface.co/AverageBusinessUser/aidapal>
//! * <https://github.com/atredispartners/aidapal>
//!
//! ## Installing
//! The easiest way to get the latest release is via [crates.io](https://crates.io/crates/oneiromancer):
//! ```sh
//! TODO
//! ```
//!
//! ## Compiling
//! Alternatively, you can build from [source](https://github.com/0xdea/oneiromancer):
//! ```sh
//! TODO
//! ```
//!
//! ## Usage
//! ```sh
//! TODO
//! ```
//!
//! ## Examples
//! TODO:
//! ```sh
//! TODO
//! ```
//!
//! TODO:
//! ```sh
//! TODO
//! ```
//!
//! ## Tested on/with
//! * TODO
//!
//! ## Changelog
//! * <https://github.com/0xdea/oneiromancer/blob/master/CHANGELOG.md>
//!
//! ## Credits
//! * TODO
//!
//! ## TODO
//! * TODO
//!

#![doc(html_logo_url = "https://raw.githubusercontent.com/0xdea/oneiromancer/master/.img/logo.png")]

use std::path::Path;

/// TODO
pub fn run(filepath: &Path) -> anyhow::Result<()> {
    // Open target source code file - TODO not needed? Open error handling should be enough
    println!("[*] Trying to analyze source code file {filepath:?}");
    if !filepath.is_file() {
        return Err(anyhow::anyhow!("invalid file path"));
    }
    // TODO
    println!("[+] Successfully opened source code file");
    println!();

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
