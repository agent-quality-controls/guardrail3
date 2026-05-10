use g3ts_apparch_types::{
    G3TsApparchLayer, G3TsApparchPublicItemKind, G3TsApparchSourceChecksInput,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Stable identifier for the io-contracts-in-types rule.
const ID: &str = "g3ts-apparch/io-contracts-in-types";

/// Flags exported interfaces in `io/inbound` and `io/outbound`, emits inventory when clean.
pub(crate) fn check(input: &G3TsApparchSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    let violating = input
        .public_items
        .iter()
        .filter(|item| {
            matches!(
                item.layer,
                G3TsApparchLayer::IoInbound | G3TsApparchLayer::IoOutbound
            )
        })
        .filter(|item| item.kind == G3TsApparchPublicItemKind::Interface)
        .collect::<Vec<_>>();

    if violating.is_empty() {
        let has_io_files = input.files.iter().any(|file| {
            matches!(
                file.layer,
                G3TsApparchLayer::IoInbound | G3TsApparchLayer::IoOutbound
            )
        });
        if has_io_files {
            results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Info,
                    "io layers keep interface contracts out of their public surface".to_owned(),
                    "io/inbound and io/outbound expose no exported interfaces.".to_owned(),
                    None,
                    None,
                )
                .into_inventory(),
            );
        }
        return;
    }

    for item in violating {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "io layer exposes interface contract".to_owned(),
            format!(
                "`{}` exports interface `{}` from `{}`. Move shared contracts into `src/types`.",
                item.rel_path,
                item.item_name,
                if item.layer == G3TsApparchLayer::IoInbound {
                    "io/inbound"
                } else {
                    "io/outbound"
                }
            ),
            Some(item.rel_path.clone()),
            Some(item.line),
        ));
    }
}
