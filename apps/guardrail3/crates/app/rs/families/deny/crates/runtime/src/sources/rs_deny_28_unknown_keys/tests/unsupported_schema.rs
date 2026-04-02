use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_28_unknown_keys as assertions;

use super::super::build_fixture_deny_toml;

fn with_root_override(deny_toml: &str, key: &str, value: toml::Value) -> String {
    let mut parsed = toml::from_str::<toml::Value>(deny_toml).expect("valid deny TOML");
    let root = parsed.as_table_mut().expect("expected root table");
    let _ = root.insert(key.to_owned(), value);
    toml::to_string(&parsed).expect("serialize deny TOML")
}

fn with_section_override(deny_toml: &str, section: &str, key: &str, value: toml::Value) -> String {
    let mut parsed = toml::from_str::<toml::Value>(deny_toml).expect("valid deny TOML");
    let table = parsed
        .get_mut(section)
        .and_then(toml::Value::as_table_mut)
        .expect("expected section table");
    let _ = table.insert(key.to_owned(), value);
    toml::to_string(&parsed).expect("serialize deny TOML")
}

#[test]
fn warns_on_unsupported_core_section_schema() {
    let deny = with_root_override(
        &with_root_override(
            &build_fixture_deny_toml("service"),
            "graph",
            toml::Value::String("invalid".to_owned()),
        ),
        "sources",
        toml::Value::String("invalid".to_owned()),
    );
    let results = super::super::run_check(&deny);
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "unsupported [graph] schema",
                "`deny.toml` uses unsupported schema for `[graph]`; expected table.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unsupported [sources] schema",
                "`deny.toml` uses unsupported schema for `[sources]`; expected table.",
                "deny.toml",
                false,
            ),
        ],
    );
}

#[test]
fn warns_on_unsupported_nested_section_schema() {
    let deny = with_section_override(
        &with_section_override(
            &with_section_override(
                &with_section_override(
                    &with_section_override(
                        &build_fixture_deny_toml("service"),
                        "licenses",
                        "private",
                        toml::Value::String("invalid".to_owned()),
                    ),
                    "licenses",
                    "exceptions",
                    toml::Value::String("invalid".to_owned()),
                ),
                "bans",
                "features",
                toml::Value::String("invalid".to_owned()),
            ),
            "bans",
            "skip",
            toml::Value::String("invalid".to_owned()),
        ),
        "advisories",
        "ignore",
        toml::Value::String("invalid".to_owned()),
    );
    let results = super::super::run_check(&deny);
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "unsupported [advisories].ignore schema",
                "`deny.toml` uses unsupported schema for `[advisories].ignore`; expected array.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unsupported [bans].features schema",
                "`deny.toml` uses unsupported schema for `[bans].features`; expected array.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unsupported [bans].skip schema",
                "`deny.toml` uses unsupported schema for `[bans].skip`; expected array.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unsupported [licenses.private] schema",
                "`deny.toml` uses unsupported schema for `[licenses.private]`; expected table.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unsupported [licenses].exceptions schema",
                "`deny.toml` uses unsupported schema for `[licenses].exceptions`; expected array.",
                "deny.toml",
                false,
            ),
        ],
    );
}

#[test]
fn local_unsupported_schema_only_warns_for_the_owned_local_root() {
    let local_deny = with_section_override(
        &with_section_override(
            &build_fixture_deny_toml("service"),
            "licenses",
            "private",
            toml::Value::String("invalid".to_owned()),
        ),
        "bans",
        "features",
        toml::Value::String("invalid".to_owned()),
    );
    let results = super::super::run_check(&local_deny);
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "unsupported [bans].features schema",
                "`deny.toml` uses unsupported schema for `[bans].features`; expected array.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unsupported [licenses.private] schema",
                "`deny.toml` uses unsupported schema for `[licenses.private]`; expected table.",
                "deny.toml",
                false,
            ),
        ],
    );
}
