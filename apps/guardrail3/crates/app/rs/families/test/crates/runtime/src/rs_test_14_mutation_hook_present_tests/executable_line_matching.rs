#[allow(unused_imports)]
use guardrail3_app_rs_family_test_assertions::rs_test_14_mutation_hook_present::{
    assert_missing_mutation_hook, assert_rule_files, assert_rule_quiet,
};

#[allow(unused_imports)]
use super::{run_family, tempdir, write_file};
#[test]
fn comment_echo_and_version_only_mentions_do_not_count() {
    for hook_body in [
        "# cargo mutants should run here later\n",
        "#!/bin/sh\necho cargo mutants\n",
        "#!/bin/sh\ncargo mutants --version\n",
        "#!/bin/sh\ncargo-mutants --help\n",
    ] {
        let fixture = tempdir();
        write_file(
            fixture.path(),
            "Cargo.toml",
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[profile.mutants]\ninherits = \"dev\"\n",
        );
        write_file(fixture.path(), ".githooks/pre-commit", hook_body);

        let results = run_family(fixture.path());
        assert_rule_files(&results, vec!["Cargo.toml".to_owned()]);
        assert_missing_mutation_hook(&results);
    }
}

#[test]
fn real_mutation_commands_count_in_common_shell_forms() {
    for hook_body in [
        "#!/bin/sh\ncargo mutants\n",
        "#!/bin/sh\nCARGO_TERM_COLOR=always cargo mutants -- -p demo\n",
        "#!/bin/sh\nenv CARGO_TERM_COLOR=always cargo +nightly mutants -- -p demo\n",
        "#!/bin/sh\n./bin/cargo-mutants -- -p demo\n",
    ] {
        let fixture = tempdir();
        write_file(
            fixture.path(),
            "Cargo.toml",
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[profile.mutants]\ninherits = \"dev\"\n",
        );
        write_file(fixture.path(), ".githooks/pre-commit", hook_body);

        let results = run_family(fixture.path());
        assert_rule_quiet(&results);
    }
}
