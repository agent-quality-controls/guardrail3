use crate::test_support::{finding, run_family, rule_files, tempdir, write_file};
use guardrail3_domain_report::Severity;

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
        assert_eq!(rule_files(&results, "RS-TEST-14"), vec!["Cargo.toml".to_owned()]);
        let issue = finding(&results, "RS-TEST-14");
        assert_eq!(issue.severity, Severity::Warn);
        assert_eq!(issue.title, "mutation hook step missing");
        assert_eq!(issue.file.as_deref(), Some("Cargo.toml"));
        assert_eq!(issue.line, None);
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
        assert_eq!(
            rule_files(&results, "RS-TEST-14"),
            vec![".githooks/pre-commit".to_owned()]
        );
        let issue = finding(&results, "RS-TEST-14");
        assert_eq!(issue.severity, Severity::Info);
        assert_eq!(issue.title, "mutation hook step present");
        assert_eq!(issue.file.as_deref(), Some(".githooks/pre-commit"));
        assert_eq!(issue.line, None);
    }
}
