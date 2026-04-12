use clippy_toml_parser::parse;
use g3rs_garde_config_checks_assertions::rs_garde_config_05_additional_method_bans as assertions;
use g3rs_garde_config_checks_types::{G3RsGardeClippyInput, G3RsGardeConfigChecksInput};

#[test]
fn inventories_when_additional_bans_present() {
    let clippy = parse(&super::helpers::canonical_clippy_toml()).expect("valid clippy");
    let input = G3RsGardeConfigChecksInput {
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo: cargo_toml_parser::parse(
            "[workspace]\nmembers = []\n[workspace.dependencies]\ngarde = \"0.22\"\n",
        )
            .expect("minimal cargo fixture should parse"),
        clippy_input: G3RsGardeClippyInput::Parsed {
            rel_path: "clippy.toml".to_owned(),
            clippy,
        },
    };

    let results = crate::run::check(&input);

    assertions::assert_contains(
        &results,
        assertions::info(
            "additional garde method bans present",
            "All additional garde deserialization entry-point bans are present in the covering clippy configuration.",
            "clippy.toml",
        ),
    );
}
