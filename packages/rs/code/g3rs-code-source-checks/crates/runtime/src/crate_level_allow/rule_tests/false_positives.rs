#[test]
fn skips_unused_crate_dependencies_exception() {
    let results = super::super::check_source(
        "src/lib.rs",
        "#![allow(unused_crate_dependencies)]\nfn probe() {}\n",
        false,
    );

    assert!(results.is_empty(), "{results:#?}");
}
