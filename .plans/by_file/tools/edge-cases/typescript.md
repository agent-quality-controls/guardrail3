# TypeScript tsconfig.json Resolution Edge Cases

**Date:** 2026-03-18
**Purpose:** Document tsc/tsconfig.json resolution behavior for edge cases relevant to guardrail3 validation.

---

## 1. Does `tsc` walk up parent directories to find tsconfig.json?

**Yes.** When `tsc` is invoked without `-p` and without input files, it searches for `tsconfig.json` starting in the current directory and walking up the parent directory chain until it finds one.

**Important caveats:**
- This walk-up behavior ONLY applies when `tsc` is run without file arguments. When input files are specified on the command line, tsconfig.json files are ignored entirely.
- The `-p`/`--project` flag overrides this: it specifies an explicit path to a directory containing tsconfig.json, or a path to a valid .json config file.

**Source:** [TypeScript: What is a tsconfig.json](https://www.typescriptlang.org/docs/handbook/tsconfig-json.html)

---

## 2. Can `extends` reference node_modules or paths outside the project?

### node_modules: Yes (since TypeScript 3.2)

Since TypeScript 3.2, `extends` resolves bare package specifiers from `node_modules` using Node.js-style resolution. This is how shared config packages like `@tsconfig/node20` work:

```json
{
  "extends": "@tsconfig/node20/tsconfig.json"
}
```

TypeScript walks up ancestor directories looking for `node_modules` containing the package, same as Node.js module resolution.

### Relative paths outside the project: Yes

Relative paths like `../../other-project/tsconfig.json` are fully supported. The path is resolved relative to the tsconfig.json file containing the `extends` field.

### Critical detail: path resolution is relative to the originating config

All relative paths found in a configuration file are resolved relative to the configuration file they originated in -- NOT relative to the inheriting config. This means if `base/tsconfig.json` has `"outDir": "./dist"`, that `./dist` is relative to `base/`, even when extended by `app/tsconfig.json`.

**Sources:**
- [TypeScript: TSConfig Option: extends](https://www.typescriptlang.org/tsconfig/extends.html)
- [Resolve tsconfig.json extends path using node_modules resolution logic - Issue #18865](https://github.com/microsoft/TypeScript/issues/18865)
- [tsconfig/bases repository](https://github.com/tsconfig/bases)

---

## 3. How do composite projects and project references change compilation?

### `composite: true` enforces constraints

Setting `"composite": true` in a tsconfig enables the project to be referenced by other projects. It enforces:
- `declaration` must be `true` (or `composite` sets it implicitly)
- `declarationMap` is recommended
- All implementation files must be matched by an `include` pattern or listed in `files`
- `rootDir` defaults to the directory containing the tsconfig.json

### `references` field

The `references` array lists projects that the current project depends on:

```json
{
  "references": [
    { "path": "../shared" },
    { "path": "../utils" }
  ]
}
```

Each referenced project must have `"composite": true`.

### `--build` mode is required for orchestration

Running plain `tsc` does NOT automatically build dependencies. You must use `tsc --build` (or `tsc -b`) to get build orchestration. With `--build`, TypeScript:
- Finds all referenced projects
- Determines which are out-of-date
- Builds them in the correct dependency order
- Uses `.tsbuildinfo` files for incremental compilation

### `references` is NOT inherited

The `references` field is the only top-level property excluded from `extends` inheritance. Each project must declare its own references.

**Source:** [TypeScript: Project References](https://www.typescriptlang.org/docs/handbook/project-references.html)

---

## 4. What if `extends` points to a file that doesn't exist?

TypeScript reports errors but the behavior around exit codes has been inconsistent:

- **Error TS5058:** "The specified path does not exist"
- **Error TS6053:** File not found errors for the referenced config

### Exit code bug (Issue #26203)

There was a known bug where `tsc` would report the error but still exit with code 0, which is problematic for CI/CD pipelines. The expected behavior is a non-zero exit code.

```json
{ "extends": "./idontexist.json" }
```

This produces error output but historically could exit with code 0. This has been addressed in later TypeScript versions, but guardrail validation should be aware that older versions may have this bug.

**Source:** [tsc exits with status code 0 when tsconfig references missing file - Issue #26203](https://github.com/microsoft/TypeScript/issues/26203)

---

## 5. Does changing a base tsconfig.json trigger rebuilds in extending packages?

**Yes, with `--build` mode.** TypeScript's incremental build system (via `tsc --build`) tracks configuration dependencies. When `tsconfig.base.json` changes, projects that extend it are considered out-of-date and will be rebuilt.

The `.tsbuildinfo` files store information about the project graph from the last compilation. When the base config changes, the stored build info becomes stale, triggering a rebuild.

**Without `--build` mode:** There is no automatic rebuild mechanism. Each project must be compiled independently, and there is no tracking of base config changes. This is a manual process.

**Sources:**
- [TypeScript: TSConfig Option: incremental](https://www.typescriptlang.org/tsconfig/incremental.html)
- [TypeScript: Project References](https://www.typescriptlang.org/docs/handbook/project-references.html)

---

## 6. Can tsconfig.json extend multiple configs (array extends)?

**Yes, since TypeScript 5.0.** The `extends` field can be a string (single config) or an array of strings (multiple configs):

```json
{
  "extends": [
    "@tsconfig/strictest/tsconfig.json",
    "@tsconfig/node20/tsconfig.json"
  ]
}
```

### Merge order

Later entries in the array take precedence over earlier ones. If two base configs define the same `compilerOptions` property, the value from the last config in the array wins.

### Tool support caveat

When this feature was introduced in TS 5.0, many third-party tools (ts-node, esbuild, vite, etc.) did not immediately support the array form and would error. Most tools have since added support, but guardrail validation should be aware that the array form may not work with all tooling.

### Verification

Run `tsc --showConfig` to see the final merged configuration and verify the result of multi-config extension.

**Sources:**
- [TypeScript: TSConfig Option: extends](https://www.typescriptlang.org/tsconfig/extends.html)
- [Add support for TS 5.0 extends array - ts-node Issue #1954](https://github.com/TypeStrong/ts-node/issues/1954)
- [Support extending multiple tsconfigs - Issue #42386](https://github.com/microsoft/TypeScript/issues/42386)

---

## 7. Are `compilerOptions.paths` merged or replaced via `extends`?

**Replaced, not merged.** When a child tsconfig defines `compilerOptions.paths`, it completely replaces the `paths` from the base config. There is no merging.

This applies to all properties within `compilerOptions` -- the merge granularity is at the individual option level. If the child defines `paths`, the entire `paths` object from the base is discarded.

```jsonc
// base.json
{
  "compilerOptions": {
    "paths": {
      "@shared/*": ["./shared/*"]  // This is LOST if child defines paths
    }
  }
}

// child.json
{
  "extends": "./base.json",
  "compilerOptions": {
    "paths": {
      "@app/*": ["./app/*"]  // Only this exists in final config
    }
  }
}
```

### Workaround

You must manually duplicate all base paths in the child config. There is no `merge` directive. Feature requests for merge support exist ([Issue #57486](https://github.com/microsoft/TypeScript/issues/57486), [Issue #44589](https://github.com/microsoft/TypeScript/issues/44589)) but have not been implemented.

**Sources:**
- [tsconfig.json parameters cannot be extended, only overwritten](https://miyoon.medium.com/array-parameters-in-tsconfig-json-are-always-overwritten-11c80bb514e1)
- [Ability to extend compilerOptions paths config - Issue #44589](https://github.com/microsoft/TypeScript/issues/44589)

---

## 8. Are `include`/`exclude` inherited via `extends`?

**Yes, but they are replaced, not merged.**

If the child tsconfig does NOT define `include`/`exclude`, the values from the base config are inherited. But if the child defines its own `include` or `exclude`, that completely replaces the base value -- no merging occurs.

### Path resolution matters

Inherited `include`/`exclude` paths from a base config are resolved relative to the base config's location, not the child's location. This is consistent with the general rule that all relative paths are resolved relative to the config file they originated in.

### `files` behaves the same way

The `files` array is also replaced (not merged) when defined in the child config.

### `exclude` limitations

`exclude` only affects which files are included via the `include` setting. Files can still enter the compilation through:
- `import` statements in code
- `types` in compilerOptions
- `/// <reference>` directives
- Being listed in `files`

**Source:** [TypeScript: TSConfig Option: extends](https://www.typescriptlang.org/tsconfig/extends.html)

---

## Summary Table

| Behavior | Result |
|---|---|
| `tsc` without `-p` walks up directories | Yes |
| `extends` from node_modules | Yes (since TS 3.2, bare specifiers) |
| `extends` with `../../outside/tsconfig.json` | Yes (relative paths work) |
| Missing `extends` target | Error TS5058, but exit code 0 bug existed |
| Base config change triggers rebuild | Yes, with `tsc --build`; no tracking without it |
| Array `extends` (multiple configs) | Yes (since TS 5.0), later entries win |
| `compilerOptions.paths` inheritance | Replaced, not merged |
| `include`/`exclude` inheritance | Inherited if not defined in child; replaced (not merged) if defined |
| `references` inheritance | NOT inherited (only top-level prop excluded from extends) |
| Path resolution in inherited configs | Relative to originating config file, not the inheriting one |
| Circular extends | Not allowed, TypeScript errors |
