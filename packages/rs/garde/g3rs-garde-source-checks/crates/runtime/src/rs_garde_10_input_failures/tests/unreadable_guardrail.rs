#[test]
fn reports_unreadable_guardrail_config() {
    let fixture = crate::test_support::invalid_policy_fixture(
        &[("src/lib.rs", "fn ok() {}\n")],
        "Failed to read `guardrail3-rs.toml` for garde Rust policy resolution: file is not readable",
    );

    let results = fixture.run();

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-SOURCE-10"
                && result.file() == Some("guardrail3-rs.toml")
                && result.title() == "garde-family input failure"
        }),
        "{results:#?}"
    );
}
