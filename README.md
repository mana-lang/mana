<div align="center">
  <img alt="Mana logo" src="https://raw.githubusercontent.com/mana-lang/mana/main/assets/logo.png" height="117" />
</div>
<div align="center">
  <h1>Mana</h1>
  <p>Opinionated, concise and multi-paradigm Rust superset</p>
  <a href="https://crates.io/crates/mana"><img alt="Crates.io" src="https://img.shields.io/crates/v/mana"></a>
  <!--<a href="https://crates.io/crates/mana"><img alt="Crates.io" src="https://docs.rs/mana/badge.svg"></a>-->
  <a href="#"><img alt="Windows Status" src="https://img.shields.io/github/workflow/status/mana-lang/mana/build?logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAB4AAAAeCAYAAAA7MK6iAAABHUlEQVRIS+1Xi23CMBB9NwHdgG4AG7RswAZ0E2ATVmCCZgTYADZgg1c9dAnBUmLnQyokW4qi6Ox7d8/23Yvhn4ZNiUtyCeBiZrfRgd35DMA3gA8AAvv0R3muzKzoDUxSjhfuvA4SI7EdOIhczkrnRzPbkWQMocH+ACYpKrY1SvTdNPZjAiub38QMMnAiUdW0pz3OVLfRlw/XoMOlgvGT6KFQrSW5a5hfRPycqibhlWvTsuCiruL2K4Cb1+nEWJ+mnevAna4TAGWVWunC4Prf4wzccaP7U53b4ltSrcp1cOEmATdNkwhRvJKVklRvSVRJ1S8A43Wnjnt0n05yHWhmBSc9HRvDdHWTd9fbMoeCfu5rXgPclu5Lf2FiPJf2P5TA7h8vNz6jAAAAAElFTkSuQmCC&label=Windows"></a>
  <a href="#"><img alt="Linux Status" src="https://img.shields.io/github/workflow/status/mana-lang/mana/build?logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAB4AAAAeCAYAAAA7MK6iAAACVUlEQVRIS8WXgVEUQRBF/49AiUCMQIlAiUCJAMlAIhAjUCJQIsAMhAiACIQMIIK23lbPVt/UHburnHbV1VXdzPTv/vO7e876T+Zt4UbEB0mHkm4lHdu+r1hbAY6Ij5K+FKBr23v/AvhC0puOzT3b1+23P8o4It5LeidpV9LbAgDgsaSTXK/YO5XuRcAR8VwSd1dp7GVyJumrJIJ4loufbRPMaLOAExCwOxxEBIJ58YgwuWMCeC3pvlK8iOqI4G5e5aGX6fBc0g2OU7nQDlDLkuD4YGQMA/MzjojvWRbt0IXt/YjYtd0cjw6zjKC6BcAawSGucf+jVEcEwvnZUXpkm2A2Wl4NGTaW2DsEPIvqjmLO0AjIZtISnCuqWthvlG/MOCK4r6uCcGm7ls4c8J6xM9tUhVaA836IEEVSOrUkDmz/mETrNnQVgMJ3VoDXiOggVfpp2GjPKr0+sIigfgcfaUMjGZyhUEm/ukOX2YEQF/XLnsWWvutZ+vZs4Fvb1O9iS+AqsJsROLPu63X7VLc0UlzQQhCIC5r5xp5CXA+2B39LymmlAczhfE0DWl9OaxRZezTLSxsIfaAKa7qB5L3/TcvkmmhCzVYa0GRtrqnvwcHEkGCENm0A/EAQs4dEEd2mscjv/VhsgHdlLJ4sHotJOc4YDtTznIfAkSTaK9MJJY9vrZbMJNVVcDlxeF3UFthr8jTLsZbiqW3OjbYIuFDPY48Piq2vSdosAE/72JtTu3k123neTgWQHfBb2Ud/rqW1uXNNOZ9aT3CGPsJC1dv/CzMVFOu/AZB7Bi7zlZ76AAAAAElFTkSuQmCC&label=Ubuntu"></a>
  <a href="#"><img alt="MacOS Status" src="https://img.shields.io/github/workflow/status/mana-lang/mana/build?logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAB4AAAAeCAYAAAA7MK6iAAABrUlEQVRIS92Wi03DQBBEZyoAOiAdhAoIFUAqACoAKgAqgA6ACiAVEDqgA0IFQAWDBp2R49z5bHwBiZUsRcr63u2s90P8kbEUV9I2gH0AryQfcucWAUu6AHAeYGckr9cOlnQL4LAG2iH5vFawpAMA9zXIE8lJDur/B0ktaQ5gN4A+AEy6RFsCrJ9AS4CdX0f9QPK9i8SVT6vUkjYBnFjC8CxqoJkPCWVkH+fbJWWr/C5J+veKJcHhw7kBYHjMHKGfCpbycXlZmSWLgiU5wsc+0mV8t5qpWAEHeV9aIu17n+NOEUs6BXDV9/SEv3PsrpbPcaM2h/Dds5P5j0ld1eYQqN+9I3mUOmSd4KTMvsy/BM9IuqlELRaxm8LG0ASH90edO5ckbw/eJEqYe/i0azmVrGMz3S7dNpeGSKpzvZUIt3bGguSofmaqV5eU27yVD+23hsQeSc/tb2sbi6WijpZVG9hz2EN8SGl5DxvHSiq3gTS3yL7f3DS13Ge3TElu9N5E6uZInDPvz+OwFjWVic7h6pAs2I5hr/IFLP88FkVYlXyJr9pNdaxe4L76dvH/BL1gpR+vRyf1AAAAAElFTkSuQmCC&label=MacOS"></a>
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
- Classes ⟹ Interfaces
- Multi-level pseudo-inheritance
- Expanded standard library
  - [x] Lightweight random number generator
  - [ ] ECMAScript like `Array` type (`map`, `filter`, `reduce`, etc.)
  - [ ] Date (wrapper around `std::time`)

#### Command-line tool

- Cross-platform
- Preprocessing Mana → Rust
- Benchmarking
- Binary dev-dependencies
- Custom scripts

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
        <li>Lint configuration in <code>Cargo.toml</code> (<a href="https://github.com/rust-lang/cargo/issues/5034">rust-lang/cargo#5034</a> — <strong>2018</strong></li>
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

