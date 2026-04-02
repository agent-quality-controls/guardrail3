use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ToolchainRootInput;

const ID: &str = "RS-TOOLCHAIN-03";

pub fn check(input: &ToolchainRootInput<'_>, results: &mut Vec<CheckResult>) {
    if input.legacy_toolchain_rel.is_some() {
        return;
    }

    let Some(rel) = input.toolchain_toml_rel else {
        return;
    };
    let Some(parsed) = input.parsed else {
        return;
    };

    let channel = parsed
        .get("toolchain")
        .and_then(|value| value.get("channel"))
        .and_then(toml::Value::as_str);

    let Some(channel) = channel else {
        return;
    };
    let Some(toolchain_version) = parse_pinned_stable_version(channel) else {
        return;
    };

    if input.cargo_toml_rel.is_none() {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "Cargo.toml missing blocks MSRV check".to_owned(),
            "Cargo.toml is required to compare pinned toolchain against declared MSRV.".to_owned(),
            Some(input.cargo_rel_path.to_owned()),
            None,
            false,
        ));
        return;
    }

    if let Some(parse_error) = input.cargo_parse_error {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "Cargo.toml parse error blocks MSRV check".to_owned(),
            format!("Invalid root Cargo.toml: {parse_error}"),
            Some(input.cargo_rel_path.to_owned()),
            None,
            false,
        ));
        return;
    }

    if input.cargo_rust_version_invalid {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "Cargo rust-version is invalid".to_owned(),
            "`Cargo.toml` `rust-version` must be a string version.".to_owned(),
            Some(input.cargo_rel_path.to_owned()),
            None,
            false,
        ));
        return;
    }

    let Some(cargo_msrv) = input.cargo_rust_version else {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "Cargo rust-version not declared".to_owned(),
                message:
                    "No `rust-version` found in Cargo.toml, so MSRV consistency cannot be checked."
                        .to_owned(),
                file: Some(input.cargo_rel_path.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
        return;
    };

    let Some(msrv_version) = parse_manifest_version(cargo_msrv) else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "Cargo rust-version is invalid".to_owned(),
            message: format!(
                "Cannot compare pinned toolchain against invalid Cargo rust-version `{cargo_msrv}`."
            ),
            file: Some(input.cargo_rel_path.to_owned()),
            line: None,
            inventory: false,
        });
        return;
    };

    if toolchain_version < msrv_version {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "pinned toolchain is older than MSRV".to_owned(),
            message: format!(
                "Pinned toolchain `{channel}` is older than Cargo rust-version `{cargo_msrv}`."
            ),
            file: Some(rel.to_owned()),
            line: None,
            inventory: false,
        });
    } else {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "pinned toolchain satisfies MSRV".to_owned(),
                message: format!(
                    "Pinned toolchain `{channel}` is compatible with Cargo rust-version `{cargo_msrv}`."
                ),
                file: Some(rel.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}

fn parse_pinned_stable_version(raw: &str) -> Option<(u64, u64, u64)> {
    let normalized = raw.trim().to_ascii_lowercase();
    let mut segments = normalized.split('-');
    let version_part = segments.next()?;
    if segments.any(|segment| segment.starts_with("nightly") || segment.starts_with("beta")) {
        return None;
    }

    let version_part = version_part.trim_start_matches('v');
    let mut parts = version_part.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    let patch = parts.next().unwrap_or("0").parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((major, minor, patch))
}

fn parse_manifest_version(raw: &str) -> Option<(u64, u64, u64)> {
    let normalized = raw.trim().trim_start_matches('v');
    let mut parts = normalized.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    let patch = parts.next().unwrap_or("0").parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((major, minor, patch))
}

#[cfg(test)]
pub(crate) fn test_input<'a>(
    toolchain_toml_rel: Option<&'a str>,
    legacy_toolchain_rel: Option<&'a str>,
    parsed: Option<&'a toml::Value>,
    parse_error: Option<&'a str>,
    cargo_rust_version: Option<&'a str>,
    cargo_parse_error: Option<&'a str>,
) -> ToolchainRootInput<'a> {
    test_input_for_root(
        "",
        "Cargo.toml",
        toolchain_toml_rel,
        legacy_toolchain_rel,
        parsed,
        parse_error,
        cargo_rust_version,
        cargo_parse_error,
    )
}

#[cfg(test)]
pub(crate) fn test_input_for_root<'a>(
    rel_dir: &'a str,
    cargo_rel_path: &'a str,
    toolchain_toml_rel: Option<&'a str>,
    legacy_toolchain_rel: Option<&'a str>,
    parsed: Option<&'a toml::Value>,
    parse_error: Option<&'a str>,
    cargo_rust_version: Option<&'a str>,
    cargo_parse_error: Option<&'a str>,
) -> ToolchainRootInput<'a> {
    ToolchainRootInput {
        rel_dir,
        cargo_rel_path,
        cargo_toml_rel: Some(cargo_rel_path),
        toolchain_toml_rel,
        legacy_toolchain_rel,
        parsed,
        parse_error,
        cargo_rust_version,
        cargo_rust_version_invalid: false,
        cargo_parse_error,
    }
}

#[cfg(test)]
pub(crate) fn test_input_invalid_cargo_rust_version_type<'a>(
    toolchain_toml_rel: Option<&'a str>,
    legacy_toolchain_rel: Option<&'a str>,
    parsed: Option<&'a toml::Value>,
    parse_error: Option<&'a str>,
    cargo_parse_error: Option<&'a str>,
) -> ToolchainRootInput<'a> {
    ToolchainRootInput {
        rel_dir: "",
        cargo_rel_path: "Cargo.toml",
        cargo_toml_rel: Some("Cargo.toml"),
        toolchain_toml_rel,
        legacy_toolchain_rel,
        parsed,
        parse_error,
        cargo_rust_version: None,
        cargo_rust_version_invalid: true,
        cargo_parse_error,
    }
}

#[cfg(test)]

mod rs_toolchain_03_msrv_consistency_tests;
