use super::super::super::inputs::RustCodeFileInput;
use super::super::super::parse::parse_rust_file;
use super::super::check;

#[test]
fn golden_file_with_documented_item_allow_has_no_undocumented_hits() {
    let content = "#[allow(clippy::unwrap_used)] // reason: compatibility shim\nfn foo() {}";
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
