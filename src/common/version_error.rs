// version_error.rs : src/common

use std::{
    error as std_error,
    fmt as std_fmt,
    io as std_io,
};


/// Failure to obtain or parse a tool's `--version` output.
#[derive(Debug)]
pub enum VersionError {
    /// Failed to execute the tool with `--version`.
    InvocationFailed(std_io::Error),
    /// Tool `--version` stdout was not valid UTF-8.
    InvalidUtf8,
    /// Output did not contain a recognisable version token.
    UnrecognisedOutput,
    /// A major, minor, or patch component was not a valid unsigned integer.
    InvalidVersionComponent {
        component : &'static str,
        value :     String,
    },
}


// Trait implementations

impl std_fmt::Display for VersionError {
    fn fmt(
        &self,
        f : &mut std_fmt::Formatter<'_>,
    ) -> std_fmt::Result {
        match self {
            Self::InvocationFailed(e) => {
                write!(f, "failed to execute tool `--version`: {e}")
            },
            Self::InvalidUtf8 => {
                write!(f, "tool `--version` stdout was not valid UTF-8")
            },
            Self::UnrecognisedOutput => {
                write!(f, "tool `--version` output was not recognised")
            },
            Self::InvalidVersionComponent { component, value } => {
                write!(
                    f,
                    "invalid {component} component in tool `--version` output: {value:?}"
                )
            },
        }
    }
}

impl std_error::Error for VersionError {
    fn source(&self) -> Option<&(dyn std_error::Error + 'static)> {
        match self {
            Self::InvocationFailed(e) => Some(e),
            _ => None,
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //
