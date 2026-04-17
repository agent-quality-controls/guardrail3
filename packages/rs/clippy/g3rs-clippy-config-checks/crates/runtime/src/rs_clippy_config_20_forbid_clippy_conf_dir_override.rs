use g3rs_clippy_types::{G3RsClippyCargoConfigState, G3RsClippyConfigChecksInput};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-CLIPPY-CONFIG-20";

pub(crate) fn check(input: &G3RsClippyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if input.cargo_configs.is_empty() {
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

    for cargo_config in &input.cargo_configs {
        let Some((title, message, rel_path)) = finding(cargo_config) else {
            continue;
        };

        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            title,
            message,
            Some(rel_path),
            None,
        ));
    }
}

fn finding(cargo_config: &G3RsClippyCargoConfigState) -> Option<(String, String, String)> {
    match cargo_config {
        G3RsClippyCargoConfigState::Unreadable { rel_path, reason }
        | G3RsClippyCargoConfigState::ParseError { rel_path, reason } => Some((
            "cargo config override surface is not parseable".to_owned(),
            format!(
                "Failed to parse `{rel_path}` while checking for forbidden `CLIPPY_CONF_DIR` overrides: {reason}"
            ),
            rel_path.clone(),
        )),
        G3RsClippyCargoConfigState::Parsed {
            rel_path,
            cargo_config,
        } => cargo_config
            .env
            .get("CLIPPY_CONF_DIR")
            .map(|_| (
                "clippy config dir override is forbidden".to_owned(),
                format!(
                    "`{rel_path}` sets `CLIPPY_CONF_DIR`, which bypasses the routed clippy policy-root model. Remove the `CLIPPY_CONF_DIR` setting from `{rel_path}`."
                ),
                rel_path.clone(),
            )),
    }
}

#[cfg(test)]
#[path = "rs_clippy_config_20_forbid_clippy_conf_dir_override_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rs_clippy_config_20_forbid_clippy_conf_dir_override_tests;
