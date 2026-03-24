use crate::domain::report::Severity;

use super::super::super::inputs::CodeInputFailureInput;
use super::super::check;

#[test]
fn emits_exact_error_for_direct_input_failure_surface() {
    let input = CodeInputFailureInput {
        rel_path: "src/lib.rs",
        message: "Failed to parse Rust source file: unexpected token",
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CODE-30");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "code-family input failure");
    assert_eq!(result.file.as_deref(), Some("src/lib.rs"));
    assert_eq!(result.line, None);
    assert_eq!(
        result.message,
        "Failed to parse Rust source file: unexpected token"
    );
    assert!(!result.inventory);
}
