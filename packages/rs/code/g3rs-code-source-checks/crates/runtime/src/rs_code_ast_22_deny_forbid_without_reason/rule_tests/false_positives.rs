#[test]
fn skips_useful_reason() {
    let results = super::super::check_source(
        "src/lib.rs",
        "#[deny(dead_code)] // reason: generated ffi shim\nfn probe() {}\n",
        false,
    );

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn skips_known_false_cfg_attr_deny() {
    let results = super::super::check_source(
        "src/lib.rs",
        "#[cfg_attr(any(), deny(dead_code))]\nfn probe() {}\n",
        false,
    );

    assert!(results.is_empty(), "{results:#?}");
}
