#[test]
fn reports_malformed_rust_source() {
    let fixture = crate::test_support::fixture(
        &[("src/lib.rs", "fn broken( {\n")],
        crate::test_support::default_guardrail_toml(),
    );

    let results = fixture.run();

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-10"
                && result.file() == Some("src/lib.rs")
                && result.title() == "garde-family input failure"
        }),
        "{results:#?}"
    );
}
