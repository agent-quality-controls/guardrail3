#[test]
fn syncpack_version_group_is_normalized_without_policy_verdicts() {
    let group = syncpack_config_parser::types::SyncpackVersionGroup {
        label: Some("style policy".to_owned()),
        dependencies: vec!["g3ts-eslint-plugin-style-policy".to_owned()],
        dependency_types: vec!["prod".to_owned(), "dev".to_owned()],
        packages: None,
        specifier_types: None,
        pin_version: Some("0.1.3".to_owned()),
        is_banned: None,
        is_ignored: None,
    };

    let normalized = super::super::syncpack_version_group(group);

    g3ts_style_ingestion_assertions::syncpack::assert_version_group_dependency(
        &normalized,
        "g3ts-eslint-plugin-style-policy",
    );
    g3ts_style_ingestion_assertions::syncpack::assert_version_group_pin(
        &normalized,
        "0.1.3",
    );
}
