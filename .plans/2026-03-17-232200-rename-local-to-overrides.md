# Rename local/ to .guardrail3/overrides/ — convention over configuration

**Date:** 2026-03-17 23:22
**Task:** Remove [local] config section, use convention path .guardrail3/overrides/ instead

## Goal
guardrail3 should find override files by convention at `.guardrail3/overrides/` relative to
the project root, without any config section. The `[local]` config section and `LocalConfig`
struct are removed entirely.

## Approach

### Files to edit

1. **types.rs** — Remove `LocalConfig` struct, remove `local` field from `GuardrailConfig`
2. **generate.rs** — Rewrite `load_local_overrides` to read from convention path, remove config dependency. Update warning message.
3. **guardrail3.toml** — Remove `[local]` section
4. **help_gen.rs** — Replace all `local/` references with `.guardrail3/overrides/`
5. **guide.rs** — Replace all `local/` references with `.guardrail3/overrides/`
6. **CLAUDE.md** — Replace `[local]` config example, update all references
7. **hook_script_checks.rs** — Update `local/pre-commit.d/` to `.guardrail3/overrides/pre-commit.d/`
8. **GUARDRAIL3_GUIDE.md** — Generated file, update references

### Key decisions
- HooksConfig stays — it's unrelated to local overrides (it's for extra_dir)
- Override filenames use hyphens: clippy-methods.toml, deny-bans.toml etc.
- load_local_overrides takes only project_path, no config reference needed
