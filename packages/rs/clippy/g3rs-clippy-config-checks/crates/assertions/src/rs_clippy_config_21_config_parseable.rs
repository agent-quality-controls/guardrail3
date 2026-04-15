crate::define_result_assertions!("RS-CLIPPY-CONFIG-21");

pub fn assert_parse_error_contains(
    results: &[guardrail3_check_types::G3CheckResult],
    needle: &str,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-CLIPPY-CONFIG-21"
                && result.title() == "clippy.toml parse error"
                && result.message().contains(needle)
        }),
        "{:#?}",
        findings(results)
    );
}
