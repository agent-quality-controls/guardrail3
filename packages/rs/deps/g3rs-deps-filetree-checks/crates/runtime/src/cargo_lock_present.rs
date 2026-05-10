use g3rs_deps_types::G3RsDepsFileTreeChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_rs_toml_parser::types::RustProfile;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-deps/cargo-lock-present";

/// Emits a finding when the workspace root is missing `Cargo.lock`.
pub(crate) fn check(input: &G3RsDepsFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    if input.cargo_lock_exists {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "Cargo.lock committed".to_owned(),
                format!(
                    "Workspace root has `{}` committed.",
                    input.cargo_lock_rel_path
                ),
                Some(input.cargo_lock_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
        return;
    }

    let severity = if input.profile == Some(RustProfile::Library) {
        G3Severity::Info
    } else {
        G3Severity::Error
    };
    let message = if input.profile == Some(RustProfile::Library) {
        format!(
            "Library-profile workspace is missing `{}`.",
            input.cargo_lock_rel_path
        )
    } else {
        format!(
            "`{}` is missing. Run `cargo generate-lockfile` and commit the result.",
            input.cargo_lock_rel_path
        )
    };

    results.push(G3CheckResult::new(
        ID.to_owned(),
        severity,
        "Cargo.lock missing".to_owned(),
        message,
        Some(input.cargo_lock_rel_path.clone()),
        None,
    ));
}

#[cfg(test)]
#[path = "cargo_lock_present_tests/mod.rs"]
mod cargo_lock_present_tests;
