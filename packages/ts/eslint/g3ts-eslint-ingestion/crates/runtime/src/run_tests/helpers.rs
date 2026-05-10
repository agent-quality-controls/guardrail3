#![expect(
    clippy::disallowed_methods,
    reason = "test-helper file: writes temp eslint config + source fixtures with std::fs to drive ingestion; production code uses the centralized fs module"
)]

use tempfile::TempDir;

use g3_workspace_crawl::{
    G3RsWorkspaceCrawl as G3WorkspaceCrawl, G3RsWorkspaceEntry as G3WorkspaceEntry,
    G3RsWorkspaceEntryKind as G3WorkspaceEntryKind,
    G3RsWorkspaceIgnoreState as G3WorkspaceIgnoreState, G3RsWorkspacePath as G3WorkspacePath,
};

pub(super) fn fake_eslint_workspace() -> TempDir {
    let root = TempDir::new().expect("tempdir should be created");
    std::fs::create_dir_all(root.path().join("src")).expect("src directory should be created");
    std::fs::create_dir_all(root.path().join("node_modules/eslint"))
        .expect("fake eslint module directory should be created");
    std::fs::write(
        root.path().join("eslint.config.mjs"),
        "export default [];\n",
    )
    .expect("eslint config should be written");
    std::fs::write(root.path().join("eslint.config.js"), "export default [];\n")
        .expect("js eslint config should be written");
    std::fs::write(
        root.path().join("src/index.ts"),
        "export const value = 1;\n",
    )
    .expect("ts source should be written");
    std::fs::write(
        root.path().join("node_modules/eslint/package.json"),
        "{\n  \"name\": \"eslint\",\n  \"version\": \"0.0.0-test\",\n  \"main\": \"index.js\"\n}\n",
    )
    .expect("fake eslint package manifest should be written");
    std::fs::write(
        root.path().join("node_modules/eslint/index.js"),
        r#"const path = require("node:path");

class ESLint {
  constructor(options) {
    this.cwd = options.cwd;
    this.overrideConfigFile = options.overrideConfigFile;
  }

  async isPathIgnored(_filePath) {
    return false;
  }

  async calculateConfigForFile(_filePath) {
    return {
      plugins: { "@typescript-eslint": {} },
      rules: {
        "@typescript-eslint/no-explicit-any": "error",
        "no-console": "error",
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

pub(super) fn crawl_with_entries(root: &TempDir, rel_paths: &[&str]) -> G3WorkspaceCrawl {
    G3WorkspaceCrawl {
        root_abs_path: root.path().to_path_buf(),
        entries: rel_paths
            .iter()
            .map(|rel_path| entry(root, rel_path))
            .collect(),
    }
}

fn entry(root: &TempDir, rel_path: &str) -> G3WorkspaceEntry {
    G3WorkspaceEntry {
        path: G3WorkspacePath {
            rel_path: rel_path.to_owned(),
            abs_path: root.path().join(rel_path),
        },
        kind: G3WorkspaceEntryKind::File,
        ignore_state: G3WorkspaceIgnoreState::Included,
        readable: true,
    }
}
