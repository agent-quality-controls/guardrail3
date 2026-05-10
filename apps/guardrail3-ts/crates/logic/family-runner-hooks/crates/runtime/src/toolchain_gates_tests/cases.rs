#![expect(
    clippy::disallowed_methods,
    reason = "test-fixture: these cases set up real on-disk tempdir layouts (package.json, tsconfig.json, stylelint/typecov configs) to drive toolchain-gate skip logic; centralized fs helpers do not own write semantics in this CLI"
)]

use g3ts_hooks_contract_types::{G3TsHookCommandRequirement, PackageManager};
use guardrail3_ts_app_types::SupportedFamily;
use guardrail3_ts_family_runner_hooks_assertions::toolchain_gates::{
    assert_argvs_contain_requirement, assert_argvs_skip_requirement,
};

use super::super::toolchain_gate_list;

#[test]
fn toolchain_gate_list_sources_all_runnable_requirements_from_contracts() {
    let tempdir = tempfile::tempdir().expect("create temporary ts workspace for gate list test");
    std::fs::write(tempdir.path().join("package.json"), "{}\n")
        .expect("write package.json fixture for gate list test");
    std::fs::write(tempdir.path().join("tsconfig.json"), "{}\n")
        .expect("write tsconfig.json for gate list test");
    std::fs::write(
        tempdir.path().join("stylelint.config.js"),
        "module.exports = {};\n",
    )
    .expect("write stylelint config for gate list test");
    std::fs::write(tempdir.path().join("type-coverage.json"), "{}\n")
        .expect("write type-coverage config for gate list test");

    let manager = PackageManager::Pnpm;
    let gates = toolchain_gate_list(tempdir.path(), manager, &[]);
    let argvs: Vec<Vec<String>> = gates.into_iter().map(|gate| gate.argv).collect();

    for requirement in [
        G3TsHookCommandRequirement::Tsc,
        G3TsHookCommandRequirement::Eslint,
        G3TsHookCommandRequirement::Prettier,
        G3TsHookCommandRequirement::Cspell,
        G3TsHookCommandRequirement::Stylelint,
        G3TsHookCommandRequirement::SyncpackLint,
        G3TsHookCommandRequirement::TypeCoverage,
    ] {
        assert_argvs_contain_requirement(&argvs, requirement, manager);
    }
}

#[test]
fn toolchain_gate_list_skips_disabled_family_via_contract() {
    let tempdir =
        tempfile::tempdir().expect("create temporary ts workspace for disabled gate test");
    std::fs::write(tempdir.path().join("package.json"), "{}\n")
        .expect("write package.json for disabled gate test");

    let manager = PackageManager::Pnpm;
    let gates = toolchain_gate_list(tempdir.path(), manager, &[SupportedFamily::Fmt]);
    let argvs: Vec<Vec<String>> = gates.into_iter().map(|gate| gate.argv).collect();

    assert_argvs_skip_requirement(
        &argvs,
        G3TsHookCommandRequirement::Prettier,
        manager,
        "when fmt family is disabled",
    );
}

#[test]
fn toolchain_gate_list_skips_path_disabled_requirements() {
    let tempdir = tempfile::tempdir().expect("create empty ts workspace for path skip test");

    let manager = PackageManager::Pnpm;
    let gates = toolchain_gate_list(tempdir.path(), manager, &[]);
    let argvs: Vec<Vec<String>> = gates.into_iter().map(|gate| gate.argv).collect();

    for requirement in [
        G3TsHookCommandRequirement::Stylelint,
        G3TsHookCommandRequirement::TypeCoverage,
        G3TsHookCommandRequirement::SyncpackLint,
        G3TsHookCommandRequirement::Tsc,
    ] {
        assert_argvs_skip_requirement(
            &argvs,
            requirement,
            manager,
            "when path-level prerequisite is missing",
        );
    }
}
