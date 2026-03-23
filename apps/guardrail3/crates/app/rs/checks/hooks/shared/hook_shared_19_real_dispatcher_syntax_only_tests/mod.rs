use crate::app::rs::checks::hooks::shell::parse_script;
use crate::domain::report::Severity;

use super::super::hook_shared_19_real_dispatcher_syntax_only::check;
use super::super::inputs::DispatcherSyntaxInput;

#[test]
fn warns_when_modular_dir_exists_but_only_comment_mentions_dispatcher() {
    let content = "# source .githooks/pre-commit.d/10-rust.sh\n";
    let parsed = parse_script(content);
    let input = DispatcherSyntaxInput {
        rel_path: ".githooks/pre-commit",
        has_modular_dir: true,
        parsed: &parsed,
    };

    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "HOOK-SHARED-19");
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn passes_when_real_dispatcher_command_exists() {
    let content = r#". ".githooks/pre-commit.d/10-rust.sh""#;
    let parsed = parse_script(content);
    let input = DispatcherSyntaxInput {
        rel_path: ".githooks/pre-commit",
        has_modular_dir: true,
        parsed: &parsed,
    };

    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}
