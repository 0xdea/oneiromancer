# oneiromancer

[![](https://img.shields.io/github/stars/0xdea/oneiromancer.svg?style=flat&color=yellow)](https://github.com/0xdea/oneiromancer)
[![](https://img.shields.io/crates/v/oneiromancer?style=flat&color=green)](https://crates.io/crates/oneiromancer)
[![](https://img.shields.io/crates/d/oneiromancer?style=flat&color=red)](https://crates.io/crates/oneiromancer)
[![](https://img.shields.io/badge/twitter-%400xdea-blue.svg)](https://twitter.com/0xdea)
[![](https://img.shields.io/badge/mastodon-%40raptor-purple.svg)](https://infosec.exchange/@raptor)
[![build](https://github.com/0xdea/oneiromancer/actions/workflows/build.yml/badge.svg)](https://github.com/0xdea/oneiromancer/actions/workflows/build.yml)
[![doc](https://github.com/0xdea/oneiromancer/actions/workflows/doc.yml/badge.svg)](https://github.com/0xdea/oneiromancer/actions/workflows/doc.yml)

> "A large fraction of the flaws in software development are due to programmers not fully understanding all the possible
> states their code may execute in."
>
> -- John Carmack

Oneiromancer is a research engineering assistant that uses a locally running LLM that has been fine-tuned for Hex-Rays
pseudo-code, to aid with code analysis. It can analyze a function or a smaller code snippet, returning a high-level
description of what the code does, a suggested name for the function, and variable renaming suggestions, based on the
results of the analysis.

TODO: add screenshot

![](https://raw.githubusercontent.com/0xdea/oneiromancer/master/.img/screen01.png)

## Features

* Easy integration with pseudo-code extractor [haruspex](https://github.com/0xdea/haruspex) and popular IDEs.
* Code description, suggested function name, and variable renaming suggestions are printed to the terminal.
* Modified pseudo-code of each analyzed function is stored in a separated file for easy inspection.
* External crates can invoke `analyze_file` or `analyze_code` to analyze pseudo-code and process analysis results.

## Blog post

* TODO

## See also

* <https://www.atredis.com/blog/2024/6/3/how-to-train-your-large-language-model>
* <https://huggingface.co/AverageBusinessUser/aidapal>
* <https://github.com/atredispartners/aidapal>

## Installing

The easiest way to get the latest release is via [crates.io](https://crates.io/crates/oneiromancer):

```sh
TODO
```

## Compiling

Alternatively, you can build from [source](https://github.com/0xdea/oneiromancer):

```sh
TODO
```

## Usage

```sh
TODO
```

## Examples

TODO:

```sh
TODO
```

TODO:

```sh
TODO
```

## Tested on/with

* TODO

## Changelog

* [CHANGELOG.md](CHANGELOG.md)

## Credits

* Chris (@AverageBusinessUser) at Atredis Partners for his fine-tuned LLM `aidapal` <3

## TODO

* Extensive testing on the `windows` target family to confirm that it works properly even in edge cases.
* Implement other features of the IDAPython `aidapal` IDA Pro plugin (e.g., context).
* Implement a "minority report" protocol (i.e., make three queries and select the best ones).
* Integrate with [haruspex](https://github.com/0xdea/haruspex) and [idalib](https://github.com/binarly-io/idalib).
* Investigate other use cases for the `aidapal` LLM, implement a modular LLM architecture to plug in custom local LLMs.
* Consider pulling in [ollama-rs](https://lib.rs/crates/ollama-rs) or a similar crate for more advanced features.
* Consider improving variable renaming logic with a custom C parser...
