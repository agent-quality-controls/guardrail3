use super::super::super::test_support::canonical_deny_toml_service;

#[test]
fn generated_sources_baseline_keeps_allow_git_empty() {
    let parsed =
        toml::from_str::<toml::Value>(&canonical_deny_toml_service()).expect("valid deny TOML");
    let sources = parsed.get("sources").expect("sources section");
    let allow_git = sources
        .get("allow-git")
        .and_then(toml::Value::as_array)
        .cloned()
        .unwrap_or_default();

    assert!(
        allow_git.is_empty(),
        "expected generated allow-git baseline to stay empty: {allow_git:#?}"
    );
}
