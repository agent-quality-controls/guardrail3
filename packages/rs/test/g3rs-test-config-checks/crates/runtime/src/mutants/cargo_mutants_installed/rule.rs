use g3rs_test_types::G3RsTestConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// `ID` constant.
const ID: &str = "g3rs-test/cargo-mutants-installed";

/// `check` function.
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
