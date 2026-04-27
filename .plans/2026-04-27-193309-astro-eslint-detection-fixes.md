Goal:
- Fix the Astro ESLint detection bugs reported by the landing app without weakening the architecture rules.

Approach:
- Reproduce the failures against the local landing app with the installed G3TS CLI.
- Add focused tests before changing behavior:
  - Setup rule 05 accepts an effective Astro plugin when ESLint reports `astro/valid-compile` and either package metadata or plugin metadata proves `eslint-plugin-astro`.
  - Content rule 18 accepts configured adapter directories and equivalent file globs such as `src/content/**/*`.
  - MDX rule 20 accepts an effective MDX plugin when ESLint reports `mdx/remark` and either package metadata or plugin metadata proves `eslint-plugin-mdx`.
- Fix the architecturally correct layer:
  - If parser output misses package metadata for namespace imports, fix or relax the check against effective plugin/rule evidence in the config-check package.
  - Keep package presence checks separate; config checks should prove effective ESLint behavior, not duplicate package-resolution internals.
  - Normalize adapter globs in the content config checker so directory contracts and file-glob ESLint options compare by meaning, not literal syntax.
- Re-run:
  - package-local Astro tests
  - app G3TS validation on landing
  - G3RS scan on changed packages
  - adversarial review against this plan

Key decisions:
- Do not change landing app config to satisfy a bad literal comparison.
- Do not require exact ESLint source syntax. G3TS must validate the effective config facts.
- Keep rule messages specific about accepted directory/glob equivalence.

Files to modify:
- `packages/ts/astro/setup/g3ts-astro-setup-config-checks/crates/runtime/src/ts_astro_config_05_astro_eslint_plugin_wired.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-config-checks/crates/runtime/src/lib_tests`
- `packages/ts/astro/content/g3ts-astro-content-config-checks/crates/runtime/src/ts_astro_config_18_content_adapter_rule.rs`
- `packages/ts/astro/content/g3ts-astro-content-config-checks/crates/runtime/src/lib_tests`
- `packages/ts/astro/mdx/g3ts-astro-mdx-config-checks/crates/runtime/src/ts_astro_config_20_mdx_lane.rs`
- `packages/ts/astro/mdx/g3ts-astro-mdx-config-checks/crates/runtime/src/lib_tests`
