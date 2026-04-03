use guardrail3_domain_report::{CheckResult, Severity};

use crate::deny_support::{expected_licenses, section};
use crate::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(licenses) = section(config, "licenses") else {
        results.push(CheckResult::from_parts(
            "RS-DENY-14".to_owned(),
            Severity::Error,
            "[licenses] section missing".to_owned(),
            format!("`{}` has no `[licenses]` section.", config.rel_path),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
        return;
    };

    let expected = expected_licenses();
    let actual: std::collections::BTreeSet<String> = licenses
        .get("allow")
        .and_then(toml::Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter_map(toml::Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();

    for name in &expected {
        if !actual.contains(name.as_str()) {
            results.push(CheckResult::from_parts(
                "RS-DENY-14".to_owned(),
                Severity::Error,
                "baseline license missing".to_owned(),
                format!("`{}` is missing allowed license `{name}`.", config.rel_path),
                Some(config.rel_path.clone()),
                None,
                false,
            ));
        }
    }

    for name in actual.difference(&expected) {
        results.push(CheckResult::from_parts(
            "RS-DENY-14".to_owned(),
            Severity::Error,
            "unexpected allowed license".to_owned(),
            format!("`{}` allows unexpected license `{name}`.", config.rel_path),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
    }

    let private_ignore = licenses
        .get("private")
        .and_then(|value| value.get("ignore"))
        .and_then(toml::Value::as_bool);
    if private_ignore != Some(true) {
        results.push(CheckResult::from_parts(
            "RS-DENY-14".to_owned(),
            Severity::Error,
            "licenses.private.ignore must be true".to_owned(),
            format!(
                "`{}` must set `[licenses.private].ignore = true`.",
                config.rel_path
            ),
            Some(config.rel_path.clone()),
            None,
            false,
        ));
    }
}
