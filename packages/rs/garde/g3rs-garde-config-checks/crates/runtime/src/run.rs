use g3rs_garde_config_checks_types::{G3RsGardeClippyInput, G3RsGardeConfigChecksInput};
use guardrail3_check_types::G3CheckResult;

use crate::support::has_garde_dependency;

/// Run extracted garde config checks.
///
/// Always runs the dependency-present check (Cargo.toml is required).
/// Runs clippy ban checks only when garde is present. Missing or invalid clippy
/// input is surfaced here as rule-local warn results.
#[must_use]
pub fn check(input: &G3RsGardeConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    crate::rs_garde_config_01_dependency_present::check(
        &input.cargo_rel_path,
        &input.cargo,
        &mut results,
    );

    if !has_garde_dependency(&input.cargo) {
        return results;
    }

    match &input.clippy_input {
        G3RsGardeClippyInput::Missing => {
            crate::rs_garde_config_02_core_method_bans::check_unverifiable(
                None,
                "No clippy.toml found. Create one with a `disallowed-methods` section.",
                &mut results,
            );
            crate::rs_garde_config_03_extractor_type_bans::check_unverifiable(
                None,
                "No clippy.toml found. Create one with a `disallowed-types` section.",
                &mut results,
            );
            crate::rs_garde_config_04_reqwest_json_ban::check_unverifiable(
                None,
                "No clippy.toml found. Create one with a `disallowed-methods` section.",
                &mut results,
            );
            crate::rs_garde_config_05_additional_method_bans::check_unverifiable(
                None,
                "No clippy.toml found. Create one with a `disallowed-methods` section.",
                &mut results,
            );
        }
        G3RsGardeClippyInput::Invalid { rel_path, message } => {
            crate::rs_garde_config_02_core_method_bans::check_unverifiable(
                Some(rel_path),
                message,
                &mut results,
            );
            crate::rs_garde_config_03_extractor_type_bans::check_unverifiable(
                Some(rel_path),
                message,
                &mut results,
            );
            crate::rs_garde_config_04_reqwest_json_ban::check_unverifiable(
                Some(rel_path),
                message,
                &mut results,
            );
            crate::rs_garde_config_05_additional_method_bans::check_unverifiable(
                Some(rel_path),
                message,
                &mut results,
            );
        }
        G3RsGardeClippyInput::Parsed { rel_path, clippy } => {
            crate::rs_garde_config_02_core_method_bans::check(rel_path, clippy, &mut results);
            crate::rs_garde_config_03_extractor_type_bans::check(rel_path, clippy, &mut results);
            crate::rs_garde_config_04_reqwest_json_ban::check(rel_path, clippy, &mut results);
            crate::rs_garde_config_05_additional_method_bans::check(rel_path, clippy, &mut results);
        }
    }

    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod tests;
