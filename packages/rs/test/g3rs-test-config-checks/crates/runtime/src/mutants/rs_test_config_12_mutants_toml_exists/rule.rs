use g3rs_test_types::G3RsTestConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-TEST-CONFIG-12";

pub(crate) fn check(input: &G3RsTestConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if input.mutants_exists && input.mutants.is_some() {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "mutants config exists".to_owned(),
                format!("Found `{}`.", input.mutants_rel_path),
                Some(input.mutants_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    } else if !input.mutants_exists {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "mutants config missing".to_owned(),
            format!(
                "{} is missing required mutation config `{}`. Create the mutation config file.",
                display_root(&input.root_rel_dir),
                input.mutants_rel_path
            ),
            Some(input.mutants_rel_path.clone()),
            None,
        ));
    }
}

fn display_root(rel_dir: &str) -> String {
    if rel_dir.is_empty() {
        "project root".to_owned()
    } else {
        format!("`{rel_dir}`")
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
