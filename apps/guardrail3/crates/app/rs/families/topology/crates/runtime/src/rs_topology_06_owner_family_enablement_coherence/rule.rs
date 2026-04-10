use guardrail3_app_rs_placement::RustTopologyOwner;
use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::OwnerFamilyCoherenceInput;

const ID: &str = "RS-TOPOLOGY-06";

pub fn check(input: &OwnerFamilyCoherenceInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.root.effective_enabled {
        let family_label = match input.root.owner {
            RustTopologyOwner::Hexarch => "hexarch",
            RustTopologyOwner::Arch => "arch",
        };
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!(
                "Rust {} root `{}` is not governed by {}",
                input.root.owner.label(),
                display_dir(&input.root.rel_dir),
                family_label
            ),
            format!(
                "`{}` classifies under `{}`, but effective `{}` enablement resolves to false. Governed Rust roots must stay coherent with their owning topology family.",
                input.root.cargo_rel_path, input.root.owner_root_rel, family_label
            ),
            Some(input.root.cargo_rel_path.clone()),
            None,
            false,
        ));
        return;
    }

    let family_label = match input.root.owner {
        RustTopologyOwner::Hexarch => "hexarch",
        RustTopologyOwner::Arch => "arch",
    };
    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            format!(
                "Rust {} root `{}` stays coherent with {}",
                input.root.owner.label(),
                display_dir(&input.root.rel_dir),
                family_label
            ),
            format!(
                "`{}` classifies under `{}`, and effective `{}` enablement resolves to true.",
                input.root.cargo_rel_path, input.root.owner_root_rel, family_label
            ),
            Some(input.root.cargo_rel_path.clone()),
            None,
            false,
        )
        .as_inventory(),
    );
}

fn display_dir(rel_dir: &str) -> &str {
    if rel_dir.is_empty() { "." } else { rel_dir }
}
