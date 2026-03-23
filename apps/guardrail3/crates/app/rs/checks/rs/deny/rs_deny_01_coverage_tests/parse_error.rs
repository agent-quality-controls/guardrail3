use crate::domain::report::Severity;

use super::super::super::test_support::{copy_fixture, run_family, write_file};

#[test]
fn errors_when_an_allowed_deny_config_cannot_be_parsed() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "deny.toml", "[sources");

    let results = run_family(tmp.path());
    let parse_errors = results
        .iter()
        .filter(|result| {
            result.id == "RS-DENY-01"
                && result.severity == Severity::Error
                && result.title == "deny config parse error"
        })
        .collect::<Vec<_>>();

    assert_eq!(
        parse_errors.len(),
        1,
        "expected one parse error: {parse_errors:#?}"
    );
    let result = parse_errors[0];
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
    assert!(!result.inventory);
    assert!(
        result
            .message
            .starts_with("`deny.toml` could not be parsed: ")
    );
}
