use g3rs_arch_types::types::G3RsArchModuleDir;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-ARCH-FILETREE-03";

pub(crate) fn check(module_dir: &G3RsArchModuleDir, results: &mut Vec<G3CheckResult>) {
    if module_dir.has_sibling_file && !module_dir.has_mod_rs {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "module directory uses foo.rs convention".to_owned(),
            format!(
                "Module directory `{}` uses the foo.rs-alongside-foo/ convention. Use `{}/mod.rs` instead. The module facade must live inside the module directory.",
                module_dir.dir_rel, module_dir.dir_rel
            ),
            Some(module_dir.mod_decl_file.clone()),
            Some(module_dir.mod_decl_line),
        ));
        return;
    }

    if !module_dir.has_mod_rs {
        let (file, line, message) = if module_dir.mod_decl_file.is_empty() {
            (
                Some(module_dir.dir_rel.clone()),
                None,
                format!(
                    "Directory `{}` contains {} .rs files but has no mod.rs. Create `{}/mod.rs` with `mod` declarations for each .rs file in the directory.",
                    module_dir.dir_rel, module_dir.rs_file_count, module_dir.dir_rel
                ),
            )
        } else {
            (
                Some(module_dir.mod_decl_file.clone()),
                Some(module_dir.mod_decl_line),
                format!(
                    "Directory `{}` is used as a module (declared in `{}` line {}) but has no mod.rs. Every module directory must have a mod.rs facade. Create `{}/mod.rs` and declare its submodules there.",
                    module_dir.dir_rel,
                    module_dir.mod_decl_file,
                    module_dir.mod_decl_line,
                    module_dir.dir_rel
                ),
            )
        };

        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "module directory missing mod.rs".to_owned(),
            message,
            file,
            line,
        ));
        return;
    }

    results.push(
        G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Info,
            "module directory has mod.rs".to_owned(),
            format!(
                "Module directory `{}` has a proper mod.rs facade.",
                module_dir.dir_rel
            ),
            Some(format!("{}/mod.rs", module_dir.dir_rel)),
            None,
        )
        .into_inventory(),
    );
}

#[cfg(test)]
#[path = "rs_arch_03_mod_rs_required_tests/mod.rs"]
// reason: keep rule tests in the owned x_tests sidecar directory.
mod rs_arch_03_mod_rs_required_tests;
