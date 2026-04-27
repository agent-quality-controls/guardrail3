use g3rs_garde_types::{G3RsGardeApplicability, G3RsGardeClippyInput, G3RsGardeConfigChecksInput};
use guardrail3_check_types::G3CheckResult;

use crate::support::has_garde_dependency;

/// Run extracted garde config checks.
///
/// Always runs the dependency-present check (Cargo.toml is required).
/// Runs clippy ban checks only when garde is present. Missing or invalid clippy
/// input is surfaced here as rule-local warn results.
#[must_use]
pub fn check(input: &G3RsGardeConfigChecksInput) -> Vec<G3CheckResult> {
    if input.applicability == G3RsGardeApplicability::Inactive {
        return Vec::new();
    }

    let mut results = Vec::new();

    crate::dependency_present::check(&input.cargo_rel_path, &input.cargo, &mut results);

    if !has_garde_dependency(&input.cargo) {
        return results;
    }

    match &input.clippy_input {
        G3RsGardeClippyInput::Missing => {
            crate::core_method_bans::check_unverifiable(
                None,
                "No clippy.toml found. Create one with a `disallowed-methods` section.",
                &mut results,
            );
            crate::extractor_type_bans::check_unverifiable(
                None,
                "No clippy.toml found. Create one with a `disallowed-types` section.",
                &mut results,
            );
            crate::reqwest_json_ban::check_unverifiable(
                None,
                "No clippy.toml found. Create one with a `disallowed-methods` section.",
                &mut results,
            );
            crate::additional_method_bans::check_unverifiable(
                None,
                "No clippy.toml found. Create one with a `disallowed-methods` section.",
                &mut results,
            );
        }
        G3RsGardeClippyInput::Invalid { rel_path, message } => {
            crate::core_method_bans::check_unverifiable(Some(rel_path), message, &mut results);
            crate::extractor_type_bans::check_unverifiable(Some(rel_path), message, &mut results);
            crate::reqwest_json_ban::check_unverifiable(Some(rel_path), message, &mut results);
            crate::additional_method_bans::check_unverifiable(
                Some(rel_path),
                message,
                &mut results,
            );
        }
        G3RsGardeClippyInput::Parsed { rel_path, clippy } => {
            crate::core_method_bans::check(rel_path, clippy, &mut results);
            crate::extractor_type_bans::check(rel_path, clippy, &mut results);
            crate::reqwest_json_ban::check(rel_path, clippy, &mut results);
            crate::additional_method_bans::check(rel_path, clippy, &mut results);
        }
    }

    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod run_tests;
