use crate::domain::report::Severity;

use super::super::super::test_support::{repo_facts, repo_input};
use super::super::check;

#[test]
fn warns_when_workspace_section_is_missing() {
    let mut facts = repo_facts();
    facts.release_plz_exists = true;
    facts.release_plz_parsed = Some(toml::Value::Table(Default::default()));
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-03");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("release-plz.toml"));
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
    facts.release_plz_has_workspace = true;
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
    facts.release_plz_has_workspace = true;
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Warn);
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
    facts.release_plz_has_workspace = true;
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Warn);
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
    facts.release_plz_has_workspace = true;
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(results[0].title.contains("release_always"));
}
