use g3rs_code_source_checks_types::{G3RsCodeSourceChecksInput, G3RsSourceFile};
use guardrail3_check_types::G3Severity;

#[test]
fn emits_code_family_input_failure_on_parse_error() {
    let input = G3RsCodeSourceChecksInput {
        source_file: G3RsSourceFile {
            rel_path: "src/lib.rs".to_owned(),
            content: "fn broken( {".to_owned(),
            is_test: false,
            profile_name: None,
            is_library_root: false,
        },
    };

    let results = crate::run::check(&input);
    assert_eq!(
        results.len(),
        1,
        "unexpected parse-failure results: {results:#?}"
    );
    let result = &results[0];

    assert_eq!(result.id(), "RS-CODE-SOURCE-30");
    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.title(), "code-family input failure");
    assert_eq!(result.file(), Some("src/lib.rs"));
    assert_eq!(result.line(), None);
    assert!(!result.inventory(), "parse failure should not be inventory");
    assert!(
        result
            .message()
            .starts_with("Failed to parse Rust source file:"),
        "unexpected parse failure message: {result:#?}"
    );
}
