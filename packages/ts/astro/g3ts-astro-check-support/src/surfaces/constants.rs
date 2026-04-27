pub(crate) const ESLINT_CONFIG_PATTERN: &str = "eslint.config.*";
pub(crate) const GUARDRAIL_CONFIG_REL_PATH: &str = "guardrail3-ts.toml";
pub(crate) const PACKAGE_JSON_REL_PATH: &str = "package.json";
pub(crate) const SYNCPACK_CONFIG_REL_PATH: &str = ".syncpackrc";
pub(crate) const ROUTE_SCOPED_PIPELINE_RULES: [&str; 8] = [
    "astro-pipeline/no-authored-content-fs-read",
    "astro-pipeline/no-authored-content-glob",
    "astro-pipeline/no-authored-content-imports",
    "astro-pipeline/no-content-data-modules-in-routes",
    "astro-pipeline/no-direct-astro-content-in-routes",
    "astro-pipeline/require-approved-content-adapter-in-routes",
    "astro-pipeline/no-side-loader-imports",
    "astro-pipeline/no-velite-imports",
];
pub(crate) const CONTENT_DATA_PIPELINE_RULES: [&str; 1] = ["astro-pipeline/no-content-data-modules-in-routes"];
pub(crate) const CONTENT_SOURCE_PIPELINE_RULES: [&str; 3] = [
    "astro-pipeline/no-authored-content-fs-read",
    "astro-pipeline/no-authored-content-glob",
    "astro-pipeline/no-authored-content-imports",
];
pub(crate) const INLINE_PUBLIC_CONTENT_RULE: &str = "i18next/no-literal-string";
pub(crate) const INLINE_PUBLIC_CONTENT_MESSAGE: &str = "Inline public copy must live in Astro content entries. Move this text into the content collection, validate it through the collection schema, and pass the typed value into source.";
pub(crate) const CONTENT_ADAPTER_PIPELINE_RULE: &str =
    "astro-pipeline/require-approved-content-adapter-in-routes";
pub(crate) const MDX_COMPONENT_MAP_PIPELINE_RULE: &str =
    "astro-pipeline/mdx-component-imports-from-approved-map";
pub(crate) const METADATA_HELPER_PIPELINE_RULE: &str =
    "astro-pipeline/require-approved-metadata-helper-in-routes";
pub(crate) const JSON_LD_HELPER_PIPELINE_RULE: &str =
    "astro-pipeline/require-approved-json-ld-helper-in-routes";
pub(crate) const SOURCE_MODULE_EXTENSIONS: [&str; 9] = [
    ".ts", ".tsx", ".js", ".jsx", ".mts", ".cts", ".mjs", ".cjs", ".astro",
];
pub(crate) const REQUIRED_SYNCPACK_PINS: [(&str, &str); 18] = [
    ("astro", "6.1.9"),
    ("@astrojs/react", "5.0.4"),
    ("@astrojs/mdx", "5.0.4"),
    ("@astrojs/check", "0.9.8"),
    ("@astrojs/sitemap", "3.7.2"),
    ("astro-robots", "2.3.1"),
    ("@nuasite/checks", "0.18.0"),
    ("g3ts-astro-nuasite-checks", "0.1.0"),
    ("schema-dts", "2.0.0"),
    ("react", "19.2.5"),
    ("react-dom", "19.2.5"),
    ("@types/react", "19.2.14"),
    ("@types/react-dom", "19.2.3"),
    ("typescript", "5.9.3"),
    ("eslint-plugin-astro", "1.7.0"),
    ("g3ts-eslint-plugin-astro-pipeline", "0.1.6"),
    ("eslint-plugin-i18next", "6.1.4"),
    ("eslint-plugin-mdx", "3.7.0"),
];
pub(crate) const FORBIDDEN_SYNCPACK_DEPS: [&str; 12] = [
    "next",
    "velite",
    "@astrojs/node",
    "eslint-plugin-astro-pipeline",
    "@codemint/astro-meta",
    "astro-seo",
    "astro-seo-meta",
    "astro-seo-schema",
    "contentlayer",
    "next-contentlayer",
    "@contentlayer/core",
    "@contentlayer/source-files",
];
pub(crate) const PIN_DEPENDENCY_TYPES: [&str; 2] = ["prod", "dev"];
pub(crate) const BAN_DEPENDENCY_TYPES: [&str; 4] = ["prod", "dev", "optional", "peer"];

