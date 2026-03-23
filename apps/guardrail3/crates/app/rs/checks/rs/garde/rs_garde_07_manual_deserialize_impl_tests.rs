use crate::domain::report::Severity;

use super::super::inputs::ManualDeserializeImplInput;
use super::super::test_support::manual_impl;
use super::check;

#[test]
fn errors_when_manual_deserialize_impl_needs_validate() {
    let mut results = Vec::new();
    check(&ManualDeserializeImplInput::new(&manual_impl(true, false)), &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-GARDE-07");
    assert_eq!(results[0].severity, Severity::Error);
}

#[test]
fn skips_manual_deserialize_impl_when_validate_present() {
    let mut results = Vec::new();
    check(&ManualDeserializeImplInput::new(&manual_impl(true, true)), &mut results);
    assert!(results.is_empty());
}
