use g3rs_apparch_types::{
    G3RsApparchIoTraitsSourceChecksInput, G3RsApparchLayer, G3RsApparchPublicItemKind,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-apparch/io-traits-in-types";

/// Pushes a violation when an io-port trait surfaces from a `*-types` package.
pub(crate) fn check(
    input: &G3RsApparchIoTraitsSourceChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    let krate = &input.krate;
    if !matches!(
        krate.layer,
        Some(G3RsApparchLayer::IoInbound | G3RsApparchLayer::IoOutbound)
    ) {
        return;
    }

    for fact in input
        .public_traits
        .iter()
        .filter(|fact| fact.kind == G3RsApparchPublicItemKind::Trait)
    {
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

    if input.public_traits.is_empty() {
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
