use std::fs;

use eslint_config_parser_runtime_assertions::parser as assertions;
use tempfile::TempDir;

use crate::parser::{EslintConfigFileKind, EslintProbeKind, EslintProbeTarget, EslintRuleSeverity};

#[test]
fn parses_effective_config_probes_via_node_helper() {
    let root = fake_workspace();
    let probes = vec![
        probe(EslintProbeKind::TsSource, "src/index.ts"),
        probe(EslintProbeKind::TsTest, "src/index.test.ts"),
        probe(EslintProbeKind::JsSource, "scripts/build.js"),
        probe(EslintProbeKind::ConfigFile, "eslint.config.mjs"),
    ];

    let snapshot = crate::parser::parse(root.path(), "eslint.config.mjs", &probes)
        .expect("parse should succeed for fake eslint workspace");

    assertions::assert_selected_config(&snapshot, "eslint.config.mjs", EslintConfigFileKind::Mjs);
    assertions::assert_project_service(&snapshot, EslintProbeKind::TsSource, Some(true));
    assertions::assert_project_service(&snapshot, EslintProbeKind::JsSource, Some(false));
    assertions::assert_rule_severity(
        &snapshot,
        EslintProbeKind::TsSource,
        "@typescript-eslint/no-explicit-any",
        EslintRuleSeverity::Error,
    );
    assertions::assert_rule_severity(
        &snapshot,
        EslintProbeKind::TsTest,
        "@typescript-eslint/no-explicit-any",
        EslintRuleSeverity::Off,
    );
    assertions::assert_rule_severity(
        &snapshot,
        EslintProbeKind::JsSource,
        "no-console",
        EslintRuleSeverity::Error,
    );
}

#[test]
fn helper_failures_surface_as_parse_errors() {
    let root = fake_workspace();
    let probes = vec![probe(EslintProbeKind::TsSource, "src/broken.ts")];

    let err = crate::parser::parse(root.path(), "eslint.config.mjs", &probes)
        .expect_err("parse should fail when fake eslint throws");

    assertions::assert_parse_error(&err);
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
    if (rel.endsWith("broken.ts")) {
      throw new Error("synthetic eslint failure");
    }

    if (rel.endsWith(".js")) {
      return {
        plugins: { unicorn: {} },
        rules: { "no-console": "error" },
        languageOptions: { parserOptions: { projectService: false } },
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
    };
  }
}

module.exports = { ESLint };
"#,
    )
    .expect("fake eslint module should be written");

    root
}
