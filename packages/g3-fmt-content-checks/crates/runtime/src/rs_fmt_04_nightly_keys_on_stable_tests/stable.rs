use g3_fmt_content_checks_assertions::rs_fmt_04_nightly_keys_on_stable as assertions;

use super::helpers::run_check;

#[test]
fn warns_when_nightly_keys_are_used_on_stable() {
    let results = run_check(
        r#"
edition = "2024"
group_imports = "StdExternalCrate"
"#,
        r#"
[toolchain]
channel = "stable"
components = ["clippy", "rustfmt"]
"#,
    );

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "nightly-only rustfmt setting `group_imports` on stable",
            "`group_imports` is nightly-only, but rust-toolchain.toml uses `stable`. Either remove `group_imports` from rustfmt.toml or switch the toolchain channel to nightly.",
            "rustfmt.toml",
            false,
        )],
    );
}
