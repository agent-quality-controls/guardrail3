use super::{collected_facts, dir_entry, project_tree, run_with_facts};
use guardrail3_domain_report::Severity;

#[test]
fn inventories_migrated_dependency_policy_rules_through_family_package_bridge() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["Cargo.toml", "guardrail3.toml"])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "guardrail3.toml",
                r#"
                    [rust.apps.api]
                    profile = "service"
                    allowed_deps = ["serde"]
                "#,
            ),
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = ["apps/api"]

                    [workspace.dependencies]
                    serde = "1"
                "#,
            ),
            (
                "apps/api/Cargo.toml",
                r#"
                    [package]
                    name = "api"
                    version = "0.1.0"
                    edition = "2024"

                    [dependencies]
                    serde.workspace = true
                "#,
            ),
        ],
    );

    let facts = collected_facts(&tree, &[]);
    let results = run_with_facts(&facts);

    let deps_05 = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-05")
        .collect::<Vec<_>>();
    assert_eq!(deps_05.len(), 1);
    assert_eq!(deps_05[0].severity(), Severity::Info);
    assert!(deps_05[0].inventory());
    assert_eq!(deps_05[0].file(), Some("apps/api/Cargo.toml"));

    let deps_08 = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-08")
        .collect::<Vec<_>>();
    assert_eq!(deps_08.len(), 1);
    assert_eq!(deps_08[0].severity(), Severity::Info);
    assert!(deps_08[0].inventory());
    assert_eq!(deps_08[0].file(), Some("apps/api/Cargo.toml"));

    let deps_11 = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-11")
        .collect::<Vec<_>>();
    assert!(deps_11.is_empty(), "unexpected input failures: {deps_11:#?}");
}

#[test]
fn reports_migrated_dependency_policy_failure_through_family_package_bridge() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["Cargo.toml", "guardrail3.toml"])),
            ("apps", dir_entry(&["api"], &[])),
            ("apps/api", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "guardrail3.toml",
                r#"
                    [rust.apps.api]
                    profile = "service"
                    allowed_deps = ["serde"]
                "#,
            ),
            (
                "Cargo.toml",
                r#"
                    [workspace]
                    members = ["apps/api"]

                    [workspace.dependencies]
                    serde = "1"
                "#,
            ),
            (
                "apps/api/Cargo.toml",
                r#"
                    [package]
                    name = "api"
                    version = "0.1.0"
                    edition = "2024"

                    [dependencies]
                    reqwest = "0.12"
                "#,
            ),
        ],
    );

    let facts = collected_facts(&tree, &[]);
    let results = run_with_facts(&facts);

    let deps_05 = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-05")
        .collect::<Vec<_>>();
    assert_eq!(deps_05.len(), 1);
    assert_eq!(deps_05[0].severity(), Severity::Error);
    assert!(!deps_05[0].inventory());
    assert_eq!(deps_05[0].file(), Some("apps/api/Cargo.toml"));
    assert!(deps_05[0].message().contains("Dependency `reqwest`"));

    let deps_11 = results
        .iter()
        .filter(|result| result.id() == "RS-DEPS-11")
        .collect::<Vec<_>>();
    assert!(deps_11.is_empty(), "unexpected input failures: {deps_11:#?}");
}
