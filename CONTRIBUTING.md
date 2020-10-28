# Contributing to Mana

First of all, thank you for your interest into the Mana language project! :pray:

The hereby document will help you getting starting contributing to the open-source Mana language project and provide useful links.



## Philosophy

Reading the [rationale](https://github.com/mana-lang/mana#rationale) first is recommended, in order to better understand the philosophy behind Mana.

The Rust language enforces imperative paradigm, while adhering to many of the tenets of functional programming. The whole thing fits well for systems, which is the primary focus of the language. It is thus not surprising that people are facing frictions when using Rust in different enterprise-grade use cases, than systems.

The Rust team has a strong preference for focusing on stability and extending the standard library and Cargo only with stable stuff and making them as small as possible, while still letting anyone create a [third-party subcommand](https://github.com/rust-lang/cargo/wiki/Third-party-cargo-subcommands) or a (procedural) [macro](https://doc.rust-lang.org/book/ch19-06-macros.html), at the cost of leaving the community implement missing key wrappers (_e.g._ error handling, async...) <sup><strong>[1]</strong></sup> and standardizing hieroglyphing the Rust language, thus increasing the learning curve.

#### Design goals

- Mana **shall be** easy to use
- Mana **shall be** easy to learn
- Mana **shall work** for Windows (MSVC & GNU), MacOS and Linux
- Mana **shall be** a single dev-dependency
- Mana **shall not require** additional dependency
- Mana **shall not require** procedural macros to be enabled
- Mana **shall not require** to use nightly Rust
- Mana **shall be** multi-paradigm
- Mana **shall be** compiled to Rust
- Mana **shall be** easy to debug
- Mana **shall provide** classes and interfaces
- Mana **shall be** able to extend classes
- Mana **shall provide** binary dev-dependencies
- Mana **shall provide** custom scripts (∼ `npm run <script>`)
- Mana **shall not set up** Git hooks
- Mana **shall be** easy to opt-out

<br>

<strong>[1]</strong> — Not all community crates use the same standards, but could _eventually_ get adopted in standard library after getting some reviews and reworks in order to meet the expected standards. When `failure` error crate superseded `error-chain` and then [got deprecated in 2020](https://github.com/rust-lang-nursery/failure/pull/347), this notably impacted `tokio`, `diesel`, `tauri`, `openethereum` and `wrangler`. Of course it could be replaced by yet another existing wrapper like `anyhow`, `snafu`, `thiserror`, `eyre`...

> _At this rate we'll soon have more Rust error crates than JavaScript web frameworks. Go, Rust! :)_
> @**AdditionalQuestion** — [Reddit](https://www.reddit.com/r/rust/comments/dfkwfo/announcing_thiserror_a_convenient_modern/f341fyz/?utm_source=reddit&utm_medium=web2x&context=3)

<br>

## Getting started

#### Prerequisite(s)

- Rust (_obviously.._)

#### Setup

For any contribution / bug fix, fork the `mana-lang/mana` repository and set up a new branch.

```bash
git clone https://github.com/<USER>/mana.git
```

The `mana-lang/mana` repository is organized as a Cargo workspace, see the root `Cargo.toml` for more details.

Compile the workspace members by using Cargo.

```bash
cargo build
```

Alternatively, you can compile and run the `mana` binary.

```bash
cargo run
```

Run tests for the entire workspace.

```bash
cargo test
```

