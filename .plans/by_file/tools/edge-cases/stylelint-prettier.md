# Stylelint & Prettier Config Resolution Edge Cases in Monorepos

Both tools use [cosmiconfig](https://github.com/cosmiconfig/cosmiconfig) for config discovery. Cosmiconfig searches upward from a starting directory, checking a list of "searchPlaces" in order at each directory level, and **stops at the first config found**. It does NOT merge configs from multiple directories.

---

## Stylelint

### 1. Walk-up: root config vs subdirectory config

**Question:** Root has `.stylelintrc.mjs` and `apps/landing/` also has one. Running `stylelint "apps/**/*.css"` from root -- which config applies to `apps/landing/src/style.css`?

**Answer:** The **nearest config wins**. Stylelint uses cosmiconfig to search **per file**, starting from the file's directory and walking up. For `apps/landing/src/style.css`, cosmiconfig starts at `apps/landing/src/`, then checks `apps/landing/`. It finds `.stylelintrc.mjs` in `apps/landing/` and stops. The root `.stylelintrc.mjs` is never reached.

For a file at `apps/other/foo.css` (no local config), cosmiconfig walks up through `apps/other/` -> `apps/` -> root, and uses the root config.

**Key insight:** The CWD where you invoke `stylelint` does NOT determine which config applies. Cosmiconfig searches from **the linted file's location**, not from CWD.

Source: [Stylelint - Configuring](https://stylelint.io/user-guide/configure/), [cosmiconfig README](https://github.com/cosmiconfig/cosmiconfig)

### 2. Does `extends` merge rules or replace them?

**Answer:** `extends` **merges**. When a config extends another:

- **Single-value properties** (e.g., `customSyntax`) are **replaced/overridden**
- **Array/object properties** (e.g., `rules`, `plugins`, `extends`) are **merged/appended**
- Rules defined in the extending config override the same rule from the extended config
- Multiple items in the `extends` array have precedence in order: last item wins

As of Stylelint 15.0.0, `overrides.extends` also merges (changed from replace behavior in earlier versions).

Source: [Stylelint - Configuring](https://stylelint.io/user-guide/configure/), [PR #6380](https://github.com/stylelint/stylelint/pull/6380), [Migration to 15.0.0](https://stylelint.io/migration-guide/to-15/)

### 3. Can you stop the cosmiconfig walk-up at a specific directory?

**Answer:** Yes, via cosmiconfig's `stopDir` option. However, Stylelint does not directly expose this as a CLI flag. Cosmiconfig v9 introduced search strategies:

- **`"none"`**: Only check the starting directory, no traversal at all
- **`"global"`** (default when `stopDir` is set): Traverse up to `stopDir` (defaults to home directory)

To control this in Stylelint, you'd need to either:
1. Use the `--config` flag to bypass cosmiconfig entirely
2. Use the Node.js API and configure cosmiconfig's `stopDir` programmatically

There is no `--stop-dir` CLI flag in Stylelint.

Source: [cosmiconfig README](https://github.com/cosmiconfig/cosmiconfig), [Stylelint issue #7224](https://github.com/stylelint/stylelint/issues/7224)

### 4. Does `--config` override cosmiconfig?

**Answer:** Yes, **completely**. When you pass `--config path/to/config.mjs`, cosmiconfig search is disabled entirely. Stylelint loads only the specified file. This applies uniformly to all linted files -- there is no per-file config resolution when `--config` is used.

The `configFile` option in the Node.js API behaves the same way.

Source: [Stylelint - Options](https://stylelint.io/user-guide/options/), [Stylelint - CLI](https://stylelint.io/user-guide/cli/)

### 5. What if `.stylelintrc.mjs` and `stylelint.config.mjs` both exist in the same directory?

**Answer:** Cosmiconfig checks searchPlaces **in order** and uses the **first match**. The default searchPlaces order for Stylelint is:

1. `package.json` (stylelint key)
2. `.stylelintrc`
3. `.stylelintrc.json`
4. `.stylelintrc.yaml` / `.stylelintrc.yml`
5. `.stylelintrc.js`
6. `.stylelintrc.cjs`
7. `.stylelintrc.mjs`
8. `stylelint.config.js`
9. `stylelint.config.cjs`
10. `stylelint.config.mjs`

So `.stylelintrc.mjs` (#7) takes precedence over `stylelint.config.mjs` (#10). The second file is silently ignored -- no error, no warning.

**Danger:** This is a common source of bugs. Someone adds `stylelint.config.mjs` not realizing `.stylelintrc.mjs` already exists and shadows it. Changes to the "wrong" file have no effect.

Source: [cosmiconfig README - searchPlaces](https://github.com/cosmiconfig/cosmiconfig), [Stylelint - Configuring](https://stylelint.io/user-guide/configure/)

---

## Prettier

### 6. Walk-up: root `.prettierrc` vs subdirectory `.prettierrc`

**Answer:** Same as Stylelint -- **nearest config wins**. Prettier resolves config starting from the file being formatted, walking up the tree. If `apps/landing/.prettierrc` exists, it applies to all files in `apps/landing/` and below. The root `.prettierrc` is never reached for those files.

**Critical difference from ESLint:** Prettier does NOT merge configs from different directory levels. The subdirectory config **completely replaces** the root config. There is no cascading.

Source: [Prettier - Configuration File](https://prettier.io/docs/configuration)

### 7. Does Prettier's walk-up stop at project root?

**Answer:** No, not inherently. Prettier uses cosmiconfig which by default walks up to the user's home directory (or the configured `stopDir`). There is no automatic stop at `package.json` or `.git` boundaries.

However, in practice, most monorepos have a `.prettierrc` at the root, so the search stops there. If there is NO config anywhere in the tree, cosmiconfig walks all the way up to `$HOME`.

**EditorConfig special case:** The `.editorconfig` search DOES stop at the project root (nearest `.git` or filesystem root), per the EditorConfig spec. But this is separate from Prettier's own config resolution.

Source: [Prettier - Configuration File](https://prettier.io/docs/configuration), [cosmiconfig README](https://github.com/cosmiconfig/cosmiconfig)

### 8. Can you have per-directory Prettier configs that EXTEND root?

**Answer:** **No native `extends` support.** Prettier does not have an `extends` keyword in its config. Each config file is standalone and completely replaces any parent config.

**Workaround with JS/MJS configs:** Use `.prettierrc.mjs` in the subdirectory and manually import/spread the root config:

```js
// apps/landing/.prettierrc.mjs
import rootConfig from "../../.prettierrc.mjs";

export default {
  ...rootConfig,
  semi: false,  // override specific option
};
```

**Workaround with shared packages:** Publish a `prettier-config-myorg` package and reference it:

```json
// apps/landing/.prettierrc
"@myorg/prettier-config"
```

Then override in `.prettierrc.mjs`:

```js
import baseConfig from "@myorg/prettier-config";
export default { ...baseConfig, semi: false };
```

**Note:** JSON/YAML `.prettierrc` files cannot extend anything. You must use JS/MJS/CJS format for extension.

Source: [Prettier - Sharing configurations](https://prettier.io/docs/sharing-configurations), [Prettier issue #3146](https://github.com/prettier/prettier/issues/3146)

### 9. Does `--config` override cosmiconfig?

**Answer:** Yes, same as Stylelint. `--config path/to/config` bypasses all automatic config discovery. The specified file is used for ALL files being formatted. No per-file resolution occurs.

Source: [Prettier - Configuration File](https://prettier.io/docs/configuration), [Prettier issue #2870](https://github.com/prettier/prettier/issues/2870)

### 10. What if `.prettierrc` and `prettier.config.mjs` both exist?

**Answer:** Cosmiconfig uses the first match from the searchPlaces list. Prettier's order:

1. `package.json` (prettier key)
2. `.prettierrc`
3. `.prettierrc.json`
4. `.prettierrc.yml` / `.prettierrc.yaml` / `.prettierrc.json5`
5. `.prettierrc.js` / `prettier.config.js`
6. `.prettierrc.mjs` / `prettier.config.mjs`
7. `.prettierrc.cjs` / `prettier.config.cjs`
8. `.prettierrc.toml`

So `.prettierrc` (#2) takes precedence over `prettier.config.mjs` (#6). The `.mjs` file is silently ignored.

**Same danger as Stylelint:** Silent shadowing with no warning.

Source: [Prettier - Configuration File](https://prettier.io/docs/configuration)

---

## Summary: Key Gotchas for Monorepo Guardrails

| Behavior | Stylelint | Prettier |
|---|---|---|
| Walk-up search | Per-file, nearest config wins | Per-file, nearest config wins |
| Config merging across dirs | No (single config used) | No (single config used) |
| `extends` keyword | Yes, merges rules | No native support |
| `--config` flag | Disables cosmiconfig entirely | Disables cosmiconfig entirely |
| Multiple configs in same dir | First in searchPlaces wins, silent | First in searchPlaces wins, silent |
| Stop walk-up at boundary | Only via cosmiconfig `stopDir` (not exposed in CLI) | Only via cosmiconfig `stopDir` (not exposed in CLI) |
| EditorConfig interaction | N/A | Merged as defaults, `.prettierrc` overrides |

### Guardrail implications

1. **Detect orphaned subdirectory configs** that accidentally shadow root config without extending it (Prettier) or without `extends` (Stylelint).
2. **Detect duplicate config files** in the same directory (e.g., `.stylelintrc.mjs` + `stylelint.config.mjs`). This is always a bug.
3. **Detect Prettier subdirectory configs in JSON/YAML format** that cannot extend root. If they exist without the full config, they likely have missing options.
4. **Warn when `--config` is used in CI scripts** alongside directory-level configs, since the flag silently overrides all per-directory configs.

---

## Sources

- [Stylelint - Configuring](https://stylelint.io/user-guide/configure/)
- [Stylelint - Options](https://stylelint.io/user-guide/options/)
- [Stylelint - CLI](https://stylelint.io/user-guide/cli/)
- [Stylelint - Migration to 15.0.0](https://stylelint.io/migration-guide/to-15/)
- [Stylelint issue #7224 - cosmiconfig v9 search strategies](https://github.com/stylelint/stylelint/issues/7224)
- [Stylelint issue #6383 - Config resolution performance](https://github.com/stylelint/stylelint/issues/6383)
- [Stylelint PR #6380 - extends in overrides](https://github.com/stylelint/stylelint/pull/6380)
- [Prettier - Configuration File](https://prettier.io/docs/configuration)
- [Prettier - Sharing configurations](https://prettier.io/docs/sharing-configurations)
- [Prettier issue #3146 - Extend config](https://github.com/prettier/prettier/issues/3146)
- [Prettier issue #2870 - --config and cosmiconfig](https://github.com/prettier/prettier/issues/2870)
- [Prettier blog - CLI Performance Deep Dive](https://prettier.io/blog/2023/11/30/cli-deep-dive.html)
- [cosmiconfig README](https://github.com/cosmiconfig/cosmiconfig)
- [cosmiconfig issue #219 - Dynamic stopDir](https://github.com/cosmiconfig/cosmiconfig/issues/219)
