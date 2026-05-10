use g3ts_apparch_types::{G3TsApparchConfigChecksInput, G3TsApparchLayer};
use guardrail3_check_types::G3CheckResult;

/// Stable rule identifier surfaced in findings.
const ID: &str = "g3ts-apparch/io-inbound-dependency-direction";

/// Validates the rule and pushes findings into `results`.
pub(crate) fn check(input: &G3TsApparchConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let violating = crate::support::violating_edges(
        input,
        G3TsApparchLayer::IoInbound,
        &[G3TsApparchLayer::App, G3TsApparchLayer::IoOutbound],
    );

    if violating.is_empty() {
        if crate::support::has_layer_files(input, G3TsApparchLayer::IoInbound) {
            results.push(crate::support::inventory(
                ID,
                "io/inbound layer depends only on allowed layers".to_owned(),
                "io/inbound layer files import only `io/inbound`, `logic`, `types`, or no apparch-owned layers."
                    .to_owned(),
            ));
        }
        return;
    }

    for edge in violating {
        results.push(crate::support::edge_error(
            ID,
            "io/inbound layer imports forbidden app layer".to_owned(),
            format!(
                "`{}` in `io/inbound` imports `{}` in `{}`. Inbound adapters may call logic, but they must not reach outbound adapters or the app shell directly.",
                edge.from_rel_path,
                edge.to_rel_path,
                crate::support::layer_label(edge.to_layer)
            ),
            edge,
        ));
    }
}
