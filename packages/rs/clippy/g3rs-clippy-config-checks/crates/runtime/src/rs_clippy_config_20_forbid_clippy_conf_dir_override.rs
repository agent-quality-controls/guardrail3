use g3rs_clippy_types::G3RsClippyConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-CLIPPY-CONFIG-20";

pub(crate) fn check(input: &G3RsClippyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if input.cargo_config_overrides.is_empty() {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "no clippy config dir overrides found".to_owned(),
                "No applicable cargo config surfaces set `CLIPPY_CONF_DIR`.".to_owned(),
                None,
                None,
            )
            .into_inventory(),
        );
        return;
    }

    for override_facts in &input.cargo_config_overrides {
        let (title, message) = match override_facts.parse_error.as_deref() {
            Some(parse_error) => (
                "cargo config override surface is not parseable".to_owned(),
                format!(
                    "Failed to parse `{}` while checking for forbidden `CLIPPY_CONF_DIR` overrides: {parse_error}",
                    override_facts.rel_path
                ),
            ),
            None => (
                "clippy config dir override is forbidden".to_owned(),
                format!(
                    "`{}` sets `CLIPPY_CONF_DIR`, which bypasses the routed clippy policy-root model. Remove the `CLIPPY_CONF_DIR` setting from `{}`.",
                    override_facts.rel_path, override_facts.rel_path
                ),
            ),
        };

        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            title,
            message,
            Some(override_facts.rel_path.clone()),
            None,
        ));
    }
}

#[cfg(test)]
#[path = "rs_clippy_config_20_forbid_clippy_conf_dir_override_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rs_clippy_config_20_forbid_clippy_conf_dir_override_tests;
