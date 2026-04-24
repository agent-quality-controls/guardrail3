use tempfile::TempDir;

use g3_workspace_crawl::{
    G3RsWorkspaceCrawl as G3WorkspaceCrawl, G3RsWorkspaceEntry as G3WorkspaceEntry,
    G3RsWorkspaceEntryKind as G3WorkspaceEntryKind,
    G3RsWorkspaceIgnoreState as G3WorkspaceIgnoreState, G3RsWorkspacePath as G3WorkspacePath,
};

pub(super) fn fake_astro_workspace() -> TempDir {
    let root = TempDir::new().expect("tempdir should be created");

    std::fs::create_dir_all(root.path().join("src/pages"))
        .expect("pages directory should be created");
    std::fs::create_dir_all(root.path().join("src/lib")).expect("lib directory should be created");
    std::fs::create_dir_all(root.path().join("src/content/posts"))
        .expect("content directory should be created");
    std::fs::create_dir_all(root.path().join("node_modules/eslint"))
        .expect("fake eslint module directory should be created");

    std::fs::write(
        root.path().join("package.json"),
        "{\n  \"devDependencies\": {\n    \"astro\": \"1.0.0\",\n    \"eslint-plugin-astro\": \"1.0.0\",\n    \"eslint-plugin-astro-pipeline\": \"1.0.0\"\n  },\n  \"scripts\": {\n    \"check\": \"astro check\"\n  }\n}\n",
    )
    .expect("package manifest should be written");
    std::fs::write(
        root.path().join("astro.config.mjs"),
        "import node from '@astrojs/node';\nexport default { output: 'server', adapter: node({ mode: 'standalone' }) };\n",
    )
    .expect("astro config should be written");
    std::fs::write(
        root.path().join("src/content.config.ts"),
        "export default {};\n",
    )
    .expect("content config should be written");
    std::fs::write(
        root.path().join("eslint.config.mjs"),
        "export default [];\n",
    )
    .expect("eslint config should be written");
    std::fs::write(
        root.path().join(".syncpackrc"),
        "{\n  \"source\": [\"package.json\"],\n  \"versionGroups\": [\n    { \"dependencies\": [\"astro\"], \"dependencyTypes\": [\"prod\", \"dev\"], \"pinVersion\": \"6.1.9\" },\n    { \"dependencies\": [\"@astrojs/node\"], \"dependencyTypes\": [\"prod\", \"dev\"], \"pinVersion\": \"10.0.6\" },\n    { \"dependencies\": [\"@astrojs/react\"], \"dependencyTypes\": [\"prod\", \"dev\"], \"pinVersion\": \"5.0.4\" },\n    { \"dependencies\": [\"@astrojs/mdx\"], \"dependencyTypes\": [\"prod\", \"dev\"], \"pinVersion\": \"5.0.4\" },\n    { \"dependencies\": [\"@astrojs/check\"], \"dependencyTypes\": [\"prod\", \"dev\"], \"pinVersion\": \"0.9.8\" },\n    { \"dependencies\": [\"react\"], \"dependencyTypes\": [\"prod\", \"dev\"], \"pinVersion\": \"19.2.5\" },\n    { \"dependencies\": [\"react-dom\"], \"dependencyTypes\": [\"prod\", \"dev\"], \"pinVersion\": \"19.2.5\" },\n    { \"dependencies\": [\"@types/react\"], \"dependencyTypes\": [\"prod\", \"dev\"], \"pinVersion\": \"19.2.14\" },\n    { \"dependencies\": [\"@types/react-dom\"], \"dependencyTypes\": [\"prod\", \"dev\"], \"pinVersion\": \"19.2.3\" },\n    { \"dependencies\": [\"typescript\"], \"dependencyTypes\": [\"prod\", \"dev\"], \"pinVersion\": \"5.9.3\" },\n    { \"dependencies\": [\"eslint-plugin-astro\"], \"dependencyTypes\": [\"prod\", \"dev\"], \"pinVersion\": \"1.7.0\" },\n    { \"dependencies\": [\"eslint-plugin-astro-pipeline\"], \"dependencyTypes\": [\"prod\", \"dev\"], \"pinVersion\": \"0.1.2\" },\n    { \"dependencies\": [\"tailwindcss\"], \"dependencyTypes\": [\"prod\", \"dev\"], \"pinVersion\": \"4.2.4\" },\n    { \"dependencies\": [\"@tailwindcss/postcss\"], \"dependencyTypes\": [\"prod\", \"dev\"], \"pinVersion\": \"4.2.4\" },\n    { \"dependencies\": [\"class-variance-authority\"], \"dependencyTypes\": [\"prod\", \"dev\"], \"pinVersion\": \"0.7.1\" },\n    { \"dependencies\": [\"clsx\"], \"dependencyTypes\": [\"prod\", \"dev\"], \"pinVersion\": \"2.1.1\" },\n    { \"dependencies\": [\"tailwind-merge\"], \"dependencyTypes\": [\"prod\", \"dev\"], \"pinVersion\": \"3.5.0\" },\n    { \"dependencies\": [\"lucide-react\"], \"dependencyTypes\": [\"prod\", \"dev\"], \"pinVersion\": \"0.577.0\" },\n    { \"dependencies\": [\"zod\"], \"dependencyTypes\": [\"prod\", \"dev\"], \"pinVersion\": \"4.3.6\" },\n    { \"dependencies\": [\"@types/node\"], \"dependencyTypes\": [\"prod\", \"dev\"], \"pinVersion\": \"25.6.0\" },\n    { \"dependencies\": [\"next\"], \"dependencyTypes\": [\"prod\", \"dev\", \"optional\", \"peer\"], \"isBanned\": true },\n    { \"dependencies\": [\"velite\"], \"dependencyTypes\": [\"prod\", \"dev\", \"optional\", \"peer\"], \"isBanned\": true },\n    { \"dependencies\": [\"eslint-mdx\"], \"dependencyTypes\": [\"prod\", \"dev\", \"optional\", \"peer\"], \"isBanned\": true }\n  ]\n}\n",
    )
    .expect("syncpack config should be written");
    std::fs::write(
        root.path().join("src/pages/index.ts"),
        "export const GET = () => null;\n",
    )
    .expect("route source should be written");
    std::fs::write(
        root.path().join("src/lib/kit-taxonomy.ts"),
        "export const taxonomy = '../../packages/spec/src/data/taxonomy.json';\n",
    )
    .expect("side loader source should be written");
    std::fs::write(root.path().join("src/pages/about.mdx"), "# about\n")
        .expect("route markdown page should be written");
    std::fs::write(
        root.path().join("node_modules/eslint/package.json"),
        "{\n  \"name\": \"eslint\",\n  \"version\": \"0.0.0-test\",\n  \"main\": \"index.js\"\n}\n",
    )
    .expect("fake eslint package manifest should be written");
    std::fs::write(
        root.path().join("node_modules/eslint/index.js"),
        r#"class ESLint {
  constructor(options) {
    this.cwd = options.cwd;
    this.overrideConfigFile = options.overrideConfigFile;
  }

  async isPathIgnored(_filePath) {
    return false;
  }

      async calculateConfigForFile(_filePath) {
    const isTsx = String(_filePath).endsWith('.tsx');
    return {
      plugins: {
        astro: {},
        "astro-pipeline": {},
      },
      rules: {
        "astro-pipeline/no-authored-content-fs-read": ["error", { routeGlobs: ["src/pages/**/*.{astro,md,mdx,html}"], endpointGlobs: ["src/pages/**/*.{ts,js}"], authoredContentGlobs: ["src/content/**"] }],
        "astro-pipeline/no-authored-content-glob": ["error", { routeGlobs: ["src/pages/**/*.{astro,md,mdx,html}"], endpointGlobs: ["src/pages/**/*.{ts,js}"], authoredContentGlobs: ["src/content/**"] }],
        "astro-pipeline/no-authored-content-imports": ["error", { routeGlobs: ["src/pages/**/*.{astro,md,mdx,html}"], endpointGlobs: ["src/pages/**/*.{ts,js}"], authoredContentGlobs: ["src/content/**"] }],
        "astro-pipeline/no-content-data-modules-in-routes": ["error", { routeGlobs: ["src/pages/**/*.{astro,md,mdx,html}"], endpointGlobs: ["src/pages/**/*.{ts,js}"], contentDataModuleGlobs: ["src/**/*.data.{ts,tsx}"] }],
        "astro-pipeline/no-direct-astro-content-in-routes": ["error", { routeGlobs: ["src/pages/**/*.{astro,md,mdx,html}"], endpointGlobs: ["src/pages/**/*.{ts,js}"] }],
        "astro-pipeline/no-runtime-mdx-eval": "error",
        "astro-pipeline/no-side-loader-imports": ["error", { routeGlobs: ["src/pages/**/*.{astro,md,mdx,html}"], endpointGlobs: ["src/pages/**/*.{ts,js}"] }],
        "astro-pipeline/no-velite-imports": ["error", { routeGlobs: ["src/pages/**/*.{astro,md,mdx,html}"], endpointGlobs: ["src/pages/**/*.{ts,js}"] }],
      },
      languageOptions: { parserOptions: { projectService: true, jsx: isTsx } },
    };
  }
}

module.exports = { ESLint };
"#,
    )
    .expect("fake eslint runtime should be written");

    root
}

pub(super) fn crawl_with_entries(root: &TempDir, rel_paths: &[&str]) -> G3WorkspaceCrawl {
    G3WorkspaceCrawl {
        root_abs_path: root.path().to_path_buf(),
        entries: rel_paths
            .iter()
            .map(|rel_path| entry(root, rel_path, true))
            .collect(),
    }
}

pub(super) fn unreadable_entry(root: &TempDir, rel_path: &str) -> G3WorkspaceEntry {
    entry(root, rel_path, false)
}

pub(super) fn ignored_entry(root: &TempDir, rel_path: &str) -> G3WorkspaceEntry {
    G3WorkspaceEntry {
        path: G3WorkspacePath {
            rel_path: rel_path.to_owned(),
            abs_path: root.path().join(rel_path),
        },
        kind: G3WorkspaceEntryKind::File,
        ignore_state: G3WorkspaceIgnoreState::Ignored,
        readable: true,
    }
}

fn entry(root: &TempDir, rel_path: &str, readable: bool) -> G3WorkspaceEntry {
    G3WorkspaceEntry {
        path: G3WorkspacePath {
            rel_path: rel_path.to_owned(),
            abs_path: root.path().join(rel_path),
        },
        kind: G3WorkspaceEntryKind::File,
        ignore_state: G3WorkspaceIgnoreState::Included,
        readable,
    }
}
