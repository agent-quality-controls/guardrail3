use g3ts_arch_types::{G3TsArchFileTreeChecksInput, G3TsArchManifestState};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "TS-ARCH-FILETREE-01";

pub(crate) fn check(input: &G3TsArchFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    let G3TsArchManifestState::Parsed { snapshot } = &input.manifest else {
        return;
    };

    for entrypoint in &snapshot.declared_entrypoints {
        let exists = input
            .existing_entrypoints
            .iter()
            .any(|rel_path| rel_path == &entrypoint.rel_path);
        if exists {
            results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Info,
                    "declared facade entrypoint exists".to_owned(),
                    format!(
                        "Declared facade entrypoint `{}` exists in the target root.",
                        entrypoint.rel_path
                    ),
                    Some(snapshot.rel_path.clone()),
                    None,
                )
                .into_inventory(),
            );
        } else {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "declared facade entrypoint missing".to_owned(),
                format!(
                    "Declared facade entrypoint `{}` does not exist. Create the facade file or fix the manifest.",
                    entrypoint.rel_path
                ),
                Some(snapshot.rel_path.clone()),
                None,
            ));
        }
    }
}
