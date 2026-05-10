use g3ts_apparch_types::{G3TsApparchConfigChecksInput, G3TsApparchLayer};
use guardrail3_check_types::G3CheckResult;

/// Stable rule identifier surfaced in findings.
const ID: &str = "g3ts-apparch/io-outbound-dependency-direction";

/// Validates the rule and pushes findings into `results`.
pub(crate) fn check(input: &G3TsApparchConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let violating = crate::support::violating_edges(
        input,
        G3TsApparchLayer::IoOutbound,
        &[
            G3TsApparchLayer::App,
            G3TsApparchLayer::Logic,
            G3TsApparchLayer::IoInbound,
        ],
    );

    if violating.is_empty() {
        if crate::support::has_layer_files(input, G3TsApparchLayer::IoOutbound) {
            results.push(crate::support::inventory(
                ID,
                "io/outbound layer depends only on allowed layers".to_owned(),
                "io/outbound layer files import only `io/outbound`, `types`, or no apparch-owned layers."
                    .to_owned(),
            ));
        }
        return;
    }

    for edge in violating {
        results.push(crate::support::edge_error(
            ID,
            "io/outbound layer imports forbidden app layer".to_owned(),
            format!(
                "`{}` in `io/outbound` imports `{}` in `{}`. Outbound adapters must not depend on logic, inbound adapters, or the app shell.",
                edge.from_rel_path,
                edge.to_rel_path,
                crate::support::layer_label(edge.to_layer)
            ),
            edge,
        ));
    }
}
