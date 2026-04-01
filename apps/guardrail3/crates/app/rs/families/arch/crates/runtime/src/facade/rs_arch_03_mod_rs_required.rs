use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::ModuleDir;

const ID: &str = "RS-ARCH-03";

pub(crate) fn check(module: &ModuleDir, results: &mut Vec<CheckResult>) {
    // Check if foo.rs convention is used (forbidden).
    if module.has_sibling_file && !module.has_mod_rs {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "module directory uses foo.rs convention".to_owned(),
            format!(
                "Module directory `{}` uses the foo.rs-alongside-foo/ convention. Use `{}/mod.rs` instead. The module facade must live inside the module directory.",
                module.dir_rel, module.dir_rel
            ),
            Some(module.mod_decl_file.clone()),
            Some(module.mod_decl_line),
            false,
        ));
        return;
    }

    if !module.has_mod_rs {
        let detail = if module.mod_decl_file.is_empty() {
            format!(
                "Directory `{}` contains {} .rs files but has no mod.rs. It is likely wired via #[path] attributes. Add a mod.rs facade and use normal module declarations instead.",
                module.dir_rel, module.rs_file_count
            )
        } else {
            format!(
                "Directory `{}` is used as a module (declared in `{}` line {}) but has no mod.rs. Every module directory must have a mod.rs facade.",
                module.dir_rel, module.mod_decl_file, module.mod_decl_line
            )
        };
        let file = if module.mod_decl_file.is_empty() {
            Some(module.dir_rel.clone())
        } else {
            Some(module.mod_decl_file.clone())
        };
        let line = if module.mod_decl_line > 0 {
            Some(module.mod_decl_line)
        } else {
            None
        };
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "module directory missing mod.rs".to_owned(),
            detail,
            file,
            line,
            false,
        ));
        return;
    }

    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "module directory has mod.rs".to_owned(),
            format!("Module directory `{}` has a proper mod.rs facade.", module.dir_rel),
            Some(format!("{}/mod.rs", module.dir_rel)),
            None,
            false,
        )
        .as_inventory(),
    );
}
