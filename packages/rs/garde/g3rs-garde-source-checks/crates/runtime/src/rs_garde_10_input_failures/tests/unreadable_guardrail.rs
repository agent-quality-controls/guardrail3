#[test]
fn reports_unreadable_guardrail_config() {
    let fixture = crate::test_support::fixture(
        &[("src/lib.rs", "fn ok() {}\n")],
        crate::test_support::default_guardrail_toml(),
    );
    fixture.make_guardrail_unreadable();

    let results = fixture.run();

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-10"
                && result.file() == Some("guardrail3.toml")
                && result.title() == "garde-family input failure"
        }),
        "{results:#?}"
    );
}
