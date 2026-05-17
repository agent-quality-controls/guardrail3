use g3rs_arch_types::types::G3RsArchPathAttrSite;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// I D const.
const ID: &str = "g3rs-arch/no-path-attr";

/// check fn.
pub(crate) fn check(site: &G3RsArchPathAttrSite, results: &mut Vec<G3CheckResult>) {
    if site
        .path_value
        .as_deref()
        .is_some_and(|path| is_test_sidecar_exempt(site, path))
    {
        return;
    }
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "#[path] attribute forbidden".to_owned(),
        format!(
            "`#[path = \"{}\"]` on `mod {}` bypasses the module facade. Use standard module resolution with mod.rs instead. Every module directory must have a mod.rs that serves as its facade.",
            site.path_value.as_deref().unwrap_or("..."),
            site.module_name
        ),
        Some(site.rel_path.clone()),
        Some(site.line),
    ));
}

/// is test sidecar exempt fn.
fn is_test_sidecar_exempt(site: &G3RsArchPathAttrSite, path_value: &str) -> bool {
    let Some(expected_module_name) = owned_sidecar_module_name(&site.rel_path) else {
        return false;
    };
    if site.module_name != expected_module_name {
        return false;
    }
    if path_value != format!("{expected_module_name}/mod.rs") {
        return false;
    }
    site.cfg_test_only
}

/// owned sidecar module name fn.
fn owned_sidecar_module_name(file_rel_path: &str) -> Option<String> {
    let file_name = file_rel_path.rsplit('/').next()?;
    let stem = file_name.strip_suffix(".rs")?;
    if stem == "lib" || stem == "mod" || stem.is_empty() {
        return None;
    }
    Some(format!("{stem}_tests"))
}
