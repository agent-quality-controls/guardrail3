mod discover;
mod facts;
mod inputs;
mod rs_toolchain_01_exists;
mod rs_toolchain_02_channel_and_components;
mod rs_toolchain_03_msrv_consistency;
mod rs_toolchain_04_legacy_file;

use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;

use self::discover::collect;
use self::inputs::ToolchainRootInput;

pub fn check(tree: &ProjectTree) -> Vec<CheckResult> {
    let facts = collect(tree);
    let input = ToolchainRootInput::from_facts(&facts);
    let mut results = Vec::new();

    rs_toolchain_01_exists::check(&input, &mut results);
    rs_toolchain_02_channel_and_components::check(&input, &mut results);
    rs_toolchain_03_msrv_consistency::check(&input, &mut results);
    rs_toolchain_04_legacy_file::check(&input, &mut results);

    results
}
