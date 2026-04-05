use g3_fmt_content_checks_types::G3FmtContentChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::inputs::cargo_edition;

const ID: &str = "RS-FMT-06";

pub(crate) fn check(input: &G3FmtContentChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(rustfmt_edition) = input.rustfmt.edition.as_deref() else {
        return;
    };

    match cargo_edition(&input.cargo) {
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
    }
}
