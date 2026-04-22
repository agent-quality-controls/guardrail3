use g3rs_fmt_config_checks_assertions::run as run_assertions;
use g3rs_fmt_types::{
    G3RsFmtCargoFacts, G3RsFmtCargoState, G3RsFmtConfigChecksInput, G3RsFmtRustPolicyState,
    G3RsFmtRustfmtConfigState, G3RsFmtRustfmtFacts, G3RsFmtToolchainFacts, G3RsFmtToolchainState,
};

#[test]
fn dispatches_prebound_fmt_facts() {
    let results = super::super::check(&G3RsFmtConfigChecksInput {
        rustfmt_rel_path: "rustfmt.toml".to_owned(),
        rustfmt_state: G3RsFmtRustfmtConfigState::Parsed(G3RsFmtRustfmtFacts {
            edition: Some("2021".to_owned()),
            style_edition: Some("2021".to_owned()),
            max_width: Some(100),
            tab_spaces: Some(4),
            use_field_init_shorthand: Some(true),
            use_try_shorthand: Some(true),
            reorder_imports: Some(true),
            reorder_modules: Some(true),
            explicit_keys: vec![
                "edition".to_owned(),
                "style_edition".to_owned(),
                "imports_granularity".to_owned(),
                "ignore".to_owned(),
            ],
            nightly_keys: vec!["imports_granularity".to_owned()],
            ignore: vec!["generated.rs".to_owned()],
        }),
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo_state: G3RsFmtCargoState::Parsed(G3RsFmtCargoFacts {
            edition: Some("2024".to_owned()),
        }),
        toolchain_rel_path: "rust-toolchain.toml".to_owned(),
        toolchain_state: G3RsFmtToolchainState::Parsed(G3RsFmtToolchainFacts {
            channel: Some("stable".to_owned()),
        }),
        rust_policy: G3RsFmtRustPolicyState::Missing,
    });

    run_assertions::assert_result_id_count(&results, "RS-FMT-CONFIG-01", 2);
    run_assertions::assert_result_id_count(&results, "RS-FMT-CONFIG-02", 1);
    run_assertions::assert_result_id_count(&results, "RS-FMT-CONFIG-03", 1);
    run_assertions::assert_result_id_count(&results, "RS-FMT-CONFIG-04", 1);
    run_assertions::assert_result_id_count(&results, "RS-FMT-CONFIG-07", 2);
}
