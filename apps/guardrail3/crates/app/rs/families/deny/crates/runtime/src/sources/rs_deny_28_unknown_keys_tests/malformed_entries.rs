use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_28_unknown_keys as assertions;

use super::super::build_fixture_deny_toml;

#[test]
fn warns_on_malformed_nested_entries_and_string_array_members() {
    let deny = r#"
[licenses]
allow = ["MIT", 123]
exceptions = [
  "serde",
  { allow = ["MIT"] },
  { name = "valid-exception", allow = ["Apache-2.0", false] }
]

[sources]
allow-registry = ["sparse+https://index.crates.io/", 123]
allow-git = "https://github.com/example/repo"

[bans]
deny = [
  123,
  { reason = "missing identifier" },
  { name = "demo-ban", reason = "valid", wrappers = ["demo::wrap", false] }
]
features = [
  "tokio",
  { deny = ["full"] },
  { name = "serde", deny = ["derive", 123], allow = "std" }
]
"#;
    let results = super::super::run_check(deny);
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "unsupported [bans].deny entry schema",
                "`deny.toml` uses unsupported schema for `[bans].deny` entry at index 0; expected string or table.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unsupported [bans].deny entry schema",
                "`deny.toml` uses unsupported schema for `[bans].deny` entry at index 1; expected string or table with `name` or `crate`.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unsupported [[bans.deny]].wrappers entry schema",
                "`deny.toml` uses unsupported schema for `[[bans.deny]].wrappers` entry at index 1; expected string.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unsupported [bans].features entry schema",
                "`deny.toml` uses unsupported schema for `[bans].features` entry at index 0; expected table.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unsupported [bans].features entry schema",
                "`deny.toml` uses unsupported schema for `[bans].features` entry at index 1; expected table with `name` or `crate`.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unsupported [[bans.features]].allow schema",
                "`deny.toml` uses unsupported schema for `[[bans.features]].allow`; expected array.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unsupported [[bans.features]].deny entry schema",
                "`deny.toml` uses unsupported schema for `[[bans.features]].deny` entry at index 1; expected string.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unsupported [licenses].allow entry schema",
                "`deny.toml` uses unsupported schema for `[licenses].allow` entry at index 1; expected string.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unsupported [licenses].exceptions entry schema",
                "`deny.toml` uses unsupported schema for `[licenses].exceptions` entry at index 0; expected table.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unsupported [licenses].exceptions entry schema",
                "`deny.toml` uses unsupported schema for `[licenses].exceptions` entry at index 1; expected table with `name` or `crate`.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unsupported [[licenses.exceptions]].allow entry schema",
                "`deny.toml` uses unsupported schema for `[[licenses.exceptions]].allow` entry at index 1; expected string.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unsupported [sources].allow-git schema",
                "`deny.toml` uses unsupported schema for `[sources].allow-git`; expected array.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unsupported [sources].allow-registry entry schema",
                "`deny.toml` uses unsupported schema for `[sources].allow-registry` entry at index 1; expected string.",
                "deny.toml",
                false,
            ),
        ],
    );
}

#[test]
fn generated_baseline_does_not_trigger_schema_warnings() {
    let results = super::super::run_check(&build_fixture_deny_toml("service"));

    assert!(results.is_empty());
}
