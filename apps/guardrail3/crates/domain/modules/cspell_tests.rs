use super::*;

#[test]
#[allow(clippy::disallowed_methods)] // reason: test — parsing generated JSON to verify structure
#[allow(clippy::panic)] // reason: test — panic on invalid JSON indicates broken generation
fn cspell_config_is_valid_json() {
    let json = build_cspell_config();
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&json) else {
        panic!("cspell config must be valid JSON");
    };
    let Some(obj) = value.as_object() else {
        panic!("root must be a JSON object");
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
