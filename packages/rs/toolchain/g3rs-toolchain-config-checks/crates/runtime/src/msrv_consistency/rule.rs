use cargo_toml_parser::{types::CargoToml, types::InheritableValue};
use guardrail3_check_types::{G3CheckResult, G3Severity};
use rust_toolchain_toml_parser::types::RustToolchainToml;

/// Stable rule identifier surfaced in findings.
const ID: &str = "g3rs-toolchain/msrv-consistency";

/// Parsed semver tuple `(major, minor, patch)`.
type ParsedVersion = (u64, u64, u64);

/// Validates that the pinned toolchain channel matches Cargo's `rust-version`.
pub(crate) fn check(
    toolchain_rel_path: &str,
    toolchain_toml: &RustToolchainToml,
    cargo_rel_path: &str,
    cargo_toml: &CargoToml,
    results: &mut Vec<G3CheckResult>,
) {
    let channel = toolchain_toml
        .toolchain
        .as_ref()
        .and_then(|toolchain| toolchain.channel.as_deref());

    let Some(channel) = channel else {
        return;
    };
    let Some(toolchain_version) = parse_pinned_stable_version(channel) else {
        return;
    };

    match cargo_rust_version(cargo_toml) {
        None => {
            results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Info,
                    "Cargo rust-version not declared".to_owned(),
                    "No `rust-version` found in Cargo.toml, so MSRV consistency cannot be checked."
                        .to_owned(),
                    Some(cargo_rel_path.to_owned()),
                    None,
                )
                .into_inventory(),
            );
        }
        Some(cargo_msrv) => {
            let Some(msrv_version) = parse_manifest_version(cargo_msrv) else {
                results.push(G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Error,
                    "Cargo rust-version is unparseable".to_owned(),
                    format!(
                        "Cannot compare pinned toolchain against invalid Cargo rust-version `{cargo_msrv}`."
                    ),
                    Some(cargo_rel_path.to_owned()),
                    None,
                ));
                return;
            };

            if toolchain_version < msrv_version {
                results.push(G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Warn,
                    "pinned toolchain is older than MSRV".to_owned(),
                    format!(
                        "Pinned toolchain `{channel}` is older than Cargo rust-version `{cargo_msrv}`. Either update the pinned toolchain to match or exceed the MSRV, or lower `rust-version` in Cargo.toml."
                    ),
                    Some(toolchain_rel_path.to_owned()),
                    None,
                ));
            } else {
                results.push(
                    G3CheckResult::new(
                        ID.to_owned(),
                        G3Severity::Info,
                        "pinned toolchain satisfies MSRV".to_owned(),
                        format!(
                            "Pinned toolchain `{channel}` is compatible with Cargo rust-version `{cargo_msrv}`."
                        ),
                        Some(toolchain_rel_path.to_owned()),
                        None,
                    )
                    .into_inventory(),
                );
            }
        }
    }
}

/// Returns the effective Cargo `rust-version` declared on workspace or package.
fn cargo_rust_version(cargo: &CargoToml) -> Option<&str> {
    cargo
        .workspace
        .as_ref()
        .and_then(|workspace| workspace.package.as_ref())
        .and_then(|package| package.rust_version.as_deref())
        .or_else(|| {
            cargo
                .package
                .as_ref()
                .and_then(|package| inheritable_string(package.rust_version.as_ref()))
        })
}

/// Resolves an [`InheritableValue<String>`] to its concrete value, if any.
fn inheritable_string(value: Option<&InheritableValue<String>>) -> Option<&str> {
    match value {
        Some(InheritableValue::Value(value)) => Some(value.as_str()),
        Some(InheritableValue::Inherit(_)) | None => None,
    }
}

/// Parses a pinned stable channel `MAJOR.MINOR[.PATCH]` into a [`ParsedVersion`].
fn parse_pinned_stable_version(raw: &str) -> Option<ParsedVersion> {
    let normalized = raw.trim().to_ascii_lowercase();
    if normalized.split_once('-').is_some() {
        return None;
    }
    let version_part = normalized.trim_start_matches('v');
    let mut parts = version_part.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    let patch = parts.next().unwrap_or("0").parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((major, minor, patch))
}

/// Parses a Cargo manifest `rust-version` into a [`ParsedVersion`].
fn parse_manifest_version(raw: &str) -> Option<ParsedVersion> {
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
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
