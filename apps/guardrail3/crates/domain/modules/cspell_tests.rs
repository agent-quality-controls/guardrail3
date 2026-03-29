use super::*;

#[test]
#[allow(clippy::disallowed_methods)] // reason: test — parsing generated JSON to verify structure
fn cspell_config_is_valid_json() {
    let json = build_cspell_config();
    let parsed = serde_json::from_str::<serde_json::Value>(&json);
    assert!(
        parsed.is_ok(),
        "cspell config must be valid JSON: {parsed:?}"
    );
    let value = match parsed {
        Ok(value) => value,
        Err(_) => return,
    };
    let Some(obj) = value.as_object() else {
        assert!(false, "root must be a JSON object");
        return;
    };
    assert_eq!(
        obj.get("version").and_then(|v| v.as_str()),
        Some("0.2"),
        "version field must be 0.2"
    );
    assert_eq!(
        obj.get("language").and_then(|v| v.as_str()),
        Some("en"),
        "language field must be en"
    );
    assert!(
        obj.get("ignorePaths").and_then(|v| v.as_array()).is_some(),
        "ignorePaths must be an array"
    );
    assert!(
        obj.get("words").and_then(|v| v.as_array()).is_some(),
        "words must be an array"
    );
}

#[test]
fn cspell_module_matches_builder() {
    assert_eq!(CSPELL_CONFIG.content, build_cspell_config());
}
