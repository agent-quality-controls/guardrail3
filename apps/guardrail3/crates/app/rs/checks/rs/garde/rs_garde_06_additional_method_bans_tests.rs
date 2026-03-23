use crate::domain::report::Severity;

use super::super::inputs::GardeRootInput;
use super::super::test_support::root_facts;
use super::check;

#[test]
fn warns_when_additional_method_bans_missing() {
    let mut facts = root_facts(true);
    facts.clippy_parsed = Some(toml::from_str("disallowed-methods = []").expect("parse"));
    let mut results = Vec::new();
    check(&GardeRootInput::new(&facts), &mut results);
    assert_eq!(results[0].id, "RS-GARDE-06");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(results[0].message.contains("serde_qs::from_str"));
}

#[test]
fn inventories_when_additional_method_bans_present() {
    let mut facts = root_facts(true);
    facts.clippy_parsed = Some(
        toml::from_str(
            r#"
disallowed-methods = [
  { path = "serde_qs::from_str", reason = "x" },
  { path = "serde_qs::from_bytes", reason = "x" },
  { path = "serde_urlencoded::from_str", reason = "x" },
  { path = "serde_urlencoded::from_bytes", reason = "x" },
  { path = "serde_urlencoded::from_reader", reason = "x" },
  { path = "ciborium::from_reader", reason = "x" },
  { path = "ciborium::de::from_reader", reason = "x" },
  { path = "rmp_serde::from_slice", reason = "x" },
  { path = "rmp_serde::from_read", reason = "x" },
  { path = "rmp_serde::decode::from_slice", reason = "x" },
  { path = "rmp_serde::decode::from_read", reason = "x" },
  { path = "bincode::deserialize", reason = "x" },
  { path = "bincode::deserialize_from", reason = "x" },
  { path = "bincode::serde::decode_from_slice", reason = "x" },
  { path = "bincode::serde::decode_from_reader", reason = "x" },
  { path = "csv::Reader::deserialize", reason = "x" },
  { path = "csv::StringRecord::deserialize", reason = "x" },
  { path = "csv::ByteRecord::deserialize", reason = "x" },
  { path = "serde_xml_rs::from_str", reason = "x" },
  { path = "serde_xml_rs::from_reader", reason = "x" },
  { path = "quick_xml::de::from_str", reason = "x" },
  { path = "quick_xml::de::from_reader", reason = "x" },
  { path = "ron::from_str", reason = "x" },
  { path = "ron::de::from_str", reason = "x" },
  { path = "serde_cbor::from_slice", reason = "x" },
  { path = "serde_cbor::from_reader", reason = "x" },
  { path = "postcard::from_bytes", reason = "x" },
  { path = "flexbuffers::from_slice", reason = "x" },
  { path = "serde_json::Deserializer::from_str", reason = "x" },
  { path = "serde_json::Deserializer::from_slice", reason = "x" },
  { path = "serde_json::Deserializer::from_reader", reason = "x" },
  { path = "toml_edit::de::from_str", reason = "x" },
  { path = "toml_edit::de::from_slice", reason = "x" },
  { path = "toml_edit::de::from_document", reason = "x" },
  { path = "config::Config::try_deserialize", reason = "x" },
  { path = "figment::Figment::extract", reason = "x" },
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
