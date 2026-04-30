use g3rs_hooks_contract_types::{G3HookCommandRequirement, G3HookRequirement};
use g3rs_hooks_source_checks_assertions::required_contract_command_present::rule as assertions;

use super::super::run_case;

#[test]
fn real_cargo_fmt_check_satisfies_contract() {
    let results = run_case(
        "#!/bin/sh\ncargo fmt --check\n",
        vec![requirement(G3HookCommandRequirement::CargoFmtCheck)],
    );

    assertions::assert_single_inventory(&results, "cargo fmt --check", "test");
}

#[test]
fn cargo_clippy_deny_warnings_satisfies_contract() {
    let results = run_case(
        "#!/bin/sh\ncargo clippy --all-targets -- -D warnings\n",
        vec![requirement(
            G3HookCommandRequirement::CargoClippyDenyWarnings,
        )],
    );

    assertions::assert_single_inventory(&results, "cargo clippy -D warnings", "test");
}

#[test]
fn rustflags_deny_warnings_clippy_satisfies_contract() {
    let results = run_case(
        "#!/bin/sh\nRUSTFLAGS='-D warnings' cargo clippy --all-targets\n",
        vec![requirement(
            G3HookCommandRequirement::CargoClippyDenyWarnings,
        )],
    );

    assertions::assert_single_inventory(&results, "cargo clippy -D warnings", "test");
}

#[test]
fn clippy_later_allow_warnings_does_not_satisfy_contract() {
    let results = run_case(
        "#!/bin/sh\ncargo clippy --all-targets -- -D warnings -A warnings\n",
        vec![requirement(
            G3HookCommandRequirement::CargoClippyDenyWarnings,
        )],
    );

    assertions::assert_missing(&results, "cargo clippy -D warnings", "test");
}

#[test]
fn cargo_deny_check_satisfies_contract() {
    let results = run_case(
        "#!/bin/sh\ncargo deny check\n",
        vec![requirement(G3HookCommandRequirement::CargoDenyCheck)],
    );

    assertions::assert_single_inventory(&results, "cargo deny check", "test");
}

#[test]
fn cargo_test_satisfies_contract() {
    let results = run_case(
        "#!/bin/sh\ncargo test --workspace\n",
        vec![requirement(G3HookCommandRequirement::CargoTest)],
    );

    assertions::assert_single_inventory(&results, "cargo test", "test");
}

#[test]
fn cargo_machete_satisfies_contract() {
    let results = run_case(
        "#!/bin/sh\ncargo machete\n",
        vec![requirement(G3HookCommandRequirement::CargoMachete)],
    );

    assertions::assert_single_inventory(&results, "cargo machete", "test");
}

#[test]
fn cargo_dupes_satisfies_contract() {
    let results = run_case(
        "#!/bin/sh\ncargo dupes check --exclude-tests\n",
        vec![requirement(G3HookCommandRequirement::CargoDupes)],
    );

    assertions::assert_single_inventory(&results, "cargo dupes", "test");
}

#[test]
fn gitleaks_satisfies_contract() {
    let results = run_case(
        "#!/bin/sh\ngitleaks detect --no-banner\n",
        vec![requirement(G3HookCommandRequirement::Gitleaks)],
    );

    assertions::assert_single_inventory(&results, "gitleaks", "test");
}

#[test]
fn comment_does_not_satisfy_contract() {
    let results = run_case(
        "#!/bin/sh\n# cargo fmt --check\n",
        vec![requirement(G3HookCommandRequirement::CargoFmtCheck)],
    );

    assertions::assert_missing(&results, "cargo fmt --check", "test");
}

#[test]
fn echo_does_not_satisfy_contract() {
    let results = run_case(
        "#!/bin/sh\necho \"cargo fmt --check\"\n",
        vec![requirement(G3HookCommandRequirement::CargoFmtCheck)],
    );

    assertions::assert_missing(&results, "cargo fmt --check", "test");
}

#[test]
fn path_qualified_g3rs_validate_satisfies_contract() {
    let results = run_case(
        "#!/bin/sh\n/usr/local/bin/g3rs validate --path .\n",
        vec![requirement(G3HookCommandRequirement::G3RsValidatePath)],
    );

    assertions::assert_single_inventory(&results, "g3rs validate --path", "test");
}

#[test]
fn g3rs_validate_with_family_filter_does_not_satisfy_contract() {
    let results = run_case(
        "#!/bin/sh\ng3rs validate --path . --family hooks\n",
        vec![requirement(G3HookCommandRequirement::G3RsValidatePath)],
    );

    assertions::assert_missing(&results, "g3rs validate --path", "test");
}

#[test]
fn g3rs_validate_empty_detached_path_does_not_satisfy_contract() {
    let results = run_case(
        "#!/bin/sh\ng3rs validate --path \"\"\n",
        vec![requirement(G3HookCommandRequirement::G3RsValidatePath)],
    );

    assertions::assert_missing(&results, "g3rs validate --path", "test");
}

#[test]
fn g3rs_validate_help_does_not_satisfy_contract() {
    let results = run_case(
        "#!/bin/sh\ng3rs validate --path . --help\n",
        vec![requirement(G3HookCommandRequirement::G3RsValidatePath)],
    );

    assertions::assert_missing(&results, "g3rs validate --path", "test");
}

#[test]
fn g3rs_validate_attached_path_option_value_does_not_satisfy_contract() {
    let results = run_case(
        "#!/bin/sh\ng3rs validate --path=--help\n",
        vec![requirement(G3HookCommandRequirement::G3RsValidatePath)],
    );

    assertions::assert_missing(&results, "g3rs validate --path", "test");
}

#[test]
fn env_wrapped_cargo_dupes_exclude_tests_satisfies_contract() {
    let results = run_case(
        "env CARGO_TERM_COLOR=always cargo dupes check --exclude-tests\n",
        vec![requirement(
            G3HookCommandRequirement::CargoDupesExcludeTests,
        )],
    );

    assertions::assert_single_inventory(&results, "cargo dupes --exclude-tests", "test");
}

#[test]
fn mixed_non_excluding_cargo_dupes_does_not_satisfy_exclude_tests_contract() {
    let results = run_case(
        "#!/bin/sh\ncargo dupes check\ncargo dupes check --exclude-tests\n",
        vec![requirement(
            G3HookCommandRequirement::CargoDupesExcludeTests,
        )],
    );

    assertions::assert_missing(&results, "cargo dupes --exclude-tests", "test");
}

#[test]
fn cargo_metadata_locked_satisfies_concrete_lockfile_contract() {
    let results = run_case(
        "#!/bin/sh\ncargo metadata --locked --format-version 1 > /dev/null\n",
        vec![requirement(
            G3HookCommandRequirement::ConcreteLockfileCommand,
        )],
    );

    assertions::assert_single_inventory(&results, "cargo metadata --locked", "test");
}

#[test]
fn cargo_update_locked_satisfies_concrete_lockfile_contract() {
    let results = run_case(
        "#!/bin/sh\ncargo update --locked --workspace\n",
        vec![requirement(
            G3HookCommandRequirement::ConcreteLockfileCommand,
        )],
    );

    assertions::assert_single_inventory(&results, "cargo metadata --locked", "test");
}

#[test]
fn cargo_metadata_locked_for_other_manifest_does_not_satisfy_contract() {
    let results = run_case(
        "#!/bin/sh\ncargo metadata --locked --manifest-path /tmp/other/Cargo.toml\n",
        vec![requirement(
            G3HookCommandRequirement::ConcreteLockfileCommand,
        )],
    );

    assertions::assert_missing(&results, "cargo metadata --locked", "test");
}

#[test]
fn cargo_alias_shadow_does_not_satisfy_cargo_contract() {
    let results = run_case(
        "#!/bin/sh\nshopt -s expand_aliases\nalias cargo='echo skipped'\ncargo clippy --workspace --all-targets -- -D warnings\n",
        vec![requirement(
            G3HookCommandRequirement::CargoClippyDenyWarnings,
        )],
    );

    assertions::assert_missing(&results, "cargo clippy -D warnings", "test");
}

#[test]
fn multi_alias_line_shadow_does_not_satisfy_cargo_contract() {
    let results = run_case(
        "#!/bin/sh\nalias foo=bar cargo='echo skipped'\ncargo fmt --check\n",
        vec![requirement(G3HookCommandRequirement::CargoFmtCheck)],
    );

    assertions::assert_missing(&results, "cargo fmt --check", "test");
}

#[test]
fn alias_after_real_command_does_not_invalidate_prior_command() {
    let results = run_case(
        "#!/bin/sh\ncargo fmt --check\nalias cargo='echo skipped'\n",
        vec![requirement(G3HookCommandRequirement::CargoFmtCheck)],
    );

    assertions::assert_single_inventory(&results, "cargo fmt --check", "test");
}

#[test]
fn path_qualified_cargo_is_not_shadowed_by_alias() {
    let results = run_case(
        "#!/bin/sh\nalias cargo='echo skipped'\n/usr/bin/cargo fmt --check\n",
        vec![requirement(G3HookCommandRequirement::CargoFmtCheck)],
    );

    assertions::assert_single_inventory(&results, "cargo fmt --check", "test");
}

#[test]
fn pnpm_frozen_lockfile_does_not_satisfy_rust_lockfile_contract() {
    let results = run_case(
        "#!/bin/sh\npnpm install --frozen-lockfile\n",
        vec![requirement(
            G3HookCommandRequirement::ConcreteLockfileCommand,
        )],
    );

    assertions::assert_missing(&results, "cargo metadata --locked", "test");
}

#[test]
fn duplicate_requirements_collapse_and_preserve_all_owner_families() {
    let results = run_case(
        "#!/bin/sh\ntrue\n",
        vec![
            owned_requirement("arch", G3HookCommandRequirement::G3RsValidatePath),
            owned_requirement("code", G3HookCommandRequirement::G3RsValidatePath),
            owned_requirement("garde", G3HookCommandRequirement::G3RsValidatePath),
        ],
    );

    assertions::assert_missing(&results, "g3rs validate --path", "arch, code, garde");
}

#[test]
fn owner_families_are_reported_for_missing_command() {
    let results = run_case(
        "#!/bin/sh\ntrue\n",
        vec![requirement(G3HookCommandRequirement::CargoDenyCheck)],
    );

    assertions::assert_missing(&results, "cargo deny check", "test");
}

fn requirement(command: G3HookCommandRequirement) -> G3HookRequirement {
    owned_requirement("test", command)
}

fn owned_requirement(owner_family: &str, command: G3HookCommandRequirement) -> G3HookRequirement {
    G3HookRequirement {
        id: "test/hook-contract".to_owned(),
        owner_family: owner_family.to_owned(),
        trigger_patterns: Vec::new(),
        required_commands: vec![command],
        critical_commands: Vec::new(),
    }
}
