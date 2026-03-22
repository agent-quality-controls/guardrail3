# Rich coverage details via tool introspection

**Date:** 2026-03-19 12:24
**Task:** Coverage map details show required_present/required_missing/user_extra by shelling out to the actual tools where needed.

## Approach

### TOML-based tools (clippy, deny, rustfmt, rust-toolchain, npmrc)
Parse ourselves. We already have TOML parsing. Diff against our known baseline.

### JS-based tools (ESLint, TypeScript)
Shell out to the tool to get resolved config:
- `pnpm exec eslint --print-config <file>` → JSON with all resolved rules
- `tsc --showConfig -p <tsconfig>` → JSON with resolved compilerOptions

If tool not installed → error, not fallback.

### Other (stylelint, prettier, cspell, jscpd)
Parse the config files ourselves (JSON/JS). Simple structure.

## Details format per tool

### clippy.toml
```json
{
  "methods": { "total": 25, "required_present": 23, "required_missing": 2, "user_extra": 2 },
  "types": { "total": 9, "required_present": 9, "required_missing": 0, "user_extra": 0 },
  "thresholds": { "total": 5, "relaxed": 0 }
}
```

### deny.toml
```json
{
  "bans": { "total": 24, "required_present": 23, "required_missing": 0, "user_extra": 1 },
  "advisory_ignores": 2,
  "licenses": { "total": 12, "required_present": 12, "required_missing": 0 }
}
```

### eslint (via --print-config)
```json
{
  "plugins": { "present": ["react", "jsx-a11y", "boundaries"], "missing": ["unicorn", "sonarjs"] },
  "rules": { "required_present": 28, "required_missing": 5 }
}
```

### tsconfig (via --showConfig)
```json
{
  "extends": "../../tsconfig.base.json",
  "strict_flags": { "required_present": 12, "required_missing": 0, "relaxed": 0 }
}
```

## Implementation order
1. clippy — we have the baseline paths in clippy.rs modules, diff against parsed TOML
2. deny — same, baseline bans in deny.rs modules
3. eslint — shell out to `pnpm exec eslint --print-config`
4. tsconfig — shell out to `tsc --showConfig`
5. Others — parse config files directly
