use g3rs_test_types::G3RsTestConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// `ID` constant.
const ID: &str = "g3rs-test/mutants-profile-present";

/// `check` function.
pub(crate) fn check(input: &G3RsTestConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if input.cargo.profile.contains_key("mutants") {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "profile.mutants configured".to_owned(),
                format!("`{}` defines `[profile.mutants]`.", input.cargo_rel_path),
                Some(input.cargo_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    } else {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "profile.mutants missing".to_owned(),
            format!(
                "`{}` does not define `[profile.mutants]` for an active mutation-testing setup. Add a `[profile.mutants]` section to this Cargo.toml.",
                input.cargo_rel_path
            ),
            Some(input.cargo_rel_path.clone()),
            None,
        ));
    }
}
