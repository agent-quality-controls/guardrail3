use crate::ast_helpers::*;
use guardrail3_app_rs_ast_assertions::ast_helpers::assert_single_cfg_attr_allow;

fn must_parse(source: &str) -> syn::File {
    match parse_file(source) {
        Some(file) => file,
        None => panic!("test input should be valid Rust"),
    }
}

fn only_item_allow<'a>(allows: &'a [(usize, String)], expected_lint: &str) -> &'a str {
    assert_eq!(allows.len(), 1, "should find exactly one item-level allow");
    let Some((_, lint)) = allows.first() else {
        panic!("expected exactly one item-level allow");
    };
    assert_eq!(lint, expected_lint);
    lint
}

fn only_cfg_attr_allow<'a>(allows: &'a [CfgAttrAllowInfo]) -> &'a CfgAttrAllowInfo {
    assert_eq!(allows.len(), 1, "should find exactly one cfg_attr allow");
    let Some(allow) = allows.first() else {
        panic!("expected exactly one cfg_attr allow");
    };
    allow
}

fn only_macro_name<'a>(macros: &'a [(usize, String)], expected_name: &str) -> &'a str {
    assert_eq!(macros.len(), 1, "should find exactly one forbidden macro");
    let Some((_, name)) = macros.first() else {
        panic!("expected exactly one forbidden macro");
    };
    assert_eq!(name, expected_name);
    name
}

fn only_unwrap_expect<'a>(items: &'a [(usize, String)], expected_name: &str) -> &'a str {
    assert_eq!(items.len(), 1, "should find exactly one unwrap/expect call");
    let Some((_, name)) = items.first() else {
        panic!("expected exactly one unwrap/expect call");
    };
    assert_eq!(name, expected_name);
    name
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
        find_crate_level_allows(&must_parse(&src)).len(),
        2,
        "two lints in one allow"
    );
}

#[test]
fn item_allow_found() {
    let attr = ["#[allow(", "clippy::unwrap_used)]"].concat();
    let src = format!("{attr}\nfn foo() {{}}");
    let allows = find_item_allows(&must_parse(&src));
    let _ = only_item_allow(&allows, "clippy::unwrap_used");
}

#[test]
fn item_allow_in_string_not_found() {
    let inner = ["#[allow(", "clippy::unwrap_used)]"].concat();
    let src = format!("fn foo() {{ let _s = \"{inner}\"; }}");
    assert!(
        find_item_allows(&must_parse(&src)).is_empty(),
        "no match in string"
    );
}

#[test]
fn item_allow_on_impl_method() {
    let attr = ["#[allow(", "dead_code)]"].concat();
    let src = format!("struct S;\nimpl S {{\n    {attr}\n    fn method(&self) {{}}\n}}");
    let allows = find_item_allows(&must_parse(&src));
    let _ = only_item_allow(&allows, "dead_code");
}

#[test]
fn cfg_attr_allow_found() {
    let allows = find_cfg_attr_allows(&must_parse(
        "#[cfg_attr(test, allow(dead_code))]\nfn foo() {}",
    ));
    let allow = only_cfg_attr_allow(&allows);
    assert_single_cfg_attr_allow(
        allows.len(),
        allow.line,
        &allow.lint,
        allow.is_always_true,
        1,
        "dead_code",
        false,
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
fn cfg_attr_all_empty_is_always_true() {
    let allows = find_cfg_attr_allows(&must_parse(
        "#[cfg_attr(all(), allow(dead_code))]\nfn foo() {}",
    ));
    let allow = only_cfg_attr_allow(&allows);
    assert_single_cfg_attr_allow(
        allows.len(),
        allow.line,
        &allow.lint,
        allow.is_always_true,
        1,
        "dead_code",
        true,
    );
}

#[test]
fn cfg_attr_all_with_args_is_not_always_true() {
    let allows = find_cfg_attr_allows(&must_parse(
        "#[cfg_attr(all(unix), allow(dead_code))]\nfn foo() {}",
    ));
    let allow = only_cfg_attr_allow(&allows);
    assert_single_cfg_attr_allow(
        allows.len(),
        allow.line,
        &allow.lint,
        allow.is_always_true,
        1,
        "dead_code",
        false,
    );
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
fn cfg_attr_allow_found_on_trait_item() {
    let allows = find_cfg_attr_allows(&must_parse(
        "trait Api {\n    #[cfg_attr(test, allow(dead_code))]\n    fn run();\n}",
    ));
    let allow = only_cfg_attr_allow(&allows);
    assert_single_cfg_attr_allow(
        allows.len(),
        allow.line,
        &allow.lint,
        allow.is_always_true,
        2,
        "dead_code",
        false,
    );
}

#[test]
fn nested_cfg_attr_allow_found() {
    let allows = find_cfg_attr_allows(&must_parse(
        "#[cfg_attr(test, cfg_attr(unix, allow(dead_code)))]\nunsafe extern \"C\" { fn puts(s: *const i8); }",
    ));
    let allow = only_cfg_attr_allow(&allows);
    assert_single_cfg_attr_allow(
        allows.len(),
        allow.line,
        &allow.lint,
        allow.is_always_true,
        1,
        "dead_code",
        false,
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
fn forbidden_macros_found() {
    let m1 = find_forbidden_macros(&must_parse("fn f() { todo!(); }"));
    let _ = only_macro_name(&m1, "todo");
    let m2 = find_forbidden_macros(&must_parse("fn f() { unimplemented!(); }"));
    let _ = only_macro_name(&m2, "unimplemented");
    let m3 = find_forbidden_macros(&must_parse("fn f() { panic!(\"oh\"); }"));
    let _ = only_macro_name(&m3, "panic");
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
fn unwrap_expect_found() {
    let u = find_unwrap_expect(&must_parse("fn f() { Some(1).unwrap(); }"));
    let _ = only_unwrap_expect(&u, "unwrap");
    let e = find_unwrap_expect(&must_parse("fn f() { Some(1).expect(\"m\"); }"));
    let _ = only_unwrap_expect(&e, "expect");
}

#[test]
fn unwrap_in_string_not_found() {
    let src = "fn foo() { let _s = \".unwrap()\"; }";
    assert!(
        find_unwrap_expect(&must_parse(src)).is_empty(),
        "no match in string"
    );
}
