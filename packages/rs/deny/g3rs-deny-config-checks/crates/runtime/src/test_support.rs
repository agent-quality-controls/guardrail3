use deny_toml_parser::parse as parse_deny_toml;
use g3rs_deny_config_checks_types::G3RsDenyConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::expected_bans;

pub(crate) fn input(deny_toml: &str, profile_name: Option<&str>, policy_context_valid: bool) -> G3RsDenyConfigChecksInput {
    G3RsDenyConfigChecksInput {
        deny_rel_path: "deny.toml".to_owned(),
        deny: parse_deny_toml(deny_toml).expect("deny fixture should parse"),
        profile_name: profile_name.map(str::to_owned),
        policy_context_valid,
    }
}

pub(crate) fn run(
    deny_toml: &str,
    profile_name: Option<&str>,
    policy_context_valid: bool,
    rule: fn(&G3RsDenyConfigChecksInput, &mut Vec<G3CheckResult>),
) -> Vec<G3CheckResult> {
    let input = input(deny_toml, profile_name, policy_context_valid);
    let mut results = Vec::new();
    rule(&input, &mut results);
    results
}

pub(crate) fn canonical_bans_toml(profile_name: &str) -> String {
    let expected = expected_bans(Some(profile_name));
    let entries = expected
        .into_iter()
        .map(|(name, expectation)| {
            if expectation.wrappers.is_empty() {
                format!("\"{name}\"")
            } else {
                let wrappers = expectation
                    .wrappers
                    .iter()
                    .map(|wrapper| format!("\"{wrapper}\""))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{{ name = \"{name}\", wrappers = [{wrappers}] }}")
            }
        })
        .collect::<Vec<_>>()
        .join(",\n  ");

    format!(
        r#"[bans]
deny = [
  {entries}
]
"#
    )
}
