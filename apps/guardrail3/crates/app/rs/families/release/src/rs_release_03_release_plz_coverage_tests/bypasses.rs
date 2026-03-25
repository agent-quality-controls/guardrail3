use guardrail3_domain_report::Severity;

use super::super::super::test_support::{repo_facts, repo_input};
use super::super::check;

#[test]
fn stays_quiet_when_release_plz_is_absent_and_existence_is_owned_elsewhere() {
    let facts = repo_facts();
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty());
}

#[test]
fn warns_when_workspace_section_is_missing() {
    let mut facts = repo_facts();
    facts.release_plz_exists = true;
    facts.release_plz_parsed = Some(toml::Value::Table(Default::default()));
    let _ = facts.publishable_crate_names.insert("cli".to_owned());
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|result| result.id == "RS-RELEASE-03"));
    assert!(
        results
            .iter()
            .all(|result| result.severity == Severity::Warn)
    );
    assert!(results.iter().all(|result| !result.inventory));
    assert!(
        results
            .iter()
            .all(|result| result.file.as_deref() == Some("release-plz.toml"))
    );
    assert!(
        results
            .iter()
            .any(|result| result.title.contains("[workspace]"))
    );
    assert!(results.iter().any(|result| result.title.contains("cli")));
}

#[test]
fn warns_when_workspace_key_exists_but_is_not_a_table() {
    let mut facts = repo_facts();
    facts.release_plz_exists = true;
    facts.release_plz_parsed = Some(
        toml::from_str(
            r#"
workspace = "oops"
"#,
        )
        .expect("valid release-plz"),
    );
    let _ = facts.publishable_crate_names.insert("cli".to_owned());
    let _ = facts.release_plz_package_names.insert("cli".to_owned());
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-03");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("release-plz.toml"));
    assert!(results[0].title.contains("[workspace]"));
}

#[test]
fn warns_once_per_missing_publishable_crate_and_ignores_non_publishable_ones() {
    let mut facts = repo_facts();
    facts.release_plz_exists = true;
    facts.release_plz_parsed = Some(
        toml::from_str(
            r#"
[workspace]
changelog_config = "cliff.toml"
git_release_enable = true
release_always = false
"#,
        )
        .expect("valid release-plz"),
    );
    let _ = facts.publishable_crate_names.insert("api".to_owned());
    let _ = facts.publishable_crate_names.insert("cli".to_owned());
    let _ = facts.release_plz_package_names.insert("api".to_owned());
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-03");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert!(results[0].title.contains("cli"));
    assert_eq!(results[0].file.as_deref(), Some("release-plz.toml"));
}

#[test]
fn warns_once_per_each_missing_publishable_crate() {
    let mut facts = repo_facts();
    facts.release_plz_exists = true;
    facts.release_plz_parsed = Some(
        toml::from_str(
            r#"
[workspace]
changelog_config = "cliff.toml"
git_release_enable = true
release_always = false
"#,
        )
        .expect("valid release-plz"),
    );
    let _ = facts.publishable_crate_names.insert("api".to_owned());
    let _ = facts.publishable_crate_names.insert("cli".to_owned());
    let _ = facts.publishable_crate_names.insert("worker".to_owned());
    let _ = facts.release_plz_package_names.insert("api".to_owned());
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|result| result.id == "RS-RELEASE-03"));
    assert!(
        results
            .iter()
            .all(|result| result.severity == Severity::Warn)
    );
    assert!(results.iter().all(|result| !result.inventory));
    assert!(
        results
            .iter()
            .all(|result| result.file.as_deref() == Some("release-plz.toml"))
    );
    assert!(results.iter().any(|result| result.title.contains("cli")));
    assert!(results.iter().any(|result| result.title.contains("worker")));
}

#[test]
fn stays_quiet_when_release_plz_is_present_but_unparseable_and_fail_closed_is_owned_elsewhere() {
    let mut facts = repo_facts();
    facts.release_plz_exists = true;
    facts.release_plz_parsed = None;
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty());
}

#[test]
fn warns_when_changelog_config_is_not_canonical() {
    let mut facts = repo_facts();
    facts.release_plz_exists = true;
    facts.release_plz_parsed = Some(
        toml::from_str(
            r#"
[workspace]
changelog_config = "other.toml"
git_release_enable = true
release_always = false
"#,
        )
        .expect("valid release-plz"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-03");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("release-plz.toml"));
    assert!(results[0].title.contains("changelog_config"));
}

#[test]
fn warns_when_changelog_config_is_missing() {
    let mut facts = repo_facts();
    facts.release_plz_exists = true;
    facts.release_plz_parsed = Some(
        toml::from_str(
            r#"
[workspace]
git_release_enable = true
release_always = false
"#,
        )
        .expect("valid release-plz"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-03");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("release-plz.toml"));
    assert!(results[0].title.contains("changelog_config"));
}

#[test]
fn warns_when_git_release_enable_is_missing() {
    let mut facts = repo_facts();
    facts.release_plz_exists = true;
    facts.release_plz_parsed = Some(
        toml::from_str(
            r#"
[workspace]
changelog_config = "cliff.toml"
release_always = false
"#,
        )
        .expect("valid release-plz"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-03");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("release-plz.toml"));
    assert!(results[0].title.contains("git_release_enable"));
}

#[test]
fn warns_when_git_release_enable_is_false() {
    let mut facts = repo_facts();
    facts.release_plz_exists = true;
    facts.release_plz_parsed = Some(
        toml::from_str(
            r#"
[workspace]
changelog_config = "cliff.toml"
git_release_enable = false
release_always = false
"#,
        )
        .expect("valid release-plz"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-03");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("release-plz.toml"));
    assert!(results[0].title.contains("git_release_enable"));
}

#[test]
fn warns_when_release_always_is_not_false() {
    let mut facts = repo_facts();
    facts.release_plz_exists = true;
    facts.release_plz_parsed = Some(
        toml::from_str(
            r#"
[workspace]
changelog_config = "cliff.toml"
git_release_enable = true
release_always = true
"#,
        )
        .expect("valid release-plz"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-03");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("release-plz.toml"));
    assert!(results[0].title.contains("release_always"));
}

#[test]
fn warns_when_release_always_is_missing() {
    let mut facts = repo_facts();
    facts.release_plz_exists = true;
    facts.release_plz_parsed = Some(
        toml::from_str(
            r#"
[workspace]
changelog_config = "cliff.toml"
git_release_enable = true
"#,
        )
        .expect("valid release-plz"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-03");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("release-plz.toml"));
    assert!(results[0].title.contains("release_always"));
}

#[test]
fn accumulates_workspace_baseline_and_missing_package_warnings_together() {
    let mut facts = repo_facts();
    facts.release_plz_exists = true;
    facts.release_plz_parsed = Some(
        toml::from_str(
            r#"
[workspace]
changelog_config = "other.toml"
git_release_enable = false
release_always = true
"#,
        )
        .expect("valid release-plz"),
    );
    let _ = facts.publishable_crate_names.insert("api".to_owned());
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 4);
    assert!(results.iter().all(|result| result.id == "RS-RELEASE-03"));
    assert!(
        results
            .iter()
            .all(|result| result.severity == Severity::Warn)
    );
    assert!(results.iter().all(|result| !result.inventory));
    assert!(
        results
            .iter()
            .all(|result| result.file.as_deref() == Some("release-plz.toml"))
    );
    assert!(
        results
            .iter()
            .any(|result| result.title.contains("changelog_config"))
    );
    assert!(
        results
            .iter()
            .any(|result| result.title.contains("git_release_enable"))
    );
    assert!(
        results
            .iter()
            .any(|result| result.title.contains("release_always"))
    );
    assert!(results.iter().any(|result| result.title.contains("api")));
}
