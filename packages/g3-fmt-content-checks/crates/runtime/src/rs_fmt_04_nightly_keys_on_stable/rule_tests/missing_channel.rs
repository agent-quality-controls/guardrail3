use g3_fmt_content_checks_assertions::rs_fmt_04_nightly_keys_on_stable as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_channel_is_missing() {
    let results = run_check(
        r#"
edition = "2024"
group_imports = "StdExternalCrate"
"#,
        r#"
[toolchain]
components = ["clippy", "rustfmt"]
"#,
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "rust-toolchain channel missing",
            "Nightly-only rustfmt settings require `[toolchain].channel` in rust-toolchain.toml.",
            "rust-toolchain.toml",
            false,
        )],
    );
}
