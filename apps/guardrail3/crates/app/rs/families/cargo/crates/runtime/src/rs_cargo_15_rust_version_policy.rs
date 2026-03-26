use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PolicyRootCargoInput;

const ID: &str = "RS-CARGO-15";

pub fn check(input: &PolicyRootCargoInput<'_>, results: &mut Vec<CheckResult>) {
    let root = input.root;
    if root.parse_error.is_some() {
        return;
    }

    let is_library = root.profile_name.as_deref() == Some("library");
    match (is_library, root.rust_version.as_deref()) {
        (true, Some(version)) => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "library rust-version declared".to_owned(),
                message: format!("`{}` declares `rust-version = \"{version}\"`.", root.cargo_rel_path),
                file: Some(root.cargo_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
        (true, None) => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "library rust-version missing".to_owned(),
            message: "Library profile must declare `rust-version` as an MSRV contract.".to_owned(),
            file: Some(root.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }),
        (false, Some(version)) => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "rust-version inventory".to_owned(),
                message: format!("`{}` declares `rust-version = \"{version}\"`.", root.cargo_rel_path),
                file: Some(root.cargo_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
        (false, None) => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "rust-version inventory".to_owned(),
                message: format!(
                    "`{}` does not declare `rust-version`; this is inventoried for non-library profiles.",
                    root.cargo_rel_path
                ),
                file: Some(root.cargo_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
    }
}

#[cfg(test)]
#[path = "rs_cargo_15_rust_version_policy_tests/mod.rs"]
mod rs_cargo_15_rust_version_policy_tests;
