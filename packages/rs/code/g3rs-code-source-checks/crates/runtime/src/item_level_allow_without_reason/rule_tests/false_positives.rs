#[test]
fn skips_useful_reason() {
    let results = super::super::check_source(
        "src/lib.rs",
        "#[allow(dead_code)] // reason: proc macro entrypoint\nfn probe() {}\n",
        false,
    );

    assert!(results.is_empty(), "{results:#?}");
}
