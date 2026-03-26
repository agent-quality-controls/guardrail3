#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_13_mutants_profile_present::{assert_inventory, assert_reported, assert_rule_files, assert_rule_quiet};

#[allow(unused_imports)]
use super::{run_family, tempdir, write_file};
use guardrail3_domain_report::Severity;

#[test]
fn hook_only_mutation_adoption_requires_the_mutants_profile() {let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(root, ".cargo/mutants.toml", "timeout_multiplier = 2.0\n");
    write_file(root, ".githooks/pre-commit", "#!/bin/sh\ncargo mutants\n");

    let results = run_family(root);

    assert_rule_files(&results, vec!["Cargo.toml".to_owned()]
    );    assert_reported(&results, "Cargo.toml", None, Severity::Warn, "profile.mutants missing");
    assert_inventory(&results, false);}