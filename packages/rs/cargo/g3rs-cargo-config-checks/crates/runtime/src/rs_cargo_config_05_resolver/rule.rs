use cargo_toml_parser::CargoToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::{CargoRole, cargo_role, error, info, workspace_resolver};

const ID: &str = "RS-CARGO-CONFIG-05";

pub(crate) fn check(cargo_rel_path: &str, cargo: &CargoToml, results: &mut Vec<G3CheckResult>) {
    if cargo_role(cargo) != CargoRole::WorkspaceRoot {
        return;
    }

    match workspace_resolver(cargo) {
        Some("2" | "3") => {
            results.push(info(
                ID,
                "workspace resolver set",
                format!(
                    "Workspace resolver = `{}`",
                    workspace_resolver(cargo).unwrap_or_default()
                ),
                cargo_rel_path,
            ));
        }
        Some(other) => {
            results.push(error(
                ID,
                "unsupported workspace resolver",
                format!(
                    "Expected resolver `2` or `3`, got `{other}`. Prefer `resolver = \"3\"` (edition 2024) if the project allows it."
                ),
                cargo_rel_path,
            ));
        }
        None => {
            results.push(error(
                ID,
                "workspace resolver missing",
                "Every workspace root must set `resolver = \"2\"` or `resolver = \"3\"` explicitly. Prefer `resolver = \"3\"` (edition 2024) if the project allows it.",
                cargo_rel_path,
            ));
        }
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
