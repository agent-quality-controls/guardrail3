use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ToolchainRootInput;

const ID: &str = "RS-TOOLCHAIN-02";

#[derive(Clone, Copy)]
enum ChannelKind {
    Stable,
    PinnedStable,
    Nightly,
    Beta,
    Unsupported,
}

pub fn check(input: &ToolchainRootInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(rel) = input.toolchain_toml_rel else {
        return;
    };

    let Some(parsed) = input.parsed else {
        if let Some(parse_error) = input.parse_error {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "rust-toolchain.toml parse error".to_owned(),
                message: format!("Invalid TOML: {parse_error}"),
                file: Some(rel.to_owned()),
                line: None,
                inventory: false,
            });
        }
        return;
    };

    let Some(toolchain) = toolchain_table(parsed, rel, results) else {
        return;
    };

    check_channel(toolchain, rel, results);
    check_components(toolchain, rel, results);
}

fn toolchain_table<'a>(
    parsed: &'a toml::Value,
    rel: &str,
    results: &mut Vec<CheckResult>,
) -> Option<&'a toml::value::Table> {
    match parsed.get("toolchain") {
        Some(toml::Value::Table(toolchain)) => Some(toolchain),
        Some(_) => {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "toolchain table is invalid".to_owned(),
                message: "`rust-toolchain.toml` must define `[toolchain]` as a table.".to_owned(),
                file: Some(rel.to_owned()),
                line: None,
                inventory: false,
            });
            None
        }
        None => {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "toolchain table missing".to_owned(),
                message: "Add a `[toolchain]` table with `channel` and `components`.".to_owned(),
                file: Some(rel.to_owned()),
                line: None,
                inventory: false,
            });
            None
        }
    }
}

fn check_channel(toolchain: &toml::value::Table, rel: &str, results: &mut Vec<CheckResult>) {
    let channel_value = toolchain.get("channel");

    match channel_value {
        Some(toml::Value::String(channel)) => match classify_channel(channel) {
            ChannelKind::Stable => results.push(
                CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Info,
                    title: "toolchain channel is stable".to_owned(),
                    message: "channel = \"stable\".".to_owned(),
                    file: Some(rel.to_owned()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            ),
            ChannelKind::Nightly => results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "toolchain channel is nightly".to_owned(),
                message: "Use `channel = \"stable\"` or a pinned stable version.".to_owned(),
                file: Some(rel.to_owned()),
                line: None,
                inventory: false,
            }),
            ChannelKind::Beta => results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "toolchain channel is beta".to_owned(),
                message: "Use `channel = \"stable\"` or a pinned stable version.".to_owned(),
                file: Some(rel.to_owned()),
                line: None,
                inventory: false,
            }),
            ChannelKind::PinnedStable => results.push(
                CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Info,
                    title: "toolchain channel is pinned".to_owned(),
                    message: format!("Pinned channel `{channel}` is acceptable."),
                    file: Some(rel.to_owned()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            ),
            ChannelKind::Unsupported => results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "toolchain channel is unsupported".to_owned(),
                message: "Use `channel = \"stable\"` or a pinned stable version.".to_owned(),
                file: Some(rel.to_owned()),
                line: None,
                inventory: false,
            }),
        },
        Some(_) => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "toolchain channel is invalid".to_owned(),
            message: "`[toolchain].channel` must be a string.".to_owned(),
            file: Some(rel.to_owned()),
            line: None,
            inventory: false,
        }),
        None => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "toolchain channel missing".to_owned(),
            message: "Add `channel = \"stable\"` under `[toolchain]`.".to_owned(),
            file: Some(rel.to_owned()),
            line: None,
            inventory: false,
        }),
    }
}

fn classify_channel(raw: &str) -> ChannelKind {
    let normalized = raw.trim().to_ascii_lowercase();

    if normalized == "stable" {
        return ChannelKind::Stable;
    }
    if normalized.contains("nightly") {
        return ChannelKind::Nightly;
    }
    if normalized.contains("beta") {
        return ChannelKind::Beta;
    }
    if parse_pinned_stable(raw).is_some() {
        return ChannelKind::PinnedStable;
    }

    ChannelKind::Unsupported
}

fn parse_pinned_stable(raw: &str) -> Option<(u64, u64, u64)> {
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

fn check_components(toolchain: &toml::value::Table, rel: &str, results: &mut Vec<CheckResult>) {
    let components_value = toolchain.get("components");

    match components_value {
        Some(toml::Value::Array(components)) => {
            if !components
                .iter()
                .all(|component| component.as_str().is_some())
            {
                results.push(CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Error,
                    title: "toolchain components are invalid".to_owned(),
                    message: "`[toolchain].components` must be an array of strings.".to_owned(),
                    file: Some(rel.to_owned()),
                    line: None,
                    inventory: false,
                });
                return;
            }

            let names: Vec<&str> = components.iter().filter_map(toml::Value::as_str).collect();
            for expected in ["clippy", "rustfmt"] {
                if names.contains(&expected) {
                    results.push(
                        CheckResult {
                            id: ID.to_owned(),
                            severity: Severity::Info,
                            title: format!("toolchain component `{expected}` present"),
                            message: format!("`{expected}` is listed in `components`."),
                            file: Some(rel.to_owned()),
                            line: None,
                            inventory: false,
                        }
                        .as_inventory(),
                    );
                } else {
                    results.push(CheckResult {
                        id: ID.to_owned(),
                        severity: Severity::Warn,
                        title: format!("toolchain component `{expected}` missing"),
                        message: format!("Add `{expected}` to `[toolchain].components`."),
                        file: Some(rel.to_owned()),
                        line: None,
                        inventory: false,
                    });
                }
            }
        }
        Some(_) => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "toolchain components are invalid".to_owned(),
            message: "`[toolchain].components` must be an array of strings.".to_owned(),
            file: Some(rel.to_owned()),
            line: None,
            inventory: false,
        }),
        None => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "toolchain components missing".to_owned(),
            message: "Add `components = [\"clippy\", \"rustfmt\"]` under `[toolchain]`.".to_owned(),
            file: Some(rel.to_owned()),
            line: None,
            inventory: false,
        }),
    }
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
    ToolchainRootInput {
        toolchain_toml_rel,
        legacy_toolchain_rel,
        parsed,
        parse_error,
        cargo_toml_rel: Some("Cargo.toml"),
        cargo_rust_version,
        cargo_rust_version_invalid: false,
        cargo_parse_error,
    }
}

#[cfg(test)]
#[path = "rs_toolchain_02_channel_and_components_tests/mod.rs"]
// reason: test-only sidecar module wiring
mod rs_toolchain_02_channel_and_components_tests;
