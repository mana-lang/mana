<div align="center">
  <img alt="Mana logo" src="https://raw.githubusercontent.com/mana-lang/mana/main/assets/logo.png" height="117" />
</div>
<div align="center">
  <h1>Mana</h1>
  <p>Opinionated, concise and multi-paradigm Rust superset</p>
  <a href="https://crates.io/crates/mana"><img alt="Crates.io" src="https://img.shields.io/crates/v/mana"></a>
  <!--<a href="https://crates.io/crates/mana"><img alt="Crates.io" src="https://docs.rs/mana/badge.svg"></a>-->
  <a href="#"><img alt="Project status" src="https://img.shields.io/badge/status-early%20concept-5E00E2"></a>
  <br>
  <br>
</div>




Mana is an opinionated Rust superset language, currently **work in progress**, intended to improve the Rust ecosystem by, hopefully, providing enough convenient tools to help developers better reuse or reason about their own code, without the need of heavy and/or hieroglyphic macros.

Simply put, Mana **shall not be** a library dependency, similarly to TypeScript.

The hereby repository is for the official Mana command-line tool source code.

<br>

#### Summary

- [Getting started](https://github.com/mana-lang/mana#getting-started)
- [Features](https://github.com/mana-lang/mana#features)
  - [Language](https://github.com/mana-lang/mana#language)
  - [Command-line tool](https://github.com/mana-lang/mana#command-line-tool)
- [Rationale](https://github.com/mana-lang/mana#rationale)
  - [Rust RFC Graveyard](https://github.com/mana-lang/mana#rust-rfc-graveyard)
- [Contributing](https://github.com/mana-lang/mana#contributing)
- [License](https://github.com/mana-lang/mana#license)

<br>

## Getting started

You can install Mana using Cargo.

```bash
cargo install mana
```

## Features

> *Notice:* Mana is still at early concept stage. Mentioned features in this section may change or are not implemented yet.

#### Language

- Rust code ⟹ valid Mana code
- ECMAScript like `Array` type (`map`, `filter`, `reduce`, etc.)
- Date (wrapper around `std::time`)
- Classes ⟹ Interfaces
- Multi-level pseudo-inheritance
- Union types

#### Command-line tool

- Cross-platform
- Preprocessing Mana → Rust
- Benchmarking
- Add / remove dependency (∼ [`cargo-edit`](https://github.com/killercup/cargo-edit))
- Binary dev-dependencies
- Custom scripts (∼ [`cargo-make`](https://github.com/sagiegurari/cargo-make) or [`cargo-run-script`](https://github.com/JoshMcguigan/cargo-run-script))

## Rationale

Rust is a great high-performance, memory-safe, imperative and functional systems language with wide platform support, but is known for high learning curves and promoting monomorphization over ergonomics.

This inevitably leads the Servo team to using the [`Deref` polymorphism](https://github.com/rust-unofficial/patterns/blob/master/anti_patterns/deref.md) anti-pattern on a common basis to emulate pseudo inheritance on [_a lot_](https://github.com/search?q=new_inherited+-%3E+repo%3Aservo%2Fservo&type=Code) of DOM elements, which prompted them to submit the infamous [rust-lang/rfcs#349](https://github.com/rust-lang/rfcs/issues/349) RFC dating back to **2014**. 

When Richard Feldman, a member of the Elm core team, had to explain [_"Why Isn't Functional Programming the Norm?"_](https://youtu.be/QyJZzq0v7Z4) in a 2019 market dominated by Object-Oriented Programming or multi-paradigm languages :

<div align="center">
    <h1>« <em>It's complicated.</em> »</h1>
</div>

No one can blame Richard. After all, the real world is way more than _barking dogs_. <sup><strong>[1]</strong></sup>

Some powerful Rust features, yet still poisonous gifts, like (procedural) macros, can help developers by enabling metaprogramming, which, in return:

- Reduce readability, which doesn't help learning curve matters
- Make hidden transactions a common thing
- Can't just be used as _dev-dependencies_ <sup><strong>[2]</strong></sup> , while stuff like Babel, TypeScript, _etc._ are now widely used in JavaScript
- Need to be explicitly set up to compile only once <sup><strong>[3]</strong></sup>  

The Mana language project is based on the belief that the ecosystem can be improved using an intermediate high-level language built on top of — compiling into — works great with — powered by Rust; almost like F# does with C# in .NET Core; instead of stacking postponed or closed ergonomics-related RFCs [in the graveyard](https://github.com/mana-lang/mana#rust-rfc-graveyard) forever.

<strong>[1]</strong> — _Barking dogs_ is a popular theoretical example used when promoting [_Composition over Inheritance_](https://en.wikipedia.org/wiki/Composition_over_inheritance), where structs (*α*) like _Dog_ can implement traits (*β*) like _Bark_. A few Rust features, like using `T` generic type or using trait default implementations can help reducing boilerplate code, but it still implies maintaining _(α β)_ <sup><em>n</em></sup> behaviors which may scale (*n*) with the data structure complexity, especially when dealing with DOM manipulation ([_see the 20,000+ LoC TypeScript definitions_](https://github.com/microsoft/TypeScript/blob/master/lib/lib.dom.d.ts)).

<strong>[2]</strong> — Given a library, if using dependencies like `duplicate` for code duplication with substitution, possibly combined with `doc_comment` for documentation, or if using some crates with embed programming macros like `cpp` (C++), `stdweb` (JavaScript) or `inline_python` (Python), users also have to compile these crates, even if purely intended for the library itself. This leads to increased **compile times**, which is already a hot topic ([rust-lang/rust#48547](https://github.com/rust-lang/rust/issues/48547)) alongside **learning curve** and **adoption** ([Rust Survey 2019 Results](https://blog.rust-lang.org/2020/04/17/Rust-survey-2019.html))

<strong>[3]</strong> — This notably happens with `regex` crate, which compiles regular expressions on instruction call, hence why [`Regex::new()` should not be used inside loops](https://github.com/rust-lang/regex#usage-avoid-compiling-the-same-regex-in-a-loop), but instead for statically assigned values, using either `lazy_static`, `once_cell` or the currently nightly-only [`std::lazy::SyncLazy`](https://doc.rust-lang.org/std/lazy/struct.SyncLazy.html).

### _Rust RFC Graveyard_

<details>
    <summary>Click to expand</summary>
    <br>
    <ul>
        <li>Efficient code reuse (<a href="https://github.com/rust-lang/rfcs/issues/349">rust-lang/rfcs#349</a>) — <strong>2014</strong></li>
        <li><code>cargo add &#60;package&#62;</code> (<a href="https://github.com/rust-lang/cargo/issues/4">rust-lang/cargo#4</a>) — <strong>2014</strong></li>
        <li>Trait inheritance (<a href="https://github.com/rust-lang/rfcs/issues/245">rust-lang/rfcs#245</a>) — <strong>2014</strong></li>
        <li>Delegation (<a href="https://github.com/rust-lang/rfcs/issues/1406">rust-lang/rfcs#1406</a>) — <strong>2015</strong></li>
        <li>Traits fields (<a href="https://github.com/rust-lang/rfcs/issues/1546">rust-lang/rfcs#1546</a>) — <strong>2016</strong></li>
        <li>Default struct field values (<a href="https://github.com/rust-lang/rfcs/issues/1806">rust-lang/rfcs#1806</a>) — <strong>2016</strong></li>
        <li>Make commands in dev-dependencies available to run (<a href="https://github.com/rust-lang/cargo/issues/2267">rust-lang/cargo#2267</a>) — <strong>2016</strong></li>
        <li><code>Vec::remove_item</code> (<a href="https://github.com/rust-lang/rfcs/issues/40062">rust-lang/rfcs#40062</a>) — <strong>2017</strong></li>
        <li>Implement binary-only dependencies (<a href="https://github.com/rust-lang/cargo/pull/3870">rust-lang/cargo#3870</a>) — <strong>2017</strong></li>
        <li>Generic integers (<a href="https://github.com/rust-lang/rfcs/issues/2581">rust-lang/rfcs#2581</a>) — <strong>2018</strong></li>
        <li><code>throw</code> expressions (<a href="https://github.com/rust-lang/rfcs/issues/2426">rust-lang/rfcs#2426</a>) — <strong>2018</strong></li>
        <li><code>cargo add &#60;package&#62;</code> (<em>round two</em>) (<a href="https://github.com/rust-lang/cargo/issues/5586">rust-lang/cargo#5586</a>) — <strong>2018</strong></li>
        <li>Delegation (<em>round two</em>) (<a href="https://github.com/rust-lang/rfcs/issues/2393">rust-lang/rfcs#2393</a>) — <strong>2018</strong></li>
    </ul>
</details>



## Contributing

For any contribution or bug fix, a [**CONTRIBUTING**](https://github.com/mana-lang/mana/blob/main/CONTRIBUTING.md) guide is available.

Submit found issues, bugs, feature requests, questions, security concerns via the [**Issues**](https://github.com/mana-lang/mana/issues) tab.

Mana language project features are listed in [**Projects**](https://github.com/mana-lang/mana/projects) tab.

Mana language ecosystem features (integrations, showcase, etc.) are listed [in a separate project](https://github.com/orgs/mana-lang/projects/1) inside the `mana-lang` organization.

## License

© Copyright 2020 The Mana developers.

The Mana language project source code is under the [MIT License](https://github.com/mana-lang/mana/master/LICENSE). ![MIT License](https://img.shields.io/badge/License-MIT-7c2.svg)

The Mana language logo and branding assets are licensed under [CC BY-SA](https://creativecommons.org/licenses/by-sa/4.0/).

[![CC-BY-SA](https://licensebuttons.net/l/by-sa/4.0/88x31.png)](https://creativecommons.org/licenses/by-sa/4.0/)

