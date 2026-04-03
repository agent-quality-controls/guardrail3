use guardrail3_app_rs_family_mapper::RsToolchainRoute;
use guardrail3_app_rs_family_view::FamilyView;
use guardrail3_domain_report::CheckResult;

use crate::discover::collect;
use crate::inputs::all_from_facts;

pub fn check(surface: &FamilyView, route: &RsToolchainRoute) -> Vec<CheckResult> {
    let tree = surface;
    let facts = collect(tree, route);
    let mut results = Vec::new();

    for input in all_from_facts(&facts) {
        crate::rs_toolchain_01_exists::check(&input, &mut results);
        crate::rs_toolchain_02_channel_and_components::check(&input, &mut results);
        crate::rs_toolchain_03_msrv_consistency::check(&input, &mut results);
        crate::rs_toolchain_04_legacy_file::check(&input, &mut results);
    }

    results
}
