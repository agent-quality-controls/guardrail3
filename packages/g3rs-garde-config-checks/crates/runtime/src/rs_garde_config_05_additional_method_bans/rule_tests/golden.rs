use clippy_toml_parser::parse;
use g3rs_garde_config_checks_assertions::rs_garde_config_05_additional_method_bans as assertions;
use g3rs_garde_config_checks_types::G3RsGardeConfigClippyBanChecksInput;

#[test]
fn inventories_when_additional_bans_present() {
    let clippy = parse(&super::helpers::canonical_clippy_toml()).expect("valid clippy");
    let input = G3RsGardeConfigClippyBanChecksInput {
        clippy_rel_path: "clippy.toml".to_owned(),
        clippy,
    };

    let results = crate::run::check_clippy_bans(&input);

    assertions::assert_contains(
        &results,
        assertions::info(
            "additional garde method bans present",
            "All additional garde deserialization entry-point bans are present in the covering clippy configuration.",
            "clippy.toml",
        ),
    );
}
