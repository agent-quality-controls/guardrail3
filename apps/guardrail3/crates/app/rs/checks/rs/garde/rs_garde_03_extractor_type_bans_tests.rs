use crate::domain::report::Severity;

use super::super::inputs::GardeRootInput;
use super::super::test_support::root_facts;
use super::check;

#[test]
fn warns_when_extractor_type_bans_missing() {
    let mut facts = root_facts(true);
    facts.clippy_parsed = Some(toml::from_str("disallowed-types = []").expect("parse"));
    let mut results = Vec::new();
    check(&GardeRootInput::new(&facts), &mut results);
    assert_eq!(results[0].id, "RS-GARDE-03");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(results[0].message.contains("axum::extract::Json"));
}

#[test]
fn inventories_when_extractor_type_bans_present() {
    let mut facts = root_facts(true);
    facts.clippy_parsed = Some(
        toml::from_str(
            r#"
disallowed-types = [
  { path = "axum::extract::Json", reason = "x" },
  { path = "axum::Json", reason = "x" },
  { path = "axum::extract::Query", reason = "x" },
  { path = "axum::extract::Form", reason = "x" },
  { path = "axum::extract::Path", reason = "x" },
  { path = "axum::extract::Multipart", reason = "x" },
  { path = "axum::extract::ConnectInfo", reason = "x" },
  { path = "axum_extra::extract::CookieJar", reason = "x" },
  { path = "axum_extra::extract::cookie::Cookie", reason = "x" },
  { path = "axum_extra::extract::TypedHeader", reason = "x" },
  { path = "axum_extra::extract::JsonDeserializer", reason = "x" },
  { path = "axum_extra::extract::JsonLines", reason = "x" },
  { path = "axum_extra::extract::Protobuf", reason = "x" },
  { path = "axum_extra::extract::Cbor", reason = "x" },
  { path = "axum_extra::extract::MsgPack", reason = "x" },
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
