use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ToolchainRootInput;

const ID: &str = "RS-TOOLCHAIN-02";

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

    check_channel(parsed, rel, results);
    check_components(parsed, rel, results);
}

fn check_channel(parsed: &toml::Value, rel: &str, results: &mut Vec<CheckResult>) {
    let channel = parsed
        .get("toolchain")
        .and_then(|value| value.get("channel"))
        .and_then(toml::Value::as_str);

    match channel {
        Some("stable") => results.push(
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
        Some("nightly") => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "toolchain channel is nightly".to_owned(),
            message: "Use `channel = \"stable\"` or a pinned stable version.".to_owned(),
            file: Some(rel.to_owned()),
            line: None,
            inventory: false,
        }),
        Some(other) => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "toolchain channel is pinned".to_owned(),
                message: format!("Pinned channel `{other}` is acceptable."),
                file: Some(rel.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
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

fn check_components(parsed: &toml::Value, rel: &str, results: &mut Vec<CheckResult>) {
    let components = parsed
        .get("toolchain")
        .and_then(|value| value.get("components"))
        .and_then(toml::Value::as_array);

    match components {
        Some(components) => {
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
#[path = "rs_toolchain_02_channel_and_components_tests.rs"]
mod tests;
