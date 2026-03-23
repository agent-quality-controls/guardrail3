use crate::domain::report::Severity;

use super::super::super::inputs::RustCodeFileInput;
use super::super::super::parse::parse_rust_file;
use super::super::check;

#[test]
fn detects_multiple_undocumented_item_level_allows_in_one_owned_file() {
    let content = r#"
#[allow(clippy::unwrap_used)]
fn first() {}

mod nested {
    #[allow(clippy::panic)]
    pub fn second() {}
}
"#;
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/foo.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|result| result.id == "RS-CODE-03"));
    assert!(
        results
            .iter()
            .all(|result| result.severity == Severity::Error)
    );
    assert_eq!(
        results.iter().map(|result| result.line).collect::<Vec<_>>(),
        vec![Some(2), Some(6)]
    );
    assert_eq!(
        results
            .iter()
            .map(|result| result.file.as_deref())
            .collect::<Vec<_>>(),
        vec![Some("src/foo.rs"), Some("src/foo.rs")]
    );
}
