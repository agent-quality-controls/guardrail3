use g3ts_apparch_types::{G3TsApparchConfigChecksInput, G3TsApparchLayer};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-APPARCH-CONFIG-01";

pub(crate) fn check(input: &G3TsApparchConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let violating = crate::support::violating_edges(
        input,
        G3TsApparchLayer::Types,
        &[
            G3TsApparchLayer::App,
            G3TsApparchLayer::Logic,
            G3TsApparchLayer::IoInbound,
            G3TsApparchLayer::IoOutbound,
        ],
    );

    if violating.is_empty() {
        if crate::support::has_layer_files(input, G3TsApparchLayer::Types) {
            results.push(crate::support::inventory(
                ID,
                "types layer depends only on allowed layers".to_owned(),
                "Types layer files import only `types` siblings or no apparch-owned layers."
                    .to_owned(),
            ));
        }
        return;
    }

    for edge in violating {
        results.push(crate::support::edge_error(
            ID,
            "types layer imports forbidden app layer".to_owned(),
            format!(
                "`{}` in `types` imports `{}` in `{}`. Keep `types` passive and move behavior or framework coupling outward.",
                edge.from_rel_path,
                edge.to_rel_path,
                crate::support::layer_label(edge.to_layer)
            ),
            edge,
        ));
    }
}
