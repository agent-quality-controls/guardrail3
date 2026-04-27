use g3ts_apparch_types::{G3TsApparchConfigChecksInput, G3TsApparchLayer};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "g3ts-apparch/app-no-direct-outbound";

pub(crate) fn check(input: &G3TsApparchConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let violating = crate::support::violating_edges(
        input,
        G3TsApparchLayer::App,
        &[G3TsApparchLayer::IoOutbound],
    );

    if violating.is_empty() {
        if crate::support::has_layer_files(input, G3TsApparchLayer::App) {
            results.push(crate::support::inventory(
                ID,
                "app shell avoids direct outbound imports".to_owned(),
                "App shell files do not import `io/outbound` directly.".to_owned(),
            ));
        }
        return;
    }

    for edge in violating {
        results.push(crate::support::edge_error(
            ID,
            "app shell imports io/outbound directly".to_owned(),
            format!(
                "`{}` in `app` imports `{}` in `io/outbound`. Keep Next entry files thin and delegate through inbound adapters or logic.",
                edge.from_rel_path, edge.to_rel_path
            ),
            edge,
        ));
    }
}
