use g3ts_hooks_contract_types::{
    G3TsHookCommandRequirement, G3TsHookRequirement, G3TsHookTriggerPattern,
};
use g3ts_hooks_source_checks_assertions::run as assertions;
use g3ts_hooks_types::{G3TsHookScriptKind, G3TsHooksSourceChecksInput};
use guardrail3_check_types::G3CheckResult;

fn run_case(script: &str) -> Vec<G3CheckResult> {
    let input = G3TsHooksSourceChecksInput::new(
        ".githooks/pre-commit".to_owned(),
        G3TsHookScriptKind::PreCommit,
        hook_shell_parser::parse_script(script),
        false,
        vec!["apps/landing".to_owned()],
        vec![G3TsHookRequirement::new(
            "test/hook-contract".to_owned(),
            "test".to_owned(),
            vec![G3TsHookTriggerPattern::Glob("src/**/*.ts".to_owned())],
            vec![G3TsHookCommandRequirement::AppValidateScript],
            Vec::new(),
        )],
    );
    super::super::check(&input)
}

#[test]
fn comments_do_not_satisfy_g3ts_or_validate_commands() {
    let results = run_case(
        r#"
# g3ts validate --path apps/landing
# pnpm --filter landing run validate
FILES=$(git diff --cached --name-only)
case "$FILES" in
  *guardrail3-ts.toml*) echo "changed" ;;
esac
"#,
    );

    assertions::assert_has_id(
        &results,
        "g3ts-hooks/g3ts-validate-staged-present",
        "commented g3ts command",
    );
    assertions::assert_has_id(
        &results,
        "g3ts-hooks/ts-app-validate-step-present",
        "commented validate command",
    );
}

#[test]
fn echo_text_does_not_satisfy_g3ts_or_validate_commands() {
    let results = run_case(
        r#"
echo "g3ts validate --path apps/landing"
echo "pnpm --filter landing run validate"
if git diff --cached --name-only | grep -q guardrail3-ts.toml; then
  echo "guardrail config changed"
fi
"#,
    );

    assertions::assert_has_id(
        &results,
        "g3ts-hooks/g3ts-validate-staged-present",
        "echoed g3ts command",
    );
    assertions::assert_has_id(
        &results,
        "g3ts-hooks/ts-app-validate-step-present",
        "echoed validate command",
    );
}

#[test]
fn real_commands_satisfy_g3ts_and_validate_contracts() {
    let results = run_case(
        r#"
FILES=$(git diff --cached --name-only)
echo "$FILES" | grep -q guardrail3-ts.toml
g3ts validate --path apps/landing
pnpm --filter landing run validate
"#,
    );

    assertions::assert_missing_id(
        &results,
        "g3ts-hooks/g3ts-validate-staged-present",
        "real g3ts validate command",
    );
    assertions::assert_missing_id(
        &results,
        "g3ts-hooks/ts-app-validate-step-present",
        "real app validate command",
    );
}

#[test]
fn wrong_app_commands_do_not_satisfy_app_contract() {
    let results = run_case(
        r#"
FILES=$(git diff --cached --name-only)
echo "$FILES" | grep -q guardrail3-ts.toml
g3ts validate --path apps/web
pnpm --filter web run validate
"#,
    );

    assertions::assert_has_id(
        &results,
        "g3ts-hooks/g3ts-validate-staged-present",
        "wrong g3ts app root",
    );
    assertions::assert_has_id(
        &results,
        "g3ts-hooks/ts-app-validate-step-present",
        "wrong validate app root",
    );
}

#[test]
fn family_scoped_g3ts_does_not_satisfy_full_hook_contract() {
    let results = run_case(
        r#"
FILES=$(git diff --cached --name-only)
echo "$FILES" | grep -q guardrail3-ts.toml
g3ts validate --path apps/landing --family hooks
pnpm --filter landing run validate
"#,
    );

    assertions::assert_has_id(
        &results,
        "g3ts-hooks/g3ts-validate-staged-present",
        "family-scoped g3ts validate",
    );
}

#[test]
fn fail_open_critical_command_is_error() {
    let input = G3TsHooksSourceChecksInput::new(
        ".githooks/pre-commit".to_owned(),
        G3TsHookScriptKind::PreCommit,
        hook_shell_parser::parse_script(
            r#"
g3ts validate --path apps/landing || true
pnpm --filter landing run validate
"#,
        ),
        false,
        vec!["apps/landing".to_owned()],
        vec![G3TsHookRequirement::new(
            "test/hook-contract".to_owned(),
            "test".to_owned(),
            Vec::new(),
            vec![G3TsHookCommandRequirement::AppValidateScript],
            Vec::new(),
        )],
    );

    let results = super::super::check(&input);
    assertions::assert_result_id_severity(
        &results,
        "g3ts-hooks/no-fail-open-wrappers",
        guardrail3_check_types::G3Severity::Error,
        "fail-open wrapper",
    );
}

#[test]
fn missing_tool_availability_guard_is_error() {
    let input = G3TsHooksSourceChecksInput::new(
        ".githooks/pre-commit".to_owned(),
        G3TsHookScriptKind::PreCommit,
        hook_shell_parser::parse_script(
            r#"
if command -v g3ts; then
  g3ts validate --path apps/landing
else
  echo "g3ts missing"
fi
pnpm --filter landing run validate
"#,
        ),
        false,
        vec!["apps/landing".to_owned()],
        vec![G3TsHookRequirement::new(
            "test/hook-contract".to_owned(),
            "test".to_owned(),
            Vec::new(),
            vec![G3TsHookCommandRequirement::AppValidateScript],
            Vec::new(),
        )],
    );

    let results = super::super::check(&input);
    assertions::assert_result_id_severity(
        &results,
        "g3ts-hooks/no-fail-open-wrappers",
        guardrail3_check_types::G3Severity::Error,
        "availability guard",
    );
}
