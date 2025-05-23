# oneiromancer

[![](https://img.shields.io/github/stars/0xdea/oneiromancer.svg?style=flat&color=yellow)](https://github.com/0xdea/oneiromancer)
[![](https://img.shields.io/crates/v/oneiromancer?style=flat&color=green)](https://crates.io/crates/oneiromancer)
[![](https://img.shields.io/crates/d/oneiromancer?style=flat&color=red)](https://crates.io/crates/oneiromancer)
[![](https://img.shields.io/badge/twitter-%400xdea-blue.svg)](https://twitter.com/0xdea)
[![](https://img.shields.io/badge/mastodon-%40raptor-purple.svg)](https://infosec.exchange/@raptor)
[![build](https://github.com/0xdea/oneiromancer/actions/workflows/build.yml/badge.svg)](https://github.com/0xdea/oneiromancer/actions/workflows/build.yml)

> "A large fraction of the flaws in software development are due to programmers not fully understanding all the possible
> states their code may execute in." -- John Carmack

> "Can it run Doom?" -- <https://canitrundoom.org/>

Oneiromancer is a reverse engineering assistant that uses a locally running LLM that has been fine-tuned for Hex-Rays
pseudo-code to aid with code analysis. It can analyze a function or a smaller code snippet, returning a high-level
description of what the code does, a recommended name for the function, and variable renaming suggestions, based on the
results of the analysis.

![](https://raw.githubusercontent.com/0xdea/oneiromancer/master/.img/screen01.png)

## Features

* Cross-platform support for the fine-tuned LLM [aidapal](https://huggingface.co/AverageBusinessUser/aidapal) based on
  `mistral-7b-instruct`.
* Easy integration with the pseudo-code extractor [haruspex](https://github.com/0xdea/haruspex) and popular IDEs.
* Code description, recommended function name, and variable renaming suggestions are printed on the terminal.
* Improved pseudo-code of each analyzed function is saved in a separate file for easy inspection.
* External crates can invoke `analyze_code` or `analyze_file` to analyze pseudo-code and then process analysis results.

## Blog post

* <https://security.humanativaspa.it/aiding-reverse-engineering-with-rust-and-a-local-llm>

## See also

* <https://www.atredis.com/blog/2024/6/3/how-to-train-your-large-language-model>
* <https://huggingface.co/AverageBusinessUser/aidapal>
* <https://github.com/atredispartners/aidapal>
* <https://plugins.hex-rays.com/atredispartners/aidapal>

## Installing

The easiest way to get the latest release is via [crates.io](https://crates.io/crates/oneiromancer):

```sh
cargo install oneiromancer
```

To install as a library, run the following command in your project directory:

```sh
cargo add oneiromancer
```

## Compiling

Alternatively, you can build from [source](https://github.com/0xdea/oneiromancer):

```sh
git clone https://github.com/0xdea/oneiromancer
cd oneiromancer
cargo build --release
```

## Configuration

1. Download and install [Ollama](https://ollama.com/).
2. Download the fine-tuned weights and the Ollama modelfile from [Hugging Face](https://huggingface.co/):
   ```sh
   wget https://huggingface.co/AverageBusinessUser/aidapal/resolve/main/aidapal-8k.Q4_K_M.gguf
   wget https://huggingface.co/AverageBusinessUser/aidapal/resolve/main/aidapal.modelfile
   ```
3. Configure Ollama by running the following commands within the directory in which you downloaded the files:
   ```sh
   ollama create aidapal -f aidapal.modelfile
   ollama list
   ```

## Usage

1. Run oneiromancer as follows:
   ```sh
   export OLLAMA_BASEURL=custom_baseurl # if not set, the default will be used
   export OLLAMA_MODEL=custom_model # if not set, the default will be used
   oneiromancer <target_file>.c
   ```
2. Find the extracted pseudo-code of each decompiled function in `<target_file>.out.c`:
   ```sh
   vim <target_file>.out.c
   code <target_file>.out.c
   ```

*Note: for best results, you shouldn't submit for analysis to the LLM more than one function at a time.*

## Tested on

* Apple macOS Sequoia 15.2 with Ollama 0.5.11
* Ubuntu Linux 24.04.2 LTS with Ollama 0.5.11
* Microsoft Windows 11 23H2 with Ollama 0.5.11

## Changelog

* [CHANGELOG.md](CHANGELOG.md)

## Credits

* Chris Bellows (@AverageBusinessUser) at Atredis Partners for his fine-tuned LLM `aidapal` <3

## TODO

* Improve output file handling with versioning and/or an output directory.
* Implement other features of the IDAPython `aidapal` IDA Pro plugin (e.g., context).
* Integrate with [haruspex](https://github.com/0xdea/haruspex) and [idalib](https://github.com/binarly-io/idalib).
* Use custom types in the public API and implement a provider abstraction.
* Implement a "minority report" protocol (i.e., make three queries and select the best responses).
* Investigate other use cases for the `aidapal` LLM and implement a modular architecture to plug in custom LLMs.
