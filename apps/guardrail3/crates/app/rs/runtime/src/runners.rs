#[cfg(any(feature = "family-code", feature = "family-garde"))]
use guardrail3_app_rs_family_mapper::RsScopedRootView;
#[cfg(any(
    feature = "family-arch",
    feature = "family-topology",
    feature = "family-fmt",
    feature = "family-toolchain",
    feature = "family-clippy",
    feature = "family-deny",
    feature = "family-cargo",
    feature = "family-libarch",
    feature = "family-deps",
    feature = "family-release",
    feature = "family-garde",
    feature = "family-hexarch",
))]
use guardrail3_app_rs_family_mapper::RsFamilyFileView;
#[cfg(any(
    feature = "family-arch",
    feature = "family-toolchain",
    feature = "family-clippy",
    feature = "family-deny",
    feature = "family-cargo",
    feature = "family-libarch",
    feature = "family-deps",
    feature = "family-release",
    feature = "family-test",
    feature = "family-hexarch",
))]
use guardrail3_app_rs_family_mapper::RsRootView;
#[cfg(any(
    feature = "family-arch",
    feature = "family-topology",
    feature = "family-toolchain",
    feature = "family-clippy",
    feature = "family-deny",
    feature = "family-cargo",
    feature = "family-libarch",
    feature = "family-deps",
    feature = "family-release",
    feature = "family-garde",
    feature = "family-hexarch",
    feature = "family-code",
    feature = "family-hooks-shared",
    feature = "family-hooks-rs",
    feature = "family-fmt",
    feature = "family-test",
))]
use guardrail3_app_rs_family_view::FamilyView;
use guardrail3_domain_report::CheckResult;
use guardrail3_validation_model::RustValidateFamily;

use crate::context::RustRunContext;

pub(crate) type RunnerFn = for<'a> fn(&RustRunContext<'a>) -> Vec<CheckResult>;

pub(crate) struct RustFamilyRunnerDef {
    pub(crate) family: RustValidateFamily,
    pub(crate) run: RunnerFn,
}

#[cfg(feature = "family-topology")]
fn run_topology(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let route = ctx.mapper.map_rs_topology();
    let root_rels = topology_root_rels(route.roots());
    let mut extra = route
        .roots()
        .iter()
        .map(|root| root.root().cargo_rel_path().to_owned())
        .collect::<Vec<_>>();
    extra.extend(family_file_rels(route.family_files()));
    if ctx.legality
        .dir_structure()
        .get("")
        .is_some_and(|e| e.has_file("guardrail3.toml"))
    {
        extra.push("guardrail3.toml".to_owned());
    }
    let view = FamilyView::build(
        ctx.legality.root_path().clone(),
        ctx.legality.dir_structure(),
        ctx.legality.content(),
        &root_rels,
        &extra,
        &[],
        None,
    );
    guardrail3_app_rs_family_topology::check(&view, &route)
}

#[cfg(feature = "family-arch")]
fn run_arch(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let route = ctx.mapper.map_rs_arch();
    let root_rels = route_root_rels(route.roots());
    let mut extra = route_root_cargo_files(route.roots());
    extra.extend(family_file_rels(route.family_files()));
    if ctx.legality
        .dir_structure()
        .get("")
        .is_some_and(|e| e.has_file("guardrail3.toml"))
    {
        extra.push("guardrail3.toml".to_owned());
    }
    let view = FamilyView::build(
        ctx.legality.root_path().clone(),
        ctx.legality.dir_structure(),
        ctx.legality.content(),
        &root_rels,
        &extra,
        &[],
        None,
    );
    guardrail3_app_rs_family_arch::check(&view, &route)
}

#[cfg(feature = "family-fmt")]
fn run_fmt(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let route = ctx.mapper.map_rs_fmt();
    let mut extra = family_file_rels(route.family_files());
    extra.push("Cargo.toml".to_owned());
    extra.push("rust-toolchain.toml".to_owned());
    extra.push("guardrail3.toml".to_owned());
    let view = FamilyView::build(
        ctx.legality.root_path().clone(),
        ctx.legality.dir_structure(),
        ctx.legality.content(),
        &[],
        &extra,
        &[],
        None,
    );
    guardrail3_app_rs_family_fmt::check(&view, &route)
}

#[cfg(feature = "family-toolchain")]
fn run_toolchain(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let route = ctx.mapper.map_rs_toolchain();
    route
        .roots()
        .iter()
        .flat_map(|root| {
            let workspace_route = route.for_workspace(root.rel_dir());
            let root_rels = vec![root.rel_dir().to_owned()];
            let mut extra = route_root_cargo_files(workspace_route.roots());
            extra.extend(family_file_rels(workspace_route.family_files()));
            let view = FamilyView::build(
                ctx.legality.root_path().clone(),
                ctx.legality.dir_structure(),
                ctx.legality.content(),
                &root_rels,
                &extra,
                &[],
                None,
            );
            guardrail3_app_rs_family_toolchain::check(&view, &workspace_route)
        })
        .collect()
}

#[cfg(feature = "family-clippy")]
fn run_clippy(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let route = ctx.mapper.map_rs_clippy();
    route
        .roots()
        .iter()
        .flat_map(|root| {
            let workspace_route = route.for_workspace(root.rel_dir());
            let root_rels = vec![root.rel_dir().to_owned()];
            let mut extra = route_root_cargo_files(workspace_route.roots());
            extra.extend(family_file_rels(workspace_route.family_files()));
            let view = FamilyView::build(
                ctx.legality.root_path().clone(),
                ctx.legality.dir_structure(),
                ctx.legality.content(),
                &root_rels,
                &extra,
                &[],
                None,
            );
            guardrail3_app_rs_family_clippy::check(&view, &workspace_route)
        })
        .collect()
}

#[cfg(feature = "family-deny")]
fn run_deny(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let route = ctx.mapper.map_rs_deny();
    route
        .roots()
        .iter()
        .flat_map(|root| {
            let workspace_route = route.for_workspace(root.rel_dir());
            let root_rels = vec![root.rel_dir().to_owned()];
            let mut extra = route_root_cargo_files(workspace_route.roots());
            extra.extend(family_file_rels(workspace_route.family_files()));
            let view = FamilyView::build(
                ctx.legality.root_path().clone(),
                ctx.legality.dir_structure(),
                ctx.legality.content(),
                &root_rels,
                &extra,
                &[],
                None,
            );
            guardrail3_app_rs_family_deny::check(&view, &workspace_route)
        })
        .collect()
}

#[cfg(feature = "family-cargo")]
fn run_cargo(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let route = ctx.mapper.map_rs_cargo();
    route
        .roots()
        .iter()
        .flat_map(|root| {
            let workspace_route = route.for_workspace(root.rel_dir());
            let root_rels = vec![root.rel_dir().to_owned()];
            let mut extra = route_root_cargo_files(workspace_route.roots());
            extra.extend(family_file_rels(workspace_route.family_files()));
            let view = FamilyView::build(
                ctx.legality.root_path().clone(),
                ctx.legality.dir_structure(),
                ctx.legality.content(),
                &root_rels,
                &extra,
                &[],
                None,
            );
            guardrail3_app_rs_family_cargo::check(&view, &workspace_route)
        })
        .collect()
}

#[cfg(feature = "family-code")]
fn run_code(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let route = ctx.mapper.map_rs_code();
    let root_rels = scoped_route_root_rels(route.roots());
    let extra = code_extra_file_rels(ctx.legality.dir_structure(), route.roots());
    let view = FamilyView::build(
        ctx.legality.root_path().clone(),
        ctx.legality.dir_structure(),
        ctx.legality.content(),
        &root_rels,
        &extra,
        &[],
        route.scoped_files(),
    );
    guardrail3_app_rs_family_code::check(&view, &route)
}

#[cfg(feature = "family-hexarch")]
fn run_hexarch(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let route = ctx.mapper.map_rs_hexarch();
    route
        .roots()
        .iter()
        .flat_map(|root| {
            let workspace_route = route.for_workspace(root.rel_dir());
            let root_rels = vec![root.rel_dir().to_owned()];
            let mut extra = route_root_cargo_files(workspace_route.roots());
            if let Some(repo_root_cargo) = workspace_route.repo_root_cargo_rel_path() {
                extra.push(repo_root_cargo.to_owned());
            }
            if let Some(guardrail_rel) = workspace_route.guardrail_config_rel_path() {
                extra.push(guardrail_rel.to_owned());
            }
            let view = FamilyView::build(
                ctx.legality.root_path().clone(),
                ctx.legality.dir_structure(),
                ctx.legality.content(),
                &root_rels,
                &extra,
                &[],
                None,
            );
            guardrail3_app_rs_family_hexarch::check(&view, &workspace_route)
        })
        .collect()
}

#[cfg(feature = "family-libarch")]
fn run_libarch(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let route = ctx.mapper.map_rs_libarch();
    route
        .roots()
        .iter()
        .flat_map(|root| {
            let workspace_route = route.for_workspace(root.rel_dir());
            let root_rels = vec![root.rel_dir().to_owned()];
            let mut extra = route_root_cargo_files(workspace_route.roots());
            extra.extend(family_file_rels(workspace_route.family_files()));
            let view = FamilyView::build(
                ctx.legality.root_path().clone(),
                ctx.legality.dir_structure(),
                ctx.legality.content(),
                &root_rels,
                &extra,
                &[],
                None,
            );
            guardrail3_app_rs_family_libarch::check(&view, &workspace_route)
        })
        .collect()
}

#[cfg(feature = "family-deps")]
fn run_deps(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let route = ctx.mapper.map_rs_deps();
    route
        .roots()
        .iter()
        .flat_map(|root| {
            let workspace_route = route.for_workspace(root.rel_dir());
            let root_rels = vec![root.rel_dir().to_owned()];
            let mut extra = route_root_cargo_files(workspace_route.roots());
            extra.extend(family_file_rels(workspace_route.family_files()));
            let view = FamilyView::build(
                ctx.legality.root_path().clone(),
                ctx.legality.dir_structure(),
                ctx.legality.content(),
                &root_rels,
                &extra,
                &[],
                None,
            );
            guardrail3_app_rs_family_deps::check(&view, &workspace_route, ctx.tc)
        })
        .collect()
}

#[cfg(feature = "family-garde")]
fn run_garde(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let route = ctx.mapper.map_rs_garde();
    route
        .roots()
        .iter()
        .flat_map(|root| {
            let workspace_route = route.for_workspace(root.root().rel_dir());
            let root_rels = scoped_route_root_rels(workspace_route.roots());
            let mut extra = scoped_route_root_cargo_files(workspace_route.roots());
            extra.extend(family_file_rels(workspace_route.family_files()));
            let view = FamilyView::build(
                ctx.legality.root_path().clone(),
                ctx.legality.dir_structure(),
                ctx.legality.content(),
                &root_rels,
                &extra,
                &[],
                None,
            );
            guardrail3_app_rs_family_garde::check(&view, &workspace_route)
        })
        .collect()
}

#[cfg(feature = "family-test")]
fn run_test(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let route = ctx.mapper.map_rs_test();
    let root_rels = route_root_rels(route.roots());
    let extra = route_root_cargo_files(route.roots());
    let view = FamilyView::build(
        ctx.legality.root_path().clone(),
        ctx.legality.dir_structure(),
        ctx.legality.content(),
        &root_rels,
        &extra,
        &[],
        route.scoped_files(),
    );
    guardrail3_app_rs_family_test::check(&view, &route, ctx.tc)
}

#[cfg(feature = "family-release")]
fn run_release(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let route = ctx.mapper.map_rs_release();
    route
        .roots()
        .iter()
        .flat_map(|root| {
            let workspace_route = route.for_workspace(root.rel_dir());
            let root_rels = vec![root.rel_dir().to_owned()];
            let mut extra = route_root_cargo_files(workspace_route.roots());
            extra.extend(family_file_rels(workspace_route.family_files()));
            let view = FamilyView::build(
                ctx.legality.root_path().clone(),
                ctx.legality.dir_structure(),
                ctx.legality.content(),
                &root_rels,
                &extra,
                &[],
                None,
            );
            guardrail3_app_rs_family_release::check(
                &view,
                &workspace_route,
                ctx.tc,
                ctx.thorough,
            )
        })
        .collect()
}

#[cfg(feature = "family-hooks-shared")]
fn run_hooks_shared(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let dir_structure = ctx.legality.dir_structure();
    let extra_files = hook_file_rels(dir_structure);
    let extra_dirs = hook_dir_rels(dir_structure);
    let view = FamilyView::build(
        ctx.legality.root_path().clone(),
        dir_structure,
        ctx.legality.content(),
        &[],
        &extra_files,
        &extra_dirs,
        None,
    );
    guardrail3_app_rs_family_hooks_shared::check(ctx.fs, ctx.path, &view, ctx.tc)
}

#[cfg(feature = "family-hooks-rs")]
fn run_hooks_rs(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let dir_structure = ctx.legality.dir_structure();
    let extra_files = hook_file_rels(dir_structure);
    let extra_dirs = hook_dir_rels(dir_structure);
    let view = FamilyView::build(
        ctx.legality.root_path().clone(),
        dir_structure,
        ctx.legality.content(),
        &[],
        &extra_files,
        &extra_dirs,
        None,
    );
    guardrail3_app_rs_family_hooks_rs::check(&view, ctx.tc)
}

// ---------------------------------------------------------------------------
// Route data helpers — extract data from routes, no tree access.
// ---------------------------------------------------------------------------

#[cfg(any(
    feature = "family-arch",
    feature = "family-toolchain",
    feature = "family-clippy",
    feature = "family-deny",
    feature = "family-cargo",
    feature = "family-libarch",
    feature = "family-deps",
    feature = "family-release",
    feature = "family-test",
    feature = "family-hexarch",
))]
fn route_root_rels(roots: &[RsRootView]) -> Vec<String> {
    roots.iter().map(|root| root.rel_dir().to_owned()).collect()
}

#[cfg(any(
    feature = "family-arch",
    feature = "family-toolchain",
    feature = "family-clippy",
    feature = "family-deny",
    feature = "family-cargo",
    feature = "family-libarch",
    feature = "family-deps",
    feature = "family-release",
    feature = "family-test",
    feature = "family-hexarch",
))]
fn route_root_cargo_files(roots: &[RsRootView]) -> Vec<String> {
    roots
        .iter()
        .map(|root| root.cargo_rel_path().to_owned())
        .collect()
}

#[cfg(any(feature = "family-code", feature = "family-garde"))]
fn scoped_route_root_rels(roots: &[RsScopedRootView]) -> Vec<String> {
    roots
        .iter()
        .map(|root| root.root().rel_dir().to_owned())
        .collect()
}

#[cfg(feature = "family-topology")]
fn topology_root_rels(roots: &[guardrail3_app_rs_family_mapper::RsTopologyRootView]) -> Vec<String> {
    roots
        .iter()
        .map(|root| root.root().rel_dir().to_owned())
        .collect()
}

#[cfg(feature = "family-garde")]
fn scoped_route_root_cargo_files(roots: &[RsScopedRootView]) -> Vec<String> {
    roots
        .iter()
        .map(|root| root.root().cargo_rel_path().to_owned())
        .collect()
}

#[cfg(any(
    feature = "family-arch",
    feature = "family-fmt",
    feature = "family-topology",
    feature = "family-toolchain",
    feature = "family-clippy",
    feature = "family-deny",
    feature = "family-cargo",
    feature = "family-libarch",
    feature = "family-deps",
    feature = "family-release",
    feature = "family-garde",
    feature = "family-hexarch",
))]
fn family_file_rels(family_files: &[RsFamilyFileView]) -> Vec<String> {
    family_files
        .iter()
        .map(|file| file.rel_path().to_owned())
        .collect()
}

// ---------------------------------------------------------------------------
// Code family: collect all .rs files + config files from dir_structure.
// ---------------------------------------------------------------------------

#[cfg(feature = "family-code")]
fn code_extra_file_rels(
    dir_structure: &std::collections::BTreeMap<String, guardrail3_app_rs_family_view::DirEntry>,
    roots: &[RsScopedRootView],
) -> Vec<String> {
    let mut rels = roots
        .iter()
        .map(|root| root.root().cargo_rel_path().to_owned())
        .collect::<std::collections::BTreeSet<_>>();

    for (dir_rel, entry) in dir_structure {
        for file_name in entry.files() {
            let include = file_name.ends_with(".rs")
                || matches!(
                    file_name.as_str(),
                    "guardrail3.toml"
                        | "clippy.toml"
                        | ".clippy.toml"
                        | "deny.toml"
                        | ".deny.toml"
                        | "Cargo.toml"
                        | "rustfmt.toml"
                        | ".rustfmt.toml"
                        | "rust-toolchain.toml"
                        | "rust-toolchain"
                );
            if include {
                let rel_path = if dir_rel.is_empty() {
                    file_name.to_owned()
                } else {
                    format!("{dir_rel}/{file_name}")
                };
                let _ = rels.insert(rel_path);
            }
        }
    }

    rels.into_iter().collect()
}

// ---------------------------------------------------------------------------
// Hooks helpers: check dir_structure for hook paths.
// ---------------------------------------------------------------------------

#[cfg(any(feature = "family-hooks-shared", feature = "family-hooks-rs"))]
fn hook_file_rels(
    dir_structure: &std::collections::BTreeMap<String, guardrail3_app_rs_family_view::DirEntry>,
) -> Vec<String> {
    let mut rels = [
        ".githooks/pre-commit",
        "hooks/pre-commit",
        ".husky/pre-commit",
        "lefthook.yml",
        "lefthook.yaml",
        ".lefthook.yml",
        ".lefthook.yaml",
    ]
    .into_iter()
    .filter(|rel_path| {
        let (parent, name) = split_rel_path(rel_path);
        dir_structure
            .get(parent)
            .is_some_and(|entry| entry.has_file(name))
    })
    .map(str::to_owned)
    .collect::<Vec<_>>();

    rels.extend(dir_file_rels(dir_structure, ".githooks/pre-commit.d"));
    rels.extend(dir_file_rels(dir_structure, ".guardrail3/overrides/pre-commit.d"));
    rels
}

#[cfg(any(feature = "family-hooks-shared", feature = "family-hooks-rs"))]
fn hook_dir_rels(
    dir_structure: &std::collections::BTreeMap<String, guardrail3_app_rs_family_view::DirEntry>,
) -> Vec<String> {
    [
        ".githooks/pre-commit.d",
        ".guardrail3/overrides/pre-commit.d",
    ]
    .into_iter()
    .filter(|rel_path| dir_structure.contains_key(*rel_path))
    .map(str::to_owned)
    .collect()
}

#[cfg(any(feature = "family-hooks-shared", feature = "family-hooks-rs"))]
fn dir_file_rels(
    dir_structure: &std::collections::BTreeMap<String, guardrail3_app_rs_family_view::DirEntry>,
    dir_rel: &str,
) -> Vec<String> {
    dir_structure
        .get(dir_rel)
        .map(|entry| {
            entry
                .files()
                .iter()
                .map(|file_name| {
                    if dir_rel.is_empty() {
                        file_name.to_owned()
                    } else {
                        format!("{dir_rel}/{file_name}")
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(any(feature = "family-hooks-shared", feature = "family-hooks-rs"))]
fn split_rel_path(rel: &str) -> (&str, &str) {
    rel.rsplit_once('/').unwrap_or(("", rel))
}

// ---------------------------------------------------------------------------
// Runner registry.
// ---------------------------------------------------------------------------

pub(crate) fn compiled_runners() -> Vec<RustFamilyRunnerDef> {
    let mut runners = Vec::new();

    #[cfg(feature = "family-topology")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Topology,
        run: run_topology,
    });

    #[cfg(feature = "family-arch")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Arch,
        run: run_arch,
    });

    #[cfg(feature = "family-fmt")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Fmt,
        run: run_fmt,
    });

    #[cfg(feature = "family-toolchain")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Toolchain,
        run: run_toolchain,
    });

    #[cfg(feature = "family-clippy")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Clippy,
        run: run_clippy,
    });

    #[cfg(feature = "family-deny")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Deny,
        run: run_deny,
    });

    #[cfg(feature = "family-cargo")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Cargo,
        run: run_cargo,
    });

    #[cfg(feature = "family-code")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Code,
        run: run_code,
    });

    #[cfg(feature = "family-hexarch")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Hexarch,
        run: run_hexarch,
    });

    #[cfg(feature = "family-libarch")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Libarch,
        run: run_libarch,
    });

    #[cfg(feature = "family-deps")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Deps,
        run: run_deps,
    });

    #[cfg(feature = "family-garde")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Garde,
        run: run_garde,
    });

    #[cfg(feature = "family-test")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Test,
        run: run_test,
    });

    #[cfg(feature = "family-release")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::Release,
        run: run_release,
    });

    #[cfg(feature = "family-hooks-shared")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::HooksShared,
        run: run_hooks_shared,
    });

    #[cfg(feature = "family-hooks-rs")]
    runners.push(RustFamilyRunnerDef {
        family: RustValidateFamily::HooksRs,
        run: run_hooks_rs,
    });

    runners
}
