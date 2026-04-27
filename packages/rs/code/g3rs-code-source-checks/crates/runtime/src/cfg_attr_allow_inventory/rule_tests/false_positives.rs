#[test]
fn skips_known_true_cfg_attr() {
    let results = super::super::check_source(
        "src/lib.rs",
        "#[cfg_attr(all(), allow(dead_code))]\nfn probe() {}\n",
        false,
    );

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn skips_known_false_cfg_attr() {
    let results = super::super::check_source(
        "src/lib.rs",
        "#[cfg_attr(any(), allow(dead_code))]\nfn probe() {}\n",
        false,
    );

    assert!(results.is_empty(), "{results:#?}");
}
