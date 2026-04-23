use astro_config_parser_runtime_assertions::parser as assertions;
use astro_config_parser_types::document::AstroOutputMode;

#[test]
fn parses_define_config_module_with_integrations() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "import { defineConfig } from 'astro/config';\nimport react from '@astrojs/react';\nimport checks from '@nuasite/checks';\nexport default defineConfig({ site: 'https://example.com', output: 'static', integrations: [checks(), react()] });\n",
    )
    .expect("config should be written");

    let document =
        crate::parse_document(root.path(), rel_path).expect("astro config should parse");

    assertions::assert_parsed_document(&document);
    assertions::assert_snapshot(
        &document,
        Some("https://example.com"),
        Some(AstroOutputMode::Static),
        &["@astrojs/react", "@nuasite/checks"],
        None,
    );
}

#[test]
fn parses_function_config_and_cjs_export() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.cjs";
    std::fs::write(
        root.path().join(rel_path),
        "const node = require('@astrojs/node');\nmodule.exports = { output: 'server', adapter: node({ mode: 'standalone' }) };\n",
    )
    .expect("config should be written");

    let document =
        crate::parse_document(root.path(), rel_path).expect("astro config should parse");

    assertions::assert_parsed_document(&document);
    assertions::assert_snapshot(
        &document,
        None,
        Some(AstroOutputMode::Server),
        &[],
        Some("@astrojs/node"),
    );
}

#[test]
fn parses_identifier_bound_integrations_array() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.ts";
    std::fs::write(
        root.path().join(rel_path),
        "import { defineConfig } from 'astro/config';\nimport mdx from '@astrojs/mdx';\nconst integrations = [mdx()];\nconst config = defineConfig({ integrations });\nexport default config;\n",
    )
    .expect("config should be written");

    let document =
        crate::parse_document(root.path(), rel_path).expect("astro config should parse");

    assertions::assert_parsed_document(&document);
    assertions::assert_snapshot(&document, None, None, &["@astrojs/mdx"], None);
}

#[test]
fn bare_imported_identifier_does_not_count_as_wired_integration() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "import { defineConfig } from 'astro/config';\nimport checks from '@nuasite/checks';\nexport default defineConfig({ integrations: [checks] });\n",
    )
    .expect("config should be written");

    let document =
        crate::parse_document(root.path(), rel_path).expect("astro config should parse");

    assertions::assert_parsed_document(&document);
    assertions::assert_snapshot(&document, None, None, &[], None);
}

#[test]
fn spread_integrations_resolve_when_they_point_to_array_literals() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "import { defineConfig } from 'astro/config';\nimport mdx from '@astrojs/mdx';\nimport react from '@astrojs/react';\nconst base = [mdx()];\nexport default defineConfig({ integrations: [...base, react()] });\n",
    )
    .expect("config should be written");

    let document =
        crate::parse_document(root.path(), rel_path).expect("astro config should parse");

    assertions::assert_parsed_document(&document);
    assertions::assert_snapshot(
        &document,
        None,
        None,
        &["@astrojs/mdx", "@astrojs/react"],
        None,
    );
}

#[test]
fn dynamic_spread_integrations_are_invalid() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "import { defineConfig } from 'astro/config';\nconst base = getBase();\nexport default defineConfig({ integrations: [...base] });\n",
    )
    .expect("config should be written");

    let document =
        crate::parse_document(root.path(), rel_path).expect("astro config document should exist");

    assertions::assert_invalid_document(&document, "spread elements must resolve to array literals");
}

#[test]
fn local_define_config_does_not_count_as_astro_wrapper() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "function defineConfig(value) { return value; }\nexport default defineConfig({ integrations: [] });\n",
    )
    .expect("config should be written");

    let document =
        crate::parse_document(root.path(), rel_path).expect("astro config document should exist");

    assertions::assert_invalid_document(&document, "could not reduce exported Astro config");
}
