use crate::domain::report::Severity;

use super::super::super::inputs::ExceptionCommentInput;
use super::super::check;

#[test]
fn inventories_direct_exception_comment_input() {
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
