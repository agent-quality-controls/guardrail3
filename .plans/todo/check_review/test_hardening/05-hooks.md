# Hooks Hardening Lane

## Focus

Hooks are still the biggest architecture gap and should be treated as a new-family migration plus hardening effort.

## Deliverables

1. Define migrated `HOOK-SHARED` and `HOOK-RS` family shape.
2. Create executable-command parsing model.
3. Build golden hook fixture(s).
4. Attack hook bypasses broadly.

## Main attack classes

- comments/prose masquerading as commands
- shebang and execute-bit issues
- `exit 0` and fail-open wrappers
- missing Rust steps
- bad `workspace_root`
- missing prerequisite tools
- config-change triggers absent

## Cross-family note

`RS-TEST-08` mutation-hook checking should ultimately reuse the same executable-command model rather than diverging again.

## Success condition

Hook validation no longer relies on raw substring matching for semantic command presence.
