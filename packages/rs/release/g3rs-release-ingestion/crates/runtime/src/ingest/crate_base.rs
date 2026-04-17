use std::collections::BTreeSet;
use std::path::Path;

use cargo_toml_parser::types::{
    CargoToml, InheritableValue, PackageSection, StringOrBool, WorkspacePackageSection,
};
use semver::Version;

use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

use super::collect::{CrateBase, CrateReadmeFacts, CrateReleaseFacts, ParsedCrate};

pub(super) fn build_crate_base(
    crawl: &G3RsWorkspaceCrawl,
    krate: &ParsedCrate,
    workspace_package: Option<&WorkspacePackageSection>,
) -> CrateBase {
    let package = krate.cargo.package.as_ref();
    let name = package
        .and_then(|package| package.name.clone())
        .unwrap_or_else(|| krate.cargo_rel_path.clone());
    let publish_declared = publish_declared(package);
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

    let version_string = version_string(package, workspace_package);

    CrateBase {
        name,
        cargo_rel_path: krate.cargo_rel_path.clone(),
        cargo_abs_path: krate.cargo_abs_path.clone(),
        cargo: krate.cargo.clone(),
        publish_declared,
        publishable,
        is_binary,
        is_library: is_library_crate(crawl, &krate.rel_dir, &krate.cargo),
        binary_target_names,
        readme: CrateReadmeFacts {
            declared_false: readme_declared_false,
            rel_path: readme_rel_path.clone(),
            abs_path: readme_abs_path,
            exists: !readme_declared_false
                && super::paths::file_exists(crawl, &readme_rel_path),
        },
        release: CrateReleaseFacts {
            description_present: inherited_string_present(
                package.and_then(|package| package.description.as_ref()),
                workspace_package.and_then(|workspace| workspace.description.as_deref()),
            ),
            license_present: inherited_string_present(
                package.and_then(|package| package.license.as_ref()),
                workspace_package.and_then(|workspace| workspace.license.as_deref()),
            ) || inherited_string_present(
                package.and_then(|package| package.license_file.as_ref()),
                workspace_package.and_then(|workspace| workspace.license_file.as_deref()),
            ),
            repository_present: inherited_string_present(
                package.and_then(|package| package.repository.as_ref()),
                workspace_package.and_then(|workspace| workspace.repository.as_deref()),
            ),
            keywords_count: inherited_vec_count(
                package.and_then(|package| package.keywords.as_ref()),
                workspace_package.map(|workspace| workspace.keywords.as_slice()),
            ),
            categories_count: inherited_vec_count(
                package.and_then(|package| package.categories.as_ref()),
                workspace_package.map(|workspace| workspace.categories.as_slice()),
            ),
            version_string: version_string.clone(),
            workspace_version: matches!(
                package.and_then(|package| package.version.as_ref()),
                Some(InheritableValue::Inherit(_))
            ),
            version_valid: version_string
                .as_deref()
                .is_some_and(|version| Version::parse(version).is_ok()),
            docs_rs_present: package
                .and_then(|package| package.metadata.as_ref())
                .and_then(docs_rs_table)
                .is_some_and(has_supported_docs_rs_settings),
            include_exclude_present: package.is_some_and(|package| {
                package.include.as_ref().is_some_and(non_empty_values)
                    || package.exclude.as_ref().is_some_and(non_empty_values)
            }),
            has_binstall_metadata: package
                .and_then(|package| package.metadata.as_ref())
                .and_then(|metadata: &toml::Value| metadata.get("binstall"))
                .and_then(|value: &toml::Value| value.as_table())
                .is_some(),
        },
    }
}

fn non_empty_values(value: &InheritableValue<Vec<String>>) -> bool {
    matches!(value, InheritableValue::Value(values) if !values.is_empty())
}

fn publishable(
    package: Option<&PackageSection>,
    workspace_package: Option<&WorkspacePackageSection>,
) -> bool {
    let Some(package) = package else {
        return false;
    };

    match package.publish.as_ref() {
        None => false,
        Some(InheritableValue::Value(cargo_toml_parser::types::VecStringOrBool::Bool(false))) => {
            false
        }
        Some(InheritableValue::Value(
            cargo_toml_parser::types::VecStringOrBool::VecString(values),
        )) => !values.is_empty(),
        Some(InheritableValue::Value(cargo_toml_parser::types::VecStringOrBool::Bool(true))) => {
            true
        }
        Some(InheritableValue::Inherit(_)) => {
            match workspace_package.and_then(|workspace| workspace.publish.as_ref()) {
                None => false,
                Some(cargo_toml_parser::types::VecStringOrBool::Bool(false)) => false,
                Some(cargo_toml_parser::types::VecStringOrBool::VecString(values)) => {
                    !values.is_empty()
                }
                Some(cargo_toml_parser::types::VecStringOrBool::Bool(true)) => true,
            }
        }
    }
}

fn publish_declared(package: Option<&PackageSection>) -> bool {
    package
        .and_then(|package| package.publish.as_ref())
        .is_some()
}

fn inherited_string_present(
    value: Option<&InheritableValue<String>>,
    workspace_value: Option<&str>,
) -> bool {
    match value {
        Some(InheritableValue::Value(value)) => !value.trim().is_empty(),
        Some(InheritableValue::Inherit(_)) => {
            workspace_value.is_some_and(|value| !value.trim().is_empty())
        }
        None => false,
    }
}

fn inherited_vec_count(
    value: Option<&InheritableValue<Vec<String>>>,
    workspace_values: Option<&[String]>,
) -> Option<usize> {
    match value {
        Some(InheritableValue::Value(values)) => Some(values.len()),
        Some(InheritableValue::Inherit(_)) => workspace_values.map(|values| values.len()),
        None => None,
    }
}

fn version_string(
    package: Option<&PackageSection>,
    workspace_package: Option<&WorkspacePackageSection>,
) -> Option<String> {
    match package.and_then(|package| package.version.as_ref()) {
        Some(InheritableValue::Value(value)) => Some(value.clone()),
        Some(InheritableValue::Inherit(_)) => {
            workspace_package.and_then(|workspace| workspace.version.clone())
        }
        None => None,
    }
}

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
        _ => false,
    }
}

fn readme_path_field<'a>(
    package: Option<&'a PackageSection>,
    workspace_package: Option<&'a WorkspacePackageSection>,
) -> (Option<&'a str>, bool) {
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
        _ => (None, false),
    }
}

fn docs_rs_table(metadata: &toml::Value) -> Option<&toml::map::Map<String, toml::Value>> {
    metadata
        .get("docs.rs")
        .and_then(|value| value.as_table())
        .or_else(|| {
            metadata
                .get("docs")
                .and_then(|docs| docs.as_table())
                .and_then(|docs| docs.get("rs"))
                .and_then(|value| value.as_table())
        })
}

fn has_supported_docs_rs_settings(table: &toml::map::Map<String, toml::Value>) -> bool {
    [
        "all-features",
        "features",
        "no-default-features",
        "default-target",
        "targets",
        "rustdoc-args",
        "cargo-args",
    ]
    .iter()
    .any(|key| table.contains_key(*key))
}

fn is_library_crate(crawl: &G3RsWorkspaceCrawl, rel_dir: &str, cargo: &CargoToml) -> bool {
    cargo.lib.is_some()
        || super::paths::file_exists(crawl, &super::paths::join_under_root(rel_dir, "src/lib.rs"))
}

fn is_binary_crate(crawl: &G3RsWorkspaceCrawl, rel_dir: &str, cargo: &CargoToml) -> bool {
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

fn binary_target_names(
    crawl: &G3RsWorkspaceCrawl,
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

    if super::paths::file_exists(crawl, &super::paths::join_under_root(rel_dir, "src/main.rs"))
        && let Some(package_name) = cargo
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

fn autodiscovered_bin_exists(crawl: &G3RsWorkspaceCrawl, rel_dir: &str) -> bool {
    let src_bin_rel = super::paths::join_under_root(rel_dir, "src/bin");
    super::paths::direct_child_files(crawl, &src_bin_rel)
        .iter()
        .any(|file| file.ends_with(".rs"))
        || super::paths::direct_child_dirs(crawl, &src_bin_rel)
            .iter()
            .any(|dir| {
                super::paths::file_exists(
                    crawl,
                    &super::paths::join_under_root(&src_bin_rel, &format!("{dir}/main.rs")),
                )
            })
}

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
