/// Pure rule-option matching helpers extracted from `run`.
mod rule_helpers;
/// `run` module: top-level eslint surface ingestion.
mod run;

pub(crate) use run::ingest_seo_eslint_surface;
