use clippy_toml_parser::parse;
use g3rs_garde_config_checks_assertions::rs_garde_config_02_core_method_bans as assertions;
use g3rs_garde_config_checks_types::G3RsGardeConfigChecksInput;

#[test]
fn inventories_when_all_core_bans_present() {
    let clippy = parse(&super::helpers::canonical_clippy_toml()).expect("valid clippy");
    let input = G3RsGardeConfigChecksInput {
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo: cargo_toml_parser::parse("[workspace]\nmembers = []\n")
            .expect("minimal cargo fixture should parse"),
        clippy_rel_path: Some("clippy.toml".to_owned()),
        clippy: Some(clippy),
    };

    let results = crate::run::check(&input);

    assertions::assert_contains(
        &results,
        assertions::info(
            "core garde method bans present",
            "All core serde/toml/yaml deserialization bans are present in the covering clippy configuration.",
            "clippy.toml",
        ),
    );
}
