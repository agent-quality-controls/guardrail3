crate::define_result_assertions!("g3rs-clippy/forbid-clippy-conf-dir-override");

/// Asserts the inventory finding "no clippy config dir overrides found" is present.
///
/// # Panics
///
/// Panics if the inventory finding is absent.
pub fn assert_no_overrides_inventory(results: &[guardrail3_check_types::G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-clippy/forbid-clippy-conf-dir-override"
                && result.title() == "no clippy config dir overrides found"
                && result.message() == "No applicable cargo config surfaces set `CLIPPY_CONF_DIR`."
                && result.file().is_none()
                && result.inventory()
        }),
        "{:#?}",
        findings(results)
    );
}
