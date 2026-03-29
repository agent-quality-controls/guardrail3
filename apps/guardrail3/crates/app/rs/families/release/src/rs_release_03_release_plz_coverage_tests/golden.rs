use guardrail3_app_rs_family_release_assertions::rs_release_03_release_plz_coverage as assertions;

use super::super::{repo_facts, repo_input};
use super::super::check;

#[test]
fn inventories_when_release_plz_covers_all_publishable_crates() {
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
    let _ = facts.release_plz_package_names.insert("cli".to_owned());
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            file: Some("release-plz.toml"),
            inventory: Some(true),
            title_contains: Some("baseline"),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_when_release_plz_covers_all_publishable_crates_and_has_extra_entries() {
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
    let _ = facts.release_plz_package_names.insert("api".to_owned());
    let _ = facts
        .release_plz_package_names
        .insert("internal-helper".to_owned());
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            file: Some("release-plz.toml"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}
