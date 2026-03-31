use std::collections::BTreeSet;
use std::path::Path;

use guardrail3_app_rs_family_mapper::{FamilyMapper, RsProjectSurface};
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

pub(crate) use super::run_family;
pub(crate) use test_support::{tempdir, write_file};

pub(crate) fn run_family_scoped(root: &Path, scope: &str) -> Vec<crate::CheckResult> {
    let tree = test_support::walk(root);
    let placement = guardrail3_app_rs_structure::collect(&tree);
    let selection = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Test]));
    let route = FamilyMapper::new(&tree, &placement, None, &selection, None)
        .with_validation_scope(Some(scope))
        .map_rs_test();
    crate::check(
        &RsProjectSurface::from_tree(&tree),
        &route,
        &test_support::StubToolChecker::default(),
    )
}

mod ad_hoc_shapes;
mod family_impl;
mod fixtures;
mod golden;
mod path_included_source_attack;
mod scope;
