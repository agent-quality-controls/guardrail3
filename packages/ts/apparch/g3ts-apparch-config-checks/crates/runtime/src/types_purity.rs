use g3ts_apparch_types::{G3TsApparchConfigChecksInput, G3TsApparchLayer};
use guardrail3_check_types::G3CheckResult;

/// Stable rule identifier surfaced in findings.
const ID: &str = "g3ts-apparch/types-purity";

/// Validates the rule and pushes findings into `results`.
pub(crate) fn check(input: &G3TsApparchConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let violating = crate::support::violating_framework_imports(input, G3TsApparchLayer::Types);

    if violating.is_empty() {
        if crate::support::has_layer_files(input, G3TsApparchLayer::Types) {
            results.push(crate::support::inventory(
                ID,
                "types layer stays free of Next and React runtime imports".to_owned(),
                "Types layer files do not import `next/*`, `react`, or `react-dom` runtime modules."
                    .to_owned(),
            ));
        }
        return;
    }

    for import in violating {
        results.push(crate::support::external_import_error(
            ID,
            "types layer imports framework runtime module".to_owned(),
            format!(
                "`{}` in `types` imports external module `{}`. Keep `types` passive and free of Next/React runtime coupling.",
                import.from_rel_path, import.module_name
            ),
            import,
        ));
    }
}
