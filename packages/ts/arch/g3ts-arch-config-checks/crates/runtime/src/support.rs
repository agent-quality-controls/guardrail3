use g3ts_arch_types::{G3TsArchConfigChecksInput, G3TsArchManifestSnapshot, G3TsArchManifestState};
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Borrow the parsed manifest snapshot if `input.manifest` is in the
/// `Parsed` state, returning `None` otherwise.
#[must_use]
pub(crate) const fn parsed_manifest(
    input: &G3TsArchConfigChecksInput,
) -> Option<&G3TsArchManifestSnapshot> {
    match &input.manifest {
        G3TsArchManifestState::Parsed { snapshot } => Some(snapshot),
        G3TsArchManifestState::Missing
        | G3TsArchManifestState::Unreadable { .. }
        | G3TsArchManifestState::ParseError { .. } => None,
    }
}

/// Returns `true` when `rel_path` matches one of the canonical facade
/// entry-point locations (`src/index.ts(x)` or `index.ts(x)`).
#[must_use]
pub(crate) fn canonical_entrypoint(rel_path: &str) -> bool {
    matches!(
        rel_path,
        "src/index.ts" | "src/index.tsx" | "index.ts" | "index.tsx"
    )
}

/// Build an `Error`-severity check result tagged with `file`.
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

/// Build an inventory-tagged `Info` check result for `file`.
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
