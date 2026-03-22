# ESLint Flat Config Resolution Edge Cases in Monorepos

**Researched:** 2026-03-18
**ESLint versions covered:** v9.x (latest 9.30.0) and v10.0.0 (released February 2026)

---

## 1. Root config vs subdirectory config: which wins?

**Scenario:** `eslint.config.mjs` at root AND `apps/landing/eslint.config.mjs`, running `eslint apps/landing/src/file.ts` from root.

### ESLint v9.x (default behavior)
The **root** `eslint.config.mjs` is used. ESLint v9 resolves config from the **current working directory**, not from the file being linted. The subdirectory config is completely ignored.

### ESLint v9.x with `--flag v10_config_lookup_from_file`
The **subdirectory** `apps/landing/eslint.config.mjs` is used. ESLint walks up from the linted file's directory until it finds the nearest `eslint.config.*`. Since `apps/landing/eslint.config.mjs` is closer to the file than the root config, it wins. The root config is NOT merged -- only the nearest config is loaded.

### ESLint v10.0.0 (default behavior)
Same as v9 with the flag: the **subdirectory config wins**. Config lookup from file is now the default. The root config is only used for files that don't have a closer config file in their ancestor directories.

**Critical implication:** In v10, each file uses exactly ONE config -- the nearest ancestor `eslint.config.*`. There is NO automatic merging of root + subdirectory configs. If your subdirectory config needs root rules, it must explicitly import them.

Sources:
- [ESLint Configuration Files docs](https://eslint.org/docs/latest/use/configure/configuration-files)
- [ESLint v10.0.0 release blog](https://eslint.org/blog/2026/02/eslint-v10.0.0-released/)
- [Config lookup from file RFC](https://github.com/eslint/rfcs/tree/main/designs/2024-config-lookup-from-file)

---

## 2. Is `unstable_config_lookup_from_file` now stable?

**Yes, fully stable and renamed/removed.**

Timeline:
- **v9.12.0:** Introduced as `unstable_config_lookup_from_file` (experimental)
- **v9.x (later):** Renamed to `v10_config_lookup_from_file` to signal it would become default in v10
- **v10.0.0:** The flag is **removed entirely**. This behavior is now the default and cannot be disabled. Attempting to use the flag produces an error.

If you are still on ESLint v9.x and want this behavior, use:
```bash
eslint --flag v10_config_lookup_from_file .
```

Sources:
- [ESLint v10.0.0 release blog](https://eslint.org/blog/2026/02/eslint-v10.0.0-released/)
- [ESLint v10.0.0 migration guide](https://eslint.org/docs/latest/use/migrate-to-10.0.0)
- [ESLint v9.30.0 release notes](https://eslint.org/blog/2025/06/eslint-v9.30.0-released/)

---

## 3. Does `eslint --config <path>` override discovery entirely?

**Yes.** When you use `--config` (or `-c`), ESLint skips the normal config file discovery and uses ONLY the specified config file. No searching up directories, no merging.

```bash
# Uses ONLY the specified config, ignoring any eslint.config.* in the directory tree
eslint --config ./configs/strict.config.mjs apps/landing/src/
```

**Limitation:** You cannot use `extends` with shareable configs when specifying a config via `--config` on the CLI. The config file itself can still import and spread other configs programmatically (since flat config is just JS/TS), but the CLI flag has this specific restriction.

Sources:
- [ESLint Configuration Files docs](https://eslint.org/docs/latest/use/configure/configuration-files)
- [ESLint flat config intro blog](https://eslint.org/blog/2022/08/new-config-system-part-2/)

---

## 4. Does `eslint.config.mjs` in a subdirectory EVER get picked up without the experimental flag?

### ESLint v9.x (without flag): NO
Subdirectory configs are completely invisible. ESLint only looks in the CWD for the config file. If you run ESLint from the repo root, only the root `eslint.config.*` is loaded, regardless of what exists in subdirectories.

### ESLint v10.0.0: YES (default behavior)
Subdirectory configs are always discovered. ESLint walks up from each linted file's directory to find the nearest `eslint.config.*`. This is the whole point of the v10 change.

**Summary:** On v9, subdirectory configs require either:
- The `--flag v10_config_lookup_from_file` flag, OR
- Running ESLint with CWD set to the subdirectory (e.g., `cd apps/landing && eslint .`), OR
- Using `--config apps/landing/eslint.config.mjs` explicitly

Sources:
- [ESLint Discussion #16960](https://github.com/eslint/eslint/discussions/16960)
- [ESLint Configuration Files docs](https://eslint.org/docs/latest/use/configure/configuration-files)

---

## 5. Can `eslint.config.mjs` import another `eslint.config.mjs`?

**Yes.** Flat config files are standard ES modules. You can import and spread configs freely:

```javascript
// apps/landing/eslint.config.mjs
import rootConfig from "../../eslint.config.mjs";

export default [
  ...rootConfig,
  {
    // Override or add rules specific to this package
    files: ["src/**/*.ts"],
    rules: {
      "no-console": "error",
    },
  },
];
```

### Resolution issues to watch for:

1. **Relative path resolution:** The import path is relative to the importing file, not the CWD. This works fine with standard ESM resolution.

2. **Plugin/dependency resolution:** If the root config references ESLint plugins, those plugins must be resolvable from the subdirectory's `node_modules`. In pnpm workspaces, this can fail if the plugin is only in the root `node_modules` and pnpm's strict linking prevents resolution. Fix: add the plugin as a dependency of the subdirectory package, or use pnpm's `public-hoist-pattern` to hoist it.

3. **`files` glob paths:** Glob patterns in imported configs are relative to the config file's `basePath` (its directory). When you spread a root config into a subdirectory config, the root config's glob patterns (like `src/**/*.ts`) will be evaluated relative to the subdirectory, not the root. This is usually what you want, but can cause surprises if the root config uses paths like `apps/**/*.ts`.

4. **Shared config packages:** The cleanest monorepo pattern is to extract shared config into a workspace package (e.g., `@repo/eslint-config`) and import from that, avoiding fragile relative paths.

Sources:
- [ESLint Discussion #16960](https://github.com/eslint/eslint/discussions/16960)
- [ESLint Issue #18385](https://github.com/eslint/eslint/issues/18385)
- [Turborepo ESLint docs](https://turbo.build/repo/docs/handbook/linting/eslint)

---

## 6. pnpm workspace: `pnpm -r exec eslint .` -- what config gets used?

When running `pnpm -r exec eslint .`, pnpm sets the CWD to each package's directory before executing the command.

### ESLint v9.x behavior:
ESLint looks for `eslint.config.*` in the CWD (which is now the package directory). So:
- If `packages/foo/eslint.config.mjs` exists, it is used.
- If it does NOT exist, ESLint walks up the directory tree and finds the root `eslint.config.mjs` (if present).
- Only ONE config is loaded (nearest ancestor from CWD).

### ESLint v10.x behavior:
Same effective result for `pnpm -r exec eslint .`, since the CWD is the package directory and config lookup starts from the file being linted. The nearest config to each file is used.

### pnpm-specific gotchas:

1. **Module resolution with pnpm strict mode:** pnpm's strict node_modules layout means plugins referenced in a root config may not be resolvable when ESLint runs from a subdirectory. Either:
   - Add plugins as direct dependencies of each package
   - Use `public-hoist-pattern` in `.npmrc` to hoist eslint plugins
   - Use `shamefully-hoist=true` (not recommended)

2. **NODE_PATH:** pnpm sets NODE_PATH to include the package's own dependencies, which helps with resolution but doesn't cover all cases.

3. **Lockfile conflicts:** If different packages depend on different versions of the same ESLint plugin, pnpm will correctly isolate them, but the root config may resolve a different version than expected.

Sources:
- [pnpm workspace settings docs](https://pnpm.io/next/settings)
- [ESLint Discussion #16960](https://github.com/eslint/eslint/discussions/16960)

---

## 7. Does `.eslintignore` still work with flat config?

**No. `.eslintignore` is completely dead.**

### ESLint v9.x:
`.eslintignore` is **ignored** when using flat config. No warning, no error -- it's just silently not loaded. The `--ignore-path` CLI flag is also not supported with flat config.

### ESLint v10.0.0:
`.eslintignore` support is **fully removed** along with the entire eslintrc config system.

### Migration path:

Use the `ignores` property in your flat config:

```javascript
// eslint.config.mjs
export default [
  {
    // Global ignores (no other properties in this object)
    ignores: [
      "dist/",
      "node_modules/",
      "*.generated.ts",
      "coverage/",
    ],
  },
  // ... other config objects with rules
];
```

Or use `includeIgnoreFile` to read an existing `.eslintignore` file during migration:

```javascript
import { includeIgnoreFile } from "@eslint/compat";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export default [
  includeIgnoreFile(path.resolve(__dirname, ".eslintignore")),
  // ... other config
];
```

**Important:** A config object with ONLY `ignores` (no `files`, `rules`, etc.) applies those ignores globally. A config object with `ignores` AND other properties only applies the ignores to that specific config object.

Sources:
- [ESLint Ignore Files docs](https://eslint.org/docs/latest/use/configure/ignore)
- [ESLint Configuration Migration Guide](https://eslint.org/docs/latest/use/configure/migration-guide)
- [ESLint v10.0.0 release blog](https://eslint.org/blog/2026/02/eslint-v10.0.0-released/)

---

## 8. Is `eslint.config.ts` (TypeScript config) supported?

**Yes, fully stable since ESLint v9.18.0.**

### Timeline:
- **v9.9.0:** Introduced as experimental. Required `jiti` package and `unstable_ts_config` feature flag.
- **v9.18.0 (January 2025):** Became **stable**. No feature flag needed. Still requires `jiti >= 2.0` as a dependency.
- **v10.0.0:** Continues to be supported. `jiti` is still required for Node.js.

### Supported extensions:
- `eslint.config.ts`
- `eslint.config.mts`
- `eslint.config.cts`

### Setup:
```bash
# Install jiti (required for TypeScript config in Node.js)
pnpm add -D jiti
```

Then just create `eslint.config.ts`:

```typescript
import type { Linter } from "eslint";

export default [
  {
    files: ["src/**/*.ts"],
    rules: {
      "no-unused-vars": "error",
    },
  },
] satisfies Linter.Config[];
```

### Priority / precedence:
When multiple config files exist in the same directory, ESLint uses this priority order:
1. `eslint.config.js`
2. `eslint.config.mjs`
3. `eslint.config.cjs`
4. `eslint.config.ts`
5. `eslint.config.mts`
6. `eslint.config.cts`

JS variants take precedence over TS variants. Do not have both `eslint.config.js` and `eslint.config.ts` in the same directory.

Sources:
- [ESLint Configuration Files docs](https://eslint.org/docs/latest/use/configure/configuration-files)
- [ESLint v9.9.0 release blog](https://eslint.org/blog/2024/08/eslint-v9.9.0-released/)
- [ESLint v9.18.0 release blog](https://eslint.org/blog/2025/01/eslint-v9.18.0-released/)

---

## Summary Decision Matrix

| Scenario | ESLint v9 (default) | ESLint v9 (with flag) | ESLint v10 |
|---|---|---|---|
| Subdirectory config discovered | NO | YES | YES |
| Config resolution starts from | CWD | Linted file | Linted file |
| Multiple configs in one run | NO | YES | YES |
| `.eslintignore` works | NO (silent) | NO (silent) | NO (removed) |
| `eslint.config.ts` supported | YES (stable v9.18+) | YES | YES |
| `--config` overrides discovery | YES | YES | YES |
| Configs auto-merge with parent | NO | NO | NO |

**Key takeaway for monorepos:** In v9, either use a single root config with `files` globs to target packages, or use `pnpm -r exec` to run ESLint per-package (so CWD is each package). In v10, subdirectory configs just work -- but remember they do NOT inherit from parent configs automatically. Each subdirectory config must explicitly import shared rules.
