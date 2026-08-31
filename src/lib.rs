//! Build tools for Rust.
//!
//! **bt-rs** provides helpers for build scripts and tooling that need to
//! inspect the Rust toolchain — starting with parsing `rustc --version`
//! output into a structured [`ToolVersion`].
//!
//! Tool-specific APIs live in submodules (e.g.
//! [`rustc::compiler_version`]);
//! shared types are re-exported at the crate root.
//!
//! # Installation
//!
//! Reference in **Cargo.toml** in the usual way:
//!
//! ```toml
//! bt-rs = { version = "0.0" }
//! ```
//!
//! # Components
//!
//! ## Types
//!
//! * [`ToolVersion`] — major, minor, patch, and build metadata parsed
//!   from a tool `--version` line;
//! * [`VersionError`] — failure to run or parse a tool `--version`
//!   invocation;
//!
//! ## Modules
//!
//! * [`rustc`] — `rustc` compiler version ([`rustc::compiler_version`]);
//!
//! # Examples
//!
//! ```
//! use bt_rs::rustc::compiler_version;
//!
//! # fn main() -> Result<(), bt_rs::VersionError> {
//! let version = compiler_version()?;
//! assert!(version >= (1, 74));
//! # Ok(())
//! # }
//! ```
//!
//! See the project [README](https://github.com/synesissoftware/bt-rs) for
//! further information.

// lib.rs : bt-rs


pub(crate) mod macros;

#[rustfmt::skip]
macros::declare_and_publish!(
    common,
    ToolVersion,
    VersionError,
);

pub mod rustc;


// ///////////////////////////// end of file //////////////////////////// //
