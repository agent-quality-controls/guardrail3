use crate::domain::report::Severity;

use super::super::super::inputs::RustCodeFileInput;
use super::super::super::parse::parse_rust_file;
use super::super::check;

#[test]
fn warns_between_sixteen_and_twenty_top_level_uses() {
    let mut lines: Vec<String> = (0..16)
        .map(|index| format!("use crate::mod_{index};"))
        .collect();
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
    assert_eq!(results[0].id, "RS-CODE-11");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert_eq!(results[0].line, None);
    assert_eq!(results[0].title, "many use statements");
    assert_eq!(
        results[0].message,
        "16 top-level use statements (warn at 16, max 20)."
    );
}

#[test]
fn skips_below_warn_band_in_non_test_file() {
    let mut lines: Vec<String> = (0..15)
        .map(|index| format!("use crate::mod_{index};"))
        .collect();
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

    assert!(results.is_empty());
}

#[test]
fn skips_above_warn_band_in_non_test_file() {
    let mut lines: Vec<String> = (0..21)
        .map(|index| format!("use crate::mod_{index};"))
        .collect();
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

    assert!(results.is_empty());
}
