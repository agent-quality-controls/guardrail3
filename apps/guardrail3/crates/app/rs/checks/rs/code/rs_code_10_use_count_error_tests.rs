use crate::domain::report::Severity;

use super::super::inputs::RustCodeFileInput;
use super::super::parse::parse_rust_file;
use super::check;

#[test]
fn errors_above_twenty_top_level_uses() {
    let mut lines: Vec<String> = (0..21).map(|index| format!("use crate::mod_{index};")).collect();
    lines.push("fn main() {}".to_owned());
    let content = lines.join("\n");
    let ast = parse_rust_file(&content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/foo.rs",
        content: &content,
        ast: &ast,
        is_test: false,
        profile_name: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CODE-10");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.message, "21 top-level use statements (max 20).");
}
