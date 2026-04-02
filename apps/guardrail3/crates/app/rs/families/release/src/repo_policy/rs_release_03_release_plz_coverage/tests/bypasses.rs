use guardrail3_app_rs_family_release_assertions::repo_policy::rs_release_03_release_plz_coverage as assertions;

use super::super::check;
use super::super::{repo_facts, repo_input};

#[test]
fn stays_quiet_when_release_plz_is_absent_and_existence_is_owned_elsewhere() {
    let facts = repo_facts();
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(assertions::findings(&results).is_empty());
    assertions::assert_rule_quiet(&results);
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

    assert_eq!(assertions::findings(&results).len(), 2);
    assertions::assert_rule_count(&results, 2);
    assertions::assert_rule_results(
        &results,
        &[
            assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Warn),
                title_contains: Some("[workspace]"),
                file: Some("release-plz.toml"),
                inventory: Some(false),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Warn),
                title_contains: Some("cli"),
                file: Some("release-plz.toml"),
                inventory: Some(false),
                ..Default::default()
            },
        ],
    );
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
        .expect("failed to parse release-plz fixture"),
    );
    let _ = facts.publishable_crate_names.insert("cli".to_owned());
    let _ = facts.release_plz_package_names.insert("cli".to_owned());
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title_contains: Some("[workspace]"),
            file: Some("release-plz.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
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
        .expect("failed to parse release-plz fixture"),
    );
    let _ = facts.publishable_crate_names.insert("api".to_owned());
    let _ = facts.publishable_crate_names.insert("cli".to_owned());
    let _ = facts.release_plz_package_names.insert("api".to_owned());
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title_contains: Some("cli"),
            file: Some("release-plz.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
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
        .expect("failed to parse release-plz fixture"),
    );
    let _ = facts.publishable_crate_names.insert("api".to_owned());
    let _ = facts.publishable_crate_names.insert("cli".to_owned());
    let _ = facts.publishable_crate_names.insert("worker".to_owned());
    let _ = facts.release_plz_package_names.insert("api".to_owned());
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(assertions::findings(&results).len(), 2);
    assertions::assert_rule_count(&results, 2);
    assertions::assert_rule_results(
        &results,
        &[
            assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Warn),
                title_contains: Some("cli"),
                file: Some("release-plz.toml"),
                inventory: Some(false),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Warn),
                title_contains: Some("worker"),
                file: Some("release-plz.toml"),
                inventory: Some(false),
                ..Default::default()
            },
        ],
    );
}

#[test]
fn stays_quiet_when_release_plz_is_present_but_unparseable_and_fail_closed_is_owned_elsewhere() {
    let mut facts = repo_facts();
    facts.release_plz_exists = true;
    facts.release_plz_parsed = None;
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(assertions::findings(&results).is_empty());
    assertions::assert_rule_quiet(&results);
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
        .expect("failed to parse release-plz fixture"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title_contains: Some("changelog_config"),
            file: Some("release-plz.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
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
        .expect("failed to parse release-plz fixture"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title_contains: Some("changelog_config"),
            file: Some("release-plz.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
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
        .expect("failed to parse release-plz fixture"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title_contains: Some("git_release_enable"),
            file: Some("release-plz.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
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
        .expect("failed to parse release-plz fixture"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title_contains: Some("git_release_enable"),
            file: Some("release-plz.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
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
        .expect("failed to parse release-plz fixture"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title_contains: Some("release_always"),
            file: Some("release-plz.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
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
        .expect("failed to parse release-plz fixture"),
    );
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title_contains: Some("release_always"),
            file: Some("release-plz.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
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
        .expect("failed to parse release-plz fixture"),
    );
    let _ = facts.publishable_crate_names.insert("api".to_owned());
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(assertions::findings(&results).len(), 4);
    assertions::assert_rule_count(&results, 4);
    assertions::assert_rule_results(
        &results,
        &[
            assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Warn),
                title_contains: Some("changelog_config"),
                file: Some("release-plz.toml"),
                inventory: Some(false),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Warn),
                title_contains: Some("git_release_enable"),
                file: Some("release-plz.toml"),
                inventory: Some(false),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Warn),
                title_contains: Some("release_always"),
                file: Some("release-plz.toml"),
                inventory: Some(false),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Warn),
                title_contains: Some("api"),
                file: Some("release-plz.toml"),
                inventory: Some(false),
                ..Default::default()
            },
        ],
    );
}
