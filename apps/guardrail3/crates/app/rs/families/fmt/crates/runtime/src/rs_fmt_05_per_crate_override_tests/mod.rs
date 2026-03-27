use guardrail3_app_rs_family_fmt_assertions::rs_fmt_05_per_crate_override as assertions;

use super::run_check;

#[test]
fn reports_extra_per_crate_rustfmt_configs() {
    let results = run_check(
        "crates/core/.rustfmt.toml",
        super::super::facts::RustfmtConfigKind::DotRustfmtToml,
    );

    assertions::assert_override(
        &results,
        ".rustfmt.toml below workspace root overrides root formatting policy",
        "crates/core/.rustfmt.toml",
    );
}
