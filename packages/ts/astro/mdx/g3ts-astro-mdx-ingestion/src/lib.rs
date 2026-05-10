/// `ESLint` config surface state and reader.
#[cfg(feature = "api")]
mod eslint;
/// `ESLint` directive parsing helpers.
#[cfg(feature = "api")]
mod eslint_directives;
/// `ESLint` rule and option introspection helpers.
#[cfg(feature = "api")]
mod eslint_helpers;
/// `ESLint` suppression detection helpers.
#[cfg(feature = "api")]
mod eslint_suppression;
/// `package.json` surface state and reader.
#[cfg(feature = "api")]
mod package;
/// MDX policy surface state and reader.
#[cfg(feature = "api")]
mod policy;
/// App-root and rel-path computations.
#[cfg(feature = "api")]
mod roots;
/// Public ingestion entry points.
#[cfg(feature = "api")]
mod run;
/// MDX source file enumeration.
#[cfg(feature = "api")]
mod sources;

#[cfg(feature = "api")]
pub use run::ingest_for_config_checks;
