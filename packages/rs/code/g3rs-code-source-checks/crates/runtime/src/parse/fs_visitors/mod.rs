/// Rule implementation for `api`.
mod api;
/// Rule implementation for `inline std fs`.
mod inline_std_fs;
/// Rule implementation for `std fs glob import`.
mod std_fs_glob_import;
/// Rule implementation for `std fs import`.
mod std_fs_import;
/// Internal support helpers shared by this crate's rules.
mod support;

pub(crate) use api::{
    find_inline_std_fs_call_lines, find_std_fs_glob_import_lines, find_std_fs_import_lines,
};
