use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::PolicyRootCargoInput;

const ID: &str = "RS-CARGO-05";

pub fn check(input: &PolicyRootCargoInput<'_>, results: &mut Vec<CheckResult>) {
    let root = input.root;
    if root.parse_error.is_some() {
        return;
    }
    if root.edition_invalid {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "edition invalid".to_owned(),
            format!(
                "`{}` must declare edition as a string value in `[package]` or `[workspace.package]`.",
                root.cargo_rel_path
            ),
            Some(root.cargo_rel_path.clone()),
            None,
            false,
        ));
        return;
    }

    match root.edition.as_deref() {
        Some("2024" | "2021") => {
            if root.guardrail_parse_error {
                return;
            }
            results.push(
                CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Info,
                    "edition policy satisfied".to_owned(),
                    format!(
                        "`{}` declares edition `{}`.",
                        root.cargo_rel_path,
                        root.edition.as_deref().unwrap_or_default()
                    ),
                    Some(root.cargo_rel_path.clone()),
                    None,
                    false,
                )
                .as_inventory(),
            )
        }
        Some(other) => match edition_rank(other) {
            Some(_) => results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "edition below minimum".to_owned(),
                format!(
                    "`{}` declares edition `{other}`. Cargo policy requires edition `2021` or newer.",
                    root.cargo_rel_path
                ),
                Some(root.cargo_rel_path.clone()),
                None,
                false,
            )),
            None => results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "edition unrecognized".to_owned(),
                format!(
                    "`{}` declares unknown edition `{other}`. Use a supported Cargo edition string.",
                    root.cargo_rel_path
                ),
                Some(root.cargo_rel_path.clone()),
                None,
                false,
            )),
        },
        None => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "edition missing".to_owned(),
            format!(
                "`{}` must declare an edition for this {}. Add `edition = \"2024\"` to the package metadata.",
                root.cargo_rel_path,
                root.kind.label()
            ),
            Some(root.cargo_rel_path.clone()),
            None,
            false,
        )),
    }
}

fn edition_rank(edition: &str) -> Option<usize> {
    match edition {
        "2015" => Some(0),
        "2018" => Some(1),
        "2021" => Some(2),
        "2024" => Some(3),
        _ => None,
    }
}
