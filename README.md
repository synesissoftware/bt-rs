# bt-rs <!-- omit in toc -->

Build tools, for Rust


![Language](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![GitHub release](https://img.shields.io/github/v/release/synesissoftware/bt-rs.svg)](https://github.com/synesissoftware/bt-rs/releases/latest)
[![Last Commit](https://img.shields.io/github/last-commit/synesissoftware/bt-rs)](https://github.com/synesissoftware/bt-rs/commits/master)
[![Crates.io](https://img.shields.io/crates/v/bt-rs.svg)](https://crates.io/crates/bt-rs)
![MSRV](https://img.shields.io/badge/MSRV-1.74-lightgrey)
[![CI](https://github.com/synesissoftware/bt-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/synesissoftware/bt-rs/actions/workflows/ci.yml)
[![docs.rs](https://docs.rs/bt-rs/badge.svg)](https://docs.rs/bt-rs)


## Table of Contents <!-- omit in toc -->

- [Introduction](#introduction)
- [Developer Experience](#developer-experience)
- [Installation](#installation)
- [Components](#components)
  - [Constants](#constants)
  - [Enumerations](#enumerations)
  - [Features](#features)
  - [Functions](#functions)
  - [Macros](#macros)
  - [Structures](#structures)
    - [`ToolVersion`](#toolversion)
  - [Traits](#traits)
- [Examples](#examples)
- [Project Information](#project-information)
  - [Where to get help](#where-to-get-help)
  - [Contribution guidelines](#contribution-guidelines)
  - [Minimum Supported Rust Version (MSRV)](#minimum-supported-rust-version-msrv)
  - [Dependencies](#dependencies)
    - [Efferent (fan-out)](#efferent-fan-out)
      - [Development Dependencies](#development-dependencies)
    - [Afferent (fan-in)](#afferent-fan-in)
  - [Related projects](#related-projects)
  - [License](#license)


## Introduction

**bt-rs** provides **b**uild **t**ools for **R**u**s**t projects. The first facility parses `rustc --version` into a structured [`ToolVersion`](https://docs.rs/bt-rs/latest/bt_rs/struct.ToolVersion.html), so build scripts and tooling can compare the compiler without scraping the text themselves.


## Developer Experience

**Need**: Rust **1.74** or newer (the declared MSRV). The crate has no runtime dependencies.

**Build** and **try**:

```bash
cargo test --locked
cargo run --example versions
```

**Simplest use**:

```rust
use bt_rs::rustc::compiler_version;

fn main() -> Result<(), bt_rs::VersionError> {
    let version = compiler_version()?;

    assert!(version >= (1, 74));
    Ok(())
}
```

`compiler_version()` honours the `RUSTC` environment variable when it is set, matching Cargo.

**Read next**: [EXAMPLES.md](./EXAMPLES.md) and the [docs.rs](https://docs.rs/bt-rs) API reference.


## Installation

Reference in **Cargo.toml** in the usual way:

```toml
bt-rs = { version = "0.0" }
```


## Components

### Constants

No public constants are defined at this time.


### Enumerations

`VersionError` is the failure type for obtaining or parsing a tool `--version` line:

* `InvocationFailed` — the tool could not be executed;
* `InvalidUtf8` — stdout was not valid UTF-8;
* `UnrecognisedOutput` — the text did not contain a recognisable version token;
* `InvalidVersionComponent` — a major, minor, or patch field was not an unsigned integer;


### Features

| Name             | Effect                                                                 | Is `"default"`? |
| ---------------- | ---------------------------------------------------------------------- | --------------- |
| `"null-feature"` | Has no effect; useful for simplifying driver scripts                   | **No**          |


### Functions

`rustc::compiler_version()` runs `rustc --version` (or `$RUSTC --version`) and returns `Result<ToolVersion, VersionError>`.


### Macros

No public macros are defined at this time.


### Structures


#### `ToolVersion`

```rust
#[derive(Clone)]
#[derive(Debug)]
pub struct ToolVersion {
    pub major : u32,
    pub minor : u32,
    pub patch : u32,
    pub build : String,
}
```

Two `ToolVersion` values compare in the usual order: major, then minor, then patch, then the build string.

The same order works against a tuple, so a check can name only the prefix it cares about. Fields the tuple omits are treated as zero, and an omitted build is treated as empty:

```rust
let version = compiler_version()?;

assert!(version >= (1, 74));       // 1.74.0, empty build
assert!(version >= (1, 74, 0));
assert!(version == (1, 74, 0, ""));
```

`version >= (1, 74)` means at least 1.74.0. A compiler reporting `1.74.1` still satisfies `(1, 74)`. A compiler reporting `1.73.9` does not.


### Traits

No public traits are defined at this time.


## Examples

| Name         | Source                                          | Summary                                      |
| ------------ | ----------------------------------------------- | -------------------------------------------- |
| **versions** | [Source](./examples/versions/main.rs)           | Prints the crate name and package version.  |

See [EXAMPLES.md](./EXAMPLES.md). The crate-level rustdoc includes a compiling `compiler_version()` example.


## Project Information

### Where to get help

[GitHub Page](https://github.com/synesissoftware/bt-rs "GitHub Page")


### Contribution guidelines

Defect reports, feature requests, and pull requests are welcome on https://github.com/synesissoftware/bt-rs.


### Minimum Supported Rust Version (MSRV)

The declared Minimum Supported Rust Version (MSRV) for **bt-rs** is **1.74**.

This MSRV guarantee applies to the library crate itself, its runtime dependencies (`[dependencies]`), and its build dependencies (`[build-dependencies]`). Downstream consumers compiling this crate as a dependency are guaranteed that it builds cleanly on the declared MSRV toolchain.

Development dependencies (`[dev-dependencies]`, such as benchmarking frameworks like **criterion**) may require newer Rust toolchains for local development or performance testing. These dev-dependencies are never fetched or compiled by downstream consumers and do not affect the library's MSRV guarantee.


### Dependencies


#### Efferent (fan-out)

Libraries upon which **bt-rs** depends:

None at this time.


##### Development Dependencies

None at this time.


#### Afferent (fan-in)

Projects that depend on **bt-rs**:

None at this time.


### Related projects

None at this time.


### License

**bt-rs** is released under the 3-clause BSD license. See [LICENSE](./LICENSE) for details.


<!-- ########################### end of file ########################### -->
