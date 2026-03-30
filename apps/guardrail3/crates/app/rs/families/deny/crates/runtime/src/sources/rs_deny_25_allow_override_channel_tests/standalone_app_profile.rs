use guardrail3_app_rs_family_deny_assertions::rs_deny_25_allow_override_channel as assertions;

use super::super::{build_fixture_deny_toml, copy_fixture, set_bans_allow_entries, write_file};

#[test]
fn standalone_app_root_uses_rust_apps_library_profile_for_allow_override_checks() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(tmp.path(), "deny.toml", &build_fixture_deny_toml("service"));
    write_file(
        tmp.path(),
        "guardrail3.toml",
        "version = \"0.1\"\n\n[profile]\nname = \"service\"\n\n[rust]\nworkspace_root = \".\"\n\n[rust.apps.backend]\ntype = \"service\"\n\n[rust.apps.worker]\ntype = \"service\"\n\n[rust.apps.devctl]\ntype = \"service\"\n\n[rust.apps.libsite]\ntype = \"library\"\n\n[rust.packages]\ntype = \"library\"\n",
    );
    write_file(
        tmp.path(),
        "apps/libsite/Cargo.toml",
        "[workspace]\nmembers = []\n[package]\nname = \"libsite\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    );
    write_file(
        tmp.path(),
        "apps/libsite/deny.toml",
        &set_bans_allow_entries(
            &build_fixture_deny_toml("library"),
            vec![toml::Value::String("axum".to_owned())],
        ),
    );

    let results = super::super::run_family(tmp.path());

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "allow-list overrides deny-list",
                "`apps/libsite/deny.toml` allows `axum` even though it is banned.",
                "apps/libsite/deny.toml",
                false,
            ),
            assertions::error(
                "bans allow-list present",
                "`apps/libsite/deny.toml` has non-empty `[bans].allow`: axum.",
                "apps/libsite/deny.toml",
                false,
            ),
        ],
    );
}
