# Auth Role Model - MVP

## Decision

- MVP role model has two roles:
  - `owner`
  - `member`
- Every user provisioned into their private default org starts as `owner`

## Why

- This keeps the MVP simple
- This avoids hardcoding "all users are always owner" into the system
- This leaves a clean path for invites and shared orgs later without redesigning the role model

## MVP behavior

- Private default-org flow provisions the initial user as `owner`
- No in-app role management UI exists in MVP
- No team member management UI exists in MVP
- No rich permissions system exists in MVP

## Meaning of roles

### `owner`

- Full control of the org in product terms
- For MVP, this is the only role users will actually experience

### `member`

- Reserved for future shared-org support
- Exists in the data model and contracts now
- Does not require UI or custom product behavior in MVP unless needed later

## What this does not mean

- This is not a full RBAC system
- This is not a permission matrix
- This is not admin/editor/viewer style role expansion

## Rules

- Role values are part of membership state
- Role values may appear in request-scoped internal auth context
- Authorization logic should not be scattered across handlers as raw role string checks
- Any role-based checks should live behind a small policy layer once real shared-org behavior exists

## Non-goals

- Rich role editor
- Permission builder
- Fine-grained per-feature permissions
- Cross-org admin roles

## Summary

- Define `owner` and `member` now
- Provision all MVP users as `owner`
- Build no role UI yet
