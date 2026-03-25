use guardrail3_domain_report::Severity;

use super::super::super::test_support::{repo_facts, repo_input};
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

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-03");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("release-plz.toml"));
    assert!(results[0].title.contains("baseline"));
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

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-03");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("release-plz.toml"));
}
