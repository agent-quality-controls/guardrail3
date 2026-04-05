use deny_toml_parser::DenyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::{expected_bans_settings, warn};

const ID: &str = "RS-DENY-13";

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(bans) = deny.bans.as_ref() else {
        return;
    };

    let (expected, _, _) = expected_bans_settings();
    let actual = bans.wildcards.as_deref();
    if actual.map(str::to_owned) != expected {
        results.push(warn(
            ID,
            "wildcards differs from baseline",
            format!(
                "`{deny_rel_path}` sets `[bans].wildcards = {}`.",
                actual.unwrap_or("<missing>")
            ),
            deny_rel_path,
        ));
    }
}
