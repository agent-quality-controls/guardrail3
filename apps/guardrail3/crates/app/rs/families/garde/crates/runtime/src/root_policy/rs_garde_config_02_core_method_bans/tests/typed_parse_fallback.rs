use guardrail3_app_rs_family_garde_assertions::{
    rs_garde_config_02_core_method_bans as garde_02, rs_garde_config_03_extractor_type_bans as garde_03,
    rs_garde_config_04_reqwest_json_ban as garde_04, rs_garde_config_05_additional_method_bans as garde_06,
};
use guardrail3_domain_report::Severity;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn typed_clippy_parse_failure_falls_back_to_rule_specific_verification_warnings() {
    let root = temp_root("typed-parse-fallback-garde-root-policy");
    let mut parsed = toml::from_str::<toml::Value>(&super::helpers::canonical_clippy_toml())
        .expect("valid clippy TOML");
    let _ = parsed
        .as_table_mut()
        .expect("top-level clippy table")
        .insert("unknown-key".to_owned(), toml::Value::Boolean(true));
    let clippy_toml = toml::to_string(&parsed).expect("serialize clippy TOML");

    let tree = project_tree(
        vec![("", dir_entry(&[], &["Cargo.toml", "clippy.toml"]))],
        vec![
            (
                "Cargo.toml",
                "[workspace]\nmembers = []\n[package]\nname = \"demo\"\nversion = \"0.1.0\"\n[dependencies]\ngarde = \"0.1\"\n",
            ),
            ("clippy.toml", clippy_toml.as_str()),
        ],
        root.clone(),
    );

    let results = super::helpers::run_family(&tree);

    garde_02::assert_rule_results(
        &results,
        &[garde_02::ExpectedRuleResult {
            severity: Some(Severity::Warn),
            title: Some("cannot verify core garde method bans"),
            file: Some("clippy.toml"),
            message_contains: Some("Failed to parse `clippy.toml` for garde clippy-ban validation"),
            ..Default::default()
        }],
    );
    garde_03::assert_rule_results(
        &results,
        &[garde_03::ExpectedRuleResult {
            severity: Some(Severity::Warn),
            title: Some("cannot verify garde extractor bans"),
            file: Some("clippy.toml"),
            message_contains: Some("Failed to parse `clippy.toml` for garde clippy-ban validation"),
            ..Default::default()
        }],
    );
    garde_04::assert_rule_results(
        &results,
        &[garde_04::ExpectedRuleResult {
            severity: Some(Severity::Warn),
            title: Some("cannot verify reqwest garde ban"),
            file: Some("clippy.toml"),
            message_contains: Some("Failed to parse `clippy.toml` for garde clippy-ban validation"),
            ..Default::default()
        }],
    );
    garde_06::assert_rule_results(
        &results,
        &[garde_06::ExpectedRuleResult {
            severity: Some(Severity::Warn),
            title: Some("cannot verify additional garde method bans"),
            file: Some("clippy.toml"),
            message_contains: Some("Failed to parse `clippy.toml` for garde clippy-ban validation"),
            ..Default::default()
        }],
    );

    std::fs::remove_dir_all(&root).expect("remove temp root");
}
