use g3rs_arch_types::types::G3RsArchFacadeSurface;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// I D const.
const ID: &str = "g3rs-arch/mod-facade-only";

/// check fn.
pub(crate) fn check(surface: &G3RsArchFacadeSurface, results: &mut Vec<G3CheckResult>) {
    for item in &surface.body_items {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "mod.rs must be facade-only".to_owned(),
            format!(
                "mod.rs contains {} `{}`. Move it to a sibling .rs file in the same directory. mod.rs must only contain mod/use declarations and re-exports.",
                item.kind, item.name
            ),
            Some(surface.rel_path.clone()),
            Some(item.line),
        ));
    }

    for item in crate::run::broad_reexports(surface) {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "mod.rs has broad re-export".to_owned(),
            format!(
                "mod.rs uses broad re-export `pub use {}`. Use specific item re-exports instead (e.g., `pub use foo::Bar`).",
                item.name
            ),
            Some(surface.rel_path.clone()),
            Some(item.line),
        ));
    }

    if surface.body_items.is_empty() && surface.broad_reexports.is_empty() {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "mod.rs is facade-only".to_owned(),
                format!(
                    "mod.rs at `{}` contains only facade declarations.",
                    surface.rel_path
                ),
                Some(surface.rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    }
}
