use std::collections::{BTreeMap, BTreeSet};

use g3rs_apparch_types::{
    G3RsApparchCrate, G3RsApparchLayer, G3RsApparchPublicItemKind, G3RsApparchSourceChecksInput,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-APPARCH-SOURCE-04";

pub(crate) fn check(
    input: &G3RsApparchSourceChecksInput,
    crates_by_path: &BTreeMap<String, &G3RsApparchCrate>,
    results: &mut Vec<G3CheckResult>,
) {
    let mut crates_with_traits = BTreeSet::new();

    for fact in input
        .public_items
        .iter()
        .filter(|fact| fact.kind == G3RsApparchPublicItemKind::Trait)
    {
        let Some(krate) = crates_by_path.get(&fact.cargo_rel_path).copied() else {
            continue;
        };
        if !matches!(
            krate.layer,
            Some(G3RsApparchLayer::IoInbound) | Some(G3RsApparchLayer::IoOutbound)
        ) {
            continue;
        }
        let _ = crates_with_traits.insert(krate.cargo_rel_path.clone());
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            format!(
                "io crate `{}` defines public trait `{}`",
                crate::run::display_crate(krate),
                fact.item_name
            ),
            format!(
                "io crates must not define public traits. Move trait `{}` into `types/` so both logic and io/outbound can share the contract without leaking transport or implementation concerns.",
                fact.item_name
            ),
            Some(fact.rel_path.clone()),
            None,
        ));
    }

    for krate in &input.crates {
        if !matches!(
            krate.layer,
            Some(G3RsApparchLayer::IoInbound) | Some(G3RsApparchLayer::IoOutbound)
        ) {
            continue;
        }
        if crates_with_traits.contains(&krate.cargo_rel_path) {
            continue;
        }
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                format!(
                    "io crate `{}` defines no public traits",
                    crate::run::display_crate(krate)
                ),
                format!(
                    "io crate `{}` keeps trait contracts out of the io layer.",
                    crate::run::display_crate(krate)
                ),
                Some(krate.cargo_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    }
}

#[cfg(test)]
#[path = "rs_apparch_source_04_io_traits_in_types_tests/mod.rs"]
mod rs_apparch_source_04_io_traits_in_types_tests;
