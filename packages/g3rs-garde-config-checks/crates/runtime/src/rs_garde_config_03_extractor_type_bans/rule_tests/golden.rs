use clippy_toml_parser::parse;
use g3rs_garde_config_checks_assertions::rs_garde_config_03_extractor_type_bans as assertions;
use g3rs_garde_config_checks_types::G3RsGardeConfigClippyBanChecksInput;

#[test]
fn inventories_when_all_extractor_bans_present() {
    let clippy = parse(&super::helpers::canonical_clippy_toml()).expect("valid clippy");
    let input = G3RsGardeConfigClippyBanChecksInput {
        clippy_rel_path: "clippy.toml".to_owned(),
        clippy,
    };

    let results = crate::run::check_clippy_bans(&input);

    assertions::assert_contains(
        &results,
        assertions::info(
            "garde extractor bans present",
            "All required Axum extractor bans are present in the covering clippy configuration.",
            "clippy.toml",
        ),
    );
}
