use guardrail3_domain_report::Severity;

use super::super::super::inputs::RustCodeFileInput;
use super::super::super::parse::parse_rust_file;
use super::super::check;

#[test]
fn warns_on_public_result_string_in_library_profile() {
    let content = "pub fn parse() -> Result<(), String> { Ok(()) }";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/lib.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: Some("library"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-CODE-25");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "weak public error type");
    assert_eq!(
        results[0].message,
        "Public function `parse` returns `Result<_, String>`. Use a typed error instead."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn warns_on_public_result_box_dyn_error_in_library_profile() {
    let content = "pub fn parse() -> Result<(), Box<dyn std::error::Error>> { Ok(()) }";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/lib.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: Some("library"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-CODE-25");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "weak public error type");
    assert_eq!(
        results[0].message,
        "Public function `parse` returns `Result<_, Box<dyn Error>>`. Use a typed error instead."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}
