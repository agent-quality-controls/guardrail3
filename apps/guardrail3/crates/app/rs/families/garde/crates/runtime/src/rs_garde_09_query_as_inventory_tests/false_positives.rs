use crate::test_fixtures::canonical_clippy_toml;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn does_not_flag_query_scalar() {
    let root = temp_root("rs-garde-09-query-scalar");
    let clippy_toml = canonical_clippy_toml();

    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml", "guardrail3.toml"]),
            ),
            ("src", dir_entry(&[], &["db.rs"])),
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
            (
                "src/db.rs",
                r#"
fn load() {
    let _value = sqlx::query_scalar!("select 1");
}
"#,
            ),
        ],
        root.clone(),
    );

    let results: Vec<_> = crate::test_fixtures::run_family(&tree)
        .into_iter()
        .filter(|result| result.id == "RS-GARDE-09")
        .collect();
    assert!(results.is_empty());

    std::fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn ignores_non_sqlx_query_as_macro_with_same_name() {
    let root = temp_root("rs-garde-09-non-sqlx-query-as");
    let clippy_toml = canonical_clippy_toml();

    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml", "guardrail3.toml"]),
            ),
            ("src", dir_entry(&[], &["db.rs"])),
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
            (
                "src/db.rs",
                r#"
macro_rules! query_as {
    ($ty:ty, $sql:expr) => {
        ($sql, stringify!($ty))
    };
}

fn load() {
    let _value = query_as!(User, "select 1");
}
"#,
            ),
        ],
        root.clone(),
    );

    let results: Vec<_> = crate::test_fixtures::run_family(&tree)
        .into_iter()
        .filter(|result| result.id == "RS-GARDE-09")
        .collect();
    assert!(
        results.is_empty(),
        "non-sqlx query_as macros should not be inventoried"
    );

    std::fs::remove_dir_all(root).expect("cleanup");
}
