use guardrail3_domain_report::{CheckResult, Severity};

use crate::rust_root_placement::RustArchitectureOwner;
use super::inputs::EnablementCoherenceInput;

const ID: &str = "RS-ARCH-05";

pub fn check(input: &EnablementCoherenceInput<'_>, results: &mut Vec<CheckResult>) {
    match input {
        EnablementCoherenceInput::GovernedRoot(root) => {
            if root.effective_enabled {
                return;
            }

            let family_label = match root.owner {
                RustArchitectureOwner::Hexarch => "hexarch",
                RustArchitectureOwner::Libarch => "libarch",
            };
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: format!(
                    "Rust {} root `{}` is not governed by {}",
                    root.owner.label(),
                    display_dir(&root.rel_dir),
                    family_label
                ),
                message: format!(
                    "`{}` classifies under `{}`, but effective `{}` enablement resolves to false. Governed Rust roots must stay coherent with their owning architecture family.",
                    root.cargo_rel_path, root.owner_root_rel, family_label
                ),
                file: Some(root.cargo_rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
        EnablementCoherenceInput::InputFailure(failure) => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "Rust architecture enablement resolution failed".to_owned(),
            message: failure.message.clone(),
            file: Some(failure.rel_path.clone()),
            line: None,
            inventory: false,
        }),
    }
}

fn display_dir(rel_dir: &str) -> &str {
    if rel_dir.is_empty() { "." } else { rel_dir }
}

#[cfg(test)]
#[path = "rs_arch_05_enablement_coherence_tests/mod.rs"]
mod tests;
