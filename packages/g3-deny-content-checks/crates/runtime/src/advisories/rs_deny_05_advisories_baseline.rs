use deny_toml_parser::DenyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, expected_advisory_baseline};

const ID: &str = "RS-DENY-05";

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(advisories) = deny.advisories.as_ref() else {
        results.push(error(
            ID,
            "[advisories] section missing",
            format!("`{deny_rel_path}` has no `[advisories]` section."),
            deny_rel_path,
        ));
        return;
    };

    let (expected_unmaintained, expected_yanked) = expected_advisory_baseline();
    check_value(
        deny_rel_path,
        advisories.unmaintained.as_deref(),
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

fn check_value(
    deny_rel_path: &str,
    actual: Option<&str>,
    key: &str,
    expected: &str,
    results: &mut Vec<G3CheckResult>,
) {
    match actual {
        Some(value) if value == expected => {}
        Some(value) => results.push(error(
            ID,
            format!("advisories `{key}` has wrong value"),
            format!(
                "`{deny_rel_path}` must set `[advisories].{key} = \"{expected}\"`, found `{value}`."
            ),
            deny_rel_path,
        )),
        None => results.push(error(
            ID,
            format!("advisories `{key}` missing"),
            format!("`{deny_rel_path}` must set `[advisories].{key} = \"{expected}\"`."),
            deny_rel_path,
        )),
    }
}
