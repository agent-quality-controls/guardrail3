use crate::domain::report::Severity;

use super::super::inputs::GardeRootInput;
use super::super::test_support::root_facts;
use super::check;

#[test]
fn warns_when_core_garde_method_bans_missing() {
    let mut facts = root_facts(true);
    facts.clippy_parsed = Some(toml::from_str("disallowed-methods = []").expect("parse"));
    let mut results = Vec::new();
    check(&GardeRootInput::new(&facts), &mut results);
    assert_eq!(results[0].id, "RS-GARDE-02");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(results[0].message.contains("serde_json::from_str"));
}

#[test]
fn inventories_when_core_garde_method_bans_present() {
    let mut facts = root_facts(true);
    facts.clippy_parsed = Some(
        toml::from_str(
            r#"
disallowed-methods = [
  { path = "serde_json::from_str", reason = "x" },
  { path = "serde_json::from_slice", reason = "x" },
  { path = "serde_json::from_value", reason = "x" },
  { path = "serde_json::from_reader", reason = "x" },
  { path = "toml::from_str", reason = "x" },
  { path = "serde_yaml::from_str", reason = "x" },
  { path = "serde_yaml::from_reader", reason = "x" },
]
"#,
        )
        .expect("parse"),
    );
    let mut results = Vec::new();
    check(&GardeRootInput::new(&facts), &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}
