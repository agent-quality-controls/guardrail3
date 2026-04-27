use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-arch/mod-rs-required";

pub fn assert_foo_rs_convention_error(results: &[G3CheckResult], file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Error
                && result.title() == "module directory uses foo.rs convention"
                && result.file() == Some(file)
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

pub fn assert_missing_mod_rs_error(results: &[G3CheckResult], file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Error
                && result.title() == "module directory missing mod.rs"
                && result.file() == Some(file)
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

pub fn assert_mod_rs_inventory(results: &[G3CheckResult], file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && result.severity() == G3Severity::Info
                && result.title() == "module directory has mod.rs"
                && result.file() == Some(file)
                && result.inventory()
        }),
        "{results:#?}"
    );
}
