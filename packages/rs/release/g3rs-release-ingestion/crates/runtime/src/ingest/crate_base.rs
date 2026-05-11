use std::collections::BTreeSet;
use std::path::Path;

use cargo_toml_parser::types::{
    CargoToml, InheritableValue, PackageSection, StringOrBool, VecStringOrBool,
    WorkspacePackageSection,
};

use g3_workspace_crawl::G3WorkspaceCrawl;

use super::collect::{CrateBase, CrateReadmeFacts, ParsedCrate};

/// `(readme path field, inherited from workspace)`.
type ReadmePathField<'a> = (Option<&'a str>, bool);

/// `build_crate_base` function.
pub(super) fn build_crate_base(
    crawl: &G3WorkspaceCrawl,
    krate: &ParsedCrate,
    workspace_package: Option<&WorkspacePackageSection>,
) -> CrateBase {
    let package = krate.cargo.package.as_ref();
    let name = package
        .and_then(|package| package.name.clone())
        .unwrap_or_else(|| krate.cargo_rel_path.clone());
    let publishable = publishable(package, workspace_package);
    let is_binary = is_binary_crate(crawl, &krate.rel_dir, &krate.cargo);
    let binary_target_names = binary_target_names(crawl, &krate.rel_dir, &krate.cargo);
    let readme_declared_false = readme_declared_false(package, workspace_package);
    let (readme_field, readme_from_workspace) = readme_path_field(package, workspace_package);
    let readme_base_rel_dir = if readme_from_workspace {
        ""
    } else {
        krate.rel_dir.as_str()
    };
    let (readme_rel_path, readme_abs_path) = super::paths::resolve_manifest_relative_path(
        crawl,
        readme_base_rel_dir,
        readme_field.unwrap_or("README.md"),
    );

    CrateBase {
        name,
        cargo_rel_path: krate.cargo_rel_path.clone(),
        cargo_abs_path: krate.cargo_abs_path.clone(),
        cargo: krate.cargo.clone(),
        publishable,
        is_binary,
        is_library: is_library_crate(crawl, &krate.rel_dir, &krate.cargo),
        binary_target_names,
        readme: CrateReadmeFacts {
            declared_false: readme_declared_false,
            rel_path: readme_rel_path.clone(),
            abs_path: readme_abs_path,
            exists: !readme_declared_false && super::paths::file_exists(crawl, &readme_rel_path),
        },
    }
}

/// `publishable` function.
fn publishable(
    package: Option<&PackageSection>,
    workspace_package: Option<&WorkspacePackageSection>,
) -> bool {
    let Some(package) = package else {
        return false;
    };

    match package.publish.as_ref() {
        None => false,
        Some(InheritableValue::Value(value)) => publish_value_is_publishable(value),
        Some(InheritableValue::Inherit(_)) => workspace_package
            .and_then(|workspace| workspace.publish.as_ref())
            .is_some_and(publish_value_is_publishable),
    }
}

/// True when a `publish` value renders the crate publishable.
fn publish_value_is_publishable(value: &VecStringOrBool) -> bool {
    match value {
        VecStringOrBool::Bool(flag) => *flag,
        VecStringOrBool::VecString(values) => !values.is_empty(),
    }
}

/// `readme_declared_false` function.
fn readme_declared_false(
    package: Option<&PackageSection>,
    workspace_package: Option<&WorkspacePackageSection>,
) -> bool {
    match package.and_then(|package| package.readme.as_ref()) {
        Some(InheritableValue::Value(StringOrBool::Bool(false))) => true,
        Some(InheritableValue::Inherit(_)) => matches!(
            workspace_package.and_then(|workspace| workspace.readme.as_ref()),
            Some(StringOrBool::Bool(false))
        ),
        Some(InheritableValue::Value(StringOrBool::Bool(true) | StringOrBool::String(_)))
        | None => false,
    }
}

/// `readme_path_field` function.
fn readme_path_field<'a>(
    package: Option<&'a PackageSection>,
    workspace_package: Option<&'a WorkspacePackageSection>,
) -> ReadmePathField<'a> {
    match package.and_then(|package| package.readme.as_ref()) {
        Some(InheritableValue::Value(StringOrBool::String(path))) => (Some(path.as_str()), false),
        Some(InheritableValue::Inherit(_)) => (
            workspace_package
                .and_then(|workspace| workspace.readme.as_ref())
                .and_then(|value| match value {
                    StringOrBool::String(path) => Some(path.as_str()),
                    StringOrBool::Bool(_) => None,
                }),
            true,
        ),
        Some(InheritableValue::Value(StringOrBool::Bool(_))) | None => (None, false),
    }
}

/// `is_library_crate` function.
fn is_library_crate(crawl: &G3WorkspaceCrawl, rel_dir: &str, cargo: &CargoToml) -> bool {
    cargo.lib.is_some()
        || super::paths::file_exists(crawl, &super::paths::join_under_root(rel_dir, "src/lib.rs"))
}

/// `is_binary_crate` function.
fn is_binary_crate(crawl: &G3WorkspaceCrawl, rel_dir: &str, cargo: &CargoToml) -> bool {
    if !cargo.bin.is_empty() {
        return true;
    }
    let autobins_disabled = cargo
        .package
        .as_ref()
        .and_then(|package| package.autobins)
        .is_some_and(|autobins| !autobins);
    !autobins_disabled
        && (super::paths::file_exists(
            crawl,
            &super::paths::join_under_root(rel_dir, "src/main.rs"),
        ) || autodiscovered_bin_exists(crawl, rel_dir))
}

/// `binary_target_names` function.
fn binary_target_names(
    crawl: &G3WorkspaceCrawl,
    rel_dir: &str,
    cargo: &CargoToml,
) -> BTreeSet<String> {
    let mut names = BTreeSet::new();

    for bin in &cargo.bin {
        if let Some(name) = bin.name.clone() {
            let _ = names.insert(name);
            continue;
        }
        if let Some(path) = bin.path.as_deref()
            && let Some(name) = binary_name_from_path(path)
        {
            let _ = names.insert(name);
        }
    }

    let autobins_disabled = cargo
        .package
        .as_ref()
        .and_then(|package| package.autobins)
        .is_some_and(|autobins| !autobins);
    if autobins_disabled {
        return names;
    }

    if super::paths::file_exists(
        crawl,
        &super::paths::join_under_root(rel_dir, "src/main.rs"),
    ) && let Some(package_name) = cargo
        .package
        .as_ref()
        .and_then(|package| package.name.as_ref())
    {
        let _ = names.insert(package_name.clone());
    }

    let src_bin_rel = super::paths::join_under_root(rel_dir, "src/bin");
    for file in super::paths::direct_child_files(crawl, &src_bin_rel) {
        if let Some(name) = binary_name_from_path(&file) {
            let _ = names.insert(name);
        }
    }
    for dir in super::paths::direct_child_dirs(crawl, &src_bin_rel) {
        if super::paths::file_exists(
            crawl,
            &super::paths::join_under_root(&src_bin_rel, &format!("{dir}/main.rs")),
        ) {
            let _ = names.insert(dir);
        }
    }

    names
}

/// `autodiscovered_bin_exists` function.
fn autodiscovered_bin_exists(crawl: &G3WorkspaceCrawl, rel_dir: &str) -> bool {
    let src_bin_rel = super::paths::join_under_root(rel_dir, "src/bin");
    super::paths::direct_child_files(crawl, &src_bin_rel)
        .iter()
        .any(|file| has_rs_extension(file))
        || super::paths::direct_child_dirs(crawl, &src_bin_rel)
            .iter()
            .any(|dir| {
                super::paths::file_exists(
                    crawl,
                    &super::paths::join_under_root(&src_bin_rel, &format!("{dir}/main.rs")),
                )
            })
}

/// True when `path`'s extension is `rs` (case-insensitive).
fn has_rs_extension(path: &str) -> bool {
    Path::new(path)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("rs"))
}

/// `binary_name_from_path` function.
fn binary_name_from_path(path: &str) -> Option<String> {
    let path = Path::new(path);

    if path.file_name().and_then(|name| name.to_str()) == Some("main.rs") {
        return path
            .parent()
            .and_then(Path::file_name)
            .and_then(|name| name.to_str())
            .filter(|name| !name.is_empty())
            .map(str::to_owned);
    }

    path.file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| !stem.is_empty())
        .map(str::to_owned)
}
