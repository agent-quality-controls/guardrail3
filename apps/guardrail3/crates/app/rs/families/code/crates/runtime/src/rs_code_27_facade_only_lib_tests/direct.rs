use guardrail3_domain_report::Severity;

use super::super::super::inputs::RustCodeFileInput;
use super::super::super::parse::parse_rust_file;
use super::super::check;

#[test]
fn errors_on_private_use_in_library_lib_rs() {
    let content = "use crate::internal::Thing;";
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
    assert_eq!(results[0].id, "RS-CODE-27");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "lib.rs should stay facade-only");
    assert_eq!(
        results[0].message,
        "lib.rs contains private use `crate::internal::Thing`. Keep lib.rs limited to facade declarations and type/const definitions."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}
