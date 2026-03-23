use crate::domain::report::Severity;

use super::super::inputs::GardeRootInput;
use super::super::test_support::root_facts;
use super::check;

#[test]
fn errors_when_garde_dependency_missing() {
    let mut results = Vec::new();
    check(&GardeRootInput::new(&root_facts(false)), &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-GARDE-01");
    assert_eq!(results[0].severity, Severity::Error);
}

#[test]
fn inventories_when_garde_dependency_present() {
    let mut results = Vec::new();
    check(&GardeRootInput::new(&root_facts(true)), &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-GARDE-01");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}
