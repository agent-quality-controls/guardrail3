use super::super::super::inputs::RustCodeFileInput;
use super::super::super::parse::parse_rust_file;
use super::super::check;

#[test]
fn golden_non_primitive_garde_skip_with_reason_is_not_flagged_here() {
    let content =
        "struct Foo {\n    #[garde(skip)] // reason: validated elsewhere\n    field: String,\n}";
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

    assert!(results.is_empty());
}
