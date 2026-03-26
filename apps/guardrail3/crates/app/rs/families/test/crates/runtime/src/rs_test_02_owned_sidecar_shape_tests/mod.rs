pub(crate) use guardrail3_app_rs_family_test_assertions::rs_test_02_owned_sidecar_shape::{
    finding, rule_files, run_family,
};
pub(crate) use test_support::{tempdir, write_file};
mod ad_hoc_shapes;
mod family_impl;
mod golden;
mod path_included_source_attack;
