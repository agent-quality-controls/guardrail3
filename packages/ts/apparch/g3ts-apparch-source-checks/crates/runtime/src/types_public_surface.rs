use g3ts_apparch_types::{
    G3TsApparchLayer, G3TsApparchPublicItemKind, G3TsApparchSourceChecksInput,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3ts-apparch/types-public-surface";

pub(crate) fn check(input: &G3TsApparchSourceChecksInput, results: &mut Vec<G3CheckResult>) {
    let violating = input
        .public_items
        .iter()
        .filter(|item| item.layer == G3TsApparchLayer::Types)
        .filter(|item| {
            matches!(
                item.kind,
                G3TsApparchPublicItemKind::Function | G3TsApparchPublicItemKind::Class
            )
        })
        .collect::<Vec<_>>();

    if violating.is_empty() {
        if input
            .files
            .iter()
            .any(|file| file.layer == G3TsApparchLayer::Types)
        {
            results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Info,
                    "types layer keeps behavioral API out of its public surface".to_owned(),
                    "Types layer files expose no exported functions or classes.".to_owned(),
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
            "types layer exposes behavioral API".to_owned(),
            format!(
                "`{}` exports `{}` from `types`. Keep behavior out of `types` and move it into `logic` or another owning runtime layer.",
                item.rel_path, item.item_name
            ),
            Some(item.rel_path.clone()),
            Some(item.line),
        ));
    }
}
