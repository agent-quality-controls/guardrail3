use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PolicyRootCargoInput;

const ID: &str = "RS-CARGO-15";

pub fn check(input: &PolicyRootCargoInput<'_>, results: &mut Vec<CheckResult>) {
    let root = input.root;
    if root.parse_error.is_some() {
        return;
    }
    if root.guardrail_parse_error {
        return;
    }
    if root.rust_version_invalid {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "rust-version invalid".to_owned(),
            format!(
                "`{}` must declare `rust-version` as a string value when it is present.",
                root.cargo_rel_path
            ),
            Some(root.cargo_rel_path.clone()),
            None,
            false,
        ));
        return;
    }

    let is_library = root.profile_name.as_deref() == Some("library");
    match (is_library, root.rust_version.as_deref()) {
        (true, Some(version)) => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "library rust-version declared".to_owned(),
                format!("`{}` declares `rust-version = \"{version}\"`.", root.cargo_rel_path),
                Some(root.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        ),
        (true, None) => results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    "library rust-version missing".to_owned(),
    "Library profile must declare `rust-version` as an MSRV contract.".to_owned(),
    Some(root.cargo_rel_path.clone()),
    None,
    false,
        )),
        (false, Some(version)) => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "rust-version inventory".to_owned(),
                format!("`{}` declares `rust-version = \"{version}\"`.", root.cargo_rel_path),
                Some(root.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        ),
        (false, None) => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "rust-version inventory".to_owned(),
                format!(
                    "`{}` does not declare `rust-version`; this is inventoried for non-library profiles.",
                    root.cargo_rel_path
                ),
                Some(root.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        ),
    }
}

#[cfg(test)]
#[path = "rs_cargo_15_rust_version_policy_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_cargo_15_rust_version_policy_tests;
