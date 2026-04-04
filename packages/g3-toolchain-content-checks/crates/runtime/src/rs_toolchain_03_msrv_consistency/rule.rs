use g3_toolchain_content_checks_types::{G3CargoRustVersion, G3ToolchainContentChecksInput};
use guardrail3_check_types::{GrdzCheckResult, GrdzSeverity};

const ID: &str = "RS-TOOLCHAIN-03";

pub(crate) fn check(input: &G3ToolchainContentChecksInput, results: &mut Vec<GrdzCheckResult>) {
    let channel = input
        .toolchain_toml
        .get("toolchain")
        .and_then(|value| value.get("channel"))
        .and_then(toml::Value::as_str);

    let Some(channel) = channel else {
        return;
    };
    let Some(toolchain_version) = parse_pinned_stable_version(channel) else {
        return;
    };

    match &input.cargo_rust_version {
        G3CargoRustVersion::MissingManifest => {
            results.push(GrdzCheckResult::new(
                ID.to_owned(),
                GrdzSeverity::Error,
                "Cargo.toml missing blocks MSRV check".to_owned(),
                "Cargo.toml is required to compare pinned toolchain against declared MSRV."
                    .to_owned(),
                Some(input.cargo_rel_path.clone()),
                None,
            ));
        }
        G3CargoRustVersion::ParseError(parse_error) => {
            results.push(GrdzCheckResult::new(
                ID.to_owned(),
                GrdzSeverity::Error,
                "Cargo.toml parse error blocks MSRV check".to_owned(),
                format!("Invalid root Cargo.toml: {parse_error}"),
                Some(input.cargo_rel_path.clone()),
                None,
            ));
        }
        G3CargoRustVersion::Missing => {
            results.push(
                GrdzCheckResult::new(
                    ID.to_owned(),
                    GrdzSeverity::Info,
                    "Cargo rust-version not declared".to_owned(),
                    "No `rust-version` found in Cargo.toml, so MSRV consistency cannot be checked."
                        .to_owned(),
                    Some(input.cargo_rel_path.clone()),
                    None,
                )
                .into_inventory(),
            );
        }
        G3CargoRustVersion::InvalidType => {
            results.push(GrdzCheckResult::new(
                ID.to_owned(),
                GrdzSeverity::Error,
                "Cargo rust-version is invalid".to_owned(),
                "`Cargo.toml` `rust-version` must be a string version.".to_owned(),
                Some(input.cargo_rel_path.clone()),
                None,
            ));
        }
        G3CargoRustVersion::Version(cargo_msrv) => {
            let Some(msrv_version) = parse_manifest_version(cargo_msrv) else {
                results.push(GrdzCheckResult::new(
                    ID.to_owned(),
                    GrdzSeverity::Error,
                    "Cargo rust-version is unparseable".to_owned(),
                    format!(
                        "Cannot compare pinned toolchain against invalid Cargo rust-version `{cargo_msrv}`."
                    ),
                    Some(input.cargo_rel_path.clone()),
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
                    Some(input.toolchain_rel_path.clone()),
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
                        Some(input.toolchain_rel_path.clone()),
                        None,
                    )
                    .into_inventory(),
                );
            }
        }
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
