use g3rs_hooks_contract_types::{G3HookCommandRequirement, G3HookRequirement};

use crate::required_contract_command_present::rule::run_case;

#[test]
fn real_cargo_fmt_check_satisfies_contract() {
    let results = run_case(
        "#!/bin/sh\ncargo fmt --check\n",
        vec![requirement(G3HookCommandRequirement::CargoFmtCheck)],
    );

    assert!(
        results
            .iter()
            .all(guardrail3_check_types::G3CheckResult::inventory),
        "valid cargo fmt command should only emit inventory"
    );
}

#[test]
fn cargo_clippy_deny_warnings_satisfies_contract() {
    let results = run_case(
        "#!/bin/sh\ncargo clippy --all-targets -- -D warnings\n",
        vec![requirement(
            G3HookCommandRequirement::CargoClippyDenyWarnings,
        )],
    );

    assert!(
        results
            .iter()
            .all(guardrail3_check_types::G3CheckResult::inventory),
        "cargo clippy -D warnings should satisfy hook contract"
    );
}

#[test]
fn cargo_deny_check_satisfies_contract() {
    let results = run_case(
        "#!/bin/sh\ncargo deny check\n",
        vec![requirement(G3HookCommandRequirement::CargoDenyCheck)],
    );

    assert!(
        results
            .iter()
            .all(guardrail3_check_types::G3CheckResult::inventory),
        "cargo deny check should satisfy hook contract"
    );
}

#[test]
fn cargo_test_satisfies_contract() {
    let results = run_case(
        "#!/bin/sh\ncargo test --workspace\n",
        vec![requirement(G3HookCommandRequirement::CargoTest)],
    );

    assert!(
        results
            .iter()
            .all(guardrail3_check_types::G3CheckResult::inventory),
        "cargo test should satisfy hook contract"
    );
}

#[test]
fn cargo_machete_satisfies_contract() {
    let results = run_case(
        "#!/bin/sh\ncargo machete\n",
        vec![requirement(G3HookCommandRequirement::CargoMachete)],
    );

    assert!(
        results
            .iter()
            .all(guardrail3_check_types::G3CheckResult::inventory),
        "cargo machete should satisfy hook contract"
    );
}

#[test]
fn cargo_dupes_satisfies_contract() {
    let results = run_case(
        "#!/bin/sh\ncargo dupes check --exclude-tests\n",
        vec![requirement(G3HookCommandRequirement::CargoDupes)],
    );

    assert!(
        results
            .iter()
            .all(guardrail3_check_types::G3CheckResult::inventory),
        "cargo dupes should satisfy hook contract"
    );
}

#[test]
fn gitleaks_satisfies_contract() {
    let results = run_case(
        "#!/bin/sh\ngitleaks detect --no-banner\n",
        vec![requirement(G3HookCommandRequirement::Gitleaks)],
    );

    assert!(
        results
            .iter()
            .all(guardrail3_check_types::G3CheckResult::inventory),
        "gitleaks should satisfy hook contract"
    );
}

#[test]
fn comment_does_not_satisfy_contract() {
    let results = run_case(
        "#!/bin/sh\n# cargo fmt --check\n",
        vec![requirement(G3HookCommandRequirement::CargoFmtCheck)],
    );

    assert!(
        results.iter().any(|result| !result.inventory()),
        "commented cargo fmt command should not satisfy hook contract"
    );
}

#[test]
fn echo_does_not_satisfy_contract() {
    let results = run_case(
        "#!/bin/sh\necho \"cargo fmt --check\"\n",
        vec![requirement(G3HookCommandRequirement::CargoFmtCheck)],
    );

    assert!(
        results.iter().any(|result| !result.inventory()),
        "echoed cargo fmt command should not satisfy hook contract"
    );
}

#[test]
fn path_qualified_g3rs_validate_satisfies_contract() {
    let results = run_case(
        "#!/bin/sh\n/usr/local/bin/g3rs validate --path .\n",
        vec![requirement(G3HookCommandRequirement::G3RsValidatePath)],
    );

    assert!(
        results
            .iter()
            .all(guardrail3_check_types::G3CheckResult::inventory),
        "path-qualified g3rs validate should satisfy hook contract"
    );
}

#[test]
fn g3rs_validate_help_does_not_satisfy_contract() {
    let results = run_case(
        "#!/bin/sh\ng3rs validate --path . --help\n",
        vec![requirement(G3HookCommandRequirement::G3RsValidatePath)],
    );

    assert!(
        results.iter().any(|result| !result.inventory()),
        "help command should not satisfy g3rs validate contract"
    );
}

#[test]
fn g3rs_validate_attached_path_option_value_does_not_satisfy_contract() {
    let results = run_case(
        "#!/bin/sh\ng3rs validate --path=--help\n",
        vec![requirement(G3HookCommandRequirement::G3RsValidatePath)],
    );

    assert!(
        results.iter().any(|result| !result.inventory()),
        "option-looking attached path value should not satisfy g3rs validate contract"
    );
}

#[test]
fn env_wrapped_cargo_dupes_exclude_tests_satisfies_contract() {
    let results = run_case(
        "env CARGO_TERM_COLOR=always cargo dupes check --exclude-tests\n",
        vec![requirement(
            G3HookCommandRequirement::CargoDupesExcludeTests,
        )],
    );

    assert!(
        results
            .iter()
            .all(guardrail3_check_types::G3CheckResult::inventory),
        "env-wrapped cargo dupes should satisfy dupes contract"
    );
}

#[test]
fn pnpm_frozen_lockfile_satisfies_concrete_lockfile_contract() {
    let results = run_case(
        "#!/bin/sh\npnpm install --frozen-lockfile\n",
        vec![requirement(
            G3HookCommandRequirement::ConcreteLockfileCommand,
        )],
    );

    assert!(
        results
            .iter()
            .all(guardrail3_check_types::G3CheckResult::inventory),
        "pnpm install --frozen-lockfile should satisfy concrete lockfile contract"
    );
}

#[test]
fn owner_families_are_reported_for_missing_command() {
    let results = run_case(
        "#!/bin/sh\ntrue\n",
        vec![requirement(G3HookCommandRequirement::CargoDenyCheck)],
    );

    assert!(
        results.iter().any(|result| {
            !result.inventory()
                && result.id() == "g3rs-hooks/required-contract-command-present"
                && result.message().contains("Owner families: test")
        }),
        "missing command finding should include rule id and owner family"
    );
}

fn requirement(command: G3HookCommandRequirement) -> G3HookRequirement {
    G3HookRequirement {
        id: "test/hook-contract".to_owned(),
        owner_family: "test".to_owned(),
        trigger_patterns: Vec::new(),
        required_commands: vec![command],
        critical_commands: Vec::new(),
    }
}
