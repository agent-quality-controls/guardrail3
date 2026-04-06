use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_config_22_ignore_accumulation as assertions;

use crate::inputs::ConfigDenyInput;
use super::helpers::config_facts;

#[test]
fn counts_entries_by_container_length_even_when_some_ignore_entries_are_malformed() {
    let deny = config_facts(
        "[advisories]\nignore = [\"A\", { reason = \"good enough reason text\" }, \"C\", \"D\", \"E\", \"F\"]\n",
    );
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "advisory ignore list is large",
            "`deny.toml` has 6 `[advisories].ignore` entries (threshold: 5).",
            "deny.toml",
            false,
        )],
    );
}
