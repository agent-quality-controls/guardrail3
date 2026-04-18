# Auth Identity and Org Contract - MVP

## Goal

- Keep the MVP auth/org model simple
- Keep `web` as the auth edge and BFF
- Avoid locking business data and backend contracts to PropelAuth internals
- Avoid a future migration where org support has to be retrofitted across the system

## Decisions

### 1. Local org IDs are app-owned

- Websmasher business data uses app-owned IDs
- `org_id` in crawls, reports, billing, and backend contracts refers to the app's org ID
- PropelAuth org IDs are provider mapping data, not business IDs

Implication:
- We do not store PropelAuth org IDs directly as the primary org identifier on product data

### 2. Local identity projection exists from day 1

- Websmasher stores thin local projections for:
  - `users`
  - `orgs`
  - `memberships`
- These records are the app's local identity model
- PropelAuth remains the source of truth for authentication and org membership state
- Local records exist so the app can:
  - attach product data to stable local IDs
  - survive provider changes
  - enforce a clean backend contract

### 3. MVP org model remains the same

- Every user gets one private default org
- MVP supports exactly one org per user
- No org switcher in the app
- No org management UI in the app
- All product data is org-scoped from day 1

### 4. Default-org provisioning runs in `web`

- `web` is the auth edge
- `web` is responsible for ensuring the authenticated user has a default org before the app continues
- `backend` does not talk to PropelAuth
- `backend` does not provision orgs

### 5. Provisioning trigger is first authenticated app entry

- Canonical trigger: first authenticated server-side app entry in `web`
- Not webhooks
- Not backend
- Not client-side code

Reason:
- This matches the current BFF shape
- It avoids extra infrastructure for MVP
- It keeps provider-specific provisioning logic at the auth edge

### 6. Provisioning must be idempotent

- `ensure_user_has_default_org(user)` must be safe to call multiple times
- Parallel tabs, retries, and repeated first-loads must not create duplicate orgs or duplicate memberships
- Duplicate creation must be prevented at the storage boundary, not only by "best effort" code flow

### 7. `web -> backend` identity is request-scoped

- `web` authenticates the browser session with PropelAuth
- `web` resolves the current local user and local org
- `web` sends a server-generated identity envelope to `backend` for that request
- `backend` trusts only requests authenticated as coming from `web`

### 8. `backend` stays provider-agnostic

- `backend` does not know PropelAuth tokens, session formats, or org APIs
- `backend` only knows the app's actor model and internal service auth contract
- This keeps business logic and workflows decoupled from the auth vendor

## Local model

### `users`

- app-owned `user_id`
- provider mapping fields, such as `propelauth_user_id`
- minimal profile fields needed locally

### `orgs`

- app-owned `org_id`
- provider mapping fields, such as `propelauth_org_id`
- display fields needed locally

### `memberships`

- app-owned membership record between `user_id` and `org_id`
- local role field
- provider mapping or sync metadata as needed

## Identity envelope to backend

Backend accepts one server-generated request context from `web`.

Minimum shape:

```ts
type UserActorContext = {
  actor_type: "user";
  user_id: string;
  org_id: string;
  org_role: "owner" | "member";
};
```

Rules:
- This context is created server-side by `web`
- Browser input never supplies these values directly
- The context is request-scoped
- `org_role` is used as current request context, not as a durable cached claim

Not part of auth:
- `request_id`
- tracing metadata
- debug metadata such as `authenticated_via`

Those may exist on requests, but they are not authorization inputs

## Trust model

- Browser authenticates to `web` through PropelAuth
- `web` authenticates to `backend` through internal service auth
- `backend` accepts identity context only on requests authenticated as coming from `web`
- `backend` does not accept browser session material directly

## Fail-closed rules

### Zero-org anomaly

- If an authenticated user has no resolved local org after provisioning attempt:
  - stop request processing
  - return a clear server error state
  - do not continue in a user-only fallback mode

### Multi-org anomaly during MVP

- If an authenticated user resolves to more than one org during MVP:
  - stop request processing
  - return a clear unsupported-state error
  - do not silently pick one

## What is intentionally deferred

- Multi-org switching
- Invite flows in app UI
- Team management UI
- Rich RBAC
- Cross-org access
- Backend verification of PropelAuth browser tokens

## Implementation notes for the next step

- Add local identity tables with provider mappings
- Implement one server-side `ensure_user_has_default_org(...)` path in `web`
- Make provisioning idempotent
- Define the internal `web -> backend` auth transport
- Keep all business tables keyed by local `org_id`

## Summary

- Product data is org-scoped now
- Org IDs are app-owned
- PropelAuth is the auth and membership authority, not the business ID authority
- `web` provisions the default org on first authenticated entry
- `web` sends a request-scoped identity envelope to `backend`
- `backend` stays provider-agnostic
