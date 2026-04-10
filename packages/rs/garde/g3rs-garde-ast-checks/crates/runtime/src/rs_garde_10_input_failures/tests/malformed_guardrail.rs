#[test]
fn reports_malformed_guardrail_config() {
    let fixture = crate::test_support::fixture(
        &[(
            "src/lib.rs",
            "use sqlx::query_as;\nfn load() { let _ = query_as!(User, \"select 1\"); }\n",
        )],
        "[[broken",
    );

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
