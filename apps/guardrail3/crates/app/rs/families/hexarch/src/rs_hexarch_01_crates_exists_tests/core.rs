use guardrail3_domain_report::Severity;

use super::super::check;
use crate::inputs::AppHexarchInput;

fn input(top_level_crates_entry_count: usize) -> AppHexarchInput<'static> {
    AppHexarchInput {
        app_name: "backend",
        app_rel_dir: "apps/backend",
        cargo_rel_path: "apps/backend/Cargo.toml",
        cargo_parse_error: None,
        is_workspace: true,
        top_level_crates_entry_count,
        src_dir_exists: false,
    }
}

#[test]
fn passes_when_top_level_crates_dir_has_entries() {
    let input = input(1);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn fails_when_top_level_crates_dir_has_no_entries() {
    let input = input(0);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1, "{results:#?}");
    assert_eq!(results[0].id, "RS-HEXARCH-01");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].file.as_deref(), Some("apps/backend"));
    assert!(results[0].title.contains("missing crates/"), "{results:#?}");
}
