use crate::domain::report::Severity;

use super::super::super::inputs::RustCodeFileInput;
use super::super::super::parse::parse_rust_file;
use super::super::check;

#[test]
fn warns_on_pub_use_glob_in_library_lib_rs() {
    let content = "pub use crate::inner::*;";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/lib.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: Some("library"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-CODE-26");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "glob re-export in lib.rs");
    assert_eq!(
        results[0].message,
        "`pub use crate::inner::*` creates an unstable API surface."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}
