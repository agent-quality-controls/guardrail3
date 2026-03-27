use guardrail3_domain_report::Severity;

use super::super::super::inputs::RustCodeFileInput;
use super::super::super::parse::parse_rust_file;
use super::super::check;

#[test]
fn errors_on_allow_attr_on_extern_block() {
    let content = r#"
#[allow(improper_ctypes)]
unsafe extern "C" {
    fn puts(s: *const i8);
}
"#;
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/ffi.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-CODE-20");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "allow on extern block");
    assert_eq!(
        results[0].message,
        "`#[allow(improper_ctypes)]` on an `extern` block hides FFI risk behind a broad suppression."
    );
    assert_eq!(results[0].file.as_deref(), Some("src/ffi.rs"));
    assert_eq!(results[0].line, Some(2));
    assert!(!results[0].inventory);
}

#[test]
fn errors_on_multiple_lints_from_one_extern_block_allow_attribute() {
    let content = r#"
#[allow(improper_ctypes, clippy::all)]
unsafe extern "C" {
    fn puts(s: *const i8);
}
"#;
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/ffi.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    results.sort_by_key(|result| result.message.clone());
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].id, "RS-CODE-20");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "allow on extern block");
    assert_eq!(results[0].file.as_deref(), Some("src/ffi.rs"));
    assert!(!results[0].inventory);
    assert_eq!(
        results[0].message,
        "`#[allow(clippy::all)]` on an `extern` block hides FFI risk behind a broad suppression."
    );
    assert_eq!(results[1].id, "RS-CODE-20");
    assert_eq!(results[1].severity, Severity::Error);
    assert_eq!(results[1].title, "allow on extern block");
    assert_eq!(results[1].file.as_deref(), Some("src/ffi.rs"));
    assert!(!results[1].inventory);
    assert_eq!(
        results[1].message,
        "`#[allow(improper_ctypes)]` on an `extern` block hides FFI risk behind a broad suppression."
    );
    assert_eq!(results[0].line, Some(2));
    assert_eq!(results[1].line, Some(2));
}

#[test]
fn errors_when_stacked_allow_attrs_cover_the_same_extern_block() {
    let content = r#"
#[allow(improper_ctypes)]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" {
    fn puts(s: *const i8);
}
"#;
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/ffi.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    results.sort_by_key(|result| (result.line, result.message.clone()));
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].line, Some(2));
    assert_eq!(results[1].line, Some(3));
    assert_eq!(
        results[0].message,
        "`#[allow(improper_ctypes)]` on an `extern` block hides FFI risk behind a broad suppression."
    );
    assert_eq!(
        results[1].message,
        "`#[allow(improper_ctypes_definitions)]` on an `extern` block hides FFI risk behind a broad suppression."
    );
}

#[test]
fn errors_when_cfg_attr_allow_covers_an_extern_block() {
    let content = r#"
mod ffi_surface {
    #[cfg_attr(feature = "ffi", allow(improper_ctypes))]
    unsafe extern "C" {
        fn puts(s: *const i8);
    }
}
"#;
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/ffi.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-CODE-20");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "allow on extern block");
    assert_eq!(results[0].file.as_deref(), Some("src/ffi.rs"));
    assert!(!results[0].inventory);
    assert_eq!(
        results[0].message,
        "`#[cfg_attr(..., allow(improper_ctypes))]` on an `extern` block hides FFI risk behind a broad suppression."
    );
    assert_eq!(results[0].line, Some(3));
}

#[test]
fn errors_on_multiple_lints_from_one_cfg_attr_allow_on_extern_block() {
    let content = r#"
#[cfg_attr(feature = "ffi", allow(improper_ctypes, clippy::all))]
unsafe extern "C" {
    fn puts(s: *const i8);
}
"#;
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/ffi.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    results.sort_by_key(|result| result.message.clone());
    assert_eq!(results.len(), 2);
    assert_eq!(
        results[0].message,
        "`#[cfg_attr(..., allow(clippy::all))]` on an `extern` block hides FFI risk behind a broad suppression."
    );
    assert_eq!(
        results[1].message,
        "`#[cfg_attr(..., allow(improper_ctypes))]` on an `extern` block hides FFI risk behind a broad suppression."
    );
    assert_eq!(results[0].line, Some(2));
    assert_eq!(results[1].line, Some(2));
}

#[test]
fn errors_on_mixed_allow_and_cfg_attr_on_the_same_extern_block() {
    let content = r#"
#[allow(improper_ctypes)]
#[cfg_attr(feature = "ffi", allow(improper_ctypes_definitions))]
unsafe extern "C" {
    fn puts(s: *const i8);
}
"#;
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/ffi.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    results.sort_by_key(|result| (result.line, result.message.clone()));
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].line, Some(2));
    assert_eq!(results[1].line, Some(3));
    assert_eq!(
        results[0].message,
        "`#[allow(improper_ctypes)]` on an `extern` block hides FFI risk behind a broad suppression."
    );
    assert_eq!(
        results[1].message,
        "`#[cfg_attr(..., allow(improper_ctypes_definitions))]` on an `extern` block hides FFI risk behind a broad suppression."
    );
}
