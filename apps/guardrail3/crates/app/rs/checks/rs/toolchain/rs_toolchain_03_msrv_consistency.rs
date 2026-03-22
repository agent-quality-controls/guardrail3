use crate::domain::report::{CheckResult, Severity};

use super::inputs::ToolchainRootInput;

const ID: &str = "RS-TOOLCHAIN-03";

pub fn check(input: &ToolchainRootInput<'_>, results: &mut Vec<CheckResult>) {
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

    let Some(cargo_msrv) = input.cargo_rust_version else {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "Cargo rust-version not declared".to_owned(),
                message: "No `rust-version` found in Cargo.toml, so MSRV consistency cannot be checked."
                    .to_owned(),
                file: Some(rel.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
        return;
    };

    let Some(channel) = channel else {
        return;
    };

    if channel == "stable" || channel == "nightly" {
        return;
    }

    let Some(toolchain_version) = parse_version(channel) else {
        return;
    };
    let Some(msrv_version) = parse_version(cargo_msrv) else {
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

fn parse_version(raw: &str) -> Option<(u64, u64, u64)> {
    let normalized = raw.trim().trim_start_matches('v');
    let mut parts = normalized.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    let patch = parts.next().unwrap_or("0").parse().ok()?;
    Some((major, minor, patch))
}
