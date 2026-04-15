crate::define_result_assertions!("RS-CLIPPY-CONFIG-20");

pub fn assert_no_overrides_inventory(results: &[guardrail3_check_types::G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-CLIPPY-CONFIG-20"
                && result.title() == "no clippy config dir overrides found"
                && result.message() == "No applicable cargo config surfaces set `CLIPPY_CONF_DIR`."
                && result.file().is_none()
                && result.inventory()
        }),
        "{:#?}",
        findings(results)
    );
}
