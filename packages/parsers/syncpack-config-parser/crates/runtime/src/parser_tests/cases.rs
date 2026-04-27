use syncpack_config_parser_runtime_assertions::parser::{
    SyncpackVersionGroup, assert_group_exact, assert_has_banned_group, assert_has_pinned_group,
    assert_invalid_document, assert_parsed_document, assert_source,
};

#[test]
fn parses_syncpack_policy_groups() {
    let document = super::super::parse_document(
        r#"
        {
          "$schema": "./node_modules/syncpack/schema.json",
          "source": ["package.json", "apps/*/package.json"],
          "versionGroups": [
            {
              "label": "Pin Astro",
              "dependencies": ["astro"],
              "dependencyTypes": ["prod", "dev"],
              "pinVersion": "6.1.9"
            },
            {
              "label": "Ban foreign stacks",
              "dependencies": ["next", "velite", "eslint-mdx"],
              "isBanned": true
            }
          ]
        }
        "#,
    )
    .expect("Syncpack config JSON should parse");

    assert_parsed_document(&document);
    assert_source(&document, &["package.json", "apps/*/package.json"]);
    assert_group_exact(
        &document,
        0,
        &SyncpackVersionGroup {
            label: Some("Pin Astro".to_owned()),
            dependencies: vec!["astro".to_owned()],
            dependency_types: vec!["prod".to_owned(), "dev".to_owned()],
            packages: None,
            specifier_types: None,
            pin_version: Some("6.1.9".to_owned()),
            is_banned: None,
            is_ignored: None,
        },
    );
    assert_has_pinned_group(&document, "astro", "6.1.9");
    assert_has_banned_group(&document, "next");
    assert_has_banned_group(&document, "velite");
    assert_has_banned_group(&document, "eslint-mdx");
}

#[test]
fn parses_packages_and_ignored_groups() {
    let document = super::super::parse_document(
        r#"
        {
          "source": ["package.json"],
          "versionGroups": [
            {
              "label": "Ignored package scoped group",
              "packages": ["landing"],
              "dependencies": ["astro"],
              "dependencyTypes": ["**"],
              "specifierTypes": ["!range"],
              "pinVersion": "6.1.9",
              "isIgnored": true
            }
          ]
        }
        "#,
    )
    .expect("Syncpack config JSON should parse");

    assert_source(&document, &["package.json"]);
    assert_group_exact(
        &document,
        0,
        &SyncpackVersionGroup {
            label: Some("Ignored package scoped group".to_owned()),
            dependencies: vec!["astro".to_owned()],
            dependency_types: vec!["**".to_owned()],
            packages: Some(vec!["landing".to_owned()]),
            specifier_types: Some(vec!["!range".to_owned()]),
            pin_version: Some("6.1.9".to_owned()),
            is_banned: None,
            is_ignored: Some(true),
        },
    );
}

#[test]
fn preserves_explicit_empty_and_false_policy_fields() {
    let document = super::super::parse_document(
        r#"
        {
          "source": ["package.json"],
          "versionGroups": [
            {
              "dependencies": ["astro"],
              "dependencyTypes": ["prod", "dev"],
              "packages": [],
              "specifierTypes": [],
              "pinVersion": "6.1.9",
              "isBanned": false,
              "isIgnored": false
            }
          ]
        }
        "#,
    )
    .expect("Syncpack config JSON should parse");

    assert_group_exact(
        &document,
        0,
        &SyncpackVersionGroup {
            label: None,
            dependencies: vec!["astro".to_owned()],
            dependency_types: vec!["prod".to_owned(), "dev".to_owned()],
            packages: Some(Vec::new()),
            specifier_types: Some(Vec::new()),
            pin_version: Some("6.1.9".to_owned()),
            is_banned: Some(false),
            is_ignored: Some(false),
        },
    );
}

#[test]
fn missing_version_groups_parses_as_empty_policy() {
    let document = super::super::parse_document(
        r#"
        {
          "source": ["package.json"]
        }
        "#,
    )
    .expect("Syncpack config JSON should parse");

    assert_parsed_document(&document);
}

#[test]
fn rejects_non_object_root() {
    let document = super::super::parse_document(r#"[]"#)
        .expect("Syncpack config JSON should still produce a document");

    assert_invalid_document(&document, "root must be a JSON object");
}

#[test]
fn rejects_non_array_version_groups() {
    let document = super::super::parse_document(
        r#"
        {
          "versionGroups": true
        }
        "#,
    )
    .expect("Syncpack config JSON should still produce a document");

    assert_invalid_document(&document, "`versionGroups` must be an array");
}

#[test]
fn rejects_non_array_source() {
    for (document, reason) in [
        (
            r#"
            {
              "source": "package.json"
            }
            "#,
            "`source` must be an array",
        ),
        (
            r#"
            {
              "source": [true]
            }
            "#,
            "`source[0]` must be a string",
        ),
    ] {
        let document = super::super::parse_document(document)
            .expect("Syncpack config JSON should still produce a document");

        assert_invalid_document(&document, reason);
    }
}

#[test]
fn rejects_non_object_version_group_entry() {
    let document = super::super::parse_document(
        r#"
        {
          "versionGroups": [true]
        }
        "#,
    )
    .expect("Syncpack config JSON should still produce a document");

    assert_invalid_document(&document, "`versionGroups[0]` must be an object");
}

#[test]
fn rejects_non_string_dependency_entry() {
    let document = super::super::parse_document(
        r#"
        {
          "versionGroups": [
            {
              "dependencies": [true],
              "pinVersion": "6.1.9"
            }
          ]
        }
        "#,
    )
    .expect("Syncpack config JSON should still produce a document");

    assert_invalid_document(&document, "`dependencies[0]` must be a string");
}

#[test]
fn rejects_invalid_structural_version_group_field_types() {
    for (document, reason) in [
        (
            r#"{ "versionGroups": [{ "label": true }] }"#,
            "`versionGroups[0].label` must be a string",
        ),
        (
            r#"{ "versionGroups": [{ "dependencyTypes": [true] }] }"#,
            "`dependencyTypes[0]` must be a string",
        ),
        (
            r#"{ "versionGroups": [{ "packages": [true] }] }"#,
            "`packages[0]` must be a string",
        ),
        (
            r#"{ "versionGroups": [{ "specifierTypes": [true] }] }"#,
            "`specifierTypes[0]` must be a string",
        ),
        (
            r#"{ "versionGroups": [{ "isIgnored": "yes" }] }"#,
            "`versionGroups[0].isIgnored` must be a boolean",
        ),
    ] {
        let document = super::super::parse_document(document)
            .expect("Syncpack config JSON should still produce a document");

        assert_invalid_document(&document, reason);
    }
}

#[test]
fn rejects_invalid_pin_version_and_banned_types() {
    for (document, reason) in [
        (
            r#"{ "versionGroups": [{ "dependencies": ["astro"], "pinVersion": true }] }"#,
            "`versionGroups[0].pinVersion` must be a string",
        ),
        (
            r#"{ "versionGroups": [{ "dependencies": ["astro"], "isBanned": "yes" }] }"#,
            "`versionGroups[0].isBanned` must be a boolean",
        ),
    ] {
        let document = super::super::parse_document(document)
            .expect("Syncpack config JSON should still produce a document");

        assert_invalid_document(&document, reason);
    }
}
