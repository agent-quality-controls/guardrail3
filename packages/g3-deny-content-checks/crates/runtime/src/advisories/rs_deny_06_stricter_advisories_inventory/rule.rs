use deny_toml_parser::{AdvisoryScope, DenyToml};
use guardrail3_check_types::G3CheckResult;

use crate::support::{expected_advisory_baseline, inventory};

const ID: &str = "RS-DENY-06";

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(advisories) = deny.advisories.as_ref() else {
        return;
    };
    let (expected_unmaintained, expected_yanked) = expected_advisory_baseline();

    check_value(
        deny_rel_path,
        advisories.unmaintained.as_ref().map(advisory_scope_str),
        "unmaintained",
        &expected_unmaintained,
        results,
    );
    check_value(
        deny_rel_path,
        advisories.yanked.as_deref(),
        "yanked",
        &expected_yanked,
        results,
    );
}

const fn advisory_scope_str(scope: &AdvisoryScope) -> &'static str {
    match scope {
        AdvisoryScope::All => "all",
        AdvisoryScope::Workspace => "workspace",
        AdvisoryScope::Transitive => "transitive",
        AdvisoryScope::None => "none",
    }
}

fn check_value(
    deny_rel_path: &str,
    actual: Option<&str>,
    key: &str,
    expected: &str,
    results: &mut Vec<G3CheckResult>,
) {
    if matches!(actual, Some("deny")) && expected != "deny" {
        results.push(inventory(
            ID,
            format!("advisories `{key}` stricter than baseline"),
            format!("`{deny_rel_path}` sets `[advisories].{key} = \"deny\"`."),
            deny_rel_path,
        ));
    }
}
