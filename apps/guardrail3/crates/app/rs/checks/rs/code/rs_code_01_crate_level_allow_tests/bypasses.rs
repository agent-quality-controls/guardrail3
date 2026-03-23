use crate::domain::report::Severity;

use super::super::super::inputs::RustCodeFileInput;
use super::super::super::parse::parse_rust_file;
use super::super::check;

#[test]
fn attacks_crate_and_nested_module_wide_allows_in_one_owned_file() {
    let content = r#"
#![allow(clippy::unwrap_used)]

mod outer {
    #![allow(clippy::panic)]

    mod inner {
        #![allow(clippy::expect_used)]
        pub fn helper() {}
    }
}
"#;
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

    assert_eq!(results.len(), 3);
    assert_eq!(
        results
            .iter()
            .map(|result| result.line.expect("line"))
            .collect::<Vec<_>>(),
        vec![2, 5, 8]
    );
    assert!(results.iter().all(|result| result.id == "RS-CODE-01"));
    assert!(
        results
            .iter()
            .all(|result| result.severity == Severity::Error)
    );
    assert_eq!(
        results
            .iter()
            .map(|result| result.file.as_deref())
            .collect::<Vec<_>>(),
        vec![Some("src/lib.rs"), Some("src/lib.rs"), Some("src/lib.rs")]
    );
}
