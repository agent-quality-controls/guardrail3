use std::collections::BTreeSet;

use deny_toml_parser::DenyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, expected_licenses};

const ID: &str = "RS-DENY-CONFIG-10";

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(licenses) = deny.licenses.as_ref() else {
        results.push(error(
            ID,
            "[licenses] section missing",
            format!("`{deny_rel_path}` has no `[licenses]` section."),
            deny_rel_path,
        ));
        return;
    };

    let expected = expected_licenses();
    let actual = licenses.allow.iter().cloned().collect::<BTreeSet<_>>();

    for name in &expected {
        if !actual.contains(name) {
            results.push(error(
                ID,
                "baseline license missing",
                format!("`{deny_rel_path}` is missing allowed license `{name}`."),
                deny_rel_path,
            ));
        }
    }

    for name in actual.difference(&expected) {
        results.push(error(
            ID,
            "unexpected allowed license",
            format!("`{deny_rel_path}` allows unexpected license `{name}`."),
            deny_rel_path,
        ));
    }

    let private_ignore = licenses.private.as_ref().and_then(|private| private.ignore);
    if private_ignore != Some(true) {
        results.push(error(
            ID,
            "licenses.private.ignore must be true",
            format!("`{deny_rel_path}` must set `[licenses.private].ignore = true`."),
            deny_rel_path,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
