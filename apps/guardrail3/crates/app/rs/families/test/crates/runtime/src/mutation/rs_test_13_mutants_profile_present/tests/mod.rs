mod helpers;
pub(crate) use helpers::run_family;
pub(crate) use test_support::{tempdir, write_file};

mod activation;
mod golden;
mod missing_profile;
mod mixed_roots;
