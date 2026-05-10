use g3ts_apparch_types::{G3TsApparchConfigChecksInput, G3TsApparchLayer};
use guardrail3_check_types::G3CheckResult;

/// Stable rule identifier surfaced in findings.
const ID: &str = "g3ts-apparch/logic-purity";

/// Validates the rule and pushes findings into `results`.
pub(crate) fn check(input: &G3TsApparchConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let violating = crate::support::violating_framework_imports(input, G3TsApparchLayer::Logic);

    if violating.is_empty() {
        if crate::support::has_layer_files(input, G3TsApparchLayer::Logic) {
            results.push(crate::support::inventory(
                ID,
                "logic layer stays free of Next and React runtime imports".to_owned(),
                "Logic layer files do not import `next/*`, `react`, or `react-dom` runtime modules."
                    .to_owned(),
            ));
        }
        return;
    }

    for import in violating {
        results.push(crate::support::external_import_error(
            ID,
            "logic layer imports framework runtime module".to_owned(),
            format!(
                "`{}` in `logic` imports external module `{}`. Keep `logic` independent from Next/React runtime code.",
                import.from_rel_path, import.module_name
            ),
            import,
        ));
    }
}
