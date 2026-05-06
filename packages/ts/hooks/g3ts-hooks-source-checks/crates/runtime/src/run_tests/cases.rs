use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use g3ts_hooks_source_checks_assertions::run as assertions;
use g3ts_hooks_types::{
    G3TsHookScriptKind, G3TsHooksEnabledCategories, G3TsHooksSourceChecksInput,
};
use guardrail3_check_types::G3CheckResult;

fn pre_commit(script: &str) -> G3TsHooksSourceChecksInput {
    input(".githooks/pre-commit", G3TsHookScriptKind::PreCommit, script)
}

fn verifier(script: &str) -> G3TsHooksSourceChecksInput {
    input("scripts/g3ts/verify", G3TsHookScriptKind::Verifier, script)
}

fn input(rel_path: &str, kind: G3TsHookScriptKind, script: &str) -> G3TsHooksSourceChecksInput {
    input_with_categories(
        rel_path,
        kind,
        script,
        G3TsHooksEnabledCategories::all(),
    )
}

fn input_with_categories(
    rel_path: &str,
    kind: G3TsHookScriptKind,
    script: &str,
    enabled_categories: G3TsHooksEnabledCategories,
) -> G3TsHooksSourceChecksInput {
    G3TsHooksSourceChecksInput::new(
        rel_path.to_owned(),
        kind,
        hook_shell_parser::parse_script(script),
        false,
        vec!["apps/landing".to_owned()],
        enabled_categories,
        Vec::new(),
    )
}

fn valid_pre_commit() -> G3TsHooksSourceChecksInput {
    pre_commit(
        r#"
REPO_ROOT="$(git rev-parse --show-toplevel)"
"$REPO_ROOT/scripts/g3ts/verify" --mode pre-commit --scope "$REPO_ROOT/apps/landing"
"#,
    )
}

fn valid_verifier() -> G3TsHooksSourceChecksInput {
    verifier(
        r#"
#!/usr/bin/env bash
set -euo pipefail
MODE=""
SCOPE_ARG=""
usage() { exit 2; }
case "$1" in
  --mode) MODE="$2" ;;
  --scope) SCOPE_ARG="$2" ;;
  --*) usage; exit 2 ;;
esac
[ -n "$MODE" ] || { usage; exit 2; }
[ -n "$SCOPE_ARG" ] || { usage; exit 2; }
case "$MODE" in
  pre-commit|workspace) ;;
  *) usage; exit 2 ;;
esac
g3ts validate --path "$SCOPE"
pnpm exec tsc --noEmit
pnpm exec eslint --max-warnings 0 "$SCOPE"
pnpm exec prettier --check "$SCOPE"
pnpm exec cspell "$SCOPE"
pnpm exec stylelint "$SCOPE/**/*.css"
pnpm exec syncpack lint
pnpm exec type-coverage --at-least 100
"#,
    )
}

fn run_case(inputs: Vec<G3TsHooksSourceChecksInput>) -> Vec<G3CheckResult> {
    super::super::check_effective(&inputs)
}

#[test]
fn hook_passes_when_it_calls_g3ts_verifier_with_mode_and_scope() {
    let results = run_case(vec![valid_pre_commit(), valid_verifier()]);

    assertions::assert_missing_id(
        &results,
        "g3ts-hooks/pre-commit-invokes-g3ts-verifier",
        "valid verifier hook line",
    );
}

#[test]
fn hook_fails_when_it_does_not_call_g3ts_verifier() {
    let results = run_case(vec![pre_commit("g3ts validate --path apps/landing"), valid_verifier()]);

    assertions::assert_has_id(
        &results,
        "g3ts-hooks/pre-commit-invokes-g3ts-verifier",
        "missing verifier line",
    );
}

#[test]
fn hook_fails_when_g3ts_verifier_line_omits_pre_commit_mode() {
    let results = run_case(vec![
        pre_commit(r#""$REPO_ROOT/scripts/g3ts/verify" --scope apps/landing"#),
        valid_verifier(),
    ]);

    assertions::assert_has_id(
        &results,
        "g3ts-hooks/pre-commit-invokes-g3ts-verifier",
        "missing pre-commit mode",
    );
}

#[test]
fn hook_fails_when_g3ts_verifier_line_omits_scope() {
    let results = run_case(vec![
        pre_commit(r#""$REPO_ROOT/scripts/g3ts/verify" --mode pre-commit"#),
        valid_verifier(),
    ]);

    assertions::assert_has_id(
        &results,
        "g3ts-hooks/pre-commit-invokes-g3ts-verifier",
        "missing scope",
    );
}

#[test]
fn verifier_fails_when_missing_g3ts_validate() {
    let results = run_case(vec![
        valid_pre_commit(),
        verifier(&valid_verifier_script_without("g3ts validate")),
    ]);

    assertions::assert_has_id(
        &results,
        "g3ts-hooks/verifier-runs-g3ts-validate",
        "missing g3ts validate",
    );
}

#[test]
fn verifier_fails_when_g3ts_validate_uses_scope_prefix_variable() {
    let script = valid_verifier_script().replace(
        "g3ts validate --path \"$SCOPE\"",
        "g3ts validate --path \"$SCOPELESS\"",
    );
    let results = run_case(vec![valid_pre_commit(), verifier(&script)]);

    assertions::assert_has_id(
        &results,
        "g3ts-hooks/verifier-runs-g3ts-validate",
        "scope prefix variable",
    );
}

#[test]
fn verifier_fails_when_g3ts_validate_uses_different_scope_variable() {
    let script = valid_verifier_script().replace(
        "g3ts validate --path \"$SCOPE\"",
        "g3ts validate --path \"$NOT_SCOPE\"",
    );
    let results = run_case(vec![valid_pre_commit(), verifier(&script)]);

    assertions::assert_has_id(
        &results,
        "g3ts-hooks/verifier-runs-g3ts-validate",
        "different scope variable",
    );
}

#[test]
fn verifier_fails_when_g3ts_validate_uses_bad_suffix_path() {
    for bad_path in [
        "$NOT_SCOPE/apps/landing",
        "$SCOPELESS/apps/landing",
        "/tmp/apps/landing",
    ] {
        let script = valid_verifier_script().replace(
            "g3ts validate --path \"$SCOPE\"",
            format!("g3ts validate --path \"{bad_path}\"").as_str(),
        );
        let results = run_case(vec![valid_pre_commit(), verifier(&script)]);

        assertions::assert_has_id(
            &results,
            "g3ts-hooks/verifier-runs-g3ts-validate",
            bad_path,
        );
    }
}

#[test]
fn verifier_false_positive_tokens_do_not_satisfy_required_categories() {
    let script = r#"
g3ts validate --path "$SCOPE"
echo tsc
echo eslint
echo prettier --check
echo cspell
echo stylelint
echo syncpack
echo type-coverage
prettier --write "$SCOPE"
"#;
    let results = run_case(vec![valid_pre_commit(), verifier(script)]);

    for expected_id in [
        "g3ts-hooks/verifier-runs-typecheck",
        "g3ts-hooks/verifier-runs-lint",
        "g3ts-hooks/verifier-runs-format-check",
        "g3ts-hooks/verifier-runs-spelling-check",
        "g3ts-hooks/verifier-runs-stylelint",
        "g3ts-hooks/verifier-runs-package-policy",
        "g3ts-hooks/verifier-runs-typecov",
    ] {
        assertions::assert_has_id(&results, expected_id, expected_id);
    }
}

#[test]
fn repository_g3ts_verifier_satisfies_required_categories() {
    let script = fs::read_to_string(repo_root().join("scripts/g3ts/verify"))
        .expect("read repository G3TS verifier");
    let results = run_case(vec![valid_pre_commit(), verifier(script.as_str())]);

    for expected_id in [
        "g3ts-hooks/verifier-runs-g3ts-validate",
        "g3ts-hooks/verifier-runs-typecheck",
        "g3ts-hooks/verifier-runs-lint",
        "g3ts-hooks/verifier-runs-format-check",
        "g3ts-hooks/verifier-runs-spelling-check",
        "g3ts-hooks/verifier-runs-stylelint",
        "g3ts-hooks/verifier-runs-package-policy",
        "g3ts-hooks/verifier-runs-typecov",
    ] {
        assertions::assert_missing_id(&results, expected_id, expected_id);
    }
}

#[test]
fn verifier_fails_when_missing_each_required_category() {
    let cases = [
        ("tsc", "g3ts-hooks/verifier-runs-typecheck"),
        ("eslint", "g3ts-hooks/verifier-runs-lint"),
        ("prettier", "g3ts-hooks/verifier-runs-format-check"),
        ("cspell", "g3ts-hooks/verifier-runs-spelling-check"),
        ("stylelint", "g3ts-hooks/verifier-runs-stylelint"),
        ("syncpack", "g3ts-hooks/verifier-runs-package-policy"),
        ("type-coverage", "g3ts-hooks/verifier-runs-typecov"),
    ];

    for (removed, expected_id) in cases {
        let results = run_case(vec![
            valid_pre_commit(),
            verifier(&valid_verifier_script_without(removed)),
        ]);
        assertions::assert_has_id(&results, expected_id, removed);
    }
}

#[test]
fn optional_categories_pass_when_disabled() {
    let categories = G3TsHooksEnabledCategories::none();
    let pre_commit = input_with_categories(
        ".githooks/pre-commit",
        G3TsHookScriptKind::PreCommit,
        r#"
REPO_ROOT="$(git rev-parse --show-toplevel)"
"$REPO_ROOT/scripts/g3ts/verify" --mode pre-commit --scope "$REPO_ROOT/apps/landing"
"#,
        categories,
    );
    let verifier = input_with_categories(
        "scripts/g3ts/verify",
        G3TsHookScriptKind::Verifier,
        &valid_verifier_script_without("stylelint")
            .lines()
            .filter(|line| !line.contains("syncpack") && !line.contains("type-coverage"))
            .collect::<Vec<_>>()
            .join("\n"),
        categories,
    );

    let results = run_case(vec![pre_commit, verifier]);

    assertions::assert_missing_id(
        &results,
        "g3ts-hooks/verifier-runs-stylelint",
        "style disabled",
    );
    assertions::assert_missing_id(
        &results,
        "g3ts-hooks/verifier-runs-package-policy",
        "package policy disabled",
    );
    assertions::assert_missing_id(
        &results,
        "g3ts-hooks/verifier-runs-typecov",
        "typecov disabled",
    );
}

#[test]
fn verifier_fails_when_it_calls_g3rs_or_cargo() {
    let script = format!("{}\ng3rs validate --path .\ncargo test\n", valid_verifier_script());
    let results = run_case(vec![valid_pre_commit(), verifier(&script)]);

    assertions::assert_has_id(
        &results,
        "g3ts-hooks/verifier-does-not-call-g3rs",
        "g3rs call",
    );
    assertions::assert_has_id(
        &results,
        "g3ts-hooks/verifier-does-not-call-cargo",
        "cargo call",
    );
}

#[test]
fn verifier_fails_when_it_calls_g3rs_verifier_path() {
    let script = format!(
        "{}\n\"$REPO_ROOT/scripts/g3rs/verify\" --mode pre-commit --scope \"$REPO_ROOT\"\n",
        valid_verifier_script()
    );
    let results = run_case(vec![valid_pre_commit(), verifier(&script)]);

    assertions::assert_has_id(
        &results,
        "g3ts-hooks/verifier-does-not-call-g3rs",
        "g3rs verifier path",
    );
}

#[test]
fn verifier_comments_do_not_satisfy_required_commands() {
    let results = run_case(vec![
        valid_pre_commit(),
        verifier("# g3ts validate --path \"$SCOPE\"\n# pnpm exec tsc --noEmit\n"),
    ]);

    assertions::assert_has_id(
        &results,
        "g3ts-hooks/verifier-runs-g3ts-validate",
        "commented validate",
    );
    assertions::assert_has_id(
        &results,
        "g3ts-hooks/verifier-runs-typecheck",
        "commented typecheck",
    );
}

#[test]
fn verifier_comments_do_not_trigger_forbidden_tool_findings() {
    let script = format!(
        "{}\n# g3rs validate --path .\n# cargo test\n",
        valid_verifier_script()
    );
    let results = run_case(vec![valid_pre_commit(), verifier(&script)]);

    assertions::assert_missing_id(
        &results,
        "g3ts-hooks/verifier-does-not-call-g3rs",
        "commented g3rs",
    );
    assertions::assert_missing_id(
        &results,
        "g3ts-hooks/verifier-does-not-call-cargo",
        "commented cargo",
    );
}

#[test]
fn typescript_hook_rules_pass_when_g3rs_verifier_does_not_exist() {
    let results = run_case(vec![valid_pre_commit(), valid_verifier()]);

    assertions::assert_missing_id(
        &results,
        "g3ts-hooks/verifier-does-not-call-g3rs",
        "no g3rs verifier fact required",
    );
}

#[test]
fn missing_verifier_script_fails() {
    let results = run_case(vec![valid_pre_commit()]);

    assertions::assert_has_id(
        &results,
        "g3ts-hooks/verifier-exists",
        "missing verifier input",
    );
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
    write_fake_tool(&fixture, "g3ts", "echo g3ts-called >> \"$G3TS_LOG\"\nexit 1\n");
    let log = fixture.join("calls.log");

    let status = verifier_command(&fixture)
        .env("PATH", fake_path(&fixture))
        .env("G3TS_LOG", &log)
        .env("G3TS_DIFF_LOG", fixture.join("git-diff.log"))
        .args(["--mode", "pre-commit", "--scope", "."])
        .status()
        .expect("spawn verifier for irrelevant staged file test");

    assert!(status.success(), "irrelevant staged files should skip checks");
    assert!(!log.exists(), "g3ts should not run for irrelevant staged files");
}

#[test]
fn pre_commit_runs_for_typescript_source_files() {
    let fixture = script_fixture("ts-source");
    write_fake_tool(&fixture, "git", git_fake_script(&fixture, "src/index.ts\n"));
    write_fake_tool(&fixture, "g3ts", "echo \"g3ts $*\" >> \"$G3TS_LOG\"\nexit 0\n");
    write_fake_tool(&fixture, "pnpm", "echo \"pnpm $*\" >> \"$G3TS_LOG\"\nexit 0\n");
    let log = fixture.join("calls.log");

    let status = verifier_command(&fixture)
        .env("PATH", fake_path(&fixture))
        .env("G3TS_LOG", &log)
        .env("G3TS_DIFF_LOG", fixture.join("git-diff.log"))
        .args(["--mode", "pre-commit", "--scope", "."])
        .status()
        .expect("spawn verifier for TypeScript source staged file test");

    assert!(status.success(), "TypeScript source should run verifier");
    let log_content = fs::read_to_string(log).expect("read verifier call log");
    assert!(log_content.contains("g3ts validate"));
    assert!(log_content.contains("pnpm exec"));
}

#[test]
fn pre_commit_runs_for_package_config_files() {
    let fixture = script_fixture("package-config");
    fs::write(fixture.join("package.json"), "{}").expect("write package fixture");
    write_fake_tool(&fixture, "git", git_fake_script(&fixture, "package.json\n"));
    write_fake_tool(&fixture, "g3ts", "echo \"g3ts $*\" >> \"$G3TS_LOG\"\nexit 0\n");
    write_fake_tool(&fixture, "pnpm", "echo \"pnpm $*\" >> \"$G3TS_LOG\"\nexit 0\n");
    let log = fixture.join("calls.log");

    let status = verifier_command(&fixture)
        .env("PATH", fake_path(&fixture))
        .env("G3TS_LOG", &log)
        .env("G3TS_DIFF_LOG", fixture.join("git-diff.log"))
        .args(["--mode", "pre-commit", "--scope", "."])
        .status()
        .expect("spawn verifier for package config staged file test");

    assert!(status.success(), "package config should run verifier");
    let log_content = fs::read_to_string(log).expect("read verifier call log");
    assert!(log_content.contains("g3ts validate"));
    assert!(
        log_content.contains("syncpack"),
        "package config should run package policy when package.json exists; got {log_content}"
    );
}

#[test]
fn workspace_mode_does_not_read_staged_paths_before_running_checks() {
    let fixture = script_fixture("workspace-no-staged");
    write_fake_tool(&fixture, "git", git_fake_script(&fixture, "src/index.ts\n"));
    write_fake_tool(&fixture, "g3ts", "echo g3ts-called >> \"$G3TS_LOG\"\nexit 0\n");
    write_fake_tool(&fixture, "pnpm", "echo pnpm-called >> \"$G3TS_LOG\"\nexit 1\n");
    let log = fixture.join("calls.log");
    let diff_log = fixture.join("git-diff.log");

    let status = verifier_command(&fixture)
        .env("PATH", fake_path(&fixture))
        .env("G3TS_LOG", &log)
        .env("G3TS_DIFF_LOG", &diff_log)
        .args(["--mode", "workspace", "--scope", "."])
        .status()
        .expect("spawn verifier for workspace mode staged-path test");

    assert!(!status.success(), "fake pnpm should stop workspace verification");
    assert!(log.exists(), "workspace mode should start verification");
    assert!(!diff_log.exists(), "workspace mode must not inspect staged paths");
}

fn valid_verifier_script_without(needle: &str) -> String {
    valid_verifier_script()
        .lines()
        .filter(|line| !line.contains(needle))
        .collect::<Vec<_>>()
        .join("\n")
}

fn valid_verifier_script() -> String {
    let input = valid_verifier();
    input
        .parsed()
        .source_lines
        .iter()
        .map(|line| line.raw.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

fn script_fixture(name: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("g3ts-verifier-{name}-{unique}"));
    fs::create_dir_all(root.join("bin")).expect("create fake bin dir");
    fs::create_dir_all(root.join("scripts/g3ts")).expect("create script dir");
    let _bytes = fs::copy(
        repo_root().join("scripts/g3ts/verify"),
        root.join("scripts/g3ts/verify"),
    )
    .expect("copy repository verifier into isolated script fixture");
    root
}

fn verifier_command(fixture: &Path) -> Command {
    let mut command = Command::new(fixture.join("scripts/g3ts/verify"));
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
  echo diff-called >> "$G3TS_DIFF_LOG"
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
            .expect("read fake tool metadata")
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).expect("mark fake tool executable");
    }
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(6)
        .expect("manifest should be below repository root")
        .to_path_buf()
}
