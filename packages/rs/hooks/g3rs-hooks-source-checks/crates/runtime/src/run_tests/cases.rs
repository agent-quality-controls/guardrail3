use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookRequirement, G3HookTriggerPattern,
};
use g3rs_hooks_source_checks_assertions::run as assertions;
use g3rs_hooks_source_checks_assertions::run::ExpectedRuleResult;
use g3rs_hooks_types::{G3RsHookScriptKind, G3RsHooksSourceChecksInput};
use hook_shell_parser::parse_script;

use super::super::check;
use super::super::check_all;

const VALID_PRECOMMIT: &str = r#"#!/usr/bin/env bash
set -euo pipefail
REPO_ROOT="$(git rev-parse --show-toplevel)"
"$REPO_ROOT/scripts/g3rs/verify" --mode pre-commit --scope "$REPO_ROOT/apps/guardrail3-rs"
"#;

const VALID_VERIFIER: &str = r#"#!/usr/bin/env bash
set -euo pipefail
usage() { exit 2; }
[[ -n "$SCOPE_ARG" ]] || usage 'missing --scope'
case "$MODE" in
  pre-commit)
    ;;
  workspace)
    ;;
  *)
    usage "unknown --mode: $MODE"
    ;;
esac
export CARGO_TARGET_DIR="$REPO_ROOT/.cargo-target"
g3rs validate --path "$SCOPE"
cargo metadata --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
cargo machete
cargo test --workspace
cargo mutants --check --in-place
cargo dupes check --max-exact 85 --max-exact-percent 10 --exclude-tests
"#;

#[test]
fn hook_passes_when_it_calls_g3rs_verifier_with_mode_and_scope() {
    let results = check(&input(
        ".githooks/pre-commit",
        G3RsHookScriptKind::PreCommit,
        VALID_PRECOMMIT,
        Vec::new(),
    ));

    assert_inventory(
        &results,
        "g3rs-hooks/precommit-calls-g3rs-verifier",
        "pre-commit calls Rust verifier",
    );
}

#[test]
fn hook_fails_when_it_does_not_call_g3rs_verifier() {
    let results = check(&input(
        ".githooks/pre-commit",
        G3RsHookScriptKind::PreCommit,
        "#!/usr/bin/env bash\nset -euo pipefail\ncargo test --workspace\n",
        Vec::new(),
    ));

    assert_finding(
        &results,
        "g3rs-hooks/precommit-calls-g3rs-verifier",
        ".githooks/pre-commit must run scripts/g3rs/verify",
    );
}

#[test]
fn hook_fails_when_g3rs_verifier_line_omits_precommit_mode() {
    let results = check(&input(
        ".githooks/pre-commit",
        G3RsHookScriptKind::PreCommit,
        "#!/usr/bin/env bash\nset -euo pipefail\n\"$REPO_ROOT/scripts/g3rs/verify\" --scope apps/guardrail3-rs\n",
        Vec::new(),
    ));

    assert_finding(
        &results,
        "g3rs-hooks/precommit-calls-g3rs-verifier",
        "--mode pre-commit",
    );
}

#[test]
fn hook_fails_when_g3rs_verifier_line_omits_scope() {
    let results = check(&input(
        ".githooks/pre-commit",
        G3RsHookScriptKind::PreCommit,
        "#!/usr/bin/env bash\nset -euo pipefail\n\"$REPO_ROOT/scripts/g3rs/verify\" --mode pre-commit\n",
        Vec::new(),
    ));

    assert_finding(
        &results,
        "g3rs-hooks/precommit-calls-g3rs-verifier",
        "--scope",
    );
}

#[test]
fn verifier_facts_pass_with_required_commands_and_no_g3ts_script() {
    let results = check_all(&[
        input(
            ".githooks/pre-commit",
            G3RsHookScriptKind::PreCommit,
            VALID_PRECOMMIT,
            Vec::new(),
        ),
        input(
            "scripts/g3rs/verify",
            G3RsHookScriptKind::G3RsVerifier,
            VALID_VERIFIER,
            Vec::new(),
        ),
    ]);

    assert_inventory(
        &results,
        "g3rs-hooks/g3rs-verifier-exists",
        "Rust verifier script exists",
    );
    assert_no_finding(
        &results,
        "g3rs-hooks/g3rs-verifier-forbidden-tools",
        "must not call g3ts",
    );
}

#[test]
fn verifier_fails_when_missing_g3rs_validate_scope() {
    assert_verifier_missing(
        &VALID_VERIFIER.replace("g3rs validate --path \"$SCOPE\"\n", ""),
        "g3rs validate --path \"$SCOPE\"",
    );
}

#[test]
fn verifier_fails_when_missing_cargo_metadata_locked() {
    assert_verifier_missing(
        &VALID_VERIFIER.replace("cargo metadata --locked\n", ""),
        "cargo metadata --locked",
    );
}

#[test]
fn verifier_fails_when_missing_cargo_fmt_all_check() {
    assert_verifier_missing(
        &VALID_VERIFIER.replace("cargo fmt --all -- --check\n", ""),
        "cargo fmt --all -- --check",
    );
}

#[test]
fn verifier_fails_when_clippy_omits_warning_denial() {
    assert_verifier_missing(
        &VALID_VERIFIER.replace(" -- -D warnings", ""),
        "cargo clippy --workspace --all-targets --all-features -- -D warnings",
    );
}

#[test]
fn verifier_fails_when_missing_cargo_deny_check() {
    assert_verifier_missing(
        &VALID_VERIFIER.replace("cargo deny check\n", ""),
        "cargo deny check",
    );
}

#[test]
fn verifier_fails_when_missing_cargo_machete() {
    assert_verifier_missing(
        &VALID_VERIFIER.replace("cargo machete\n", ""),
        "cargo machete",
    );
}

#[test]
fn verifier_fails_when_missing_cargo_test_workspace() {
    assert_verifier_missing(
        &VALID_VERIFIER.replace("cargo test --workspace\n", ""),
        "cargo test --workspace",
    );
}

#[test]
fn verifier_fails_when_missing_cargo_mutants_check_in_place() {
    assert_verifier_missing(
        &VALID_VERIFIER.replace("cargo mutants --check --in-place\n", ""),
        "cargo mutants --check --in-place",
    );
}

#[test]
fn verifier_fails_when_cargo_dupes_omits_thresholds() {
    assert_verifier_missing(
        &VALID_VERIFIER.replace(" --max-exact 85 --max-exact-percent 10", ""),
        "cargo dupes check --max-exact 85 --max-exact-percent 10 --exclude-tests",
    );
}

#[test]
fn verifier_fails_when_cargo_dupes_omits_exclude_tests() {
    assert_verifier_missing(
        &VALID_VERIFIER.replace(" --exclude-tests", ""),
        "cargo dupes check --max-exact 85 --max-exact-percent 10 --exclude-tests",
    );
}

#[test]
fn verifier_fails_when_it_calls_g3ts() {
    let results = check(&input(
        "scripts/g3rs/verify",
        G3RsHookScriptKind::G3RsVerifier,
        &format!("{VALID_VERIFIER}g3ts validate --path \"$SCOPE\"\n"),
        Vec::new(),
    ));

    assert_finding(
        &results,
        "g3rs-hooks/g3rs-verifier-forbidden-tools",
        "must not call g3ts",
    );
}

#[test]
fn verifier_fails_when_it_calls_g3ts_verifier_path() {
    let results = check(&input(
        "scripts/g3rs/verify",
        G3RsHookScriptKind::G3RsVerifier,
        &format!(
            "{VALID_VERIFIER}\"$REPO_ROOT/scripts/g3ts/verify\" --mode pre-commit --scope \"$REPO_ROOT\"\n"
        ),
        Vec::new(),
    ));

    assert_finding(
        &results,
        "g3rs-hooks/g3rs-verifier-forbidden-tools",
        "must not call g3ts",
    );
}

#[test]
fn verifier_fails_when_it_calls_typescript_package_managers() {
    for package_manager in ["pnpm", "npm", "yarn", "bun"] {
        let results = check(&input(
            "scripts/g3rs/verify",
            G3RsHookScriptKind::G3RsVerifier,
            &format!("{VALID_VERIFIER}{package_manager} install\n"),
            Vec::new(),
        ));

        assert_finding(
            &results,
            "g3rs-hooks/g3rs-verifier-forbidden-tools",
            "must not call pnpm, npm, yarn, or bun",
        );
    }
}

#[test]
fn missing_mode_exits_non_zero() {
    let fixture = script_fixture("missing-mode");
    let status = verifier_command(&fixture)
        .args(["--scope", "."])
        .status()
        .expect("spawn verifier for missing mode test");

    assert!(!status.success(), "missing --mode should fail");
}

#[test]
fn missing_scope_exits_non_zero() {
    let fixture = script_fixture("missing-scope");
    let status = verifier_command(&fixture)
        .args(["--mode", "pre-commit"])
        .status()
        .expect("spawn verifier for missing scope test");

    assert!(!status.success(), "missing --scope should fail");
}

#[test]
fn unknown_modes_exit_non_zero() {
    for mode in ["unknown", "worktree", "files", "current"] {
        let fixture = script_fixture(mode);
        let status = verifier_command(&fixture)
            .args(["--mode", mode, "--scope", "."])
            .status()
            .expect("spawn verifier for unknown mode test");

        assert!(!status.success(), "{mode} mode should fail");
    }
}

#[test]
fn unknown_flag_exits_non_zero() {
    let fixture = script_fixture("unknown-flag");
    let status = verifier_command(&fixture)
        .args(["--mode", "pre-commit", "--scope", ".", "--wat"])
        .status()
        .expect("spawn verifier for unknown flag test");

    assert!(!status.success(), "unknown flag should fail");
}

#[test]
fn pre_commit_exits_zero_when_no_relevant_staged_files_exist() {
    let fixture = script_fixture("no-relevant-staged");
    write_fake_tool(&fixture, "git", git_fake_script(&fixture, "README.md\n"));
    write_fake_tool(
        &fixture,
        "g3rs",
        "echo g3rs-called >> \"$G3RS_LOG\"\nexit 1\n",
    );
    write_fake_tool(
        &fixture,
        "cargo",
        "echo cargo-called >> \"$G3RS_LOG\"\nexit 1\n",
    );
    let log = fixture.join("calls.log");

    let status = verifier_command(&fixture)
        .env("PATH", fake_path(&fixture))
        .env("G3RS_LOG", &log)
        .env("G3RS_DIFF_LOG", fixture.join("git-diff.log"))
        .args(["--mode", "pre-commit", "--scope", "."])
        .status()
        .expect("spawn verifier for irrelevant staged file test");

    assert!(
        status.success(),
        "irrelevant staged files should skip checks"
    );
    assert!(
        !log.exists(),
        "Rust tools should not run for irrelevant staged files"
    );
}

#[test]
fn workspace_mode_does_not_read_staged_paths_before_running_checks() {
    let fixture = script_fixture("workspace-no-staged");
    write_fake_tool(&fixture, "git", git_fake_script(&fixture, "src/lib.rs\n"));
    write_fake_tool(
        &fixture,
        "g3rs",
        "echo g3rs-called >> \"$G3RS_LOG\"\nexit 0\n",
    );
    write_fake_tool(
        &fixture,
        "cargo",
        "echo cargo-called >> \"$G3RS_LOG\"\nexit 1\n",
    );
    let log = fixture.join("calls.log");
    let diff_log = fixture.join("git-diff.log");

    let status = verifier_command(&fixture)
        .env("PATH", fake_path(&fixture))
        .env("G3RS_LOG", &log)
        .env("G3RS_DIFF_LOG", &diff_log)
        .args(["--mode", "workspace", "--scope", "."])
        .status()
        .expect("spawn verifier for workspace mode staged-path test");

    assert!(
        !status.success(),
        "fake cargo should stop workspace verification"
    );
    assert!(log.exists(), "workspace mode should start verification");
    assert!(
        !diff_log.exists(),
        "workspace mode must not inspect staged paths"
    );
}

#[test]
fn required_contract_commands_are_checked_across_modular_hook_surface() {
    let inputs = vec![
        input(
            ".githooks/pre-commit",
            G3RsHookScriptKind::PreCommit,
            "#!/bin/sh\nrun-parts .githooks/pre-commit.d\n",
            vec![requirement(G3HookCommandRequirement::G3RsValidatePath)],
        ),
        input(
            ".githooks/pre-commit.d/rust",
            G3RsHookScriptKind::Modular,
            "#!/bin/sh\ng3rs validate --path .\n",
            Vec::new(),
        ),
    ];

    let results = check_all(&inputs);

    assertions::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            inventory: Some(true),
            message_contains: Some("Owner families: test"),
            ..ExpectedRuleResult::default()
        }],
    );
}

#[test]
fn orphan_modular_hook_script_does_not_satisfy_pre_commit_contract_surface() {
    let inputs = vec![
        input(
            ".githooks/pre-commit",
            G3RsHookScriptKind::PreCommit,
            "#!/bin/sh\necho no dispatcher\n",
            vec![requirement(G3HookCommandRequirement::G3RsValidatePath)],
        ),
        input(
            ".githooks/pre-commit.d/rust",
            G3RsHookScriptKind::Modular,
            "#!/bin/sh\ng3rs validate --path .\n",
            Vec::new(),
        ),
    ];

    let results = check_all(&inputs);

    assertions::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            inventory: Some(false),
            message_contains: Some("Owner families: test"),
            ..ExpectedRuleResult::default()
        }],
    );
}

#[test]
fn sourcing_one_modular_script_does_not_dispatch_entire_directory() {
    let inputs = vec![
        input(
            ".githooks/pre-commit",
            G3RsHookScriptKind::PreCommit,
            "#!/bin/sh\nsource .githooks/pre-commit.d/bootstrap\n",
            vec![requirement(G3HookCommandRequirement::G3RsValidatePath)],
        ),
        input(
            ".githooks/pre-commit.d/rust",
            G3RsHookScriptKind::Modular,
            "#!/bin/sh\ng3rs validate --path .\n",
            Vec::new(),
        ),
    ];

    let results = check_all(&inputs);

    assertions::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            inventory: Some(false),
            message_contains: Some("Owner families: test"),
            ..ExpectedRuleResult::default()
        }],
    );
}

fn input(
    rel_path: &str,
    kind: G3RsHookScriptKind,
    content: &str,
    requirements: Vec<G3HookRequirement>,
) -> G3RsHooksSourceChecksInput {
    G3RsHooksSourceChecksInput {
        rel_path: rel_path.to_owned(),
        kind,
        exists: true,
        parsed: parse_script(content),
        has_modular_dir: true,
        is_workspace_project: true,
        requirements,
    }
}

fn requirement(command: G3HookCommandRequirement) -> G3HookRequirement {
    G3HookRequirement {
        id: "test/hook-contract".to_owned(),
        owner_family: "test".to_owned(),
        trigger_patterns: vec![G3HookTriggerPattern::Glob("**/*.rs".to_owned())],
        required_commands: vec![command],
        critical_commands: Vec::new(),
    }
}

fn assert_verifier_missing(content: &str, expected_message: &str) {
    let results = check(&input(
        "scripts/g3rs/verify",
        G3RsHookScriptKind::G3RsVerifier,
        content,
        Vec::new(),
    ));

    assert_finding(
        &results,
        "g3rs-hooks/g3rs-verifier-required-commands",
        expected_message,
    );
}

fn assert_finding(results: &[guardrail3_check_types::G3CheckResult], id: &str, message: &str) {
    assert!(
        results.iter().any(|result| result.id() == id
            && !result.inventory()
            && result.message().contains(message)),
        "expected non-inventory result {id} containing {message}; got {results:#?}",
    );
}

fn assert_inventory(results: &[guardrail3_check_types::G3CheckResult], id: &str, title: &str) {
    assert!(
        results.iter().any(|result| result.id() == id
            && result.inventory()
            && result.title().contains(title)),
        "expected inventory result {id} containing {title}; got {results:#?}",
    );
}

fn assert_no_finding(results: &[guardrail3_check_types::G3CheckResult], id: &str, message: &str) {
    assert!(
        !results.iter().any(|result| result.id() == id
            && !result.inventory()
            && result.message().contains(message)),
        "expected no non-inventory result {id} containing {message}; got {results:#?}",
    );
}

fn script_fixture(name: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("g3rs-verifier-{name}-{unique}"));
    fs::create_dir_all(root.join("bin")).expect("create fake bin dir");
    fs::create_dir_all(root.join("scripts/g3rs")).expect("create script dir");
    let _bytes = fs::copy(
        repo_root().join("scripts/g3rs/verify"),
        root.join("scripts/g3rs/verify"),
    )
    .expect("copy repository verifier into isolated script fixture");
    make_executable(&root.join("scripts/g3rs/verify"));
    root
}

fn verifier_command(fixture: &Path) -> Command {
    let mut command = Command::new(fixture.join("scripts/g3rs/verify"));
    let _command = command.current_dir(fixture);
    command
}

fn write_fake_tool(fixture: &Path, name: &str, body: impl AsRef<str>) {
    let path = fixture.join("bin").join(name);
    fs::write(
        &path,
        format!("#!/usr/bin/env bash\nset -euo pipefail\n{}", body.as_ref()),
    )
    .expect("write fake tool");
    make_executable(&path);
}

fn fake_path(fixture: &Path) -> String {
    format!(
        "{}:{}",
        fixture.join("bin").display(),
        std::env::var("PATH").unwrap_or_default()
    )
}

fn git_fake_script(fixture: &Path, staged: &str) -> String {
    format!(
        r#"if [ "$1" = "rev-parse" ]; then
  echo "{}"
  exit 0
fi
if [ "$1" = "diff" ]; then
  echo diff-called >> "$G3RS_DIFF_LOG"
  printf '{}'
  exit 0
fi
exit 1
"#,
        fixture.display(),
        staged
    )
}

fn make_executable(path: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(path)
            .expect("read executable metadata")
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).expect("mark executable");
    }
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(6)
        .expect("manifest should be below repository root")
        .to_path_buf()
}
