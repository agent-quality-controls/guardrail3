use guardrail3_app_rs_family_deny_assertions::rs_deny_09_ban_baseline_complete as assertions;

use super::super::{build_fixture_deny_toml, copy_fixture, run_family, write_file};

#[test]
fn generated_library_baseline_passes_for_standalone_app_root() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/full_golden");
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
        &build_fixture_deny_toml("library"),
    );

    let results = run_family(tmp.path());

    assertions::assert_no_findings_for_file(&results, "apps/libsite/deny.toml");
}
