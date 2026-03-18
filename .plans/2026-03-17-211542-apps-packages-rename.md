# Rename [rust.crates.*] to [rust.apps.*] + [rust.packages]

**Date:** 2026-03-17 21:15
**Task:** Config naming should match directory structure. Apps are apps, packages are packages.

## Config shape change

Before:
```toml
[rust.crates.api]
profile = "service"
layer = "composition-root"

[rust.crates.domain-types]
profile = "library"
layer = "pure"
```

After:
```toml
[rust.apps.api]
profile = "service"
layer = "composition-root"

[rust.packages]
profile = "library"
layer = "pure"
```

## Same pattern for TS (already done mostly):
```toml
[typescript.apps.landing]
type = "content"
[typescript.apps.landing.checks]
...

[typescript.packages]
type = "library"
```

## Files to change

### Schema (domain/config/types.rs)
- Rename `crates: Option<CrateMap>` to `apps: Option<CrateMap>` in RustConfig
- Add `packages: Option<CrateConfig>` to RustConfig (single entry, not a map)
- CrateConfig and CrateMap types stay the same internally

### Consumers (read crate configs)
- app/rs/validate/mod.rs — reads `rust.crates` → change to `rust.apps` + merge packages
- app/rs/validate/hex_arch_checks.rs — uses crate configs extensively
- app/rs/validate/dependency_allowlist.rs — reads allowed_deps
- commands/generate.rs — reads crate configs for clippy generation

### Producers (write configs)
- commands/init.rs — generate_rs_config_content
- guardrail3.toml — self-config

### Docs
- help_gen.rs
- domain/modules/guide.rs
- CLAUDE.md
