use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::{CrateNode, FacadeSurface};

const ID: &str = "RS-ARCH-02";

pub(crate) fn check(
    node: &CrateNode,
    surface: Option<&FacadeSurface>,
    results: &mut Vec<CheckResult>,
) {
    let Some(lib_rel) = &node.lib_rs_rel else {
        return;
    };
    let Some(surface) = surface else {
        return;
    };

    // Check for implementation logic in lib.rs.
    for item in &surface.body_items {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "lib.rs must be facade-only".to_owned(),
            format!(
                "lib.rs contains {} `{}`. Keep lib.rs limited to pub mod/use declarations, type/const definitions, and specific re-exports.",
                item.kind, item.name
            ),
            Some(lib_rel.clone()),
            Some(item.line),
            false,
        ));
    }

    // Check for broad re-exports.
    for item in &surface.pub_uses {
        if item.is_broad_reexport {
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "lib.rs has broad re-export".to_owned(),
                format!(
                    "lib.rs uses broad re-export `pub use {}`. Use specific item re-exports instead (e.g., `pub use foo::Bar`).",
                    item.name
                ),
                Some(lib_rel.clone()),
                Some(item.line),
                false,
            ));
        }
    }

    if surface.body_items.is_empty()
        && !surface.pub_uses.iter().any(|u| u.is_broad_reexport)
    {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "lib.rs is facade-only".to_owned(),
                format!("lib.rs in `{}` contains only facade declarations.", node.rel_dir),
                Some(lib_rel.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    }
}
