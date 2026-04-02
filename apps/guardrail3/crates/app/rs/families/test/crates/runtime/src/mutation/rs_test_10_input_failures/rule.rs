use crate::{CheckResult, Severity};

use crate::inputs::InputFailureTestInput;

const ID: &str = "RS-TEST-10";

pub fn check(input: &InputFailureTestInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "test-family input failure".to_owned(),
        input.failure.message.clone(),
        Some(input.failure.rel_path.clone()),
        None,
        false,
    ));
}

pub(crate) fn emit_inventory_if_clean(
    root: &crate::facts::TestRootFacts,
    results: &mut Vec<CheckResult>,
    has_failures: bool,
) {
    if has_failures {
        return;
    }
    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "test-family input failures evaluated".to_owned(),
            format!(
                "Root `{}` was checked for input failures and none were found.",
                root.rel_dir
            ),
            Some(root.cargo_rel_path.clone()),
            None,
            false,
        )
        .as_inventory(),
    );
}

