use crate::test_support::{canonical_clippy_toml, dir_entry, project_tree, temp_root};
use guardrail3_domain_report::Severity;

#[test]
fn errors_when_ctx_usage_has_no_type_level_context() {
    let root = temp_root("rs-garde-13-missing");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    let clippy_toml = canonical_clippy_toml();
    std::fs::create_dir_all(source_abs.parent().expect("parent")).expect("mkdir");
    std::fs::write(
        &source_abs,
        r#"
use serde::Deserialize;
use garde::Validate;

struct ValidationConfig {
    title_min: usize,
    title_max: usize,
}

#[derive(Deserialize, Validate)]
struct Input {
    #[garde(length(chars, min = ctx.title_min, max = ctx.title_max))]
    title: String,
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
garde = { version = "0.22", features = ["derive"] }
"#,
            ),
            ("clippy.toml", clippy_toml.as_str()),
            ("guardrail3.toml", "[profile]\nname = \"service\"\n"),
        ],
        root.clone(),
    );

    let results: Vec<_> = crate::test_support::run_family(&tree)
        .into_iter()
        .filter(|result| result.id == "RS-GARDE-13")
        .collect();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].file.as_deref(), Some(source_rel));
    assert_eq!(
        results[0].title,
        "boundary `Input` uses ctx without garde(context)"
    );
    assert_eq!(
        results[0].message,
        "Field `title` in validated boundary `Input` references `ctx` in a garde validator, but the boundary type is missing `#[garde(context(...))]`."
    );

    std::fs::remove_dir_all(root).expect("cleanup");
}
