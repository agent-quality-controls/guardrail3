use cargo_toml_parser::parse as parse_cargo;
use clippy_toml_parser::parse as parse_clippy;
use g3rs_garde_config_checks_types::{
    G3RsGardeApplicability, G3RsGardeClippyInput, G3RsGardeConfigChecksInput,
};

#[test]
fn warns_when_clippy_config_is_missing_for_garde_root() {
    let input = G3RsGardeConfigChecksInput {
        applicability: G3RsGardeApplicability::Active,
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo: parse_cargo(
            "[workspace]\nmembers = []\n[workspace.dependencies]\ngarde = \"0.22\"\n",
        )
        .expect("cargo should parse"),
        clippy_input: G3RsGardeClippyInput::Missing,
    };

    let results = crate::run::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-CONFIG-02"
                && result.title() == "cannot verify core garde method bans"
                && result.file().is_none()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-CONFIG-03"
                && result.title() == "cannot verify garde extractor bans"
                && result.file().is_none()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-CONFIG-04"
                && result.title() == "cannot verify reqwest garde ban"
                && result.file().is_none()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-CONFIG-05"
                && result.title() == "cannot verify additional garde method bans"
                && result.file().is_none()
        }),
        "{results:#?}"
    );
}

#[test]
fn keeps_ban_rules_quiet_when_garde_is_absent() {
    let input = G3RsGardeConfigChecksInput {
        applicability: G3RsGardeApplicability::Active,
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo: parse_cargo("[workspace]\nmembers = []\n").expect("cargo should parse"),
        clippy_input: G3RsGardeClippyInput::Parsed {
            rel_path: "clippy.toml".to_owned(),
            clippy: parse_clippy(&crate::test_support::canonical_clippy_toml())
                .expect("clippy should parse"),
        },
    };

    let results = crate::run::check(&input);

    assert!(
        results.iter().any(|result| result.id() == "RS-GARDE-CONFIG-01"),
        "{results:#?}"
    );
    assert!(
        results
            .iter()
            .all(|result| !matches!(result.id(), "RS-GARDE-CONFIG-02" | "RS-GARDE-CONFIG-03" | "RS-GARDE-CONFIG-04" | "RS-GARDE-CONFIG-05")),
        "{results:#?}"
    );
}

#[test]
fn warns_when_clippy_config_is_invalid_for_garde_root() {
    let input = G3RsGardeConfigChecksInput {
        applicability: G3RsGardeApplicability::Active,
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo: parse_cargo(
            "[workspace]\nmembers = []\n[workspace.dependencies]\ngarde = \"0.22\"\n",
        )
        .expect("cargo should parse"),
        clippy_input: G3RsGardeClippyInput::Invalid {
            rel_path: "clippy.toml".to_owned(),
            message: "Failed to parse `clippy.toml` for garde clippy-ban validation: invalid clippy.toml"
                .to_owned(),
        },
    };

    let results = crate::run::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-CONFIG-02"
                && result.title() == "cannot verify core garde method bans"
                && result.file() == Some("clippy.toml")
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-CONFIG-05"
                && result.title() == "cannot verify additional garde method bans"
                && result.file() == Some("clippy.toml")
        }),
        "{results:#?}"
    );
}

#[test]
fn keeps_ban_rules_quiet_when_garde_is_absent_and_clippy_is_invalid() {
    let input = G3RsGardeConfigChecksInput {
        applicability: G3RsGardeApplicability::Active,
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo: parse_cargo("[workspace]\nmembers = []\n").expect("cargo should parse"),
        clippy_input: G3RsGardeClippyInput::Invalid {
            rel_path: "clippy.toml".to_owned(),
            message: "Failed to parse `clippy.toml` for garde clippy-ban validation: invalid clippy.toml"
                .to_owned(),
        },
    };

    let results = crate::run::check(&input);

    assert!(
        results.iter().any(|result| result.id() == "RS-GARDE-CONFIG-01"),
        "{results:#?}"
    );
    assert!(
        results
            .iter()
            .all(|result| !matches!(result.id(), "RS-GARDE-CONFIG-02" | "RS-GARDE-CONFIG-03" | "RS-GARDE-CONFIG-04" | "RS-GARDE-CONFIG-05")),
        "{results:#?}"
    );
}

#[test]
fn returns_no_results_when_family_is_inactive() {
    let input = G3RsGardeConfigChecksInput {
        applicability: G3RsGardeApplicability::Inactive,
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo: parse_cargo("[workspace]\nmembers = []\n").expect("cargo should parse"),
        clippy_input: G3RsGardeClippyInput::Missing,
    };

    let results = crate::run::check(&input);

    assert!(results.is_empty(), "{results:#?}");
}
