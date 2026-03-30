use guardrail3_domain_report::{CheckResult, Severity};

use super::facts::CargoEditionState;
use super::inputs::RustfmtRootInput;

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

#[cfg(test)]
pub(crate) enum TestCargoEditionState {
    Edition(&'static str),
}

#[cfg(test)]
pub(crate) fn run_check(
    cargo_edition: TestCargoEditionState,
    rustfmt_edition: &str,
) -> Vec<CheckResult> {
    let cargo_edition = match cargo_edition {
        TestCargoEditionState::Edition(value) => CargoEditionState::Present(value.to_owned()),
    };
    let input = RustfmtRootInput {
        config_rel: Some("rustfmt.toml".to_owned()),
        parsed: Some(
            toml::from_str::<toml::Value>(&format!(
                "edition = \"{rustfmt_edition}\"\nmax_width = 100\ntab_spaces = 4\nuse_field_init_shorthand = true\nuse_try_shorthand = true\nreorder_imports = true\nreorder_modules = true\n"
            ))
            .expect("RS-FMT-06 in-memory rustfmt TOML fixture should parse"),
        ),
        cargo_edition,
        toolchain_channel: super::facts::ToolchainChannelState::Present("stable".to_owned()),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
#[path = "rs_fmt_06_edition_mismatch_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_fmt_06_edition_mismatch_tests;
