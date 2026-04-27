use g3rs_arch_types::types::{G3RsArchFacadeSurface, G3RsArchSourceCrate};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-arch/lib-facade-only";

pub(crate) fn check(
    node: &G3RsArchSourceCrate,
    surface: Option<&G3RsArchFacadeSurface>,
    results: &mut Vec<G3CheckResult>,
) {
    let Some(lib_rel) = &node.lib_rs_rel else {
        return;
    };
    let Some(surface) = surface else {
        return;
    };

    for item in &surface.body_items {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "lib.rs must be facade-only".to_owned(),
            format!(
                "lib.rs contains {} `{}`. Move it to a submodule. lib.rs must only contain mod/use declarations and re-exports.",
                item.kind, item.name
            ),
            Some(lib_rel.clone()),
            Some(item.line),
        ));
    }

    for item in crate::run::broad_reexports(surface) {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "lib.rs has broad re-export".to_owned(),
            format!(
                "lib.rs uses broad re-export `pub use {}`. Use specific item re-exports instead (e.g., `pub use foo::Bar`).",
                item.name
            ),
            Some(lib_rel.clone()),
            Some(item.line),
        ));
    }

    if surface.body_items.is_empty() && surface.broad_reexports.is_empty() {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "lib.rs is facade-only".to_owned(),
                format!(
                    "lib.rs in `{}` contains only facade declarations.",
                    node.rel_dir
                ),
                Some(lib_rel.clone()),
                None,
            )
            .into_inventory(),
        );
    }
}
