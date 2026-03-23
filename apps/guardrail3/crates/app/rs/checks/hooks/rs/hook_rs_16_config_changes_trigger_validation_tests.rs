use crate::app::rs::checks::hooks::shell::parse_script;
use crate::domain::report::Severity;

use super::super::inputs::RustHookCommandInput;
use super::check;

#[test]
fn warns_when_config_names_only_appear_in_comment() {
    let content = "# guardrail3.toml clippy.toml .clippy.toml deny.toml .deny.toml rustfmt.toml .rustfmt.toml rust-toolchain.toml\n";
    let parsed = parse_script(content);
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(content, &input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_trigger_logic_checks_all_rust_guardrail_configs() {
    let content = r#"
if echo "$STAGED_FILES" | grep -qE '(guardrail3\.toml|clippy\.toml|\.clippy\.toml|deny\.toml|\.deny\.toml|rustfmt\.toml|\.rustfmt\.toml|rust-toolchain\.toml)$'; then
    guardrail3 rs validate --staged .
fi
"#;
    let parsed = parse_script(content);
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(content, &input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(results[0].inventory);
}
