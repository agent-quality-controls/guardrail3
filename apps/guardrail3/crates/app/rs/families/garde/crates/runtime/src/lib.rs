mod discover;
mod facts;
mod garde_support;
mod inputs;
mod parse;
mod root_policy;

use glob as _;
use guardrail3_app_core as _;
use guardrail3_domain_config as _;
use guardrail3_domain_modules as _;
use guardrail3_outbound_traits as _;
use semver as _;
use serde_yaml as _;

mod run;
pub use run::check;

#[cfg(test)]
use guardrail3_app_rs_family_mapper::RsGardeRoute;
#[cfg(test)]
use guardrail3_domain_report::CheckResult;

#[cfg(test)]
pub(crate) fn check_test_tree(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    route: &RsGardeRoute,
) -> Vec<CheckResult> {
    check(tree, route)
}
