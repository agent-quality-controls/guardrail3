use g3ts_arch_types::{G3TsArchConfigChecksInput, G3TsArchManifestState};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "g3ts-arch/root-manifest-parseable";

pub(crate) fn check(input: &G3TsArchConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    match &input.manifest {
        G3TsArchManifestState::Missing => {}
        G3TsArchManifestState::Unreadable { rel_path, reason } => {
            results.push(crate::support::error(
                ID,
                "root package.json unreadable",
                format!("Root package manifest `{rel_path}` is unreadable: {reason}."),
                rel_path,
            ))
        }
        G3TsArchManifestState::ParseError { rel_path, reason } => {
            results.push(crate::support::error(
                ID,
                "root package.json parse failed",
                format!("Root package manifest `{rel_path}` could not be parsed: {reason}."),
                rel_path,
            ))
        }
        G3TsArchManifestState::Parsed { snapshot } => results.push(crate::support::info(
            ID,
            "root package.json parseable",
            format!(
                "Root package manifest `{}` parsed successfully.",
                snapshot.rel_path
            ),
            &snapshot.rel_path,
        )),
    }
}
