use g3ts_apparch_types::{G3TsApparchConfigChecksInput, G3TsApparchLayer};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "TS-APPARCH-CONFIG-02";

pub(crate) fn check(input: &G3TsApparchConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let violating = crate::support::violating_edges(
        input,
        G3TsApparchLayer::Logic,
        &[
            G3TsApparchLayer::App,
            G3TsApparchLayer::IoInbound,
            G3TsApparchLayer::IoOutbound,
        ],
    );

    if violating.is_empty() {
        if crate::support::has_layer_files(input, G3TsApparchLayer::Logic) {
            results.push(crate::support::inventory(
                ID,
                "logic layer depends only on allowed layers".to_owned(),
                "Logic layer files import only `logic`, `types`, or no apparch-owned layers."
                    .to_owned(),
            ));
        }
        return;
    }

    for edge in violating {
        results.push(crate::support::edge_error(
            ID,
            "logic layer imports forbidden app layer".to_owned(),
            format!(
                "`{}` in `logic` imports `{}` in `{}`. Logic must stay independent of app shell and concrete io adapters.",
                edge.from_rel_path,
                edge.to_rel_path,
                crate::support::layer_label(edge.to_layer)
            ),
            edge,
        ));
    }
}
