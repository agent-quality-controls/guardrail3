use package_json_parser::types::PackageJsonSnapshot;

use crate::{G3TsPackageLocalSnapshot, G3TsPackageRootSnapshot};

#[must_use]
pub fn root_snapshot(rel_path: &str, snapshot: &PackageJsonSnapshot) -> G3TsPackageRootSnapshot {
    G3TsPackageRootSnapshot {
        rel_path: rel_path.to_owned(),
        private_field: snapshot.private_field,
        package_manager: snapshot.package_manager.clone(),
        engines_node: snapshot.engines_node.clone(),
        engines_pnpm: snapshot.engines_pnpm.clone(),
        preinstall_script: snapshot.scripts.get("preinstall").cloned(),
        prepare_script: snapshot.scripts.get("prepare").cloned(),
        lint_script: snapshot.scripts.get("lint").cloned(),
        typecheck_script: snapshot.scripts.get("typecheck").cloned(),
        validate_script: snapshot.scripts.get("validate").cloned(),
        dependencies: snapshot.dependencies.clone(),
        dev_dependencies: snapshot.dev_dependencies.clone(),
        pnpm_override_keys: snapshot.pnpm_override_keys.clone(),
        pnpm_only_built_dependencies: snapshot.pnpm_only_built_dependencies.clone(),
        script_commands: Vec::new(),
        script_tool_invocations: Vec::new(),
        script_parse_blockers: Vec::new(),
        safely_runs_only_allow_pnpm: false,
        safely_runs_syncpack_lint: false,
    }
}

#[must_use]
pub fn local_snapshot(rel_path: &str, snapshot: &PackageJsonSnapshot) -> G3TsPackageLocalSnapshot {
    G3TsPackageLocalSnapshot {
        rel_path: rel_path.to_owned(),
        dependencies: snapshot.dependencies.clone(),
        dev_dependencies: snapshot.dev_dependencies.clone(),
    }
}
