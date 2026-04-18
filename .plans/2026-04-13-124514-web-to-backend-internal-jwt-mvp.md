# Web to Backend Internal JWT - MVP

## Decision

- `web` authenticates the browser session with PropelAuth
- `web` mints a short-lived internal JWT for calls to `backend`
- `backend` verifies that internal JWT
- `backend` does not verify browser session tokens from PropelAuth

## Why

- This keeps `backend` provider-agnostic
- This gives one canonical internal actor envelope instead of scattered forwarded headers
- This is simple enough for MVP and scales better than raw `user_id` / `org_id` forwarding with a shared secret alone

## Scope

- This JWT is for internal service-to-service calls from `web` to `backend`
- It is not the browser session token
- It is not exposed as a public API token
- It is not the same thing as customer API keys

## Trust model

- Browser authenticates to `web` through PropelAuth
- `web` resolves the authenticated local user and current local org
- `web` mints an internal JWT for the specific backend request
- `backend` verifies signature, issuer, audience, and expiry
- `backend` trusts user/org identity only from verified internal JWTs

## Token shape

Minimum claims:

```json
{
  "iss": "websmasher-web",
  "aud": "websmasher-backend",
  "sub": "user:<user_id>",
  "actor_type": "user",
  "user_id": "<user_id>",
  "org_id": "<org_id>",
  "org_role": "owner",
  "iat": 1710000000,
  "exp": 1710000300
}
```

Notes:
- `user_id` is the app-owned local user ID
- `org_id` is the app-owned local org ID
- `org_role` is request context, not a permanent source of truth

## Lifetime

- Token must be short-lived
- MVP target: minutes, not hours
- Short lifetime limits damage from leakage and reduces stale auth context risk

## What backend must verify

- signature
- issuer
- audience
- expiry
- expected actor shape

If verification fails:
- reject the request

## What is not an auth claim

- request ID
- tracing metadata
- debug metadata

Those may travel separately, but they are not authorization inputs

## Staleness rule

- `org_role` in the token is acceptable for normal request/response flows because the token is short-lived
- It is not acceptable as a long-term persisted authorization artifact
- Long-running jobs must not rely on a stale copied JWT forever

## Long-running job rule

- When `backend` starts a long-running workflow, it may persist actor identity for audit purposes
- It should not persist the internal JWT itself as the long-term source of authorization
- Workflow execution should use the app's own actor model and current system rules, not a stale token blob

## Actor model

Initial actor type:

```json
{
  "actor_type": "user"
}
```

Expected future expansion:
- `user`
- `service`

The transport should support more than one actor type without redesign

## Non-goals

- Public API auth
- Browser auth replacement
- Rich permissions encoded into JWT claims
- mTLS
- external auth provider verification inside `backend`

## Implementation notes

- `web` signs the JWT with an internal signing key
- `backend` verifies with the matching verification key or shared secret, depending on signing mode
- Prefer a standard JWT library in both services
- Keep claims minimal

## Summary

- Use a short-lived internal JWT from `web` to `backend`
- Keep `backend` decoupled from PropelAuth
- Treat the JWT as request-scoped internal identity, not as a permanent authorization record
