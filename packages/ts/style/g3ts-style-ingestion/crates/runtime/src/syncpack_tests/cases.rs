#[test]
fn canonical_style_policy_pin_group_is_required() {
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

    g3ts_style_ingestion_assertions::syncpack::assert_canonical_pin_group_accepted(
        super::super::has_one_canonical_pin_group(
            &[group],
            "g3ts-eslint-plugin-style-policy",
            "0.1.3",
            &["prod", "dev"],
        )
    );
}

#[test]
fn non_canonical_style_policy_pin_group_is_rejected() {
    let group = syncpack_config_parser::types::SyncpackVersionGroup {
        label: Some("style policy".to_owned()),
        dependencies: vec!["g3ts-eslint-plugin-style-policy".to_owned()],
        dependency_types: vec!["dev".to_owned()],
        packages: None,
        specifier_types: None,
        pin_version: Some("0.1.3".to_owned()),
        is_banned: None,
        is_ignored: None,
    };

    g3ts_style_ingestion_assertions::syncpack::assert_canonical_pin_group_rejected(
        super::super::has_one_canonical_pin_group(
            &[group],
            "g3ts-eslint-plugin-style-policy",
            "0.1.3",
            &["prod", "dev"],
        )
    );
}
