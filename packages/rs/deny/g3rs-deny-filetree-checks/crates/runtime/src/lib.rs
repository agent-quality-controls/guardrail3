mod rs_deny_filetree_01_coverage;
mod rs_deny_filetree_03_shadowing;
mod run;
#[cfg(test)]
mod test_support;

#[cfg(feature = "checks")]
pub use run::check;
