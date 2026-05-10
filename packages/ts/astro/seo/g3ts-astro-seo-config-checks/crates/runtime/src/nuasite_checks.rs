use g3ts_astro_seo_types::{G3TsAstroConfigSurfaceState, G3TsAstroSeoIntegrationContractInput};
use guardrail3_check_types::G3CheckResult;

/// Static rule data.
const ID: &str = "g3ts-astro-seo/nuasite-checks";
/// Static rule data.
const DEPENDENCY_NAME: &str = "@nuasite/checks";

/// Validates the rule and pushes findings into `results`.
/// Internal helper exported within the runtime crate.
pub(crate) fn check(
    contract: &G3TsAstroSeoIntegrationContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = crate::support::astro_config_rel_path(&contract.astro_config);
    let has_package = crate::support::package_has_dependency(&contract.package, DEPENDENCY_NAME);
    let has_build_script = crate::support::package_safely_runs_astro_build(&contract.package);
    let has_static_output = crate::support::astro_config_is_static(&contract.astro_config);
    let has_checks = match &contract.astro_config {
        G3TsAstroConfigSurfaceState::Parsed { snapshot } => {
            crate::nuasite_options::astro_config_has_nuasite_checks_with_required_options(snapshot)
        }
        G3TsAstroConfigSurfaceState::Missing { .. }
        | G3TsAstroConfigSurfaceState::Unreadable { .. }
        | G3TsAstroConfigSurfaceState::ParseError { .. } => false,
    };

    let presence = NuasitePartsPresence::from_parts([
        NuasitePart {
            label: "package dependency",
            present: has_package,
        },
        NuasitePart {
            label: "safe `astro build` script",
            present: has_build_script,
        },
        NuasitePart {
            label: "explicit static output",
            present: has_static_output,
        },
        NuasitePart {
            label: "fail-closed `checks()` integration",
            present: has_checks,
        },
    ]);
    if presence.all_present() {
        results.push(crate::support::info(
                ID,
                "Nuasite rendered-output checks are installed and wired",
                format!("`{rel_path}` wires `checks()` from `@nuasite/checks` with fail-closed options and the package scripts safely run `astro build`."),
                rel_path,
            ));
        return;
    }

    results.push(crate::support::error(
            ID,
            "Nuasite rendered-output checks are not installed and wired",
            format!(
                "This Astro app must list `{DEPENDENCY_NAME}`, safely run `astro build`, set `output: \"static\"`, and wire `checks()` from `{DEPENDENCY_NAME}` with `failOnError: true`, `failOnWarning: true`, `reportJson: true`, no disabled `seo`, `performance`, `accessibility`, or `geo` lanes, and `customChecks: [structuredDataPresentCheck]`. Missing pieces: {}.",
                presence.missing_parts().join(", ")
            ),
            Some(rel_path),
        ));
}

/// One Nuasite contract piece tracked alongside its label.
#[derive(Debug, Clone, Copy)]
struct NuasitePart {
    /// Display label for the missing-parts message.
    label: &'static str,
    /// True when the piece is wired.
    present: bool,
}

/// Tracks which Nuasite contract pieces are wired in the workspace.
#[derive(Debug, Clone, Copy)]
struct NuasitePartsPresence {
    /// Required pieces in stable presentation order.
    parts: [NuasitePart; 4],
}

impl NuasitePartsPresence {
    /// Build a presence record from a list of parts.
    const fn from_parts(parts: [NuasitePart; 4]) -> Self {
        Self { parts }
    }

    /// True when every contract piece is wired.
    fn all_present(self) -> bool {
        self.parts.iter().all(|part| part.present)
    }

    /// Returns labels for missing contract pieces.
    fn missing_parts(self) -> Vec<&'static str> {
        self.parts
            .iter()
            .filter(|part| !part.present)
            .map(|part| part.label)
            .collect()
    }
}
