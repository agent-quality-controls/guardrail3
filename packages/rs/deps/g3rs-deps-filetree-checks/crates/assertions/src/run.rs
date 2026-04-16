use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn assert_combined_missing_and_ignored(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-DEPS-FILETREE-09"
                && result.severity() == G3Severity::Error
                && result.title() == "Cargo.lock missing"
                && result.message()
                    == "`Cargo.lock` is missing. Run `cargo generate-lockfile` and commit the result."
                && !result.inventory()
                && result.file() == Some("Cargo.lock")
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-DEPS-FILETREE-10"
                && result.severity() == G3Severity::Error
                && result.title() == "Cargo.lock ignored in gitignore"
                && result.message()
                    == "`.gitignore` ignores `Cargo.lock`. Remove the line ignoring `Cargo.lock` from this `.gitignore`."
                && !result.inventory()
                && result.file() == Some(".gitignore")
        }),
        "{results:#?}"
    );
}
