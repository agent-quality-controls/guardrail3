use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn assert_committed_lockfile_inventory(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-deps/cargo-lock-present"
                && result.severity() == G3Severity::Info
                && result.title() == "Cargo.lock committed"
                && result.inventory()
                && result.file() == Some("Cargo.lock")
        }),
        "{results:#?}"
    );
}

pub fn assert_missing_library_lockfile_info(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-deps/cargo-lock-present"
                && result.severity() == G3Severity::Info
                && result.title() == "Cargo.lock missing"
                && result.message() == "Library-profile workspace is missing `Cargo.lock`."
                && !result.inventory()
                && result.file() == Some("Cargo.lock")
        }),
        "{results:#?}"
    );
}
