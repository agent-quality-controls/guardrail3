use guardrail3_check_types::{G3CheckResult, G3Severity};

/// `ID` constant.
const ID: &str = "g3rs-hooks/modular-directory-inventory";

/// `check` function.
pub(crate) fn check(has_modular_dir: bool, results: &mut Vec<G3CheckResult>) {
    let (title, message, file) = if has_modular_dir {
        (
            "pre-commit.d directory exists",
            "Hook uses modular pre-commit scripts.",
            Some(".githooks/pre-commit.d".to_owned()),
        )
    } else {
        (
            "pre-commit.d directory missing",
            "Hook currently uses a monolithic pre-commit script.",
            Some(".githooks".to_owned()),
        )
    };

    results.push(
        G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Info,
            title.to_owned(),
            message.to_owned(),
            file,
            None,
        )
        .into_inventory(),
    );
}
