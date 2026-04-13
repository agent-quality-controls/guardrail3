use g3rs_deny_config_checks_assertions::rs_deny_config_23_ban_baseline_complete as assertions;

use crate::test_support::{canonical_bans_toml, run};

#[test]
fn errors_when_bans_section_is_missing() {
    let results = run(
        "",
        Some("service"),
        true,
        crate::rs_deny_config_23_ban_baseline_complete::check,
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "[bans] section missing",
            "`deny.toml` has no `[bans]` section.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn errors_when_bans_deny_is_missing() {
    let results = run(
        "[bans]\n",
        Some("service"),
        true,
        crate::rs_deny_config_23_ban_baseline_complete::check,
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "[bans].deny missing",
            "`deny.toml` must contain `[bans].deny`.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn errors_when_canonical_bans_are_missing() {
    let deny_toml = canonical_bans_toml("service")
        .replace("\"actix-web\",\n", "")
        .replace("\"lazy_static\",\n", "");
    let results = run(
        &deny_toml,
        Some("service"),
        true,
        crate::rs_deny_config_23_ban_baseline_complete::check,
    );

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "missing canonical ban",
                "`deny.toml` is missing deny ban `actix-web`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "missing canonical ban",
                "`deny.toml` is missing deny ban `lazy_static`.",
                "deny.toml",
                false,
            ),
        ],
    );
}
