use std::fs;

use eslint_config_parser_runtime_assertions::parser as assertions;
use tempfile::TempDir;

use crate::parser::{
    EslintConfigFileKind, EslintProbeKind, EslintProbeTarget, EslintReportUnusedSetting,
    EslintRuleSeverity,
};

#[test]
fn parses_effective_config_probes_via_node_helper() {
    let root = fake_workspace();
    let probes = vec![
        probe(EslintProbeKind::AstroSource, "src/pages/index.astro"),
        probe(EslintProbeKind::TsSource, "src/index.ts"),
        probe(EslintProbeKind::TsxSource, "src/app/page.tsx"),
        probe(EslintProbeKind::MdxContent, "src/content/posts/post.mdx"),
        probe(EslintProbeKind::AstroContentConfig, "src/content.config.ts"),
        probe(EslintProbeKind::TsTest, "src/index.test.ts"),
        probe(EslintProbeKind::JsSource, "scripts/build.js"),
        probe(EslintProbeKind::ConfigFile, "eslint.config.mjs"),
    ];

    let snapshot = crate::parser::parse(root.path(), "eslint.config.mjs", &probes)
        .expect("parse should succeed for fake eslint workspace");

    assertions::assert_selected_config(&snapshot, "eslint.config.mjs", EslintConfigFileKind::Mjs);
    assertions::assert_probe_kinds(
        &snapshot,
        &[
            EslintProbeKind::AstroSource,
            EslintProbeKind::TsSource,
            EslintProbeKind::TsxSource,
            EslintProbeKind::MdxContent,
            EslintProbeKind::AstroContentConfig,
            EslintProbeKind::TsTest,
            EslintProbeKind::JsSource,
            EslintProbeKind::ConfigFile,
        ],
    );
    assertions::assert_project_service(&snapshot, EslintProbeKind::AstroSource, Some(false));
    assertions::assert_project_service(&snapshot, EslintProbeKind::TsSource, Some(true));
    assertions::assert_project_service(&snapshot, EslintProbeKind::TsxSource, Some(true));
    assertions::assert_project_service(&snapshot, EslintProbeKind::MdxContent, Some(false));
    assertions::assert_project_service(&snapshot, EslintProbeKind::AstroContentConfig, Some(true));
    assertions::assert_project_service(&snapshot, EslintProbeKind::JsSource, Some(false));
    assertions::assert_plugins(
        &snapshot,
        EslintProbeKind::TsSource,
        &["@typescript-eslint", "react"],
    );
    assertions::assert_plugins(&snapshot, EslintProbeKind::MdxContent, &["astro-pipeline"]);
    assertions::assert_plugin_meta_name(
        &snapshot,
        EslintProbeKind::MdxContent,
        "astro-pipeline",
        "g3ts-eslint-plugin-astro-pipeline",
    );
    assertions::assert_plugin_package_name(
        &snapshot,
        EslintProbeKind::MdxContent,
        "astro-pipeline",
        "g3ts-eslint-plugin-astro-pipeline",
    );
    assertions::assert_no_inline_config(&snapshot, EslintProbeKind::TsSource, Some(true));
    assertions::assert_report_unused_disable_directives(
        &snapshot,
        EslintProbeKind::TsSource,
        Some(EslintReportUnusedSetting::Error),
    );
    assertions::assert_report_unused_inline_configs(
        &snapshot,
        EslintProbeKind::TsSource,
        Some(EslintReportUnusedSetting::Error),
    );
    assertions::assert_no_inline_config(&snapshot, EslintProbeKind::JsSource, None);
    assertions::assert_report_unused_disable_directives(&snapshot, EslintProbeKind::JsSource, None);
    assertions::assert_report_unused_inline_configs(&snapshot, EslintProbeKind::JsSource, None);
    assertions::assert_rule_severity(
        &snapshot,
        EslintProbeKind::AstroSource,
        "astro/no-set-html-directive",
        EslintRuleSeverity::Error,
    );
    assertions::assert_rule_severity(
        &snapshot,
        EslintProbeKind::TsSource,
        "@typescript-eslint/no-explicit-any",
        EslintRuleSeverity::Error,
    );
    assertions::assert_rule_options_len(&snapshot, EslintProbeKind::TsSource, "max-lines", 1);
    assertions::assert_rule_severity(
        &snapshot,
        EslintProbeKind::TsTest,
        "@typescript-eslint/no-explicit-any",
        EslintRuleSeverity::Off,
    );
    assertions::assert_rule_severity(
        &snapshot,
        EslintProbeKind::TsxSource,
        "@typescript-eslint/no-explicit-any",
        EslintRuleSeverity::Error,
    );
    assertions::assert_rule_severity(
        &snapshot,
        EslintProbeKind::JsSource,
        "no-console",
        EslintRuleSeverity::Error,
    );
}

#[test]
fn resolves_plugin_package_identity_from_selected_config_location() {
    let root = TempDir::new().expect("tempdir should be created");
    fs::create_dir_all(root.path().join("apps/site/src"))
        .expect("nested app source directory should be created");
    fs::create_dir_all(root.path().join("apps/site/node_modules/eslint"))
        .expect("nested fake eslint module directory should be created");
    fs::create_dir_all(
        root.path()
            .join("apps/site/node_modules/g3ts-eslint-plugin-astro-pipeline"),
    )
    .expect("nested fake astro pipeline plugin module directory should be created");
    fs::write(
        root.path().join("apps/site/eslint.config.mjs"),
        "import astroPipeline from \"g3ts-eslint-plugin-astro-pipeline\";\nexport default [{ plugins: { \"astro-pipeline\": astroPipeline } }];\n",
    )
    .expect("nested eslint config should be written");
    fs::write(
        root.path().join("apps/site/src/index.ts"),
        "export const value = 1;\n",
    )
    .expect("nested source should be written");
    fs::write(
        root.path()
            .join("apps/site/node_modules/eslint/package.json"),
        "{\n  \"name\": \"eslint\",\n  \"version\": \"0.0.0-test\",\n  \"main\": \"index.js\"\n}\n",
    )
    .expect("nested fake eslint package manifest should be written");
    fs::write(
        root.path()
            .join("apps/site/node_modules/g3ts-eslint-plugin-astro-pipeline/package.json"),
        "{\n  \"name\": \"g3ts-eslint-plugin-astro-pipeline\",\n  \"version\": \"0.0.0-test\",\n  \"main\": \"index.js\"\n}\n",
    )
    .expect("nested fake astro pipeline package manifest should be written");
    fs::write(
        root.path()
            .join("apps/site/node_modules/g3ts-eslint-plugin-astro-pipeline/index.js"),
        "module.exports = { meta: { name: \"g3ts-eslint-plugin-astro-pipeline\" } };\n",
    )
    .expect("nested fake astro pipeline module should be written");
    fs::write(
        root.path().join("apps/site/node_modules/eslint/index.js"),
        r#"const astroPipeline = require("g3ts-eslint-plugin-astro-pipeline");

class ESLint {
  async isPathIgnored(_filePath) {
    return false;
  }

  async calculateConfigForFile(_filePath) {
    return {
      plugins: { "astro-pipeline": astroPipeline },
      rules: { "astro-pipeline/no-velite-imports": "error" },
    };
  }
}

module.exports = { ESLint };
"#,
    )
    .expect("nested fake eslint module should be written");

    let snapshot = crate::parser::parse(
        root.path(),
        "apps/site/eslint.config.mjs",
        &[probe(EslintProbeKind::TsSource, "apps/site/src/index.ts")],
    )
    .expect("parse should resolve nested config-local package identity");

    assertions::assert_plugin_package_name(
        &snapshot,
        EslintProbeKind::TsSource,
        "astro-pipeline",
        "g3ts-eslint-plugin-astro-pipeline",
    );
}

#[test]
fn resolves_plugin_package_identity_from_namespace_wrapped_astro_plugin() {
    let root = TempDir::new().expect("tempdir should be created");
    fs::create_dir_all(root.path().join("src/pages"))
        .expect("astro source directory should be created");
    fs::create_dir_all(root.path().join("node_modules/eslint"))
        .expect("fake eslint module directory should be created");
    fs::create_dir_all(root.path().join("node_modules/eslint-plugin-astro"))
        .expect("fake astro plugin module directory should be created");
    fs::write(
        root.path().join("eslint.config.mjs"),
        "import * as astro from \"eslint-plugin-astro\";\nexport default [{ plugins: { astro } }];\n",
    )
    .expect("eslint config should be written");
    fs::write(
        root.path().join("src/pages/index.astro"),
        "---\n---\n<html></html>\n",
    )
    .expect("astro source should be written");
    fs::write(
        root.path().join("node_modules/eslint/package.json"),
        "{\n  \"name\": \"eslint\",\n  \"version\": \"0.0.0-test\",\n  \"main\": \"index.js\"\n}\n",
    )
    .expect("fake eslint package manifest should be written");
    fs::write(
        root.path()
            .join("node_modules/eslint-plugin-astro/package.json"),
        "{\n  \"name\": \"eslint-plugin-astro\",\n  \"version\": \"0.0.0-test\",\n  \"main\": \"index.js\"\n}\n",
    )
    .expect("fake astro plugin package manifest should be written");
    fs::write(
        root.path()
            .join("node_modules/eslint-plugin-astro/index.js"),
        r#"module.exports = {
  meta: { name: "eslint-plugin-astro" },
  rules: {
    "valid-compile": {
      meta: {
        docs: { description: "real package rule" },
        schema: []
      },
      create() { return {}; }
    }
  }
};
"#,
    )
    .expect("fake astro plugin module should be written");
    fs::write(
        root.path().join("node_modules/eslint/index.js"),
        r#"const astroDefault = require("eslint-plugin-astro");
const astro = { ...astroDefault, default: astroDefault };

class ESLint {
  async isPathIgnored(_filePath) {
    return false;
  }

  async calculateConfigForFile(_filePath) {
    return {
      plugins: { astro },
      rules: { "astro/valid-compile": "error" },
    };
  }
}

module.exports = { ESLint };
"#,
    )
    .expect("fake eslint module should be written");

    let snapshot = crate::parser::parse(
        root.path(),
        "eslint.config.mjs",
        &[probe(EslintProbeKind::AstroSource, "src/pages/index.astro")],
    )
    .expect("parse should resolve namespace-wrapped astro plugin package identity");

    assertions::assert_plugin_package_name(
        &snapshot,
        EslintProbeKind::AstroSource,
        "astro",
        "eslint-plugin-astro",
    );
}

#[test]
fn resolves_plugin_package_identity_from_namespace_wrapped_mdx_plugin() {
    let root = TempDir::new().expect("tempdir should be created");
    fs::create_dir_all(root.path().join("content/posts"))
        .expect("content directory should be created");
    fs::create_dir_all(root.path().join("node_modules/eslint"))
        .expect("fake eslint module directory should be created");
    fs::create_dir_all(root.path().join("node_modules/eslint-plugin-mdx"))
        .expect("fake mdx plugin module directory should be created");
    fs::write(
        root.path().join("eslint.config.mjs"),
        "import * as mdx from \"eslint-plugin-mdx\";\nexport default [{ plugins: { ...mdx.configs.flat.plugins } }];\n",
    )
    .expect("eslint config should be written");
    fs::write(root.path().join("content/posts/post.mdx"), "# Post\n")
        .expect("mdx content should be written");
    fs::write(
        root.path().join("node_modules/eslint/package.json"),
        "{\n  \"name\": \"eslint\",\n  \"version\": \"0.0.0-test\",\n  \"main\": \"index.js\"\n}\n",
    )
    .expect("fake eslint package manifest should be written");
    fs::write(
        root.path()
            .join("node_modules/eslint-plugin-mdx/package.json"),
        "{\n  \"name\": \"eslint-plugin-mdx\",\n  \"version\": \"0.0.0-test\",\n  \"main\": \"index.js\"\n}\n",
    )
    .expect("fake mdx plugin package manifest should be written");
    fs::write(
        root.path().join("node_modules/eslint-plugin-mdx/index.js"),
        "module.exports = { meta: { name: \"eslint-plugin-mdx\" }, rules: { remark: {} } };\n",
    )
    .expect("fake mdx plugin module should be written");
    fs::write(
        root.path().join("node_modules/eslint/index.js"),
        r#"const mdxDefault = require("eslint-plugin-mdx");
const mdx = { ...mdxDefault, default: mdxDefault };

class ESLint {
  async isPathIgnored(_filePath) {
    return false;
  }

  async calculateConfigForFile(_filePath) {
    return {
      plugins: { mdx },
      rules: { "mdx/remark": "error" },
    };
  }
}

module.exports = { ESLint };
"#,
    )
    .expect("fake eslint module should be written");

    let snapshot = crate::parser::parse(
        root.path(),
        "eslint.config.mjs",
        &[probe(EslintProbeKind::MdxContent, "content/posts/post.mdx")],
    )
    .expect("parse should resolve namespace-wrapped mdx plugin package identity");

    assertions::assert_plugin_package_name(
        &snapshot,
        EslintProbeKind::MdxContent,
        "mdx",
        "eslint-plugin-mdx",
    );
}

#[test]
fn resolves_plugin_package_identity_from_esm_only_import_export() {
    let root = TempDir::new().expect("tempdir should be created");
    fs::create_dir_all(root.path().join("src")).expect("src directory should be created");
    fs::create_dir_all(root.path().join("node_modules/eslint"))
        .expect("fake eslint module directory should be created");
    fs::create_dir_all(
        root.path()
            .join("node_modules/g3ts-eslint-plugin-style-policy"),
    )
    .expect("fake style policy plugin module directory should be created");
    fs::write(
        root.path().join("eslint.config.mjs"),
        "import stylePolicy from \"g3ts-eslint-plugin-style-policy\";\nexport default [{ plugins: { \"style-policy\": stylePolicy } }];\n",
    )
    .expect("eslint config should be written");
    fs::write(
        root.path().join("src/index.tsx"),
        "export function Page() { return <main className=\"text-black\" />; }\n",
    )
    .expect("tsx source should be written");
    fs::write(
        root.path().join("node_modules/eslint/package.json"),
        "{\n  \"name\": \"eslint\",\n  \"version\": \"0.0.0-test\",\n  \"main\": \"index.js\"\n}\n",
    )
    .expect("fake eslint package manifest should be written");
    fs::write(
        root.path()
            .join("node_modules/g3ts-eslint-plugin-style-policy/package.json"),
        "{\n  \"name\": \"g3ts-eslint-plugin-style-policy\",\n  \"version\": \"0.0.0-test\",\n  \"type\": \"module\",\n  \"exports\": {\n    \".\": {\n      \"import\": \"./index.js\"\n    }\n  }\n}\n",
    )
    .expect("fake style policy package manifest should be written");
    fs::write(
        root.path()
            .join("node_modules/g3ts-eslint-plugin-style-policy/index.js"),
        r#"const rule = {
  meta: { docs: { description: "deny configured Tailwind tokens" }, schema: [] },
  create() { return {}; }
};

export default {
  meta: { name: "g3ts-eslint-plugin-style-policy" },
  rules: { "no-denied-class-tokens": rule },
};
"#,
    )
    .expect("fake style policy plugin module should be written");
    fs::write(
        root.path().join("node_modules/eslint/index.js"),
        r#"class ESLint {
  async isPathIgnored(_filePath) {
    return false;
  }

  async calculateConfigForFile(_filePath) {
    const stylePolicy = (await import("g3ts-eslint-plugin-style-policy")).default;
    return {
      plugins: { "style-policy": stylePolicy },
      rules: {
        "style-policy/no-denied-class-tokens": ["error", { denyList: ["text-black"] }],
      },
    };
  }
}

module.exports = { ESLint };
"#,
    )
    .expect("fake eslint module should be written");

    let snapshot = crate::parser::parse(
        root.path(),
        "eslint.config.mjs",
        &[probe(EslintProbeKind::TsxSource, "src/index.tsx")],
    )
    .expect("parse should resolve ESM-only plugin package identity");

    assertions::assert_plugin_package_name(
        &snapshot,
        EslintProbeKind::TsxSource,
        "style-policy",
        "g3ts-eslint-plugin-style-policy",
    );
}

#[test]
fn rejects_spoofed_astro_plugin_when_imported_binding_is_only_object_key() {
    let root = TempDir::new().expect("tempdir should be created");
    fs::create_dir_all(root.path().join("src/pages"))
        .expect("astro source directory should be created");
    fs::create_dir_all(root.path().join("node_modules/eslint"))
        .expect("fake eslint module directory should be created");
    fs::create_dir_all(root.path().join("node_modules/eslint-plugin-astro"))
        .expect("fake astro plugin module directory should be created");
    fs::write(
        root.path().join("eslint.config.mjs"),
        r#"import astro from "eslint-plugin-astro";

const fakeAstro = {
  meta: { name: "eslint-plugin-astro" },
  rules: { "valid-compile": {} }
};

export default [{
  plugins: { astro: fakeAstro },
  rules: { "astro/valid-compile": "error" }
}];
"#,
    )
    .expect("eslint config should be written");
    fs::write(
        root.path().join("src/pages/index.astro"),
        "---\n---\n<html></html>\n",
    )
    .expect("astro source should be written");
    fs::write(
        root.path().join("node_modules/eslint/package.json"),
        "{\n  \"name\": \"eslint\",\n  \"version\": \"0.0.0-test\",\n  \"main\": \"index.js\"\n}\n",
    )
    .expect("fake eslint package manifest should be written");
    fs::write(
        root.path()
            .join("node_modules/eslint-plugin-astro/package.json"),
        "{\n  \"name\": \"eslint-plugin-astro\",\n  \"version\": \"0.0.0-test\",\n  \"main\": \"index.js\"\n}\n",
    )
    .expect("fake astro plugin package manifest should be written");
    fs::write(
        root.path()
            .join("node_modules/eslint-plugin-astro/index.js"),
        r#"module.exports = {
  meta: { name: "eslint-plugin-astro" },
  rules: {
    "valid-compile": {
      meta: {
        docs: { description: "real package rule" },
        schema: []
      },
      create() { return {}; }
    }
  }
};
"#,
    )
    .expect("fake astro plugin package should be written");
    fs::write(
        root.path().join("node_modules/eslint/index.js"),
        r#"const fakeAstro = {
  meta: { name: "eslint-plugin-astro" },
  rules: { "valid-compile": {} }
};

class ESLint {
  async isPathIgnored(_filePath) {
    return false;
  }

  async calculateConfigForFile(_filePath) {
    return {
      plugins: { astro: fakeAstro },
      rules: { "astro/valid-compile": "error" },
    };
  }
}

module.exports = { ESLint };
"#,
    )
    .expect("fake eslint module should be written");

    let snapshot = crate::parser::parse(
        root.path(),
        "eslint.config.mjs",
        &[probe(EslintProbeKind::AstroSource, "src/pages/index.astro")],
    )
    .expect("parse should not trust imported binding used only as an object key");
    let probe = snapshot
        .probes
        .iter()
        .find(|probe| probe.probe == EslintProbeKind::AstroSource)
        .expect("astro probe should exist");

    assert!(
        !probe
            .plugin_package_names
            .get("astro")
            .is_some_and(|package_names| package_names
                .iter()
                .any(|name| name == "eslint-plugin-astro")),
        "object-key import reference must not prove package identity: {probe:?}"
    );
}

#[test]
fn rejects_spoofed_plugin_package_identity_with_matching_shape() {
    let root = TempDir::new().expect("tempdir should be created");
    fs::create_dir_all(root.path().join("src")).expect("src directory should be created");
    fs::create_dir_all(root.path().join("node_modules/eslint"))
        .expect("fake eslint module directory should be created");
    fs::create_dir_all(
        root.path()
            .join("node_modules/g3ts-eslint-plugin-astro-pipeline"),
    )
    .expect("fake astro pipeline plugin module directory should be created");
    fs::write(
        root.path().join("eslint.config.mjs"),
        r#"import astroPipeline from "g3ts-eslint-plugin-astro-pipeline";

const noopRule = { create() { return {}; } };
const fakeAstroPipeline = {
  meta: { name: "g3ts-eslint-plugin-astro-pipeline" },
  rules: {
    "no-authored-content-fs-read": noopRule,
    "no-authored-content-glob": noopRule,
    "no-authored-content-imports": noopRule,
    "no-content-data-modules-in-routes": noopRule,
    "no-direct-astro-content-in-routes": noopRule,
            "no-runtime-mdx-eval": noopRule,
            "no-side-loader-imports": noopRule,
            "no-velite-imports": noopRule,
            "require-approved-content-adapter-in-routes": noopRule
  }
};

export default [{ plugins: { "astro-pipeline": fakeAstroPipeline } }];
"#,
    )
    .expect("eslint config should be written");
    fs::write(
        root.path().join("src/index.ts"),
        "export const value = 1;\n",
    )
    .expect("source file should be written");
    fs::write(
        root.path().join("node_modules/eslint/package.json"),
        "{\n  \"name\": \"eslint\",\n  \"version\": \"0.0.0-test\",\n  \"main\": \"index.js\"\n}\n",
    )
    .expect("fake eslint package manifest should be written");
    fs::write(
        root.path()
            .join("node_modules/g3ts-eslint-plugin-astro-pipeline/package.json"),
        "{\n  \"name\": \"g3ts-eslint-plugin-astro-pipeline\",\n  \"version\": \"0.0.0-test\",\n  \"main\": \"index.js\"\n}\n",
    )
    .expect("fake astro pipeline package manifest should be written");
    fs::write(
        root.path()
            .join("node_modules/g3ts-eslint-plugin-astro-pipeline/index.js"),
        r#"const noopRule = { create() { return {}; } };
module.exports = {
  meta: { name: "g3ts-eslint-plugin-astro-pipeline" },
  rules: {
    "no-authored-content-fs-read": { ...noopRule, meta: { docs: { description: "package rule" } } },
    "no-authored-content-glob": { ...noopRule, meta: { docs: { description: "package rule" } } },
    "no-authored-content-imports": { ...noopRule, meta: { docs: { description: "package rule" } } },
    "no-content-data-modules-in-routes": { ...noopRule, meta: { docs: { description: "package rule" } } },
    "no-direct-astro-content-in-routes": { ...noopRule, meta: { docs: { description: "package rule" } } },
    "no-runtime-mdx-eval": { ...noopRule, meta: { docs: { description: "package rule" } } },
    "no-side-loader-imports": { ...noopRule, meta: { docs: { description: "package rule" } } },
    "no-velite-imports": { ...noopRule, meta: { docs: { description: "package rule" } } },
    "require-approved-content-adapter-in-routes": { ...noopRule, meta: { docs: { description: "package rule" } } }
  }
};
"#,
    )
    .expect("fake astro pipeline package should be written");
    fs::write(
        root.path().join("node_modules/eslint/index.js"),
        r#"const noopRule = { create() { return {}; } };
const fakeAstroPipeline = {
  meta: { name: "g3ts-eslint-plugin-astro-pipeline" },
  rules: {
    "no-authored-content-fs-read": noopRule,
    "no-authored-content-glob": noopRule,
    "no-authored-content-imports": noopRule,
    "no-content-data-modules-in-routes": noopRule,
    "no-direct-astro-content-in-routes": noopRule,
    "no-runtime-mdx-eval": noopRule,
    "no-side-loader-imports": noopRule,
    "no-velite-imports": noopRule,
    "require-approved-content-adapter-in-routes": noopRule
  }
};

class ESLint {
  async isPathIgnored(_filePath) {
    return false;
  }

  async calculateConfigForFile(_filePath) {
    return {
      plugins: { "astro-pipeline": fakeAstroPipeline },
      rules: { "astro-pipeline/no-velite-imports": "error" },
    };
  }
}

module.exports = { ESLint };
"#,
    )
    .expect("fake eslint module should be written");

    let snapshot = crate::parser::parse(
        root.path(),
        "eslint.config.mjs",
        &[probe(EslintProbeKind::TsSource, "src/index.ts")],
    )
    .expect("parse should succeed without trusting fake plugin package identity");
    let probe = snapshot
        .probes
        .iter()
        .find(|probe| probe.probe == EslintProbeKind::TsSource)
        .expect("ts probe should exist");

    assert!(
        !probe
            .plugin_package_names
            .get("astro-pipeline")
            .is_some_and(|package_names| package_names
                .iter()
                .any(|name| name == "g3ts-eslint-plugin-astro-pipeline")),
        "matching meta/rule shape must not prove package identity: {probe:?}"
    );
}

#[test]
fn rejects_raw_package_identity_when_effective_plugin_is_different_object() {
    let root = TempDir::new().expect("tempdir should be created");
    fs::create_dir_all(root.path().join("src")).expect("src directory should be created");
    fs::create_dir_all(root.path().join("node_modules/eslint"))
        .expect("fake eslint module directory should be created");
    fs::create_dir_all(
        root.path()
            .join("node_modules/g3ts-eslint-plugin-astro-pipeline"),
    )
    .expect("fake astro pipeline plugin module directory should be created");
    fs::write(
        root.path().join("eslint.config.mjs"),
        r#"import realAstroPipeline from "g3ts-eslint-plugin-astro-pipeline";

const noopRule = { create() { return {}; } };
const fakeAstroPipeline = {
  meta: { name: "g3ts-eslint-plugin-astro-pipeline" },
  rules: { "no-velite-imports": noopRule }
};

export default [
  { files: ["ignored/**"], plugins: { "astro-pipeline": realAstroPipeline } },
  { files: ["src/**"], plugins: { "astro-pipeline": fakeAstroPipeline } }
];
"#,
    )
    .expect("eslint config should be written");
    fs::write(
        root.path().join("src/index.ts"),
        "export const value = 1;\n",
    )
    .expect("source file should be written");
    fs::write(
        root.path().join("node_modules/eslint/package.json"),
        "{\n  \"name\": \"eslint\",\n  \"version\": \"0.0.0-test\",\n  \"main\": \"index.js\"\n}\n",
    )
    .expect("fake eslint package manifest should be written");
    fs::write(
        root.path()
            .join("node_modules/g3ts-eslint-plugin-astro-pipeline/package.json"),
        "{\n  \"name\": \"g3ts-eslint-plugin-astro-pipeline\",\n  \"version\": \"0.0.0-test\",\n  \"main\": \"index.js\"\n}\n",
    )
    .expect("fake astro pipeline package manifest should be written");
    fs::write(
        root.path()
            .join("node_modules/g3ts-eslint-plugin-astro-pipeline/index.js"),
        "module.exports = { meta: { name: \"g3ts-eslint-plugin-astro-pipeline\" } };\n",
    )
    .expect("fake astro pipeline package should be written");
    fs::write(
        root.path().join("node_modules/eslint/index.js"),
        r#"const noopRule = { create() { return {}; } };
const fakeAstroPipeline = {
  meta: { name: "g3ts-eslint-plugin-astro-pipeline" },
  rules: { "no-velite-imports": noopRule }
};

class ESLint {
  async isPathIgnored(_filePath) {
    return false;
  }

  async calculateConfigForFile(_filePath) {
    return {
      plugins: { "astro-pipeline": fakeAstroPipeline },
      rules: { "astro-pipeline/no-velite-imports": "error" },
    };
  }
}

module.exports = { ESLint };
"#,
    )
    .expect("fake eslint module should be written");

    let snapshot = crate::parser::parse(
        root.path(),
        "eslint.config.mjs",
        &[probe(EslintProbeKind::TsSource, "src/index.ts")],
    )
    .expect("parse should not smear raw package identity onto a fake effective plugin");
    let probe = snapshot
        .probes
        .iter()
        .find(|probe| probe.probe == EslintProbeKind::TsSource)
        .expect("ts probe should exist");

    assert!(
        !probe
            .plugin_package_names
            .get("astro-pipeline")
            .is_some_and(|package_names| package_names
                .iter()
                .any(|name| name == "g3ts-eslint-plugin-astro-pipeline")),
        "raw config package identity must not apply to a different effective plugin: {probe:?}"
    );
}

#[test]
fn boolean_linter_option_true_is_warn() {
    let root = fake_workspace();
    let probes = vec![probe(EslintProbeKind::TsSource, "src/boolean-linter.ts")];

    let snapshot = crate::parser::parse(root.path(), "eslint.config.mjs", &probes)
        .expect("parse should succeed for boolean linter option probe");

    assertions::assert_report_unused_disable_directives(
        &snapshot,
        EslintProbeKind::TsSource,
        Some(EslintReportUnusedSetting::Warn),
    );
    assertions::assert_report_unused_inline_configs(
        &snapshot,
        EslintProbeKind::TsSource,
        Some(EslintReportUnusedSetting::Warn),
    );
}

#[test]
fn ignored_probes_are_retained() {
    let root = fake_workspace();
    let probes = vec![probe(EslintProbeKind::TsSource, "src/ignored/index.ts")];

    let snapshot = crate::parser::parse(root.path(), "eslint.config.mjs", &probes)
        .expect("parse should succeed for ignored probe");

    assertions::assert_probe_kinds(&snapshot, &[EslintProbeKind::TsSource]);
    assertions::assert_probe_ignored(&snapshot, "src/ignored/index.ts", true);
}

#[test]
fn helper_failures_surface_as_parse_errors() {
    let root = fake_workspace();
    let probes = vec![probe(EslintProbeKind::TsSource, "src/broken.ts")];

    let err = crate::parser::parse(root.path(), "eslint.config.mjs", &probes)
        .expect_err("parse should fail when fake eslint throws");

    assertions::assert_parse_error(&err);
}

#[test]
fn unsupported_linter_option_severity_surfaces_as_parse_error() {
    let root = fake_workspace();
    let probes = vec![probe(EslintProbeKind::TsSource, "src/malformed-option.ts")];

    let err = crate::parser::parse(root.path(), "eslint.config.mjs", &probes)
        .expect_err("parse should fail on unsupported linter option severity");

    assertions::assert_parse_error(&err);
}

#[test]
fn unsupported_rule_severity_surfaces_as_parse_error() {
    let root = fake_workspace();
    let probes = vec![probe(EslintProbeKind::TsSource, "src/malformed-rule.ts")];

    let err = crate::parser::parse(root.path(), "eslint.config.mjs", &probes)
        .expect_err("parse should fail on unsupported rule severity");

    assertions::assert_parse_error(&err);
}

#[test]
fn selected_config_kind_matches_all_supported_config_extensions() {
    let root = fake_workspace();

    for (rel_path, expected_kind) in [
        ("eslint.config.js", EslintConfigFileKind::Js),
        ("eslint.config.mjs", EslintConfigFileKind::Mjs),
        ("eslint.config.cjs", EslintConfigFileKind::Cjs),
        ("eslint.config.ts", EslintConfigFileKind::Ts),
        ("eslint.config.mts", EslintConfigFileKind::Mts),
        ("eslint.config.cts", EslintConfigFileKind::Cts),
    ] {
        std::fs::write(root.path().join(rel_path), "export default [];\n")
            .expect("config file should be written");

        let snapshot = crate::parser::parse(root.path(), rel_path, &[])
            .expect("parse should succeed for supported config kinds");

        assertions::assert_selected_config(&snapshot, rel_path, expected_kind);
    }
}

fn probe(probe: EslintProbeKind, rel_path: &str) -> EslintProbeTarget {
    EslintProbeTarget {
        probe,
        rel_path: rel_path.to_owned(),
    }
}

fn fake_workspace() -> TempDir {
    let root = TempDir::new().expect("tempdir should be created");
    fs::create_dir_all(root.path().join("src")).expect("src directory should be created");
    fs::create_dir_all(root.path().join("src/pages"))
        .expect("astro source directory should be created");
    fs::create_dir_all(root.path().join("src/content/posts"))
        .expect("content directory should be created");
    fs::create_dir_all(root.path().join("src/ignored"))
        .expect("ignored source directory should be created");
    fs::create_dir_all(root.path().join("scripts")).expect("scripts directory should be created");
    fs::create_dir_all(root.path().join("node_modules/eslint"))
        .expect("fake eslint module directory should be created");
    fs::create_dir_all(
        root.path()
            .join("node_modules/g3ts-eslint-plugin-astro-pipeline"),
    )
    .expect("fake astro pipeline plugin module directory should be created");

    fs::write(
        root.path().join("eslint.config.mjs"),
        "export default [];\n",
    )
    .expect("eslint config should be written");
    fs::write(
        root.path().join("src/index.ts"),
        "export const value = 1;\n",
    )
    .expect("ts source should be written");
    fs::write(
        root.path().join("src/index.test.ts"),
        "export const testValue = 1;\n",
    )
    .expect("test source should be written");
    fs::write(
        root.path().join("src/ignored/index.ts"),
        "export const ignoredValue = 1;\n",
    )
    .expect("ignored source should be written");
    fs::write(
        root.path().join("src/malformed-option.ts"),
        "export const malformedValue = 1;\n",
    )
    .expect("malformed source should be written");
    fs::write(
        root.path().join("src/malformed-rule.ts"),
        "export const malformedRuleValue = 1;\n",
    )
    .expect("malformed rule source should be written");
    fs::write(
        root.path().join("src/boolean-linter.ts"),
        "export const booleanLinterValue = 1;\n",
    )
    .expect("boolean linter source should be written");
    fs::write(
        root.path().join("src/content.config.ts"),
        "export const collections = {};\n",
    )
    .expect("content config should be written");
    fs::write(root.path().join("src/content/posts/post.mdx"), "# Post\n")
        .expect("mdx content should be written");
    fs::create_dir_all(root.path().join("src/app"))
        .expect("tsx source directory should be created");
    fs::write(
        root.path().join("src/app/page.tsx"),
        "export function Page() { return <main />; }\n",
    )
    .expect("tsx source should be written");
    fs::write(
        root.path().join("src/pages/index.astro"),
        "---\nconst title = 'Report';\n---\n<html><body>{title}</body></html>\n",
    )
    .expect("astro source should be written");
    fs::write(
        root.path().join("scripts/build.js"),
        "console.log('build');\n",
    )
    .expect("js source should be written");
    fs::write(
        root.path().join("node_modules/eslint/package.json"),
        "{\n  \"name\": \"eslint\",\n  \"version\": \"0.0.0-test\",\n  \"main\": \"index.js\"\n}\n",
    )
    .expect("fake eslint package manifest should be written");
    fs::write(
        root.path()
            .join("node_modules/g3ts-eslint-plugin-astro-pipeline/package.json"),
        "{\n  \"name\": \"g3ts-eslint-plugin-astro-pipeline\",\n  \"version\": \"0.0.0-test\",\n  \"main\": \"index.js\"\n}\n",
    )
    .expect("fake astro pipeline plugin manifest should be written");
    fs::write(
        root.path()
            .join("node_modules/g3ts-eslint-plugin-astro-pipeline/index.js"),
        "module.exports = { meta: { name: \"g3ts-eslint-plugin-astro-pipeline\" } };\n",
    )
    .expect("fake astro pipeline plugin module should be written");
    fs::write(
        root.path().join("node_modules/eslint/index.js"),
        r#"const path = require("node:path");
const astroPipeline = require("g3ts-eslint-plugin-astro-pipeline");

class ESLint {
  constructor(options) {
    this.cwd = options.cwd;
    this.overrideConfigFile = options.overrideConfigFile;
  }

  async isPathIgnored(filePath) {
    const rel = path.relative(this.cwd, filePath).split(path.sep).join("/");
    return rel.includes("/ignored/");
  }

  async calculateConfigForFile(filePath) {
    const rel = path.relative(this.cwd, filePath).split(path.sep).join("/");
    if (rel.includes("/ignored/")) {
      return undefined;
    }

    if (rel.endsWith("broken.ts")) {
      throw new Error("synthetic eslint failure");
    }

    if (rel.endsWith("malformed-option.ts")) {
      return {
        plugins: { "@typescript-eslint": {} },
        rules: { "@typescript-eslint/no-explicit-any": "error" },
        languageOptions: { parserOptions: { projectService: true } },
        linterOptions: {
          noInlineConfig: true,
          reportUnusedDisableDirectives: "fatal",
        },
      };
    }

    if (rel.endsWith("malformed-rule.ts")) {
      return {
        plugins: { "@typescript-eslint": {} },
        rules: { "@typescript-eslint/no-explicit-any": "fatal" },
        languageOptions: { parserOptions: { projectService: true } },
      };
    }

    if (rel.endsWith("boolean-linter.ts")) {
      return {
        plugins: { "@typescript-eslint": {} },
        rules: { "@typescript-eslint/no-explicit-any": "error" },
        languageOptions: { parserOptions: { projectService: true } },
        linterOptions: {
          noInlineConfig: true,
          reportUnusedDisableDirectives: true,
          reportUnusedInlineConfigs: true,
        },
      };
    }

    if (rel.endsWith(".js")) {
      return {
        plugins: { unicorn: {} },
        rules: { "no-console": "error" },
        languageOptions: { parserOptions: { projectService: false } },
      };
    }

    if (rel.endsWith(".astro")) {
      return {
        plugins: { astro: {} },
        rules: { "astro/no-set-html-directive": "error" },
        languageOptions: { parserOptions: { projectService: false } },
        linterOptions: {
          noInlineConfig: true,
          reportUnusedDisableDirectives: "error",
          reportUnusedInlineConfigs: "error",
        },
      };
    }

    if (rel.endsWith(".mdx")) {
      return {
        plugins: { "astro-pipeline": astroPipeline },
        rules: { "astro-pipeline/mdx-imports-from-approved-component-globs": "error" },
        languageOptions: { parserOptions: { projectService: false } },
        linterOptions: {
          noInlineConfig: true,
          reportUnusedDisableDirectives: "error",
          reportUnusedInlineConfigs: "error",
        },
      };
    }

    if (rel.includes(".test.")) {
      return {
        plugins: { "@typescript-eslint": {}, react: {} },
        rules: {
          "@typescript-eslint/no-explicit-any": "off",
          "max-lines": ["error", 400],
        },
        languageOptions: { parserOptions: { projectService: true } },
      };
    }

    return {
      plugins: { "@typescript-eslint": {}, react: {} },
        rules: {
          "@typescript-eslint/no-explicit-any": "error",
          "max-lines": ["warn", 200],
        },
        languageOptions: { parserOptions: { projectService: true } },
        linterOptions: {
          noInlineConfig: true,
          reportUnusedDisableDirectives: "error",
          reportUnusedInlineConfigs: "error",
        },
      };
    }
}

module.exports = { ESLint };
"#,
    )
    .expect("fake eslint module should be written");

    root
}
