/// Rule: an Astro content config file must exist for the app root.
mod content_config_exists;
/// Rule: a live content config file must exist when live content is in use.
mod live_config_exists;
/// Rule: routed markdown pages are not permitted.
mod no_route_markdown_pages;
/// Rule: a Velite config file must not exist for an Astro content app.
mod no_velite_config;
/// Rule: Velite output directories must not be present.
mod no_velite_output;
/// Family runner that dispatches to per-rule check modules.
mod run;

#[cfg(feature = "checks")]
pub use run::check;
