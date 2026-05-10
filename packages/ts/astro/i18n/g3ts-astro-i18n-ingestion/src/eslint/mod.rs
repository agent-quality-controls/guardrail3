/// Orchestrates ingestion of the i18n `ESLint` surface.
mod run;
/// Aggregates settings across `ESLint` effective config probes.
mod settings;
/// Computes `ESLint` probe targets for the i18n surface.
mod targets;

pub(crate) use run::ingest_i18n_eslint_surface;
