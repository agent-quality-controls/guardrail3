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
    assertions::assert_project_service(
        &snapshot,
        EslintProbeKind::AstroContentConfig,
        Some(true),
    );
    assertions::assert_project_service(&snapshot, EslintProbeKind::JsSource, Some(false));
    assertions::assert_plugins(
        &snapshot,
        EslintProbeKind::TsSource,
        &["@typescript-eslint", "react"],
    );
    assertions::assert_plugins(&snapshot, EslintProbeKind::MdxContent, &["astro-pipeline"]);
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
    assertions::assert_report_unused_disable_directives(
        &snapshot,
        EslintProbeKind::JsSource,
        None,
    );
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
    let probes = vec![probe(
        EslintProbeKind::TsSource,
        "src/malformed-option.ts",
    )];

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
    fs::write(
        root.path().join("src/content/posts/post.mdx"),
        "# Post\n",
    )
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
        root.path().join("node_modules/eslint/index.js"),
        r#"const path = require("node:path");

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
        plugins: { "astro-pipeline": {} },
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
