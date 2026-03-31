use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::IllegalFamilyFilePlacementInput;

const ID: &str = "RS-TOPOLOGY-16";

pub fn check(input: &IllegalFamilyFilePlacementInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        format!(
            "`{}` file `{}` is illegally placed",
            family_label(input.file.family),
            input.file.rel_path
        ),
        input.file.reason.clone(),
        Some(input.file.rel_path.clone()),
        None,
        false,
    ));
}

fn family_label(family: guardrail3_validation_model::RustValidateFamily) -> &'static str {
    match family {
        guardrail3_validation_model::RustValidateFamily::Toolchain => "toolchain",
        guardrail3_validation_model::RustValidateFamily::Clippy => "clippy",
        guardrail3_validation_model::RustValidateFamily::Deny => "deny",
        guardrail3_validation_model::RustValidateFamily::Cargo => "cargo",
        guardrail3_validation_model::RustValidateFamily::Deps => "deps",
        guardrail3_validation_model::RustValidateFamily::Garde => "garde",
        guardrail3_validation_model::RustValidateFamily::Release => "release",
        guardrail3_validation_model::RustValidateFamily::Topology => "topology",
        guardrail3_validation_model::RustValidateFamily::Fmt => "fmt",
        guardrail3_validation_model::RustValidateFamily::Code => "code",
        guardrail3_validation_model::RustValidateFamily::Hexarch => "hexarch",
        guardrail3_validation_model::RustValidateFamily::Libarch => "libarch",
        guardrail3_validation_model::RustValidateFamily::Test => "test",
        guardrail3_validation_model::RustValidateFamily::HooksShared => "hooks-shared",
        guardrail3_validation_model::RustValidateFamily::HooksRs => "hooks-rs",
    }
}

#[cfg(test)]
#[path = "rs_topology_16_workspace_local_file_placement_tests/mod.rs"]
mod rs_topology_16_workspace_local_file_placement_tests;
