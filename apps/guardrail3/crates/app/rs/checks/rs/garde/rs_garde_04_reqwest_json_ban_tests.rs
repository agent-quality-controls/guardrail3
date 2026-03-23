use crate::domain::report::Severity;

use super::super::inputs::GardeRootInput;
use super::super::test_support::root_facts;
use super::check;

#[test]
fn warns_when_reqwest_json_ban_missing() {
    let mut facts = root_facts(true);
    facts.clippy_parsed = Some(toml::from_str("disallowed-methods = []").expect("parse"));
    let mut results = Vec::new();
    check(&GardeRootInput::new(&facts), &mut results);
    assert_eq!(results[0].id, "RS-GARDE-04");
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn inventories_when_reqwest_json_ban_present() {
    let mut facts = root_facts(true);
    facts.clippy_parsed =
        Some(toml::from_str("disallowed-methods = [{ path = \"reqwest::Response::json\", reason = \"x\" }]").expect("parse"));
    let mut results = Vec::new();
    check(&GardeRootInput::new(&facts), &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}
