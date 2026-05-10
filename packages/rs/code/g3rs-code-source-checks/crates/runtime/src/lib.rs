//! Runtime rules for the `g3rs-code-source-checks` family.

/// Rule implementation for `always true cfg attr bypass`.
mod always_true_cfg_attr_bypass;
/// Rule implementation for `cfg attr allow inventory`.
mod cfg_attr_allow_inventory;
/// Rule implementation for `crate level allow`.
mod crate_level_allow;
/// Rule implementation for `deny forbid without reason`.
mod deny_forbid_without_reason;
/// Rule implementation for `direct fs usage`.
mod direct_fs_usage;
/// Rule implementation for `extern allow`.
mod extern_allow;
/// Rule implementation for `fs glob import`.
mod fs_glob_import;
/// Rule implementation for `garde skip with comment`.
mod garde_skip_with_comment;
/// Rule implementation for `garde skip without comment`.
mod garde_skip_without_comment;
/// Rule implementation for `generic parameter cap`.
mod generic_parameter_cap;
/// Rule implementation for `impl allow blast radius`.
mod impl_allow_blast_radius;
/// Rule implementation for `include bypass`.
mod include_bypass;
/// Rule implementation for `input failures`.
mod input_failures;
/// Rule implementation for `item level allow with reason`.
mod item_level_allow_with_reason;
/// Rule implementation for `item level allow without reason`.
mod item_level_allow_without_reason;
/// Rule implementation for `large trait surface`.
mod large_trait_surface;
/// Rule implementation for `large type inventory`.
mod large_type_inventory;
/// Rule implementation for `many use imports`.
mod many_use_imports;
/// Rule implementation for `panic macro`.
mod panic_macro;
/// Parser helpers for the family's structured inputs.
mod parse;
/// Rule implementation for `path attr with reason`.
mod path_attr_with_reason;
/// Rule implementation for `public struct named fields`.
mod public_struct_named_fields;
/// Rule implementation for `public weak error forms`.
mod public_weak_error_forms;
/// Family entry point that runs all rules.
mod run;
/// Rule implementation for `string dispatch cap`.
mod string_dispatch_cap;
/// Internal support helpers shared by this crate's rules.
mod support;
/// Rule implementation for `test expect message quality`.
mod test_expect_message_quality;
/// Rule implementation for `todo macros`.
mod todo_macros;
/// Rule implementation for `too many effective code lines`.
mod too_many_effective_code_lines;
/// Rule implementation for `too many use imports`.
mod too_many_use_imports;
/// Rule implementation for `unused crate dependencies allow`.
mod unused_crate_dependencies_allow;

#[cfg(feature = "checks")]
pub use run::check;
