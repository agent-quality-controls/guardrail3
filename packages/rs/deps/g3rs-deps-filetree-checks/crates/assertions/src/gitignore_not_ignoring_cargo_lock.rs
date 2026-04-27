use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn assert_ignored_lockfile(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-deps/gitignore-not-ignoring-cargo-lock"
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
