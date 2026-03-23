use crate::domain::report::Severity;

use super::super::inputs::GardeRootInput;
use super::super::test_support::{dir_entry, has_result, project_tree, root_facts, temp_root};
use super::check;

#[test]
fn errors_when_garde_dependency_missing() {
    let mut results = Vec::new();
    check(&GardeRootInput::new(&root_facts(false)), &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-GARDE-01");
    assert_eq!(results[0].severity, Severity::Error);
}

#[test]
fn inventories_when_garde_dependency_present() {
    let mut results = Vec::new();
    check(&GardeRootInput::new(&root_facts(true)), &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-GARDE-01");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn family_skips_deeper_garde_rules_when_dependency_missing() {
    let root = temp_root("rs-garde-01-skip-deeper");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    std::fs::create_dir_all(source_abs.parent().expect("parent")).expect("mkdir");
    std::fs::write(
        &source_abs,
        r#"
use serde::Deserialize;

#[derive(Deserialize)]
struct Input {
    name: String,
}
"#,
    )
    .expect("write");

    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml", "guardrail3.toml"]),
            ),
            ("src", dir_entry(&[], &["input.rs"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"[workspace]
members = []
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
"#,
            ),
            (
                "clippy.toml",
                "disallowed-methods = []\ndisallowed-types = []\n",
            ),
            ("guardrail3.toml", "[profile]\nname = \"service\"\n"),
        ],
        root.clone(),
    );

    let results = crate::app::rs::checks::rs::garde::check(&tree);
    assert!(has_result(&results, "RS-GARDE-01", |result| result
        .severity
        == Severity::Error));
    for rule_id in [
        "RS-GARDE-02",
        "RS-GARDE-03",
        "RS-GARDE-04",
        "RS-GARDE-05",
        "RS-GARDE-06",
        "RS-GARDE-07",
        "RS-GARDE-08",
        "RS-GARDE-09",
    ] {
        assert!(
            !has_result(&results, rule_id, |_| true),
            "unexpected deeper garde result {rule_id}"
        );
    }

    std::fs::remove_dir_all(root).expect("cleanup");
}
