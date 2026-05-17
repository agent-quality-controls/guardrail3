/// `file_tree_facts` module.
mod file_tree_facts;
/// `fs` module.
mod fs;
/// `run` module.
mod run;
/// `view` module.
mod view;

#[cfg(feature = "ingest")]
pub use run::ingest_for_file_tree_checks;
