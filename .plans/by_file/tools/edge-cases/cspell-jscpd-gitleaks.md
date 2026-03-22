# Config Resolution Edge Cases: cspell, jscpd, gitleaks

**Date:** 2026-03-18
**Purpose:** Document config file resolution behavior for guardrail tool wrappers

---

## cspell

### Q1: Root AND subdirectory both have cspell.json — shadow or merge?

**Neither pure shadow nor pure merge. It's "nearest wins" with merge semantics for imports.**

- By default, cspell searches the current directory and UP the hierarchy for config files. **The first configuration file found is loaded; the others are ignored.**
  - Source: [Getting Started | CSpell](https://cspell.org/docs/getting-started)
- However, when cspell checks files across a monorepo, it uses the **nearest config file** in the directory hierarchy for each file being checked.
  - If `/repo/cspell.json` exists and `/repo/packages/foo/cspell.json` also exists, files in `packages/foo/` use `packages/foo/cspell.json` exclusively.
  - The root config is NOT automatically merged — it's **shadowed** by the nearer one.
- **To get merging**, the subdirectory config must explicitly `import` the parent config:
  ```json
  { "import": ["../../cspell.json"], "words": ["subdir-specific-word"] }
  ```
- When importing, merge rules apply:
  - Scalar settings: last one wins (subdirectory overrides parent)
  - Array settings (`words`, `ignoreWords`, `dictionaries`): **unioned** (combined)
  - `overrides` and `languageSettings`: accumulated in load order
  - Source: [Importing/Extending Configuration | CSpell](https://cspell.org/docs/Configuration/imports)

**Implication for guardrail3:** If we place a root `cspell.json` and a per-package one, the per-package one must `import` the root one or root words/dictionaries will be invisible.

### Q2: Does `import` auto-resolve relative paths?

**Yes.** Relative paths in `import` are resolved relative to the config file that contains the `import` statement. Same applies to dictionary file paths.

- Source: [Configuration | CSpell](https://cspell.org/docs/Configuration)

### Q3: `--config` flag

`--config <path>` specifies an explicit config file path. When used, cspell loads that config file instead of searching the directory hierarchy.

- Source: [cspell - npm](https://www.npmjs.com/package/cspell)

### Q4: `--root` flag

`-r, --root <root folder>` sets the root directory (defaults to CWD). This affects:
- Where glob patterns are resolved from
- The `globRoot` used for matching files
- **Important quirk:** Even with absolute file paths passed to cspell, it only detects them if the root/CWD is above those paths.
- Source: [cspell - npm](https://www.npmjs.com/package/cspell), [Drupal issue #3314151](https://www.drupal.org/project/drupal/issues/3314151)

Whether `--root` also limits the config file search upward is not explicitly documented. The safe assumption is that it sets the base directory for glob resolution but config search still walks up from each file's directory.

### Q5: `.cspell.json` vs `cspell.json` — both valid?

**Yes.** CSpell searches for config files in this order (first found wins):

1. `.cspell.json`
2. `cspell.json`
3. `.cSpell.json`
4. `cSpell.json`
5. `cspell.config.js`
6. `cspell.config.cjs`
7. `cspell.config.json`
8. `cspell.config.yaml`
9. `cspell.config.yml`
10. `cspell.yaml`
11. `cspell.yml`

Additionally, `.config/` subdirectory variants are supported (e.g., `.config/cspell.config.yaml`).

- Source: [Getting Started | CSpell](https://cspell.org/docs/getting-started)

**Implication for guardrail3:** The dotfile variant (`.cspell.json`) takes priority. If both exist, only `.cspell.json` is loaded.

---

## jscpd

### Q6: Per-directory `.jscpd.json` configs?

**Depends on cosmiconfig version and searchStrategy.**

jscpd uses [cosmiconfig](https://github.com/cosmiconfig/cosmiconfig) for config file discovery. The behavior depends on which cosmiconfig version jscpd bundles:

- **cosmiconfig v9+** (default `searchStrategy: 'none'`): Does NOT walk up directories. Only checks the current working directory. This means a `.jscpd.json` in `apps/validator-rust/` would only be found if you `cd` into that directory or pass it as the search path.
- **cosmiconfig v8 and earlier** (default behavior: walk up to root): Walks up from CWD to find the nearest config file. First found wins, no merging.
- **cosmiconfig `searchStrategy: 'project'`**: Walks up until it finds a `package.json`.

jscpd does NOT support multiple config files or config merging. Whichever config file is found first by cosmiconfig is used exclusively.

- Source: [cosmiconfig README](https://github.com/cosmiconfig/cosmiconfig), [jscpd - npm](https://www.npmjs.com/package/jscpd)

Config file names searched (via cosmiconfig convention):
- `.jscpd.json` (the documented default)
- `.jscpdrc`, `.jscpdrc.json`, `.jscpdrc.yaml`, `.jscpdrc.yml`, `.jscpdrc.js`, `.jscpdrc.cjs`
- `jscpd.config.js`, `jscpd.config.ts`, `jscpd.config.mjs`, `jscpd.config.cjs`
- `jscpd` key in `package.json`

### Q7: Does jscpd walk up or only use CWD?

**Depends on cosmiconfig version** (see Q6). With modern cosmiconfig (v9+), default is CWD-only (`searchStrategy: 'none'`). Older versions walk up.

**Safe assumption for guardrail3:** Do not rely on walk-up behavior. Always either:
- Place config at CWD, or
- Use `--config` to specify explicitly

### Q8: `--config` flag

jscpd supports `-c` / `--config <path>` to specify an explicit config file path. This bypasses cosmiconfig search entirely.

- Source: [jscpd - npm](https://www.npmjs.com/package/jscpd)

### Q9: `jscpd apps/` — where does it look for config?

When you pass a path argument like `jscpd apps/`, the path argument specifies **what to scan**, not where to find config. Config resolution still starts from CWD (or uses `--config` if provided). The `.jscpd.json` in `apps/` would NOT be automatically used.

- Source: [jscpd documentation](https://www.codeac.io/documentation/jscpd.html) — "default config path is `.jscpd.json` in `<path>`" suggests it may look in the scan path, but this is inconsistent with cosmiconfig's CWD-based search. **Needs empirical verification.**

**Implication for guardrail3:** Always pass `--config` explicitly when running jscpd on subdirectories. Do not rely on auto-discovery.

---

## gitleaks

### Q10: Does `.gitleaks.toml` walk up or CWD only?

**Target path only, no walk-up.**

Gitleaks config resolution order:
1. `--config` / `-c` flag (explicit path)
2. `GITLEAKS_CONFIG` environment variable (path to file)
3. `GITLEAKS_CONFIG_TOML` environment variable (inline TOML content)
4. `(target path)/.gitleaks.toml` — looks in the **target path** (the directory being scanned), NOT CWD

If none match, gitleaks uses its built-in default config.

- Source: [gitleaks README](https://github.com/gitleaks/gitleaks/blob/master/README.md), [Stop Leaking Secrets - Configuration](https://blog.gitleaks.io/stop-leaking-secrets-configuration-2-3-aeed293b1fbf)

**Key distinction:** It's the scan target path, not CWD. If you run `gitleaks detect --source=./apps/foo`, it looks for `./apps/foo/.gitleaks.toml`, not `./.gitleaks.toml`.

### Q11: `--config` flag

`--config` / `-c` is a persistent flag that specifies an explicit path to a TOML config file. Takes highest precedence.

- Source: [gitleaks README](https://github.com/gitleaks/gitleaks/blob/master/README.md)

### Q12: `.gitleaks.toml` vs `gitleaks.toml`

**Only `.gitleaks.toml` (with dot prefix) is auto-discovered.** The non-dotfile variant `gitleaks.toml` is NOT automatically found — it must be specified via `--config` or the environment variable.

The gitleaks repo itself has both:
- `.gitleaks.toml` — the project's own allowlist config (auto-discovered)
- `config/gitleaks.toml` — the default rules definition (internal, not for auto-discovery)

- Source: [gitleaks GitHub repo](https://github.com/gitleaks/gitleaks)

### Q13: Per-directory configs?

**No.** Gitleaks does NOT support per-directory config files. It uses exactly one config file for the entire scan. There is no config merging, no directory-walking, no hierarchical config.

For multi-project setups, the recommended approach is:
- One central config with shared rules
- Per-project `[allowlist]` entries in that single file
- Or separate `--config` invocations per project

- Source: [Gitleaks for Enterprises](https://blog.rewanthtammana.com/gitleaks-for-enterprises)

**Implication for guardrail3:** Must always pass `--config` explicitly. If scanning a subdirectory, cannot rely on a root `.gitleaks.toml` being found (it looks in the target path, which would be the subdirectory).

---

## Summary Table

| Behavior | cspell | jscpd | gitleaks |
|---|---|---|---|
| Auto-discovery | Walks up from file's dir | CWD only (cosmiconfig v9+) | Target path only |
| Multiple configs | Nearest wins (no merge without `import`) | Single config, no merge | Single config, no merge |
| Config merging | Yes, via explicit `import` | No | No |
| Dotfile variant | `.cspell.json` (priority over `cspell.json`) | `.jscpd.json` | `.gitleaks.toml` only |
| Non-dotfile | `cspell.json` (also valid) | Various rc variants | Not auto-discovered |
| `--config` flag | Yes, overrides search | Yes (`-c`), overrides search | Yes (`-c`), highest precedence |
| Per-directory | Yes (nearest config per file) | No | No |
| Walk-up | Yes (up directory hierarchy) | Version-dependent | No |

## Recommendations for guardrail3

1. **Always use `--config`** for all three tools when wrapping them. Relying on auto-discovery creates fragile behavior that varies by CWD, tool version, and directory structure.
2. **cspell:** If using hierarchical configs in a monorepo, subdirectory configs MUST `import` the root config explicitly. Otherwise root dictionaries/words are invisible.
3. **jscpd:** Do not rely on walk-up. Pin config location with `--config`.
4. **gitleaks:** Remember it looks in the **target path**, not CWD. When scanning subdirectories, pass `--config` pointing to the root `.gitleaks.toml`.

---

## Sources

- [CSpell - Getting Started](https://cspell.org/docs/getting-started)
- [CSpell - Configuration](https://cspell.org/docs/Configuration)
- [CSpell - Importing/Extending Configuration](https://cspell.org/docs/Configuration/imports)
- [CSpell - Document Settings](https://cspell.org/docs/Configuration/document-settings)
- [CSpell GitHub Issue #1366 - Monorepo config loading](https://github.com/streetsidesoftware/cspell/issues/1366)
- [Drupal Issue #3314151 - cspell --root behavior](https://www.drupal.org/project/drupal/issues/3314151)
- [cspell - npm](https://www.npmjs.com/package/cspell)
- [jscpd - npm](https://www.npmjs.com/package/jscpd)
- [jscpd GitHub](https://github.com/kucherenko/jscpd)
- [jscpd - Codeac documentation](https://www.codeac.io/documentation/jscpd.html)
- [cosmiconfig GitHub](https://github.com/cosmiconfig/cosmiconfig)
- [cosmiconfig - npm](https://www.npmjs.com/package/cosmiconfig)
- [gitleaks GitHub](https://github.com/gitleaks/gitleaks)
- [gitleaks README](https://github.com/gitleaks/gitleaks/blob/master/README.md)
- [Stop Leaking Secrets - Configuration (gitleaks blog)](https://blog.gitleaks.io/stop-leaking-secrets-configuration-2-3-aeed293b1fbf)
- [Gitleaks for Enterprises](https://blog.rewanthtammana.com/gitleaks-for-enterprises)
