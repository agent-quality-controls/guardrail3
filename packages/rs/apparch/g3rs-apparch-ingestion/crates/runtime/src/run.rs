use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path};

use cargo_toml_parser::{CargoToml, Dependency};
use g3rs_apparch_ingestion_types::{
    G3RsApparchConfigChecksInput, G3RsApparchIngestionError, G3RsApparchSourceChecksInput,
};
use g3rs_apparch_types::{
    G3RsApparchConfigChecksInput as PublicConfigInput, G3RsApparchCrate,
    G3RsApparchDependencyEdge, G3RsApparchDependencyKind, G3RsApparchExternalDependency,
    G3RsApparchLayer, G3RsApparchPatchBypass, G3RsApparchPatchKind, G3RsApparchPublicItem,
    G3RsApparchPublicItemKind, G3RsApparchRustPolicyState,
    G3RsApparchSourceChecksInput as PublicSourceInput,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use glob::Pattern;
use guardrail3_rs_toml_parser::from_path as parse_guardrail3_rs_toml;

use crate::view::CrawlView;

pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsApparchConfigChecksInput, G3RsApparchIngestionError> {
    let view = CrawlView::new(crawl);
    let workspace = load_workspace_root(&view)?;
    let records = collect_workspace_crates(&view, &workspace)?;
    let dependencies = collect_dependency_collections(&records, &workspace.cargo);
    let patch_bypasses = collect_patch_bypasses(&view, &records, &workspace.cargo);

    Ok(PublicConfigInput {
        crates: records.iter().map(|record| record.krate.clone()).collect(),
        dependency_edges: dependencies.internal_edges,
        external_dependencies: dependencies.external_dependencies,
        patch_bypasses,
        rust_policy: workspace.rust_policy,
    })
}

pub fn ingest_for_source_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsApparchSourceChecksInput, G3RsApparchIngestionError> {
    let view = CrawlView::new(crawl);
    let workspace = load_workspace_root(&view)?;
    let records = collect_workspace_crates(&view, &workspace)?;

    let mut public_items = Vec::new();
    for record in &records {
        public_items.extend(collect_public_items_for_crate(&view, record)?);
    }

    Ok(PublicSourceInput {
        crates: records.iter().map(|record| record.krate.clone()).collect(),
        public_items,
    })
}

#[derive(Debug, Clone)]
struct WorkspaceRoot {
    cargo: CargoToml,
    rust_policy: G3RsApparchRustPolicyState,
}

#[derive(Debug, Clone)]
struct CrateRecord {
    krate: G3RsApparchCrate,
    cargo: CargoToml,
}

#[derive(Debug, Default)]
struct DependencyCollections {
    internal_edges: Vec<G3RsApparchDependencyEdge>,
    external_dependencies: Vec<G3RsApparchExternalDependency>,
}

fn load_workspace_root(view: &CrawlView<'_>) -> Result<WorkspaceRoot, G3RsApparchIngestionError> {
    let Some(entry) = view.entry("Cargo.toml") else {
        return Err(G3RsApparchIngestionError::CargoTomlNotFound);
    };
    if !entry.readable {
        return Err(G3RsApparchIngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let cargo = cargo_toml_parser::from_path(&entry.path.abs_path).map_err(|error| {
        G3RsApparchIngestionError::ParseFailed {
            path: entry.path.abs_path.clone(),
            reason: error.to_string(),
        }
    })?;

    if cargo.workspace.is_none() {
        return Err(G3RsApparchIngestionError::NormalizationFailed {
            path: entry.path.abs_path.clone(),
            reason: "root Cargo.toml must declare a [workspace] table".to_owned(),
        });
    }

    Ok(WorkspaceRoot {
        cargo,
        rust_policy: load_rust_policy(view),
    })
}

fn load_rust_policy(view: &CrawlView<'_>) -> G3RsApparchRustPolicyState {
    let Some(entry) = view.entry("guardrail3-rs.toml") else {
        return G3RsApparchRustPolicyState::Missing;
    };
    let rel_path = "guardrail3-rs.toml".to_owned();
    if !entry.readable {
        return G3RsApparchRustPolicyState::Unreadable {
            rel_path,
            reason: "file is not readable".to_owned(),
        };
    }

    match parse_guardrail3_rs_toml(&entry.path.abs_path) {
        Ok(parsed) => G3RsApparchRustPolicyState::Parsed {
            rel_path,
            profile: parsed.profile,
            allowed_deps: parsed.allowed_deps,
            waivers: parsed.waivers,
        },
        Err(error) => G3RsApparchRustPolicyState::ParseError {
            rel_path,
            reason: error.to_string(),
        },
    }
}

fn collect_workspace_crates(
    view: &CrawlView<'_>,
    workspace: &WorkspaceRoot,
) -> Result<Vec<CrateRecord>, G3RsApparchIngestionError> {
    let mut rel_dirs = resolve_member_dirs(view, &workspace.cargo)?;
    if workspace.cargo.package.is_some() {
        let _ = rel_dirs.insert(String::new());
    }

    let mut records = Vec::new();
    for rel_dir in rel_dirs {
        if rel_dir.contains("tests/fixtures/") {
            continue;
        }
        let cargo_rel_path = CrawlView::join_rel(&rel_dir, "Cargo.toml");
        let entry = view
            .entry(&cargo_rel_path)
            .ok_or_else(|| G3RsApparchIngestionError::NormalizationFailed {
                path: std::path::PathBuf::from(cargo_rel_path.clone()),
                reason: "workspace member pattern did not resolve to a Cargo.toml".to_owned(),
            })?;
        if !entry.readable {
            return Err(G3RsApparchIngestionError::Unreadable {
                path: entry.path.abs_path.clone(),
                reason: "file is not readable".to_owned(),
            });
        }
        let cargo = cargo_toml_parser::from_path(&entry.path.abs_path).map_err(|error| {
            G3RsApparchIngestionError::ParseFailed {
                path: entry.path.abs_path.clone(),
                reason: error.to_string(),
            }
        })?;
        let crate_name = cargo
            .package
            .as_ref()
            .and_then(|package| package.name.as_ref())
            .cloned()
            .unwrap_or_else(|| {
                if rel_dir.is_empty() {
                    "root".to_owned()
                } else {
                    rel_dir.rsplit('/').next().unwrap_or(&rel_dir).to_owned()
                }
            });

        records.push(CrateRecord {
            krate: G3RsApparchCrate {
                crate_name,
                cargo_rel_path,
                rel_dir: rel_dir.clone(),
                layer: layer_from_path(&rel_dir),
            },
            cargo,
        });
    }

    records.sort_by(|left, right| left.krate.cargo_rel_path.cmp(&right.krate.cargo_rel_path));
    Ok(records)
}

fn resolve_member_dirs(
    view: &CrawlView<'_>,
    cargo: &CargoToml,
) -> Result<BTreeSet<String>, G3RsApparchIngestionError> {
    let Some(workspace) = &cargo.workspace else {
        return Ok(BTreeSet::new());
    };

    let exclude_patterns = workspace
        .exclude
        .iter()
        .map(|pattern| {
            let pattern = normalize_member_pattern(pattern);
            Pattern::new(&pattern).map_err(|error| G3RsApparchIngestionError::NormalizationFailed {
                path: std::path::PathBuf::from("Cargo.toml"),
                reason: format!("invalid workspace exclude pattern `{pattern}`: {error}"),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut resolved = BTreeSet::new();
    for member in &workspace.members {
        for rel_dir in resolve_member_pattern(view, member)? {
            if !exclude_patterns.iter().any(|pattern| pattern.matches(&rel_dir)) {
                let _ = resolved.insert(rel_dir);
            }
        }
    }

    Ok(resolved)
}

fn resolve_member_pattern(
    view: &CrawlView<'_>,
    member: &str,
) -> Result<Vec<String>, G3RsApparchIngestionError> {
    let member = normalize_member_pattern(member);
    if member.is_empty() {
        return Ok(view
            .file_exists("Cargo.toml")
            .then_some(String::new())
            .into_iter()
            .collect());
    }

    let has_glob = member.contains('*') || member.contains('?') || member.contains('[');
    if has_glob {
        let glob = Pattern::new(&member).map_err(|error| G3RsApparchIngestionError::NormalizationFailed {
            path: std::path::PathBuf::from("Cargo.toml"),
            reason: format!("invalid workspace member pattern `{member}`: {error}"),
        })?;
        let matches = view
            .all_dir_rels()
            .filter(|rel_dir| glob.matches(rel_dir))
            .filter(|rel_dir| view.file_exists(&CrawlView::join_rel(rel_dir, "Cargo.toml")))
            .map(str::to_owned)
            .collect::<Vec<_>>();
        if matches.is_empty() {
            return Err(G3RsApparchIngestionError::NormalizationFailed {
                path: std::path::PathBuf::from("Cargo.toml"),
                reason: format!("workspace member pattern `{member}` did not resolve to any Cargo.toml"),
            });
        }
        return Ok(matches);
    }

    if view.file_exists(&CrawlView::join_rel(&member, "Cargo.toml")) {
        return Ok(vec![member]);
    }

    Err(G3RsApparchIngestionError::NormalizationFailed {
        path: std::path::PathBuf::from("Cargo.toml"),
        reason: format!("workspace member `{member}` did not resolve to a Cargo.toml"),
    })
}

fn normalize_member_pattern(member: &str) -> String {
    match member {
        "." | "./" => String::new(),
        _ => member.strip_prefix("./").unwrap_or(member).to_owned(),
    }
}

fn collect_dependency_collections(
    records: &[CrateRecord],
    root_cargo: &CargoToml,
) -> DependencyCollections {
    let crates_by_name = records
        .iter()
        .map(|record| (record.krate.crate_name.clone(), record.krate.cargo_rel_path.clone()))
        .collect::<BTreeMap<_, _>>();
    let workspace_dependencies = root_cargo
        .workspace
        .as_ref()
        .map(|workspace| &workspace.dependencies);

    let mut internal_edges = BTreeSet::new();
    let mut external_dependencies = BTreeSet::new();
    for record in records {
        collect_dependency_table(
            &record.krate.cargo_rel_path,
            &record.cargo.dependencies,
            workspace_dependencies,
            &crates_by_name,
            G3RsApparchDependencyKind::Dependency,
            &mut internal_edges,
            &mut external_dependencies,
        );
        collect_dependency_table(
            &record.krate.cargo_rel_path,
            &record.cargo.dev_dependencies,
            workspace_dependencies,
            &crates_by_name,
            G3RsApparchDependencyKind::DevDependency,
            &mut internal_edges,
            &mut external_dependencies,
        );
        collect_dependency_table(
            &record.krate.cargo_rel_path,
            &record.cargo.build_dependencies,
            workspace_dependencies,
            &crates_by_name,
            G3RsApparchDependencyKind::BuildDependency,
            &mut internal_edges,
            &mut external_dependencies,
        );
        for target in record.cargo.target.values() {
            collect_dependency_table(
                &record.krate.cargo_rel_path,
                &target.dependencies,
                workspace_dependencies,
                &crates_by_name,
                G3RsApparchDependencyKind::TargetDependency,
                &mut internal_edges,
                &mut external_dependencies,
            );
            collect_dependency_table(
                &record.krate.cargo_rel_path,
                &target.dev_dependencies,
                workspace_dependencies,
                &crates_by_name,
                G3RsApparchDependencyKind::TargetDevDependency,
                &mut internal_edges,
                &mut external_dependencies,
            );
            collect_dependency_table(
                &record.krate.cargo_rel_path,
                &target.build_dependencies,
                workspace_dependencies,
                &crates_by_name,
                G3RsApparchDependencyKind::TargetBuildDependency,
                &mut internal_edges,
                &mut external_dependencies,
            );
        }
    }

    DependencyCollections {
        internal_edges: internal_edges
            .into_iter()
            .map(|(from_cargo_rel_path, to_cargo_rel_path, dep_name, kind)| G3RsApparchDependencyEdge {
                from_cargo_rel_path,
                to_cargo_rel_path,
                dep_name,
                kind,
            })
            .collect(),
        external_dependencies: external_dependencies
            .into_iter()
            .map(|(cargo_rel_path, dep_name, kind)| G3RsApparchExternalDependency {
                cargo_rel_path,
                dep_name,
                kind,
            })
            .collect(),
    }
}

fn collect_dependency_table(
    from_cargo_rel_path: &str,
    dependencies: &BTreeMap<String, Dependency>,
    workspace_dependencies: Option<&BTreeMap<String, Dependency>>,
    crates_by_name: &BTreeMap<String, String>,
    kind: G3RsApparchDependencyKind,
    internal_edges: &mut BTreeSet<(String, String, String, G3RsApparchDependencyKind)>,
    external_dependencies: &mut BTreeSet<(String, String, G3RsApparchDependencyKind)>,
) {
    for (dep_name, dependency) in dependencies {
        let package_name = dependency_package(dep_name, dependency, workspace_dependencies);
        if let Some(to_cargo_rel_path) = crates_by_name.get(&package_name) {
            let _ = internal_edges.insert((
                from_cargo_rel_path.to_owned(),
                to_cargo_rel_path.clone(),
                package_name,
                kind,
            ));
        } else {
            let _ = external_dependencies.insert((
                from_cargo_rel_path.to_owned(),
                package_name,
                kind,
            ));
        }
    }
}

fn dependency_package(
    dep_name: &str,
    dependency: &Dependency,
    workspace_dependencies: Option<&BTreeMap<String, Dependency>>,
) -> String {
    match dependency {
        Dependency::Simple(_) => dep_name.to_owned(),
        Dependency::Detailed(detail) => {
            if detail.workspace == Some(true)
                && let Some(workspace_dep) = workspace_dependencies.and_then(|deps| deps.get(dep_name))
            {
                return dependency_package(dep_name, workspace_dep, None);
            }
            detail.package.clone().unwrap_or_else(|| dep_name.to_owned())
        }
    }
}

fn collect_patch_bypasses(
    view: &CrawlView<'_>,
    records: &[CrateRecord],
    root_cargo: &CargoToml,
) -> Vec<G3RsApparchPatchBypass> {
    let records_by_cargo_rel_path = records
        .iter()
        .map(|record| (record.krate.cargo_rel_path.clone(), &record.krate))
        .collect::<BTreeMap<_, _>>();
    let mut patch_bypasses = BTreeSet::new();

    for (registry, patch_table) in &root_cargo.patch {
        for (name, dependency) in patch_table {
            let Some(target_cargo_rel_path) = resolve_dependency_to_cargo_rel_path(view, dependency) else {
                continue;
            };
            let Some(target) = records_by_cargo_rel_path.get(&target_cargo_rel_path).copied() else {
                continue;
            };
            let _ = patch_bypasses.insert((
                format!("patch.{registry}.{name}"),
                G3RsApparchPatchKind::Patch,
                target.cargo_rel_path.clone(),
                target.rel_dir.clone(),
                target.layer,
            ));
        }
    }

    for (name, dependency) in &root_cargo.replace {
        let Some(target_cargo_rel_path) = resolve_dependency_to_cargo_rel_path(view, dependency) else {
            continue;
        };
        let Some(target) = records_by_cargo_rel_path.get(&target_cargo_rel_path).copied() else {
            continue;
        };
        let _ = patch_bypasses.insert((
            format!("replace.{name}"),
            G3RsApparchPatchKind::Replace,
            target.cargo_rel_path.clone(),
            target.rel_dir.clone(),
            target.layer,
        ));
    }

    patch_bypasses
        .into_iter()
        .map(
            |(key, kind, target_cargo_rel_path, target_rel_dir, target_layer)| G3RsApparchPatchBypass {
                cargo_rel_path: "Cargo.toml".to_owned(),
                key,
                kind,
                target_cargo_rel_path,
                target_rel_dir,
                target_layer,
            },
        )
        .collect()
}

fn resolve_dependency_to_cargo_rel_path(view: &CrawlView<'_>, dependency: &Dependency) -> Option<String> {
    let Dependency::Detailed(detail) = dependency else {
        return None;
    };
    let path = detail.path.as_deref()?;
    let normalized = normalize_relative_path(path);
    let direct = if normalized.is_empty() {
        "Cargo.toml".to_owned()
    } else {
        normalized.clone()
    };
    if view.file_exists(&direct) {
        return Some(direct);
    }

    let cargo_rel_path = CrawlView::join_rel(&normalized, "Cargo.toml");
    view.file_exists(&cargo_rel_path).then_some(cargo_rel_path)
}

fn normalize_relative_path(path: &str) -> String {
    let mut normalized = Vec::new();
    for component in Path::new(path).components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                let _ = normalized.pop();
            }
            Component::Normal(part) => normalized.push(part.to_string_lossy().into_owned()),
            Component::RootDir | Component::Prefix(_) => {
                return path.replace('\\', "/");
            }
        }
    }
    normalized.join("/")
}

fn collect_public_items_for_crate(
    view: &CrawlView<'_>,
    record: &CrateRecord,
) -> Result<Vec<G3RsApparchPublicItem>, G3RsApparchIngestionError> {
    let entrypoints = determine_entrypoints(view, record);
    let mut public_items = Vec::new();
    let mut visited = BTreeMap::new();
    for entrypoint in entrypoints {
        walk_module_file(
            view,
            &entrypoint,
            &record.krate.cargo_rel_path,
            &mut visited,
            true,
            &mut public_items,
        )?;
    }
    Ok(public_items)
}

fn determine_entrypoints(view: &CrawlView<'_>, record: &CrateRecord) -> Vec<String> {
    let mut entrypoints = BTreeSet::new();
    let rel_dir = &record.krate.rel_dir;

    if let Some(lib) = &record.cargo.lib
        && let Some(path) = &lib.path
    {
        let rel_path = CrawlView::join_rel(rel_dir, path);
        if view.file_exists(&rel_path) {
            let _ = entrypoints.insert(rel_path);
        }
    }

    for bin in &record.cargo.bin {
        if let Some(path) = &bin.path {
            let rel_path = CrawlView::join_rel(rel_dir, path);
            if view.file_exists(&rel_path) {
                let _ = entrypoints.insert(rel_path);
            }
        }
    }

    for rel_path in [
        CrawlView::join_rel(rel_dir, "src/lib.rs"),
        CrawlView::join_rel(rel_dir, "src/main.rs"),
    ] {
        if view.file_exists(&rel_path) {
            let _ = entrypoints.insert(rel_path);
        }
    }

    entrypoints.into_iter().collect()
}

fn walk_module_file(
    view: &CrawlView<'_>,
    rel_path: &str,
    cargo_rel_path: &str,
    visited: &mut BTreeMap<String, bool>,
    public_module: bool,
    public_items: &mut Vec<G3RsApparchPublicItem>,
) -> Result<(), G3RsApparchIngestionError> {
    match visited.get(rel_path).copied() {
        Some(true) => return Ok(()),
        Some(false) if !public_module => return Ok(()),
        _ => {
            let _ = visited.insert(rel_path.to_owned(), public_module);
        }
    }
    let Some(entry) = view.entry(rel_path) else {
        return Err(G3RsApparchIngestionError::NormalizationFailed {
            path: std::path::PathBuf::from(rel_path),
            reason: "source path not present in crawl".to_owned(),
        });
    };
    if !entry.readable {
        return Err(G3RsApparchIngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }
    let content = view
        .read_file(rel_path)
        .map_err(|error| G3RsApparchIngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: error.to_string(),
        })?;
    let parsed = syn::parse_file(&content).map_err(|error| G3RsApparchIngestionError::ParseFailed {
        path: entry.path.abs_path.clone(),
        reason: error.to_string(),
    })?;
    if is_cfg_test_only(&parsed.attrs) {
        return Ok(());
    }
    walk_items(
        view,
        rel_path,
        cargo_rel_path,
        &parsed.items,
        visited,
        public_module,
        public_items,
    )
}

fn walk_items(
    view: &CrawlView<'_>,
    rel_path: &str,
    cargo_rel_path: &str,
    items: &[syn::Item],
    visited: &mut BTreeMap<String, bool>,
    public_module: bool,
    public_items: &mut Vec<G3RsApparchPublicItem>,
) -> Result<(), G3RsApparchIngestionError> {
    for item in items {
        if is_cfg_test_only(item.attrs()) {
            continue;
        }
        match item {
            syn::Item::Trait(item_trait) => {
                if public_module && matches!(item_trait.vis, syn::Visibility::Public(_)) {
                    public_items.push(G3RsApparchPublicItem {
                        cargo_rel_path: cargo_rel_path.to_owned(),
                        rel_path: rel_path.to_owned(),
                        item_name: item_trait.ident.to_string(),
                        owner_name: None,
                        kind: G3RsApparchPublicItemKind::Trait,
                    });
                }
            }
            syn::Item::Fn(item_fn) => {
                if public_module && matches!(item_fn.vis, syn::Visibility::Public(_)) {
                    public_items.push(G3RsApparchPublicItem {
                        cargo_rel_path: cargo_rel_path.to_owned(),
                        rel_path: rel_path.to_owned(),
                        item_name: item_fn.sig.ident.to_string(),
                        owner_name: None,
                        kind: G3RsApparchPublicItemKind::FreeFunction,
                    });
                }
            }
            syn::Item::Impl(item_impl) => {
                if !public_module || item_impl.trait_.is_some() {
                    continue;
                }
                let owner_name = self_type_name(item_impl.self_ty.as_ref());
                for impl_item in &item_impl.items {
                    let syn::ImplItem::Fn(method) = impl_item else {
                        continue;
                    };
                    if !matches!(method.vis, syn::Visibility::Public(_)) {
                        continue;
                    }
                    public_items.push(G3RsApparchPublicItem {
                        cargo_rel_path: cargo_rel_path.to_owned(),
                        rel_path: rel_path.to_owned(),
                        item_name: method.sig.ident.to_string(),
                        owner_name: owner_name.clone(),
                        kind: G3RsApparchPublicItemKind::InherentMethod,
                    });
                }
            }
            syn::Item::Mod(item_mod) => {
                let child_public = public_module;
                if let Some((_, nested_items)) = &item_mod.content {
                    walk_items(
                        view,
                        rel_path,
                        cargo_rel_path,
                        nested_items,
                        visited,
                        child_public,
                        public_items,
                    )?;
                } else {
                    let module_path = resolve_module_path(view, rel_path, item_mod)?;
                    walk_module_file(
                        view,
                        &module_path,
                        cargo_rel_path,
                        visited,
                        child_public,
                        public_items,
                    )?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn self_type_name(self_ty: &syn::Type) -> Option<String> {
    match self_ty {
        syn::Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string()),
        _ => None,
    }
}

fn resolve_module_path(
    view: &CrawlView<'_>,
    rel_path: &str,
    item_mod: &syn::ItemMod,
) -> Result<String, G3RsApparchIngestionError> {
    let file_dir = rel_path.rsplit_once('/').map_or("", |(dir, _)| dir);
    if let Some(path_attr) = module_path_attr(item_mod) {
        let candidate = CrawlView::join_rel(file_dir, &path_attr);
        return view.file_exists(&candidate).then_some(candidate).ok_or_else(|| {
            G3RsApparchIngestionError::NormalizationFailed {
                path: std::path::PathBuf::from(rel_path),
                reason: format!(
                    "declared module `{}` points to missing file `{}`",
                    item_mod.ident, path_attr
                ),
            }
        });
    }

    let module_name = item_mod.ident.to_string();
    let file_name = rel_path.rsplit('/').next().unwrap_or(rel_path);
    let stem = file_name.strip_suffix(".rs").unwrap_or(file_name);
    let module_dir = if matches!(stem, "lib" | "main" | "mod") {
        file_dir.to_owned()
    } else {
        CrawlView::join_rel(file_dir, stem)
    };

    let direct = CrawlView::join_rel(&module_dir, &format!("{module_name}.rs"));
    if view.file_exists(&direct) {
        return Ok(direct);
    }

    let nested = CrawlView::join_rel(&module_dir, &format!("{module_name}/mod.rs"));
    if view.file_exists(&nested) {
        return Ok(nested);
    }

    Err(G3RsApparchIngestionError::NormalizationFailed {
        path: std::path::PathBuf::from(rel_path),
        reason: format!(
            "declared module `{}` has no backing file under `{}`",
            item_mod.ident, file_dir
        ),
    })
}

fn module_path_attr(item_mod: &syn::ItemMod) -> Option<String> {
    item_mod.attrs.iter().find_map(|attr| {
        if !attr.path().is_ident("path") {
            return None;
        }
        match &attr.meta {
            syn::Meta::NameValue(name_value) => match &name_value.value {
                syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                    syn::Lit::Str(value) => Some(value.value()),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        }
    })
}

fn is_cfg_test_only(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("cfg") {
            return false;
        }
        let Ok(meta) = attr.parse_args::<syn::Meta>() else {
            return false;
        };
        cfg_meta_is_test_only(&meta)
    })
}

fn cfg_meta_is_test_only(meta: &syn::Meta) -> bool {
    match meta {
        syn::Meta::Path(path) => path.is_ident("test"),
        syn::Meta::List(list) => {
            let Some(ident) = list.path.get_ident().map(|ident| ident.to_string()) else {
                return false;
            };
            let nested = list
                .parse_args_with(
                    syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                )
                .ok();
            match (ident.as_str(), nested) {
                ("all", Some(items)) | ("any", Some(items)) => {
                    !items.is_empty() && items.iter().all(cfg_meta_is_test_only)
                }
                _ => false,
            }
        }
        syn::Meta::NameValue(_) => false,
    }
}

trait ItemAttrs {
    fn attrs(&self) -> &[syn::Attribute];
}

impl ItemAttrs for syn::Item {
    fn attrs(&self) -> &[syn::Attribute] {
        match self {
            syn::Item::Const(item) => &item.attrs,
            syn::Item::Enum(item) => &item.attrs,
            syn::Item::ExternCrate(item) => &item.attrs,
            syn::Item::Fn(item) => &item.attrs,
            syn::Item::ForeignMod(item) => &item.attrs,
            syn::Item::Impl(item) => &item.attrs,
            syn::Item::Macro(item) => &item.attrs,
            syn::Item::Mod(item) => &item.attrs,
            syn::Item::Static(item) => &item.attrs,
            syn::Item::Struct(item) => &item.attrs,
            syn::Item::Trait(item) => &item.attrs,
            syn::Item::TraitAlias(item) => &item.attrs,
            syn::Item::Type(item) => &item.attrs,
            syn::Item::Union(item) => &item.attrs,
            syn::Item::Use(item) => &item.attrs,
            _ => &[],
        }
    }
}

fn layer_from_path(path: &str) -> Option<G3RsApparchLayer> {
    if contains_segment(path, "types") {
        Some(G3RsApparchLayer::Types)
    } else if contains_segment(path, "logic") {
        Some(G3RsApparchLayer::Logic)
    } else if contains_segment_pair(path, "io", "inbound") {
        Some(G3RsApparchLayer::IoInbound)
    } else if contains_segment_pair(path, "io", "outbound") {
        Some(G3RsApparchLayer::IoOutbound)
    } else {
        None
    }
}

fn contains_segment(path: &str, segment: &str) -> bool {
    path.split('/').any(|part| part == segment)
}

fn contains_segment_pair(path: &str, first: &str, second: &str) -> bool {
    let parts = path.split('/').collect::<Vec<_>>();
    parts.windows(2).any(|window| window[0] == first && window[1] == second)
}
