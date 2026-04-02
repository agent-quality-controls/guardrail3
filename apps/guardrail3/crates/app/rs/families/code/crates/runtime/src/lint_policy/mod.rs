pub(crate) mod rs_code_01_crate_level_allow;
pub(crate) mod rs_code_02_unused_crate_dependencies_allow;
pub(crate) mod rs_code_03_item_level_allow_without_reason;
pub(crate) mod rs_code_04_item_level_allow_with_reason;
pub(crate) mod rs_code_07_exception_comment_inventory;
pub(crate) mod rs_code_08_cfg_attr_allow_inventory;
pub(crate) mod rs_code_12_unsafe_code_lint;
pub(crate) mod rs_code_22_deny_forbid_without_reason;
pub(crate) mod rs_code_30_input_failures;

#[cfg(test)]
mod rs_code_01_crate_level_allow_tests;
#[cfg(test)]
mod rs_code_02_unused_crate_dependencies_allow_tests;
#[cfg(test)]
mod rs_code_03_item_level_allow_without_reason_tests;
#[cfg(test)]
mod rs_code_04_item_level_allow_with_reason_tests;
#[cfg(test)]
mod rs_code_07_exception_comment_inventory_tests;
#[cfg(test)]
mod rs_code_08_cfg_attr_allow_inventory_tests;
#[cfg(test)]
mod rs_code_12_unsafe_code_lint_tests;
#[cfg(test)]
mod rs_code_22_deny_forbid_without_reason_tests;
#[cfg(test)]
mod rs_code_30_input_failures_tests;
