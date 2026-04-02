use std::collections::BTreeSet;
use std::path::Path;

use guardrail3_app_rs_family_mapper::FamilyMapper;
use guardrail3_app_rs_family_view::FamilyView;
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

pub(crate) use super::run_family;
pub(crate) use test_support::{tempdir, write_file};

pub(crate) fn run_family_scoped(root: &Path, scope: &str) -> Vec<crate::CheckResult> {
    let tree = test_support::walk(root);
    let structure = guardrail3_app_rs_structure::collect(tree.clone(), &[]);
    let legality = guardrail3_app_rs_legality::collect(structure);
    let selection = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Test]));
    let route = FamilyMapper::from_legality(&legality, None, &selection, None)
        .with_validation_scope(Some(scope))
        .map_rs_test();
    let surface = FamilyView::build(
        tree.root().clone(),
        tree.structure(),
        tree.content(),
        &["".to_owned()],
        &[],
        &[],
        None,
    );
    crate::check(
        &surface,
        &route,
        &test_support::StubToolChecker::default(),
    )
}

mod boundaries;
mod family_impl;
mod fixtures;
mod golden;
mod scope;
