mod rs_fmt_filetree_01_exists;
mod rs_fmt_filetree_05_per_crate_override;
mod rs_fmt_filetree_08_dual_file_conflict;
mod run;
#[cfg(test)]
mod test_support;

#[cfg(feature = "checks")]
pub use run::check;
