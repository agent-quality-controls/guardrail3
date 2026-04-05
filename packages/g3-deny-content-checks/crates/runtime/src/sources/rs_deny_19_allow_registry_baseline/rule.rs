use std::collections::BTreeSet;

use deny_toml_parser::DenyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, expected_sources};

const ID: &str = "RS-DENY-19";

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(sources) = deny.sources.as_ref() else {
        results.push(error(
            ID,
            "[sources] allow-registry missing",
            format!("`{deny_rel_path}` has no valid crates.io registry allow-list."),
            deny_rel_path,
        ));
        return;
    };

    if sources.allow_registry.is_empty() {
        results.push(error(
            ID,
            "[sources] allow-registry missing",
            format!("`{deny_rel_path}` has no valid crates.io registry allow-list."),
            deny_rel_path,
        ));
        return;
    }

    let expected = expected_sources().0;
    let actual = sources.allow_registry.iter().cloned().collect::<BTreeSet<_>>();

    for registry in expected.difference(&actual) {
        results.push(error(
            ID,
            "canonical registry missing",
            format!("`{deny_rel_path}` must allow registry `{registry}`."),
            deny_rel_path,
        ));
    }

    for registry in actual.difference(&expected) {
        results.push(error(
            ID,
            "unexpected registry allowed",
            format!("`{deny_rel_path}` allows unexpected registry `{registry}`."),
            deny_rel_path,
        ));
    }

    if sources.allow_registry.len() != expected.len() {
        results.push(error(
            ID,
            "allow-registry entry count differs from baseline",
            format!(
                "`{deny_rel_path}` must contain exactly {} `[sources].allow-registry` entr{}.",
                expected.len(),
                if expected.len() == 1 { "y" } else { "ies" }
            ),
            deny_rel_path,
        ));
    }
}
