#[test]
fn reports_malformed_guardrail_config() {
    let fixture = crate::test_support::invalid_policy_fixture(
        &[(
            "src/lib.rs",
            "use sqlx::query_as;\nfn load() { let _ = query_as!(User, \"select 1\"); }\n",
        )],
        "Failed to parse `guardrail3-rs.toml` for garde Rust policy resolution: invalid guardrail3-rs.toml",
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
