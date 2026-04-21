use g3ts_arch_types::{G3TsArchFacadeFileState, G3TsArchSourceChecksInput};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "TS-ARCH-SOURCE-01";

pub(crate) fn check(input: &G3TsArchSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    for facade in &input.facades {
        match facade {
            G3TsArchFacadeFileState::Unreadable { rel_path, reason } => {
                results.push(G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Error,
                    "facade file unreadable".to_owned(),
                    format!("Facade file `{rel_path}` is unreadable: {reason}."),
                    Some(rel_path.clone()),
                    None,
                ))
            }
            G3TsArchFacadeFileState::ParseError { rel_path, reason } => {
                results.push(G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Error,
                    "facade file parse failed".to_owned(),
                    format!("Facade file `{rel_path}` could not be parsed: {reason}."),
                    Some(rel_path.clone()),
                    None,
                ))
            }
            G3TsArchFacadeFileState::Parsed { surface } => results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Info,
                    "facade file parseable".to_owned(),
                    format!("Facade file `{}` parsed successfully.", surface.rel_path),
                    Some(surface.rel_path.clone()),
                    None,
                )
                .into_inventory(),
            ),
        }
    }
}
