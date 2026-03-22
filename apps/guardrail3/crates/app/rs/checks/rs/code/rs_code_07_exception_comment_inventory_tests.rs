use crate::domain::report::Severity;

use super::super::inputs::ExceptionCommentInput;
use super::check;

#[test]
fn inventories_exception_comment() {
    let input = ExceptionCommentInput {
        rel_path: "Cargo.toml",
        line: 4,
        line_text: "# EXCEPTION: temporary override",
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-CODE-07");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}
