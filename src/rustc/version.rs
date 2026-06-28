// version.rs : src/rustc

use crate::common::{
    ToolVersion,
    VersionError,
};

use std::{
    env as std_env,
    process as std_process,
    str as std_str,
};


/// Returns the version of the `rustc` compiler on the current `PATH`.
///
/// The `RUSTC` environment variable is honoured when set, matching Cargo's
/// behaviour.
pub fn compiler_version() -> Result<ToolVersion, VersionError> {
    let output = std_process::Command::new(
        std_env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string()),
    )
    .arg("--version")
    .output()
    .map_err(VersionError::InvocationFailed)?;

    let stdout = std_str::from_utf8(&output.stdout).map_err(|_| VersionError::InvalidUtf8)?;

    parse_compiler_version_output(stdout)
}

pub(crate) fn parse_compiler_version_output(
    stdout : &str,
) -> Result<ToolVersion, VersionError> {
    let version_token = stdout.split_whitespace().nth(1).ok_or(VersionError::UnrecognisedOutput)?;

    let (major, minor, patch) = parse_version_token(version_token)?;

    let build = parse_build_token(stdout).unwrap_or_default();

    Ok(ToolVersion {
        major,
        minor,
        patch,
        build,
    })
}


// Helper functions

fn parse_version_token(version_token : &str) -> Result<(u32, u32, u32), VersionError> {
    let numeric_token = version_token.split('-').next().unwrap_or(version_token);

    let mut parts = numeric_token.split('.');
    let major_str = parts.next().ok_or(VersionError::UnrecognisedOutput)?;
    let minor_str = parts.next().ok_or(VersionError::UnrecognisedOutput)?;
    let patch_str = parts.next().unwrap_or("0");

    let major = parse_component(major_str, "major")?;
    let minor = parse_component(minor_str, "minor")?;
    let patch = parse_component(patch_str, "patch")?;

    Ok((major, minor, patch))
}

fn parse_component(
    value : &str,
    component : &'static str,
) -> Result<u32, VersionError> {
    value.parse::<u32>().map_err(|_| VersionError::InvalidVersionComponent {
        component,
        value : value.to_string(),
    })
}

fn parse_build_token(stdout : &str) -> Option<String> {
    let open = stdout.find('(')?;
    let close = stdout.rfind(')')?;

    if close <= open {
        return None;
    }

    Some(stdout[open + 1..close].trim().to_string())
}


#[cfg(test)]
mod tests {


    mod TEST_parse_compiler_version_output {
        #![allow(non_snake_case)]

        use super::super::{
            parse_compiler_version_output,
        };
        use crate::common::ToolVersion;


        #[test]
        fn TEST_parse_compiler_version_output_STABLE_RELEASE() {
            let version = parse_compiler_version_output(
                "rustc 1.96.0 (ac68faa20 2026-05-25)\n",
            )
            .unwrap();

            assert_eq!(
                version,
                ToolVersion {
                    major : 1,
                    minor : 96,
                    patch : 0,
                    build : "ac68faa20 2026-05-25".to_string(),
                }
            );
        }

        #[test]
        fn TEST_parse_compiler_version_output_NIGHTLY_RELEASE() {
            let version = parse_compiler_version_output(
                "rustc 1.84.0-nightly (abc123def 2024-11-01)\n",
            )
            .unwrap();

            assert_eq!(
                version,
                ToolVersion {
                    major : 1,
                    minor : 84,
                    patch : 0,
                    build : "abc123def 2024-11-01".to_string(),
                }
            );
        }

        #[test]
        fn TEST_parse_compiler_version_output_WITHOUT_BUILD_METADATA() {
            let version = parse_compiler_version_output("rustc 1.74.0\n").unwrap();

            assert_eq!(
                version,
                ToolVersion {
                    major : 1,
                    minor : 74,
                    patch : 0,
                    build : String::new(),
                }
            );
        }

        #[test]
        fn TEST_parse_compiler_version_output_WITHOUT_PATCH_OR_BUILD_METADATA() {
            let version = parse_compiler_version_output("rustc 1.74\n").unwrap();

            assert_eq!(
                version,
                ToolVersion {
                    major : 1,
                    minor : 74,
                    patch : 0,
                    build : String::new(),
                }
            );
        }
    }


    mod TEST_compiler_version {
        #![allow(non_snake_case)]

        use super::super::{
            compiler_version,
        };


        #[test]
        fn TEST_compiler_version_INTEGRATION() {
            let version = compiler_version().unwrap();

            assert!(version.major >= 1);
            assert!(version >= (1, 50));
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //
