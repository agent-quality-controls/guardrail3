#[cfg(any(feature = "family-code", feature = "family-garde"))]
use guardrail3_app_rs_family_mapper::RsScopedRootView;
use guardrail3_app_rs_family_mapper::RsProjectSurface;
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
))]
use guardrail3_domain_project_tree::ProjectTree;
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
    let surface = topology_surface(ctx.tree, &route);
    guardrail3_app_rs_family_topology::check(&surface, &route)
}

#[cfg(feature = "family-arch")]
fn run_arch(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let route = ctx.mapper.map_rs_arch();
    route
        .roots()
        .iter()
        .flat_map(|root| {
            let workspace_route = route.for_workspace(root.rel_dir());
            let surface = workspace_surface(
                ctx.tree,
                workspace_route.roots(),
                workspace_route.family_files(),
            );
            guardrail3_app_rs_family_arch::check(&surface, &workspace_route)
        })
        .collect()
}

#[cfg(feature = "family-fmt")]
fn run_fmt(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let route = ctx.mapper.map_rs_fmt();
    let surface = RsProjectSurface::from_route_scope(ctx.tree, &[], &fmt_extra_files(&route), None);
    guardrail3_app_rs_family_fmt::check(&surface, &route)
}

#[cfg(feature = "family-toolchain")]
fn run_toolchain(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let route = ctx.mapper.map_rs_toolchain();
    route
        .roots()
        .iter()
        .flat_map(|root| {
            let workspace_route = route.for_workspace(root.rel_dir());
            let surface = workspace_surface(
                ctx.tree,
                workspace_route.roots(),
                workspace_route.family_files(),
            );
            guardrail3_app_rs_family_toolchain::check(&surface, &workspace_route)
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
            let surface = workspace_surface(
                ctx.tree,
                workspace_route.roots(),
                workspace_route.family_files(),
            );
            guardrail3_app_rs_family_clippy::check(&surface, &workspace_route)
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
            let surface = workspace_surface(
                ctx.tree,
                workspace_route.roots(),
                workspace_route.family_files(),
            );
            guardrail3_app_rs_family_deny::check(&surface, &workspace_route)
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
            let surface = workspace_surface(
                ctx.tree,
                workspace_route.roots(),
                workspace_route.family_files(),
            );
            guardrail3_app_rs_family_cargo::check(&surface, &workspace_route)
        })
        .collect()
}

#[cfg(feature = "family-code")]
fn run_code(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let route = ctx.mapper.map_rs_code();
    let surface = code_surface(ctx.tree, &route);
    guardrail3_app_rs_family_code::check(&surface, &route)
}

#[cfg(feature = "family-hexarch")]
fn run_hexarch(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let route = ctx.mapper.map_rs_hexarch();
    route
        .roots()
        .iter()
        .flat_map(|root| {
            let workspace_route = route.for_workspace(root.rel_dir());
            let surface = hexarch_surface(ctx.tree, &workspace_route);
            guardrail3_app_rs_family_hexarch::check(&surface, &workspace_route)
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
            let surface = workspace_surface(
                ctx.tree,
                workspace_route.roots(),
                workspace_route.family_files(),
            );
            guardrail3_app_rs_family_libarch::check(&surface, &workspace_route)
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
            let surface = workspace_surface(
                ctx.tree,
                workspace_route.roots(),
                workspace_route.family_files(),
            );
            guardrail3_app_rs_family_deps::check(&surface, &workspace_route, ctx.tc)
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
            let surface = garde_surface(ctx.tree, &workspace_route);
            guardrail3_app_rs_family_garde::check(&surface, &workspace_route)
        })
        .collect()
}

#[cfg(feature = "family-test")]
fn run_test(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let route = ctx.mapper.map_rs_test();
    let surface = test_surface(ctx.tree, &route);
    guardrail3_app_rs_family_test::check(&surface, &route, ctx.tc)
}

#[cfg(feature = "family-release")]
fn run_release(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let route = ctx.mapper.map_rs_release();
    route
        .roots()
        .iter()
        .flat_map(|root| {
            let workspace_route = route.for_workspace(root.rel_dir());
            let surface = workspace_surface(
                ctx.tree,
                workspace_route.roots(),
                workspace_route.family_files(),
            );
            guardrail3_app_rs_family_release::check(
                &surface,
                &workspace_route,
                ctx.tc,
                ctx.thorough,
            )
        })
        .collect()
}

#[cfg(feature = "family-hooks-shared")]
fn run_hooks_shared(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let surface = hooks_surface(ctx.tree);
    guardrail3_app_rs_family_hooks_shared::check(ctx.fs, ctx.path, &surface, ctx.tc)
}

#[cfg(feature = "family-hooks-rs")]
fn run_hooks_rs(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    let surface = hooks_surface(ctx.tree);
    guardrail3_app_rs_family_hooks_rs::check(&surface, ctx.tc)
}

#[cfg(feature = "family-topology")]
fn topology_surface(
    tree: &ProjectTree,
    route: &guardrail3_app_rs_family_mapper::RsTopologyRoute,
) -> RsProjectSurface {
    let mut extra_file_rels = route
        .roots()
        .iter()
        .map(|root| root.root().cargo_rel_path().to_owned())
        .collect::<Vec<_>>();
    extra_file_rels.extend(family_file_rels(route.family_files()));
    if tree.file_exists("guardrail3.toml") {
        extra_file_rels.push("guardrail3.toml".to_owned());
    }
    RsProjectSurface::from_route_scope(tree, &topology_root_rels(route.roots()), &extra_file_rels, None)
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
))]
fn workspace_surface(
    tree: &ProjectTree,
    roots: &[RsRootView],
    family_files: &[RsFamilyFileView],
) -> RsProjectSurface {
    let mut extra_file_rels = route_root_cargo_files(roots);
    extra_file_rels.extend(family_file_rels(family_files));
    RsProjectSurface::from_route_scope(tree, &route_root_rels(roots), &extra_file_rels, None)
}

#[cfg(feature = "family-garde")]
fn garde_surface(
    tree: &ProjectTree,
    route: &guardrail3_app_rs_family_mapper::RsGardeRoute,
) -> RsProjectSurface {
    let mut extra_file_rels = scoped_route_root_cargo_files(route.roots());
    extra_file_rels.extend(family_file_rels(route.family_files()));
    RsProjectSurface::from_route_scope(
        tree,
        &scoped_route_root_rels(route.roots()),
        &extra_file_rels,
        None,
    )
}

#[cfg(feature = "family-hexarch")]
fn hexarch_surface(
    tree: &ProjectTree,
    route: &guardrail3_app_rs_family_mapper::RsHexarchRoute,
) -> RsProjectSurface {
    let mut extra_file_rels = route_root_cargo_files(route.roots());
    if let Some(repo_root_cargo) = route.repo_root_cargo_rel_path() {
        extra_file_rels.push(repo_root_cargo.to_owned());
    }
    if let Some(guardrail_rel) = route.guardrail_config_rel_path() {
        extra_file_rels.push(guardrail_rel.to_owned());
    }
    RsProjectSurface::from_route_scope(
        tree,
        &route_root_rels(route.roots()),
        &extra_file_rels,
        None,
    )
}

#[cfg(feature = "family-code")]
fn code_surface(
    tree: &ProjectTree,
    route: &guardrail3_app_rs_family_mapper::RsCodeRoute,
) -> RsProjectSurface {
    let extra_file_rels = code_surface_file_rels(tree, route);
    RsProjectSurface::from_route_scope(
        tree,
        &scoped_route_root_rels(route.roots()),
        &extra_file_rels,
        None,
    )
}

#[cfg(feature = "family-fmt")]
fn fmt_extra_files(route: &guardrail3_app_rs_family_mapper::RsFmtRoute) -> Vec<String> {
    let mut extra = family_file_rels(route.family_files());
    extra.push("Cargo.toml".to_owned());
    extra.push("rust-toolchain.toml".to_owned());
    extra.push("guardrail3.toml".to_owned());
    extra
}

#[cfg(feature = "family-test")]
fn test_surface(
    tree: &ProjectTree,
    route: &guardrail3_app_rs_family_mapper::RsTestRoute,
) -> RsProjectSurface {
    RsProjectSurface::from_route_scope(
        tree,
        &route_root_rels(route.roots()),
        &route_root_cargo_files(route.roots()),
        route.scoped_files(),
    )
}

#[cfg(any(feature = "family-hooks-shared", feature = "family-hooks-rs"))]
fn hooks_surface(tree: &ProjectTree) -> RsProjectSurface {
    RsProjectSurface::from_route_scope_with_dirs(
        tree,
        &[],
        &hook_file_rels(tree),
        &hook_dir_rels(tree),
        None,
    )
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

#[cfg(any(feature = "family-code", feature = "family-garde"))]
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

#[cfg(feature = "family-code")]
fn code_surface_file_rels(
    tree: &ProjectTree,
    route: &guardrail3_app_rs_family_mapper::RsCodeRoute,
) -> Vec<String> {
    let mut rels = route
        .roots()
        .iter()
        .map(|root| root.root().cargo_rel_path().to_owned())
        .collect::<std::collections::BTreeSet<_>>();

    for (dir_rel, entry) in tree.structure() {
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
                let _ = rels.insert(ProjectTree::join_rel(dir_rel, file_name));
            }
        }
    }

    rels.into_iter().collect()
}

#[cfg(any(feature = "family-hooks-shared", feature = "family-hooks-rs"))]
fn hook_file_rels(tree: &ProjectTree) -> Vec<String> {
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
    .filter(|rel_path| tree.file_exists(rel_path))
    .map(str::to_owned)
    .collect::<Vec<_>>();

    rels.extend(dir_file_rels(tree, ".githooks/pre-commit.d"));
    rels.extend(dir_file_rels(tree, ".guardrail3/overrides/pre-commit.d"));
    rels
}

#[cfg(any(feature = "family-hooks-shared", feature = "family-hooks-rs"))]
fn hook_dir_rels(tree: &ProjectTree) -> Vec<String> {
    [
        ".githooks/pre-commit.d",
        ".guardrail3/overrides/pre-commit.d",
    ]
    .into_iter()
    .filter(|rel_path| tree.dir_exists(rel_path))
    .map(str::to_owned)
    .collect()
}

#[cfg(any(feature = "family-hooks-shared", feature = "family-hooks-rs"))]
fn dir_file_rels(tree: &ProjectTree, dir_rel: &str) -> Vec<String> {
    tree.dir_contents(dir_rel)
        .map(|entry| {
            entry.files()
                .iter()
                .map(|file_name| ProjectTree::join_rel(dir_rel, file_name))
                .collect()
        })
        .unwrap_or_default()
}


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
