use cargo_toml_parser::parse as parse_cargo;
use clippy_toml_parser::parse as parse_clippy;
use g3rs_garde_config_checks_assertions::run as assertions;
use g3rs_garde_types::{G3RsGardeApplicability, G3RsGardeClippyInput, G3RsGardeConfigChecksInput};

#[test]
fn warns_when_clippy_config_is_missing_for_garde_root() {
    let input = G3RsGardeConfigChecksInput {
        applicability: G3RsGardeApplicability::Active,
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo: parse_cargo(
            "[workspace]\nmembers = []\n[workspace.dependencies]\ngarde = \"0.22\"\n",
        )
        .expect("cargo fixture with garde dependency should parse"),
        clippy_input: G3RsGardeClippyInput::Missing,
    };

    let results = crate::run::check(&input);

    assertions::assert_missing_clippy_config_warnings(&results);
}

#[test]
fn keeps_ban_rules_quiet_when_garde_is_absent() {
    let input = G3RsGardeConfigChecksInput {
        applicability: G3RsGardeApplicability::Active,
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo: parse_cargo("[workspace]\nmembers = []\n")
            .expect("cargo fixture without garde dependency should parse"),
        clippy_input: G3RsGardeClippyInput::Parsed {
            rel_path: "clippy.toml".to_owned(),
            clippy: parse_clippy(&canonical_clippy_toml())
                .expect("clippy fixture with all garde bans should parse"),
        },
    };

    let results = crate::run::check(&input);

    assertions::assert_dependency_missing_without_ban_results(&results);
}

#[test]
fn warns_when_clippy_config_is_invalid_for_garde_root() {
    let input = G3RsGardeConfigChecksInput {
        applicability: G3RsGardeApplicability::Active,
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo: parse_cargo(
            "[workspace]\nmembers = []\n[workspace.dependencies]\ngarde = \"0.22\"\n",
        )
        .expect("cargo fixture with garde dependency should parse"),
        clippy_input: G3RsGardeClippyInput::Invalid {
            rel_path: "clippy.toml".to_owned(),
            message:
                "Failed to parse `clippy.toml` for garde clippy-ban validation: invalid clippy.toml"
                    .to_owned(),
        },
    };

    let results = crate::run::check(&input);

    assertions::assert_invalid_clippy_config_warnings(&results);
}

#[test]
fn keeps_ban_rules_quiet_when_garde_is_absent_and_clippy_is_invalid() {
    let input = G3RsGardeConfigChecksInput {
        applicability: G3RsGardeApplicability::Active,
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo: parse_cargo("[workspace]\nmembers = []\n")
            .expect("cargo fixture without garde dependency should parse"),
        clippy_input: G3RsGardeClippyInput::Invalid {
            rel_path: "clippy.toml".to_owned(),
            message:
                "Failed to parse `clippy.toml` for garde clippy-ban validation: invalid clippy.toml"
                    .to_owned(),
        },
    };

    let results = crate::run::check(&input);

    assertions::assert_garde_absent_without_ban_results(&results);
}

#[test]
fn returns_no_results_when_family_is_inactive() {
    let input = G3RsGardeConfigChecksInput {
        applicability: G3RsGardeApplicability::Inactive,
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo: parse_cargo("[workspace]\nmembers = []\n")
            .expect("cargo fixture for inactive family should parse"),
        clippy_input: G3RsGardeClippyInput::Missing,
    };

    let results = crate::run::check(&input);

    assertions::assert_no_results(&results);
}

fn canonical_clippy_toml() -> String {
    let method_entries = crate::support::CORE_METHOD_BANS
        .iter()
        .chain(std::iter::once(&crate::support::REQWEST_JSON_BAN))
        .chain(crate::support::ADDITIONAL_METHOD_BANS.iter())
        .map(|path| format!("{{ path = \"{path}\" }}"))
        .collect::<Vec<_>>()
        .join(",\n    ");
    let type_entries = crate::support::EXTRACTOR_TYPE_BANS
        .iter()
        .map(|path| format!("{{ path = \"{path}\" }}"))
        .collect::<Vec<_>>()
        .join(",\n    ");

    format!(
        "disallowed-methods = [\n    {method_entries}\n]\n\ndisallowed-types = [\n    {type_entries}\n]\n"
    )
}
