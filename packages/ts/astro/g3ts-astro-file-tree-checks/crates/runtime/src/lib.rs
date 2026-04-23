mod run;
mod ts_astro_filetree_01_astro_config_exists;
mod ts_astro_filetree_02_content_config_exists;
mod ts_astro_filetree_03_live_config_exists;
mod ts_astro_filetree_04_no_route_markdown_pages;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
use g3ts_astro_file_tree_checks_assertions as _;
