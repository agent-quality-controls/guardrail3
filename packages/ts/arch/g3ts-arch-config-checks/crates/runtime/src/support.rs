use g3ts_arch_types::{G3TsArchConfigChecksInput, G3TsArchManifestSnapshot, G3TsArchManifestState};
use guardrail3_check_types::{G3CheckResult, G3Severity};

#[must_use]
pub(crate) fn parsed_manifest(
    input: &G3TsArchConfigChecksInput,
) -> Option<&G3TsArchManifestSnapshot> {
    match &input.manifest {
        G3TsArchManifestState::Parsed { snapshot } => Some(snapshot),
        G3TsArchManifestState::Missing
        | G3TsArchManifestState::Unreadable { .. }
        | G3TsArchManifestState::ParseError { .. } => None,
    }
}

#[must_use]
pub(crate) fn canonical_entrypoint(rel_path: &str) -> bool {
    matches!(
        rel_path,
        "src/index.ts" | "src/index.tsx" | "index.ts" | "index.tsx"
    )
}

#[must_use]
pub(crate) fn error(id: &str, title: &str, message: String, file: &str) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title.to_owned(),
        message,
        Some(file.to_owned()),
        None,
    )
}

#[must_use]
pub(crate) fn info(id: &str, title: &str, message: String, file: &str) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Info,
        title.to_owned(),
        message,
        Some(file.to_owned()),
        None,
    )
    .into_inventory()
}
