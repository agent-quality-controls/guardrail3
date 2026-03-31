use guardrail3_app_rs_family_arch as _;
use test_support as _;

pub mod rs_arch_01_root_classification;
pub mod rs_arch_02_no_misplaced_roots;
pub mod rs_arch_03_no_dual_ownership;
pub mod rs_arch_04_no_zone_overlap;
pub mod rs_arch_05_scoped_arch_config_forbidden;
pub mod rs_arch_06_owner_family_enablement_coherence;
pub mod rs_arch_07_required_inputs_fail_closed;
pub mod rs_arch_08_auxiliary_roots_declared;
pub mod rs_arch_16_workspace_local_file_placement;
