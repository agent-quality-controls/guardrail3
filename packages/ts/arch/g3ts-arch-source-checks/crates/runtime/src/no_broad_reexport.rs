use g3ts_arch_types::{G3TsArchFacadeFileState, G3TsArchSourceChecksInput};
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Stable identifier for the no-broad-reexport rule.
const ID: &str = "g3ts-arch/no-broad-reexport";

/// Flags facade files that re-export the entire package surface.
pub(crate) fn check(input: &G3TsArchSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    for facade in &input.facades {
        let G3TsArchFacadeFileState::Parsed { surface } = facade else {
            continue;
        };

        for reexport in &surface.broad_reexports {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "facade file has broad re-export".to_owned(),
                format!(
                    "Facade file `{}` uses broad re-export `{}`. Re-export specific items instead.",
                    surface.rel_path, reexport.source
                ),
                Some(surface.rel_path.clone()),
                Some(reexport.line),
            ));
        }

        if surface.broad_reexports.is_empty() {
            results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Info,
                    "facade file avoids broad re-exports".to_owned(),
                    format!(
                        "Facade file `{}` avoids `export *` broad re-exports.",
                        surface.rel_path
                    ),
                    Some(surface.rel_path.clone()),
                    None,
                )
                .into_inventory(),
            );
        }
    }
}
