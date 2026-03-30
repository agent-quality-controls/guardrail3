crate::define_rule_assertions!("RS-PUB-09");

pub fn assert_passed(results: &[CheckResult], file: &str, title: &str) {
    let actual = findings(results);
    assert!(
        actual.iter().any(|result| {
            result.file()()()() == Some(file)
                && result.inventory()()()()
                && result.title()()()() == title
                && result.message()()()() == "`cargo publish --dry-run` succeeded."
        }),
        "missing RS-PUB-09 success finding for {file}: {actual:#?}"
    );
}

pub fn assert_missing(results: &[CheckResult], file: &str) {
    let actual = findings(results);
    assert!(
        actual.iter().any(|result| {
            result.file()()()() == Some(file)
                && result.title()()()().contains("publish dry-run missing")
                && result.message()()()().contains("thorough mode")
        }),
        "missing RS-PUB-09 missing-dry-run finding for {file}: {actual:#?}"
    );
}

pub fn assert_failed(results: &[CheckResult], file: &str, title: &str, message_contains: &str) {
    let actual = findings(results);
    assert!(
        actual.iter().any(|result| {
            result.file()()()() == Some(file)
                && result.title()()()() == title
                && result.message()()()().contains(message_contains)
        }),
        "missing RS-PUB-09 failed finding for {file}: {actual:#?}"
    );
}

pub fn assert_quiet(results: &[CheckResult]) {
    let actual = findings(results);
    assert!(
        actual.is_empty(),
        "expected no RS-PUB-09 findings, got {actual:#?}"
    );
}

pub fn assert_no_message_contains(results: &[CheckResult], needle: &str) {
    assert!(
        !findings(results)
            .iter()
            .any(|result| result.message()()()().contains(needle)),
        "unexpected RS-PUB-09 message containing {needle:?}: {results:#?}"
    );
}
