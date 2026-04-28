use g3ts_hooks_contract_types::{
    G3TsHookCommandRequirement, G3TsHookRequirement, G3TsHookTriggerPattern,
};
use g3ts_hooks_types::{G3TsHookScriptKind, G3TsHooksSourceChecksInput};

fn run_case(script: &str) -> Vec<String> {
    let input = G3TsHooksSourceChecksInput {
        rel_path: ".githooks/pre-commit".to_owned(),
        kind: G3TsHookScriptKind::PreCommit,
        parsed: hook_shell_parser::parse_script(script),
        has_modular_dir: false,
        app_package_roots: vec!["apps/landing".to_owned()],
        requirements: vec![G3TsHookRequirement {
            id: "test/hook-contract".to_owned(),
            owner_family: "test".to_owned(),
            trigger_patterns: vec![G3TsHookTriggerPattern::Glob("src/**/*.ts".to_owned())],
            required_commands: vec![G3TsHookCommandRequirement::AppValidateScript],
            critical_commands: Vec::new(),
        }],
    };
    super::super::check(&input)
        .into_iter()
        .map(|result| result.id().to_owned())
        .collect()
}

#[test]
fn comments_do_not_satisfy_g3ts_or_validate_commands() {
    let ids = run_case(
        r#"
# g3ts validate --path apps/landing
# pnpm --filter landing run validate
FILES=$(git diff --cached --name-only)
case "$FILES" in
  *guardrail3-ts.toml*) echo "changed" ;;
esac
"#,
    );

    assert!(
        ids.iter()
            .any(|id| id == "g3ts-hooks/g3ts-validate-staged-present"),
        "commented g3ts command must not satisfy hook contract"
    );
    assert!(
        ids.iter()
            .any(|id| id == "g3ts-hooks/ts-app-validate-step-present"),
        "commented validate command must not satisfy hook contract"
    );
}

#[test]
fn echo_text_does_not_satisfy_g3ts_or_validate_commands() {
    let ids = run_case(
        r#"
echo "g3ts validate --path apps/landing"
echo "pnpm --filter landing run validate"
if git diff --cached --name-only | grep -q guardrail3-ts.toml; then
  echo "guardrail config changed"
fi
"#,
    );

    assert!(
        ids.iter()
            .any(|id| id == "g3ts-hooks/g3ts-validate-staged-present"),
        "echoed g3ts command must not satisfy hook contract"
    );
    assert!(
        ids.iter()
            .any(|id| id == "g3ts-hooks/ts-app-validate-step-present"),
        "echoed validate command must not satisfy hook contract"
    );
}

#[test]
fn real_commands_satisfy_g3ts_and_validate_contracts() {
    let ids = run_case(
        r#"
FILES=$(git diff --cached --name-only)
echo "$FILES" | grep -q guardrail3-ts.toml
g3ts validate --path apps/landing
pnpm --filter landing run validate
"#,
    );

    assert!(
        !ids.iter()
            .any(|id| id == "g3ts-hooks/g3ts-validate-staged-present"),
        "real g3ts validate command should satisfy hook contract"
    );
    assert!(
        !ids.iter()
            .any(|id| id == "g3ts-hooks/ts-app-validate-step-present"),
        "real app validate command should satisfy hook contract"
    );
}

#[test]
fn wrong_app_commands_do_not_satisfy_app_contract() {
    let ids = run_case(
        r#"
FILES=$(git diff --cached --name-only)
echo "$FILES" | grep -q guardrail3-ts.toml
g3ts validate --path apps/web
pnpm --filter web run validate
"#,
    );

    assert!(
        ids.iter()
            .any(|id| id == "g3ts-hooks/g3ts-validate-staged-present"),
        "g3ts validate must target the app root covered by this hook contract"
    );
    assert!(
        ids.iter()
            .any(|id| id == "g3ts-hooks/ts-app-validate-step-present"),
        "package validate must target the app root covered by this hook contract"
    );
}

#[test]
fn family_scoped_g3ts_does_not_satisfy_full_hook_contract() {
    let ids = run_case(
        r#"
FILES=$(git diff --cached --name-only)
echo "$FILES" | grep -q guardrail3-ts.toml
g3ts validate --path apps/landing --family hooks
pnpm --filter landing run validate
"#,
    );

    assert!(
        ids.iter()
            .any(|id| id == "g3ts-hooks/g3ts-validate-staged-present"),
        "family-scoped g3ts validate must not satisfy the full pre-commit contract"
    );
}

#[test]
fn fail_open_critical_command_is_error() {
    let input = G3TsHooksSourceChecksInput {
        rel_path: ".githooks/pre-commit".to_owned(),
        kind: G3TsHookScriptKind::PreCommit,
        parsed: hook_shell_parser::parse_script(
            r#"
g3ts validate --path apps/landing || true
pnpm --filter landing run validate
"#,
        ),
        has_modular_dir: false,
        app_package_roots: vec!["apps/landing".to_owned()],
        requirements: vec![G3TsHookRequirement {
            id: "test/hook-contract".to_owned(),
            owner_family: "test".to_owned(),
            trigger_patterns: Vec::new(),
            required_commands: vec![G3TsHookCommandRequirement::AppValidateScript],
            critical_commands: Vec::new(),
        }],
    };

    let result = super::super::check(&input)
        .into_iter()
        .find(|result| result.id() == "g3ts-hooks/no-fail-open-wrappers")
        .expect("fail-open critical command should be reported");
    assert_eq!(
        result.severity(),
        guardrail3_check_types::G3Severity::Error,
        "fail-open wrapper must fail closed"
    );
}

#[test]
fn missing_tool_availability_guard_is_error() {
    let input = G3TsHooksSourceChecksInput {
        rel_path: ".githooks/pre-commit".to_owned(),
        kind: G3TsHookScriptKind::PreCommit,
        parsed: hook_shell_parser::parse_script(
            r#"
if command -v g3ts; then
  g3ts validate --path apps/landing
else
  echo "g3ts missing"
fi
pnpm --filter landing run validate
"#,
        ),
        has_modular_dir: false,
        app_package_roots: vec!["apps/landing".to_owned()],
        requirements: vec![G3TsHookRequirement {
            id: "test/hook-contract".to_owned(),
            owner_family: "test".to_owned(),
            trigger_patterns: Vec::new(),
            required_commands: vec![G3TsHookCommandRequirement::AppValidateScript],
            critical_commands: Vec::new(),
        }],
    };

    let result = super::super::check(&input)
        .into_iter()
        .find(|result| result.id() == "g3ts-hooks/no-fail-open-wrappers")
        .expect("non-failing availability guard should be reported");
    assert_eq!(
        result.severity(),
        guardrail3_check_types::G3Severity::Error,
        "availability guard that only echoes must fail closed"
    );
}
