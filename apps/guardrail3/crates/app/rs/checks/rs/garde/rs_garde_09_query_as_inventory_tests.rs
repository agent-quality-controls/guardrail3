use crate::domain::report::Severity;

use super::super::inputs::QueryAsMacroInput;
use super::super::test_support::query_as_macro;
use super::check;

#[test]
fn inventories_query_as_usage() {
    let mut results = Vec::new();
    check(&QueryAsMacroInput::new(&query_as_macro()), &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-GARDE-09");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}
