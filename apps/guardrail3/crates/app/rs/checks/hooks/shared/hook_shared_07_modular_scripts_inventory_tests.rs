use super::super::facts::{HookScriptFacts, HookScriptKind};
use super::check;

#[test]
fn inventories_no_modular_scripts() {
    let mut results = Vec::new();
    check(&[], &mut results);
    assert!(results[0].inventory);
    assert_eq!(results[0].title, "no modular hook scripts");
}

#[test]
fn inventories_modular_script_names() {
    let scripts = vec![
        HookScriptFacts {
            rel_path: ".githooks/pre-commit.d/10-rust.sh".to_owned(),
            kind: HookScriptKind::Modular,
            content: String::new(),
        },
        HookScriptFacts {
            rel_path: ".githooks/pre-commit.d/20-ts.sh".to_owned(),
            kind: HookScriptKind::Modular,
            content: String::new(),
        },
    ];
    let mut results = Vec::new();
    check(&scripts, &mut results);
    assert!(results[0].inventory);
    assert!(results[0].message.contains("10-rust.sh"));
    assert!(results[0].message.contains("20-ts.sh"));
}
