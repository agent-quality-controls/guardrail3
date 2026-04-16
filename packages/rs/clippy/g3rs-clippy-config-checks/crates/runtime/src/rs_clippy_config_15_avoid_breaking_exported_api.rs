use g3rs_clippy_types::G3RsClippyConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{BoolSetting, bool_setting, raw_clippy, rust_policy_valid, value_kind};

const ID: &str = "RS-CLIPPY-CONFIG-15";

pub(crate) fn check(input: &G3RsClippyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !rust_policy_valid(input) {
        return;
    }
    let Some(parsed) = raw_clippy(input) else {
        return;
    };

    match bool_setting(parsed, "avoid-breaking-exported-api") {
        BoolSetting::Value(false) => results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "avoid-breaking-exported-api explicitly false".to_owned(),
                "`avoid-breaking-exported-api = false` is set.".to_owned(),
                Some(input.clippy_rel_path.clone()),
                None,
            )
            .into_inventory(),
        ),
        BoolSetting::Value(true) if input.published_library_policy => results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "library keeps avoid-breaking-exported-api enabled".to_owned(),
                "Published library profile may legitimately keep `avoid-breaking-exported-api = true`.".to_owned(),
                Some(input.clippy_rel_path.clone()),
                None,
            )
            .into_inventory(),
        ),
        BoolSetting::Value(true) => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            "avoid-breaking-exported-api enabled".to_owned(),
            "`avoid-breaking-exported-api = true` suppresses useful lints. Prefer `false`.".to_owned(),
            Some(input.clippy_rel_path.clone()),
            None,
        )),
        BoolSetting::WrongType(value) => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            "avoid-breaking-exported-api wrong type".to_owned(),
            format!(
                "`avoid-breaking-exported-api` must be a bool, found {}.",
                value_kind(value)
            ),
            Some(input.clippy_rel_path.clone()),
            None,
        )),
        BoolSetting::Missing => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            "avoid-breaking-exported-api not set".to_owned(),
            "Set `avoid-breaking-exported-api = false` explicitly unless this is a published library.".to_owned(),
            Some(input.clippy_rel_path.clone()),
            None,
        )),
    }
}

#[cfg(test)]
#[path = "rs_clippy_config_15_avoid_breaking_exported_api_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rs_clippy_config_15_avoid_breaking_exported_api_tests;
