use g3rs_fmt_types::{G3RsFmtCargoState, G3RsFmtConfigChecksInput};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-FMT-CONFIG-04";

pub(crate) fn check(input: &G3RsFmtConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let g3rs_fmt_types::G3RsFmtRustfmtConfigState::Parsed(rustfmt) = &input.rustfmt_state else {
        return;
    };
    let Some(rustfmt_edition) = rustfmt.edition.as_deref() else {
        return;
    };

    match &input.cargo_state {
        G3RsFmtCargoState::Parsed(cargo) => match cargo.edition.as_deref() {
            Some(cargo_edition) if rustfmt_edition != cargo_edition => {
                results.push(G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Warn,
                    "rustfmt edition differs from Cargo edition".to_owned(),
                    format!(
                        "rustfmt edition `{rustfmt_edition}` differs from Cargo edition `{cargo_edition}`. Update `edition` in rustfmt.toml to `{cargo_edition}`."
                    ),
                    Some(input.rustfmt_rel_path.clone()),
                    None,
                ));
            }
            Some(_) => {}
            None => results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "Cargo.toml edition missing".to_owned(),
                format!(
                    "rustfmt edition checks require `[workspace.package].edition` or `[package].edition` in {}.",
                    input.cargo_rel_path
                ),
                Some(input.cargo_rel_path.clone()),
                None,
            )),
        },
        G3RsFmtCargoState::Missing => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "Cargo.toml missing".to_owned(),
            format!(
                "rustfmt edition checks require a root {} with workspace or package edition.",
                input.cargo_rel_path
            ),
            Some(input.cargo_rel_path.clone()),
            None,
        )),
        G3RsFmtCargoState::Unreadable => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "Cargo.toml unreadable".to_owned(),
            format!(
                "rustfmt edition checks require a readable root {}.",
                input.cargo_rel_path
            ),
            Some(input.cargo_rel_path.clone()),
            None,
        )),
        G3RsFmtCargoState::ParseError => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "Cargo.toml parse error".to_owned(),
            format!(
                "rustfmt edition checks require a parseable root {}.",
                input.cargo_rel_path
            ),
            Some(input.cargo_rel_path.clone()),
            None,
        )),
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
