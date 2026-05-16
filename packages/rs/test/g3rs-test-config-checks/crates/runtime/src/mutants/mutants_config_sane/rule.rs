use g3rs_test_types::G3RsTestConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// `ID` constant.
const ID: &str = "g3rs-test/mutants-config-sane";

/// `check` function.
pub(crate) fn check(input: &G3RsTestConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(mutants) = input.mutants.as_ref() else {
        return;
    };

    let mut pushed = false;

    if mutants
        .exclude_re
        .iter()
        .any(|pattern| is_exclude_all_pattern(pattern))
    {
        pushed = true;
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "mutants config excludes everything".to_owned(),
            format!(
                "`{}` contains an `exclude_re` pattern that matches everything. Remove the overly broad exclude pattern.",
                input.mutants_rel_path
            ),
            Some(input.mutants_rel_path.clone()),
            None,
        ));
    }

    if let Some(timeout_multiplier) = mutants.timeout_multiplier
        && timeout_multiplier < 1.0
    {
        pushed = true;
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "mutants timeout multiplier too low".to_owned(),
            format!(
                "`{}` sets `timeout_multiplier = {timeout_multiplier}`. Set `timeout_multiplier` to 1.0 or higher.",
                input.mutants_rel_path
            ),
            Some(input.mutants_rel_path.clone()),
            None,
        ));
    }

    if !pushed {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "mutants config looks sane".to_owned(),
                format!(
                    "`{}` avoids the known fake-mutation configurations this family bans.",
                    input.mutants_rel_path
                ),
                Some(input.mutants_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    }
}

/// `is_exclude_all_pattern` function.
fn is_exclude_all_pattern(pattern: &str) -> bool {
    matches!(pattern.trim(), ".*" | "^.*$" | ".+" | "^.+$")
}
