use g3ts_astro_types::{G3TsAstroConfigChecksInput, G3TsAstroPackageSurfaceState};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-ASTRO-CONFIG-04";
const DEPENDENCY_NAME: &str = "velite";

pub(crate) fn check(input: &G3TsAstroConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    for contract in &input.integration_contracts {
        let rel_path = crate::support::package_rel_path(contract);

        match &contract.package {
            G3TsAstroPackageSurfaceState::Parsed { snapshot } => {
                if snapshot
                    .dependencies
                    .iter()
                    .chain(snapshot.dev_dependencies.iter())
                    .any(|dependency| dependency == DEPENDENCY_NAME)
                {
                    results.push(crate::support::error(
                        ID,
                        "Astro app package must not include `velite`",
                        format!(
                            "`{}` lists `velite` in dependencies or devDependencies. Remove `velite` from this Astro app and move page content onto Astro collections instead. Keeping Velite in an Astro app recreates the parallel content pipeline this family is meant to forbid.",
                            snapshot.rel_path
                        ),
                        Some(&snapshot.rel_path),
                    ));
                } else {
                    results.push(crate::support::info(
                        ID,
                        "velite package absent from Astro app",
                        format!("`{}` does not include `velite`.", snapshot.rel_path),
                        &snapshot.rel_path,
                    ));
                }
            }
            G3TsAstroPackageSurfaceState::Missing { .. }
            | G3TsAstroPackageSurfaceState::Unreadable { .. }
            | G3TsAstroPackageSurfaceState::ParseError { .. } => {
                results.push(crate::support::error(
                    ID,
                    "Astro app package must not include `velite`",
                    "`package.json` does not parse through an app package surface, so the Astro family cannot prove `velite` is absent. Restore `package.json` and keep `velite` out of this Astro app. Parallel Velite content pipelines are forbidden in Astro apps.".to_owned(),
                    rel_path,
                ));
            }
        }
    }
}
