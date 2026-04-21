use g3ts_arch_types::{G3TsArchConfigChecksInput, G3TsArchManifestState};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ARCH-CONFIG-01";

pub(crate) fn check(input: &G3TsArchConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    match &input.manifest {
        G3TsArchManifestState::Missing => results.push(crate::support::error(
            ID,
            "root package.json missing",
            "No root `package.json` file was found. TypeScript package architecture requires a root package manifest."
                .to_owned(),
            "package.json",
        )),
        G3TsArchManifestState::Unreadable { rel_path, .. }
        | G3TsArchManifestState::ParseError { rel_path, .. }
        | G3TsArchManifestState::Parsed { snapshot: g3ts_arch_types::G3TsArchManifestSnapshot { rel_path, .. } } => results.push(
            crate::support::info(
                ID,
                "root package.json exists",
                format!("Root package manifest `{rel_path}` exists for TS arch checks."),
                rel_path,
            ),
        ),
    }
}
