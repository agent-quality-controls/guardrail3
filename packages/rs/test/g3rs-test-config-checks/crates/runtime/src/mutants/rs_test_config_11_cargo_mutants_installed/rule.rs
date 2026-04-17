use g3rs_test_types::G3RsTestConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-TEST-CONFIG-11";

pub(crate) fn check(input: &G3RsTestConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if input.cargo_mutants_installed {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "cargo-mutants installed".to_owned(),
                "`cargo-mutants` is available on PATH.".to_owned(),
                Some(input.cargo_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    } else {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "cargo-mutants missing".to_owned(),
            "`cargo-mutants` was not found on PATH. Install with `cargo install cargo-mutants`."
                .to_owned(),
            Some(input.cargo_rel_path.clone()),
            None,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
