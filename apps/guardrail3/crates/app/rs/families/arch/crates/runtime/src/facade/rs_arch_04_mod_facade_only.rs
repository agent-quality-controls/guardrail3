use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::FacadeSurface;

const ID: &str = "RS-ARCH-04";

pub(crate) fn check(surface: &FacadeSurface, results: &mut Vec<CheckResult>) {
    if !surface.is_mod_rs {
        return;
    }

    for item in &surface.body_items {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "mod.rs must be facade-only".to_owned(),
            format!(
                "mod.rs contains {} `{}`. Move it to a sibling .rs file in the same directory. mod.rs must only contain mod/use declarations and re-exports.",
                item.kind, item.name
            ),
            Some(surface.rel_path.clone()),
            Some(item.line),
            false,
        ));
    }

    for item in &surface.pub_uses {
        if item.is_broad_reexport {
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "mod.rs has broad re-export".to_owned(),
                format!(
                    "mod.rs uses broad re-export `pub use {}`. Use specific item re-exports instead (e.g., `pub use foo::Bar`).",
                    item.name
                ),
                Some(surface.rel_path.clone()),
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
                "mod.rs is facade-only".to_owned(),
                format!("mod.rs at `{}` contains only facade declarations.", surface.rel_path),
                Some(surface.rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    }
}
