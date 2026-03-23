# TypeScript / Frontend Shift Back

## Context

We attempted to move frontend/content guardrails into Rust-native families:
- `RS-FRONTEND-ARCH`
- `RS-FRONTEND-I18N`
- `RS-FRONTEND-ROUTES`
- `RS-FRONTEND-UI`
- `RS-CONTENT`

That direction is still interesting, but it is not efficient enough yet to be the active frontend guardrail path.

So frontend/content work is shifting back to TypeScript-first planning for now.

## Decision

Reactivate TypeScript/frontend planning as an active area.

This means:
- move TS/frontend docs back out of `legacy/`
- treat TS/deploy/hooks/frontend audits as live planning material again
- keep the Rust frontend/content docs as a paused experiment, not the active path

## Active planning inputs

- `checks/deploy/ts.md`
- `checks/hooks/ts.md`
- `checks/hooks_deploy_audit.md`
- `audit/06-ts-source-scan.md`
- `audit/07-tsconfig-npmrc-jscpd.md`
- `audit/13-ts-architecture.md`
- `ts_guardrails_implementation.md`
- `ts_additional_analysis.md`
- `ts-project-types.md`
- `ts/tsconfig.md`
- `ts/tsconfig_npmrc_package_plugins_audit.md`

## Paused but preserved

- `rust_frontend_attempt/frontend_arch.md`
- `rust_frontend_attempt/frontend_i18n.md`
- `rust_frontend_attempt/frontend_routes_seo.md`
- `rust_frontend_attempt/frontend_ui.md`
- `rust_frontend_attempt/content_pipeline.md`

These remain useful as a possible future Rust-native frontend/content design, but they should not drive implementation right now.

## Immediate planning use

The revived TS/frontend docs should now be used to:
- recover the TS/frontend rule inventory
- decide what should become active checks again
- decide what stays historical
- separate frontend/content work from the Rust backend/library guardrail stream
