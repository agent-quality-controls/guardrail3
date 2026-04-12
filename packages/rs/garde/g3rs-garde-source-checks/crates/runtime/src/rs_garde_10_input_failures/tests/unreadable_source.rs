#[test]
fn reports_unreadable_rust_source() {
    let fixture = crate::test_support::fixture(
        &[("src/lib.rs", "fn ok() {}\n")],
        crate::test_support::default_guardrail_toml(),
    );
    fixture.make_source_unreadable("src/lib.rs");

    let results = fixture.run();

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-SOURCE-10"
                && result.file() == Some("src/lib.rs")
                && result.title() == "garde-family input failure"
        }),
        "{results:#?}"
    );
}
