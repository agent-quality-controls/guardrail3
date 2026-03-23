use crate::domain::report::Severity;

use super::super::super::inputs::RustCodeFileInput;
use super::super::super::parse::parse_rust_file;
use super::super::check;

#[test]
fn errors_on_trait_with_thirteen_methods() {
    let mut methods = String::new();
    for index in 0..13 {
        methods.push_str(&format!("    fn m{index}(&self);\n"));
    }
    let content = format!("pub trait Service {{\n{methods}}}");
    let ast = parse_rust_file(&content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/lib.rs",
        content: &content,
        ast: &ast,
        is_test: false,
        profile_name: Some("library"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
}
