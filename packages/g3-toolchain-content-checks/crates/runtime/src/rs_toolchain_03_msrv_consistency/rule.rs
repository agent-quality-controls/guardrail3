use cargo_toml_parser::CargoToml;
use rust_toolchain_toml_parser::RustToolchainToml;
use guardrail3_check_types::{GrdzCheckResult, GrdzSeverity};

const ID: &str = "RS-TOOLCHAIN-03";

pub(crate) fn check(
    toolchain_rel_path: &str,
    toolchain_toml: &RustToolchainToml,
    cargo_rel_path: &str,
    cargo_toml: &CargoToml,
    results: &mut Vec<GrdzCheckResult>,
) {
    let channel = toolchain_toml
        .toolchain()
        .and_then(|toolchain| toolchain.channel());

    let Some(channel) = channel else {
        return;
    };
    let Some(toolchain_version) = parse_pinned_stable_version(channel) else {
        return;
    };

    match cargo_rust_version(cargo_toml) {
        None => {
            results.push(
                GrdzCheckResult::new(
                    ID.to_owned(),
                    GrdzSeverity::Info,
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
                results.push(GrdzCheckResult::new(
                    ID.to_owned(),
                    GrdzSeverity::Error,
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
                results.push(GrdzCheckResult::new(
                    ID.to_owned(),
                    GrdzSeverity::Warn,
                    "pinned toolchain is older than MSRV".to_owned(),
                    format!(
                        "Pinned toolchain `{channel}` is older than Cargo rust-version `{cargo_msrv}`. Either update the pinned toolchain to match or exceed the MSRV, or lower `rust-version` in Cargo.toml."
                    ),
                    Some(toolchain_rel_path.to_owned()),
                    None,
                ));
            } else {
                results.push(
                    GrdzCheckResult::new(
                        ID.to_owned(),
                        GrdzSeverity::Info,
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

fn cargo_rust_version(cargo: &CargoToml) -> Option<&str> {
    cargo
        .workspace
        .as_ref()
        .and_then(|workspace| workspace.package.as_ref())
        .and_then(|package| package.rust_version.as_deref())
        .or_else(|| {
            cargo.package
                .as_ref()
                .and_then(|package| package.rust_version.as_deref())
        })
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
