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
/// behaviour. A `-nightly` or `-beta` token is kept on
/// [`ToolVersion::pre_release`]; the parenthetical commit and date stay
/// in `build`.
pub fn compiler_version() -> Result<ToolVersion, VersionError> {
    let output = std_process::Command::new(std_env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string()))
        .arg("--version")
        .output()
        .map_err(VersionError::InvocationFailed)?;

    let stdout = std_str::from_utf8(&output.stdout).map_err(|_| VersionError::InvalidUtf8)?;

    parse_compiler_version_output(stdout)
}

pub(crate) fn parse_compiler_version_output(stdout : &str) -> Result<ToolVersion, VersionError> {
    let version_token = stdout
        .split_whitespace()
        .nth(1)
        .ok_or(VersionError::UnrecognisedOutput)?;

    let (major, minor, patch, pre_release) = parse_version_token(version_token)?;

    let build = parse_build_token(stdout).unwrap_or_default();

    Ok(ToolVersion {
        major,
        minor,
        patch,
        build,
        pre_release,
    })
}

/// Prints `cargo:rustc-check-cfg` for `name`, and `cargo:rustc-cfg` when
/// `predicate` holds.
///
/// Does not spawn `rustc`. Pass a [`ToolVersion`] already obtained from
/// [`compiler_version`]. `name` is the caller's cfg, not a name chosen
/// by this crate.
pub fn emit_cfg(
    version : &ToolVersion,
    name : &str,
    predicate : impl FnOnce(&ToolVersion) -> bool,
) {
    for line in cargo_cfg_lines(name, predicate(version)) {
        println!("{line}");
    }
}

pub(crate) fn cargo_cfg_lines(
    name : &str,
    enabled : bool,
) -> Vec<String> {
    let mut lines = vec![format!("cargo:rustc-check-cfg=cfg({name})")];

    if enabled {
        lines.push(format!("cargo:rustc-cfg={name}"));
    }

    lines
}


// Helper functions

fn parse_version_token(version_token : &str) -> Result<(u32, u32, u32, Option<String>), VersionError> {
    let mut parts = version_token.splitn(2, '-');
    let numeric_token = parts.next().unwrap_or(version_token);
    let pre_release = parts.next().and_then(parse_pre_release);

    let mut parts = numeric_token.split('.');
    let major_str = parts.next().ok_or(VersionError::UnrecognisedOutput)?;
    let minor_str = parts.next().ok_or(VersionError::UnrecognisedOutput)?;
    let patch_str = parts.next().unwrap_or("0");

    let major = parse_component(major_str, "major")?;
    let minor = parse_component(minor_str, "minor")?;
    let patch = parse_component(patch_str, "patch")?;

    Ok((major, minor, patch, pre_release))
}

fn parse_pre_release(pre_release : &str) -> Option<String> {
    let channel = pre_release.split('.').next().unwrap_or(pre_release);

    if channel.is_empty() {
        None
    } else {
        Some(channel.to_string())
    }
}

fn parse_component(
    value : &str,
    component : &'static str,
) -> Result<u32, VersionError> {
    value.parse::<u32>().map_err(|_| {
        VersionError::InvalidVersionComponent {
            component,
            value : value.to_string(),
        }
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

        use super::super::parse_compiler_version_output;
        use crate::common::ToolVersion;


        #[test]
        fn TEST_parse_compiler_version_output_STABLE_RELEASE() {
            let version = parse_compiler_version_output("rustc 1.96.0 (ac68faa20 2026-05-25)\n").unwrap();

            assert_eq!(
                version,
                ToolVersion {
                    major :       1,
                    minor :       96,
                    patch :       0,
                    build :       "ac68faa20 2026-05-25".to_string(),
                    pre_release : None,
                }
            );
            assert!(!version.is_nightly());
            assert!(!version.is_beta());
        }

        #[test]
        fn TEST_parse_compiler_version_output_NIGHTLY_RELEASE() {
            let version = parse_compiler_version_output("rustc 1.84.0-nightly (abc123def 2024-11-01)\n").unwrap();

            assert_eq!(
                version,
                ToolVersion {
                    major :       1,
                    minor :       84,
                    patch :       0,
                    build :       "abc123def 2024-11-01".to_string(),
                    pre_release : Some("nightly".to_string()),
                }
            );
            assert!(version.is_nightly());
            assert!(version >= (1, 76));
            assert!(version >= (1, 79));
        }

        #[test]
        fn TEST_parse_compiler_version_output_BETA_RELEASE() {
            let version = parse_compiler_version_output("rustc 1.85.0-beta.1 (abc123def 2024-12-01)\n").unwrap();

            assert_eq!(
                version,
                ToolVersion {
                    major :       1,
                    minor :       85,
                    patch :       0,
                    build :       "abc123def 2024-12-01".to_string(),
                    pre_release : Some("beta".to_string()),
                }
            );
            assert!(version.is_beta());
            assert!(!version.is_nightly());
            assert!(version >= (1, 76));
        }

        #[test]
        fn TEST_parse_compiler_version_output_WITHOUT_BUILD_METADATA() {
            let version = parse_compiler_version_output("rustc 1.74.0\n").unwrap();

            assert_eq!(
                version,
                ToolVersion {
                    major :       1,
                    minor :       74,
                    patch :       0,
                    build :       String::new(),
                    pre_release : None,
                }
            );
        }

        #[test]
        fn TEST_parse_compiler_version_output_WITHOUT_PATCH_OR_BUILD_METADATA() {
            let version = parse_compiler_version_output("rustc 1.74\n").unwrap();

            assert_eq!(
                version,
                ToolVersion {
                    major :       1,
                    minor :       74,
                    patch :       0,
                    build :       String::new(),
                    pre_release : None,
                }
            );
        }
    }

    mod TEST_emit_cfg {
        #![allow(non_snake_case)]

        use super::super::cargo_cfg_lines;


        #[test]
        fn TEST_cargo_cfg_lines_WHEN_PREDICATE_HOLDS() {
            assert_eq!(
                cargo_cfg_lines("rustc_nightly", true),
                vec![
                    "cargo:rustc-check-cfg=cfg(rustc_nightly)".to_string(),
                    "cargo:rustc-cfg=rustc_nightly".to_string(),
                ],
            );
        }

        #[test]
        fn TEST_cargo_cfg_lines_WHEN_PREDICATE_DOES_NOT_HOLD() {
            assert_eq!(
                cargo_cfg_lines("rustc_1_76_or_newer", false),
                vec!["cargo:rustc-check-cfg=cfg(rustc_1_76_or_newer)".to_string()],
            );
        }
    }


    mod TEST_compiler_version {
        #![allow(non_snake_case)]

        use super::super::compiler_version;


        #[test]
        fn TEST_compiler_version_INTEGRATION() {
            let version = compiler_version().unwrap();

            assert!(version.major >= 1);
            assert!(version >= (1, 50));
        }
    }
}


// ///////////////////////////// end of file //////////////////////////// //
