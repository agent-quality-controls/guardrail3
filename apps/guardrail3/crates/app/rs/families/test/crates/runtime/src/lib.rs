mod analysis;
mod discover;
mod facts;
mod inputs;
mod parse;
mod assertion_quality;
mod mutation;
mod structure;

mod run;
pub use run::check;

#[cfg(feature = "api")]
pub use guardrail3_domain_report::{CheckResult, Severity};

#[cfg(test)]
use std::collections::BTreeSet;
#[cfg(test)]
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
#[cfg(test)]
use guardrail3_outbound_traits::ToolChecker;
#[cfg(test)]
use guardrail3_app_rs_family_mapper::FamilyMapper;
#[cfg(test)]
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

#[cfg(test)]
pub fn check_test_tree(tree: &ProjectTree, tc: &dyn ToolChecker) -> Vec<CheckResult> {
    let pt = guardrail3_domain_project_tree::ProjectTree::new(tree.root_path().to_path_buf(), tree.structure().clone(), tree.content().clone());
    let structure = guardrail3_app_rs_structure::collect(pt, &[]);
    let legality = guardrail3_app_rs_legality::collect(structure);
    let selection = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Test]));
    let route = FamilyMapper::from_legality(&legality, None, &selection, None).map_rs_test();
    check(tree, &route, tc)
}
