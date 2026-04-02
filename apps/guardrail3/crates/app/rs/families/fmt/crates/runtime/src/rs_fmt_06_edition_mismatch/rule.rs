use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::CargoEditionState;
use crate::inputs::RustfmtRootInput;

const ID: &str = "RS-FMT-06";

pub fn check(input: &RustfmtRootInput, results: &mut Vec<CheckResult>) {
    let Some(rel) = input.config_rel.as_deref() else {
        return;
    };
    let Some(parsed) = input.parsed.as_ref() else {
        return;
    };
    let Some(rustfmt_edition) = parsed.get("edition").and_then(toml::Value::as_str) else {
        return;
    };

    match &input.cargo_edition {
        CargoEditionState::Present(cargo_edition) if rustfmt_edition != cargo_edition => {
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Warn,
                "rustfmt edition differs from Cargo edition".to_owned(),
                format!(
                    "rustfmt edition `{rustfmt_edition}` differs from Cargo edition `{cargo_edition}`."
                ),
                Some(rel.to_owned()),
                None,
                false,
            ));
        }
        CargoEditionState::Present(_) => {}
        CargoEditionState::MissingManifest => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "Cargo.toml missing".to_owned(),
            "rustfmt edition checks require a root Cargo.toml with workspace or package edition."
                .to_owned(),
            Some("Cargo.toml".to_owned()),
            None,
            false,
        )),
        CargoEditionState::ParseError => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "Cargo.toml parse error".to_owned(),
            "rustfmt edition checks require a parseable root Cargo.toml.".to_owned(),
            Some("Cargo.toml".to_owned()),
            None,
            false,
        )),
        CargoEditionState::MissingEdition => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "Cargo.toml edition missing".to_owned(),
            message:
                "rustfmt edition checks require `[workspace.package].edition` or `[package].edition` in root Cargo.toml."
                    .to_owned(),
            file: Some("Cargo.toml".to_owned()),
            line: None,
            inventory: false,
        }),
    }
}

