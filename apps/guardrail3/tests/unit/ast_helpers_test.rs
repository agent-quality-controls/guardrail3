use guardrail3::app::rs::validate::ast_helpers::*;

fn must_parse(source: &str) -> syn::File {
    #[allow(clippy::expect_used)] // reason: test helper — panic on bad input is correct
    parse_file(source).expect("test input should be valid Rust")
}

#[test]
fn parse_file_valid_and_invalid() {
    assert!(parse_file("fn main() {}").is_some(), "valid Rust parses");
    assert!(parse_file("not rust {{{").is_none(), "invalid returns None");
}

#[test]
fn crate_level_allow_found() {
    let allows = find_crate_level_allows(&must_parse("#![allow(dead_code)]\nfn main() {}"));
    assert_eq!(allows.len(), 1, "should find one crate-level allow");
    assert_eq!(allows.first().map(|(_, s)| s.as_str()), Some("dead_code"));
}

#[test]
fn crate_level_allow_in_string_not_found() {
    let src = "fn main() { let _s = \"#![allow(dead_code)]\"; }";
    assert!(
        find_crate_level_allows(&must_parse(src)).is_empty(),
        "no match in string"
    );
}

#[test]
fn crate_level_allow_multiple_lints() {
    let src = "#![allow(dead_code, unused_variables)]\nfn main() {}";
    assert_eq!(
        find_crate_level_allows(&must_parse(src)).len(),
        2,
        "two lints in one allow"
    );
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vector
fn item_allow_found() {
    let attr = ["#[allow(", "clippy::unwrap_used)]"].concat(); // pre-commit safe
    let src = format!("{attr}\nfn foo() {{}}");
    let allows = find_item_allows(&must_parse(&src));
    assert_eq!(allows.len(), 1, "should find item-level allow");
    assert_eq!(allows[0].1, "clippy::unwrap_used");
}

#[test]
fn item_allow_in_string_not_found() {
    let inner = ["#[allow(", "clippy::unwrap_used)]"].concat(); // reason: test data built via concat
    let src = format!("fn foo() {{ let _s = \"{inner}\"; }}");
    assert!(
        find_item_allows(&must_parse(&src)).is_empty(),
        "no match in string"
    );
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vector
fn item_allow_on_impl_method() {
    let attr = ["#[allow(", "dead_code)]"].concat(); // reason: test data built via concat
    let src = format!("struct S;\nimpl S {{\n    {attr}\n    fn method(&self) {{}}\n}}");
    let allows = find_item_allows(&must_parse(&src));
    assert_eq!(allows.len(), 1, "should find allow on impl method");
    assert_eq!(allows[0].1, "dead_code");
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vector
fn cfg_attr_allow_found() {
    let allows = find_cfg_attr_allows(&must_parse(
        "#[cfg_attr(test, allow(dead_code))]\nfn foo() {}",
    ));
    assert_eq!(allows.len(), 1, "should find cfg_attr allow");
    assert_eq!(allows[0].lint, "dead_code");
    assert!(
        !allows[0].is_always_true,
        "test condition is not always true"
    );
}

#[test]
fn cfg_attr_allow_in_string_not_found() {
    let inner = "#[cfg_attr(test, allow(dead_code))]";
    let src = format!("fn foo() {{ let _s = \"{inner}\"; }}");
    assert!(
        find_cfg_attr_allows(&must_parse(&src)).is_empty(),
        "no match in string"
    );
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vector
fn cfg_attr_all_empty_is_always_true() {
    let allows = find_cfg_attr_allows(&must_parse(
        "#[cfg_attr(all(), allow(dead_code))]\nfn foo() {}",
    ));
    assert_eq!(allows.len(), 1, "should find cfg_attr allow");
    assert_eq!(allows[0].lint, "dead_code");
    assert!(
        allows[0].is_always_true,
        "all() with no args is always true"
    );
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vector
fn cfg_attr_all_with_args_is_not_always_true() {
    let allows = find_cfg_attr_allows(&must_parse(
        "#[cfg_attr(all(unix), allow(dead_code))]\nfn foo() {}",
    ));
    assert_eq!(allows.len(), 1, "should find cfg_attr allow");
    assert_eq!(allows[0].lint, "dead_code");
    assert!(!allows[0].is_always_true, "all(unix) is not always true");
}

#[test]
fn cfg_attr_any_is_not_always_true() {
    let allows = find_cfg_attr_allows(&must_parse(
        "#[cfg_attr(any(unix, windows), allow(dead_code))]\nfn foo() {}",
    ));
    assert!(!allows.is_empty(), "should find cfg_attr allow");
    assert!(
        !allows.iter().any(|a| a.is_always_true),
        "any(...) is not detected as always-true"
    );
}

#[test]
fn garde_skip_found() {
    let src = "use garde::Validate;\n\n\
        #[derive(Validate)]\nstruct Input {\n    #[garde(skip)]\n    name: String,\n}";
    assert_eq!(
        find_garde_skips(&must_parse(src)).len(),
        1,
        "should find garde(skip)"
    );
}

#[test]
fn garde_skip_in_string_not_found() {
    let src = format!("fn foo() {{ let _s = \"{}\"; }}", "#[garde(skip)]");
    assert!(
        find_garde_skips(&must_parse(&src)).is_empty(),
        "no match in string"
    );
}

#[test]
fn unsafe_block_found() {
    let src = "fn foo() { unsafe { std::ptr::null::<u8>(); } }";
    assert_eq!(
        find_unsafe_usage(&must_parse(src)).len(),
        1,
        "should find unsafe block"
    );
}

#[test]
fn unsafe_fn_found() {
    assert_eq!(
        find_unsafe_usage(&must_parse("unsafe fn d() {}")).len(),
        1,
        "unsafe fn"
    );
}

#[test]
fn unsafe_in_string_not_found() {
    let src = "fn foo() { let _s = \"unsafe { bad() }\"; }";
    assert!(
        find_unsafe_usage(&must_parse(src)).is_empty(),
        "no match in string"
    );
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vector
fn forbidden_macros_found() {
    let m1 = find_forbidden_macros(&must_parse("fn f() { todo!(); }"));
    assert_eq!(m1.len(), 1, "todo found");
    assert_eq!(m1[0].1, "todo");
    let m2 = find_forbidden_macros(&must_parse("fn f() { unimplemented!(); }"));
    assert_eq!(m2.len(), 1, "unimplemented found");
    assert_eq!(m2[0].1, "unimplemented");
    let m3 = find_forbidden_macros(&must_parse("fn f() { panic!(\"oh\"); }"));
    assert_eq!(m3.len(), 1, "panic found");
    assert_eq!(m3[0].1, "panic");
}

#[test]
fn todo_in_string_not_found() {
    let src = "fn foo() { let _s = \"todo!()\"; }";
    assert!(
        find_forbidden_macros(&must_parse(src)).is_empty(),
        "no match in string"
    );
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vector
fn unwrap_expect_found() {
    let u = find_unwrap_expect(&must_parse("fn f() { Some(1).unwrap(); }"));
    assert_eq!(u.len(), 1, "unwrap found");
    assert_eq!(u[0].1, "unwrap");
    let e = find_unwrap_expect(&must_parse("fn f() { Some(1).expect(\"m\"); }"));
    assert_eq!(e.len(), 1, "expect found");
    assert_eq!(e[0].1, "expect");
}

#[test]
fn unwrap_in_string_not_found() {
    let src = "fn foo() { let _s = \".unwrap()\"; }";
    assert!(
        find_unwrap_expect(&must_parse(src)).is_empty(),
        "no match in string"
    );
}

#[test]
fn std_fs_import_found() {
    assert_eq!(
        find_std_fs_imports(&must_parse("use std::fs;\nfn main() {}")).len(),
        1,
        "std::fs"
    );
    assert_eq!(
        find_std_fs_imports(&must_parse("use std::fs::read_to_string;\nfn main() {}")).len(),
        1,
        "std::fs::read_to_string"
    );
}

#[test]
fn std_fs_in_string_not_found() {
    let src = "fn foo() { let _s = \"use std::fs;\"; }";
    assert!(
        find_std_fs_imports(&must_parse(src)).is_empty(),
        "no match in string"
    );
}

#[test]
fn non_std_fs_not_matched() {
    assert!(
        find_std_fs_imports(&must_parse("use std::io;\nfn main() {}")).is_empty(),
        "io != fs"
    );
}

#[test]
fn count_use_statements_works() {
    let two = "use std::io;\nuse std::path::Path;\nfn main() {}";
    assert_eq!(count_use_statements(&must_parse(two)), 2, "two uses");
    assert_eq!(
        count_use_statements(&must_parse("fn main() {}")),
        0,
        "no uses"
    );
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vector
fn split_derives_merged_into_one_derive_info() {
    let src = r"
#[derive(Deserialize)]
#[derive(Validate)]
struct Foo {}

#[derive(Serialize, Clone)]
struct Bar {}
";
    let parsed = must_parse(src);
    let derives = find_derive_attributes(&parsed);
    assert_eq!(derives.len(), 2, "two items, two DeriveInfo entries");
    // Foo: split derives merged into one entry
    assert_eq!(
        derives[0].macros.len(),
        2,
        "Foo should have 2 macros from split derives"
    );
    assert_eq!(derives[0].macros[0], "Deserialize");
    assert_eq!(derives[0].macros[1], "Validate");
    // Bar: single derive with two macros
    assert_eq!(
        derives[1].macros.len(),
        2,
        "Bar should have 2 macros from single derive"
    );
    assert_eq!(derives[1].macros[0], "Serialize");
    assert_eq!(derives[1].macros[1], "Clone");
}

// ---------------------------------------------------------------------------
// R58: glob import detection (`use std::*` and `use std::fs::*`)
// ---------------------------------------------------------------------------

#[test]
fn std_glob_import_detected() {
    // `use std::*` imports everything including fs — must be caught
    assert_eq!(
        find_std_fs_imports(&must_parse("use std::*;\nfn main() {}")).len(),
        1,
        "use std::* should be detected as importing std::fs"
    );
}

#[test]
fn std_fs_glob_import_detected() {
    // `use std::fs::*` imports all fs functions — must be caught
    assert_eq!(
        find_std_fs_imports(&must_parse("use std::fs::*;\nfn main() {}")).len(),
        1,
        "use std::fs::* should be detected"
    );
}

#[test]
fn std_group_with_glob_detected() {
    // `use std::{fs::*, io}` — group containing a glob under fs
    assert_eq!(
        find_std_fs_imports(&must_parse("use std::{fs::*, io};\nfn main() {}")).len(),
        1,
        "use std::{{fs::*, io}} should be detected"
    );
}

// ---------------------------------------------------------------------------
// R30: inline module inner attribute detection
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vector
fn inline_mod_allow_detected() {
    let src = "mod foo {\n    #![allow(dead_code)]\n    fn bar() {}\n}\n";
    let parsed = must_parse(src);
    let allows = find_inline_mod_allows(&parsed);
    assert_eq!(allows.len(), 1, "should find one inline mod allow");
    assert_eq!(allows[0].lint, "dead_code");
    assert_eq!(allows[0].module_path, "foo");
}

#[test]
fn inline_mod_allow_multiple_lints() {
    let src = "mod foo {\n    #![allow(dead_code, unused_variables)]\n    fn bar() {}\n}\n";
    let parsed = must_parse(src);
    let allows = find_inline_mod_allows(&parsed);
    assert_eq!(allows.len(), 2, "should find two lints in one allow");
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vector
fn nested_inline_mod_allow_detected() {
    let src = "mod outer {\n    mod inner {\n        #![allow(clippy::all)]\n        fn baz() {}\n    }\n}\n";
    let parsed = must_parse(src);
    let allows = find_inline_mod_allows(&parsed);
    assert_eq!(allows.len(), 1, "should find nested mod allow");
    assert_eq!(allows[0].lint, "clippy::all");
    assert_eq!(allows[0].module_path, "outer::inner");
}

#[test]
fn external_mod_no_false_positive() {
    // `mod foo;` (external module, no body) should not produce results
    let src = "mod foo;\nfn main() {}\n";
    let parsed = must_parse(src);
    let allows = find_inline_mod_allows(&parsed);
    assert!(allows.is_empty(), "external mod should not produce results");
}

#[test]
fn crate_level_allow_not_in_inline_mod() {
    // `#![allow(dead_code)]` at crate level should NOT appear in inline mod results
    let src = "#![allow(dead_code)]\nfn main() {}\n";
    let parsed = must_parse(src);
    let allows = find_inline_mod_allows(&parsed);
    assert!(
        allows.is_empty(),
        "crate-level allow is not an inline mod allow"
    );
}
