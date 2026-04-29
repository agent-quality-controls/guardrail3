use astro_config_parser_runtime_assertions::parser as assertions;
use astro_config_parser_runtime_assertions::parser::{
    AstroConfigParseState, AstroOutputMode, AstroStaticValue, AstroTrailingSlashPolicy,
};

#[test]
fn parses_define_config_module_with_integrations() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "import { defineConfig } from 'astro/config';\nimport react from '@astrojs/react';\nimport checks from '@nuasite/checks';\nexport default defineConfig({ site: 'https://example.com', output: 'static', outDir: 'public-build', trailingSlash: 'always', integrations: [checks(), react()] });\n",
    )
    .expect("config should be written");

    let document =
        assertions::parse_document(root.path(), rel_path).expect("astro config should parse");

    assertions::assert_parsed_document(&document);
    assertions::assert_snapshot(
        &document,
        Some("https://example.com"),
        Some(AstroOutputMode::Static),
        Some("public-build"),
        Some(AstroTrailingSlashPolicy::Always),
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
        assertions::parse_document(root.path(), rel_path).expect("astro config should parse");

    assertions::assert_parsed_document(&document);
    assertions::assert_snapshot(
        &document,
        None,
        Some(AstroOutputMode::Server),
        None,
        None,
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
        assertions::parse_document(root.path(), rel_path).expect("astro config should parse");

    assertions::assert_parsed_document(&document);
    assertions::assert_snapshot(&document, None, None, None, None, &["@astrojs/mdx"], None);
}

#[test]
fn parses_same_file_const_strings_and_static_templates() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "import { defineConfig } from 'astro/config';\nimport robotsAudit from 'g3ts-astro-robots-auditor';\nconst siteUrl = 'https://example.com';\nconst sitemapIndexUrl = `${siteUrl}/sitemap-index.xml`;\nexport default defineConfig({ site: siteUrl, output: 'static', trailingSlash: 'always', integrations: [robotsAudit({ site: siteUrl, sitemapUrls: [sitemapIndexUrl] })] });\n",
    )
    .expect("config should be written");

    let document =
        assertions::parse_document(root.path(), rel_path).expect("astro config should parse");

    assertions::assert_parsed_document(&document);
    assertions::assert_snapshot(
        &document,
        Some("https://example.com"),
        Some(AstroOutputMode::Static),
        None,
        Some(AstroTrailingSlashPolicy::Always),
        &["g3ts-astro-robots-auditor"],
        None,
    );

    let AstroConfigParseState::Parsed(snapshot) = &document.typed else {
        panic!("expected parsed document, got {document:?}");
    };
    let options = snapshot
        .integrations
        .first()
        .and_then(|integration| integration.call.as_ref())
        .and_then(|call| call.first_arg.as_ref())
        .expect("integration options should be preserved");
    let AstroStaticValue::Object(properties) = options else {
        panic!("expected object options, got {options:?}");
    };
    let sitemap_urls = properties
        .iter()
        .find(|property| property.key == "sitemapUrls")
        .expect("sitemapUrls should be present");
    let AstroStaticValue::Array(values) = &sitemap_urls.value else {
        panic!("expected sitemapUrls array, got {:?}", sitemap_urls.value);
    };
    assert_eq!(
        values.first(),
        Some(&AstroStaticValue::String(
            "https://example.com/sitemap-index.xml".to_owned()
        ))
    );
}

#[test]
fn dynamic_template_static_values_are_invalid() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "import robotsAudit from 'g3ts-astro-robots-auditor';\nconst siteUrl = getSiteUrl();\nconst sitemapIndexUrl = `${siteUrl}/sitemap-index.xml`;\nexport default { integrations: [robotsAudit({ sitemapUrls: [sitemapIndexUrl] })] };\n",
    )
    .expect("config should be written");

    let document = assertions::parse_document(root.path(), rel_path)
        .expect("astro config document should exist");

    assertions::assert_invalid_document(&document, "must resolve to static values");
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
        assertions::parse_document(root.path(), rel_path).expect("astro config should parse");

    assertions::assert_parsed_document(&document);
    assertions::assert_snapshot(&document, None, None, None, None, &[], None);
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
        assertions::parse_document(root.path(), rel_path).expect("astro config should parse");

    assertions::assert_parsed_document(&document);
    assertions::assert_snapshot(
        &document,
        None,
        None,
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

    let document = assertions::parse_document(root.path(), rel_path)
        .expect("astro config document should exist");

    assertions::assert_invalid_document(
        &document,
        "spread elements must resolve to array literals",
    );
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

    let document = assertions::parse_document(root.path(), rel_path)
        .expect("astro config document should exist");

    assertions::assert_invalid_document(&document, "could not reduce exported Astro config");
}

#[test]
fn root_object_spreads_are_invalid() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "const serverConfig = { output: 'server' };\nexport default { site: 'https://example.com', output: 'static', ...serverConfig };\n",
    )
    .expect("config should be written");

    let document = assertions::parse_document(root.path(), rel_path)
        .expect("astro config document should exist");

    assertions::assert_invalid_document(&document, "must not contain spread properties");
}

#[test]
fn duplicate_root_config_keys_are_invalid() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "export default { output: 'static', output: 'server' };\n",
    )
    .expect("config should be written");

    let document = assertions::parse_document(root.path(), rel_path)
        .expect("astro config document should exist");

    assertions::assert_invalid_document(&document, "duplicate `output` property");
}

#[test]
fn mutable_bindings_do_not_count_as_static_config_values() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "let output = 'static';\noutput = 'server';\nexport default { output };\n",
    )
    .expect("config should be written");

    let document = assertions::parse_document(root.path(), rel_path)
        .expect("astro config document should exist");

    assertions::assert_invalid_document(
        &document,
        "shorthand `output` must resolve to an unmutated const binding",
    );
}

#[test]
fn mutated_exported_config_identifier_is_invalid() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "import checks from '@nuasite/checks';\nimport { structuredDataPresentCheck } from 'g3ts-astro-nuasite-checks';\nconst config = { site: 'https://example.com', output: 'static', integrations: [checks({ mode: 'full', failOnError: true, failOnWarning: true, reportJson: true, ai: false, customChecks: [structuredDataPresentCheck] })] };\nconfig.output = 'server';\nconfig.integrations = [];\nexport default config;\n",
    )
    .expect("config should be written");

    let document = assertions::parse_document(root.path(), rel_path)
        .expect("astro config document should exist");

    assertions::assert_invalid_document(&document, "could not reduce exported Astro config");
}

#[test]
fn object_assign_mutated_exported_config_identifier_is_invalid() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "const config = { output: 'static' };\nObject.assign(config, { output: 'server' });\nexport default config;\n",
    )
    .expect("config should be written");

    let document = assertions::parse_document(root.path(), rel_path)
        .expect("astro config document should exist");

    assertions::assert_invalid_document(&document, "could not reduce exported Astro config");
}

#[test]
fn object_assign_in_initializer_mutating_exported_config_is_invalid() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "const config = { output: 'static' };\nconst changed = Object.assign(config, { output: 'server' });\nexport default config;\n",
    )
    .expect("config should be written");

    let document = assertions::parse_document(root.path(), rel_path)
        .expect("astro config document should exist");

    assertions::assert_invalid_document(&document, "could not reduce exported Astro config");
}

#[test]
fn object_assign_in_let_initializer_mutating_exported_config_is_invalid() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "const config = { output: 'static' };\nlet changed = Object.assign(config, { output: 'server' });\nexport default config;\n",
    )
    .expect("config should be written");

    let document = assertions::parse_document(root.path(), rel_path)
        .expect("astro config document should exist");

    assertions::assert_invalid_document(&document, "could not reduce exported Astro config");
}

#[test]
fn optional_object_assign_mutating_exported_config_is_invalid() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "const config = { output: 'static' };\nObject.assign?.(config, { output: 'server' });\nexport default config;\n",
    )
    .expect("config should be written");

    let document = assertions::parse_document(root.path(), rel_path)
        .expect("astro config document should exist");

    assertions::assert_invalid_document(&document, "could not reduce exported Astro config");
}

#[test]
fn spread_object_assign_mutating_exported_config_is_invalid() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "const config = { output: 'static' };\nObject.assign(...[config, { output: 'server' }]);\nexport default config;\n",
    )
    .expect("config should be written");

    let document = assertions::parse_document(root.path(), rel_path)
        .expect("astro config document should exist");

    assertions::assert_invalid_document(&document, "could not reduce exported Astro config");
}

#[test]
fn callable_config_body_mutating_exported_config_is_invalid() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "const config = { output: 'static' };\nexport default () => { config.output = 'server'; return config; };\n",
    )
    .expect("config should be written");

    let document = assertions::parse_document(root.path(), rel_path)
        .expect("astro config document should exist");

    assertions::assert_invalid_document(&document, "could not reduce exported Astro config");
}

#[test]
fn block_nested_exported_config_mutation_is_invalid() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "const config = { output: 'static' };\nif (true) config.output = 'server';\nexport default config;\n",
    )
    .expect("config should be written");

    let document = assertions::parse_document(root.path(), rel_path)
        .expect("astro config document should exist");

    assertions::assert_invalid_document(&document, "could not reduce exported Astro config");
}

#[test]
fn alias_mutated_exported_config_identifier_is_invalid() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "const config = { output: 'static' };\nconst alias = config;\nalias.output = 'server';\nexport default config;\n",
    )
    .expect("config should be written");

    let document = assertions::parse_document(root.path(), rel_path)
        .expect("astro config document should exist");

    assertions::assert_invalid_document(&document, "could not reduce exported Astro config");
}

#[test]
fn transitive_alias_mutated_exported_config_identifier_is_invalid() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "const config = { output: 'static' };\nconst alias = config;\nconst nextAlias = alias;\nnextAlias.output = 'server';\nexport default config;\n",
    )
    .expect("config should be written");

    let document = assertions::parse_document(root.path(), rel_path)
        .expect("astro config document should exist");

    assertions::assert_invalid_document(&document, "could not reduce exported Astro config");
}

#[test]
fn mutated_integrations_array_identifier_is_invalid() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "import mdx from '@astrojs/mdx';\nconst integrations = [mdx()];\nintegrations.pop();\nexport default { integrations };\n",
    )
    .expect("config should be written");

    let document = assertions::parse_document(root.path(), rel_path)
        .expect("astro config document should exist");

    assertions::assert_invalid_document(
        &document,
        "shorthand `integrations` must resolve to an unmutated const binding",
    );
}

#[test]
fn dynamic_integration_options_are_invalid() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "import checks from '@nuasite/checks';\nconst dynamic = getChecksOptions();\nexport default { integrations: [checks(dynamic)] };\n",
    )
    .expect("config should be written");

    let document = assertions::parse_document(root.path(), rel_path)
        .expect("astro config document should exist");

    assertions::assert_invalid_document(&document, "must resolve to static values");
}

#[test]
fn spread_integration_options_are_invalid() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "import checks from '@nuasite/checks';\nconst dynamic = getChecksOptions();\nexport default { integrations: [checks({ mode: 'full', ...dynamic })] };\n",
    )
    .expect("config should be written");

    let document = assertions::parse_document(root.path(), rel_path)
        .expect("astro config document should exist");

    assertions::assert_invalid_document(&document, "must not contain spread properties");
}

#[test]
fn named_alias_and_imported_custom_check_are_preserved() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "import { checks as siteChecks } from '@nuasite/checks';\nimport { structuredDataPresentCheck } from 'g3ts-astro-nuasite-checks';\nexport default { integrations: [siteChecks({ customChecks: [structuredDataPresentCheck] })] };\n",
    )
    .expect("config should be written");

    let document = assertions::parse_document(root.path(), rel_path)
        .expect("astro config document should exist");
    let AstroConfigParseState::Parsed(snapshot) = &document.typed else {
        panic!("expected parsed document, got {document:?}");
    };
    let integration = snapshot
        .integrations
        .iter()
        .find(|integration| integration.source_module.as_deref() == Some("@nuasite/checks"))
        .expect("checks integration should be present");
    assert_eq!(integration.name.as_deref(), Some("siteChecks"));
    assert_eq!(integration.imported_name.as_deref(), Some("checks"));

    let options = integration
        .call
        .as_ref()
        .and_then(|call| call.first_arg.as_ref())
        .expect("checks options should be preserved");
    let AstroStaticValue::Object(properties) = options else {
        panic!("expected object options, got {options:?}");
    };
    let custom_checks = properties
        .iter()
        .find(|property| property.key == "customChecks")
        .expect("customChecks should be present");
    let AstroStaticValue::Array(values) = &custom_checks.value else {
        panic!("expected customChecks array, got {:?}", custom_checks.value);
    };
    assert!(matches!(
        values.first(),
        Some(AstroStaticValue::ImportedIdentifier {
            local_name,
            source_module: Some(source_module),
            imported_name: Some(imported_name),
        }) if local_name == "structuredDataPresentCheck"
            && source_module == "g3ts-astro-nuasite-checks"
            && imported_name == "structuredDataPresentCheck"
    ));
}

#[test]
fn namespace_imports_are_distinct_from_named_imports() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "astro.config.mjs";
    std::fs::write(
        root.path().join(rel_path),
        "import * as nuasite from '@nuasite/checks';\nexport default { integrations: [nuasite.checks({ mode: 'full' })] };\n",
    )
    .expect("config should be written");

    let document = assertions::parse_document(root.path(), rel_path)
        .expect("astro config document should exist");
    let AstroConfigParseState::Parsed(snapshot) = &document.typed else {
        panic!("expected parsed document, got {document:?}");
    };
    let integration = snapshot
        .integrations
        .iter()
        .find(|integration| integration.source_module.as_deref() == Some("@nuasite/checks"))
        .expect("namespace integration should be recorded");
    assert_eq!(integration.name.as_deref(), Some("checks"));
    assert_eq!(integration.imported_name.as_deref(), Some("*"));
}

#[test]
fn runtime_source_import_detection_accepts_runtime_imports_and_exports() {
    for (rel_path, source) in [
        (
            "src/content/adapter.ts",
            "import { getEntry } from 'astro:content';\nexport const loader = getEntry;\n",
        ),
        (
            "src/content/reexport.ts",
            "export { getEntry } from 'astro:content';\n",
        ),
        (
            "src/content/export-all.ts",
            "export * from 'astro:content';\n",
        ),
    ] {
        let root = tempfile::tempdir().expect("tempdir should be created");
        let abs_path = root.path().join(rel_path);
        std::fs::create_dir_all(abs_path.parent().expect("test path should have parent"))
            .expect("source directory should be created");
        std::fs::write(abs_path, source).expect("source should be written");

        assert!(
            super::super::module_has_runtime_source_import(root.path(), rel_path, "astro:content")
                .expect("source import detection should parse"),
            "runtime import should be detected for {rel_path}"
        );
    }
}

#[test]
fn runtime_source_import_detection_rejects_type_only_imports() {
    let root = tempfile::tempdir().expect("tempdir should be created");
    let rel_path = "src/content/adapter.ts";
    let abs_path = root.path().join(rel_path);
    std::fs::create_dir_all(abs_path.parent().expect("test path should have parent"))
        .expect("source directory should be created");
    std::fs::write(
        abs_path,
        "import type { CollectionEntry } from 'astro:content';\nexport type Entry = CollectionEntry<'posts'>;\n",
    )
    .expect("source should be written");

    assert!(
        !super::super::module_has_runtime_source_import(root.path(), rel_path, "astro:content")
            .expect("source import detection should parse"),
        "type-only import must not satisfy runtime import detection"
    );
}
