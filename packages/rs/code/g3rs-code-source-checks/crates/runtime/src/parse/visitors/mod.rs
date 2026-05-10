/// Rule implementation for `core`.
mod core;
/// Rule implementation for `string dispatch`.
mod string_dispatch;

pub(crate) use core::{
    find_forbidden_macros, find_generic_parameter_caps, find_large_traits, find_large_type_items,
    find_string_dispatch_sites, find_test_expect_calls,
};
