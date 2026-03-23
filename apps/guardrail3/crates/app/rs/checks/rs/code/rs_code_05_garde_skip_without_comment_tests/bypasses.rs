use crate::domain::report::Severity;

use super::super::super::inputs::RustCodeFileInput;
use super::super::super::parse::parse_rust_file;
use super::super::check;

#[test]
fn detects_multiple_non_primitive_garde_skips_without_comments() {
    let content = r#"
struct One {
    #[garde(skip)]
    field: String,
}

struct Two {
    #[garde(skip)]
    other: Vec<String>,
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
    assert!(results.iter().all(|result| result.id == "RS-CODE-05"));
    assert!(
        results
            .iter()
            .all(|result| result.severity == Severity::Error)
    );
    assert_eq!(
        results.iter().map(|result| result.line).collect::<Vec<_>>(),
        vec![Some(3), Some(8)]
    );
}
