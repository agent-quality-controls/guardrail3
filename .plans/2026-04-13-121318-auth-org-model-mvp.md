# Auth Org Model - MVP

## Decision

- Websmasher is org-scoped internally from day 1
- Every user gets a private default org
- MVP supports exactly one org per user
- No org management UI is built in Websmasher for MVP
- PropelAuth remains the source of truth for auth and org membership

## Why

- This preserves an org-based data model without forcing multi-org product work into MVP
- Billing, permissions, reports, and crawls can all attach to `org_id` immediately
- It avoids the future migration where a user-based product has to be bridged into an org-based product later
- PropelAuth already provides org membership and hosted org management, so we do not need to build custom org UI now

## MVP behavior

### First login

- User signs in with PropelAuth
- If the user has no orgs, Websmasher creates a private default org for them
- The user is added to that org as `Owner`

### After provisioning

- The app assumes one current org
- That org is the user's only org
- All product data is scoped to that `org_id`

### In-app UX

- No org switcher
- No team management UI
- No custom org settings UI
- No invite flow in the app
- If org management is ever needed before custom UI exists, link out to PropelAuth hosted pages

## Data model implications

- Crawls belong to `org_id`
- Reports belong to `org_id`
- Billing belongs to `org_id`
- Permissions are evaluated in org context

## Product constraints for MVP

- Max orgs per user: `1`
- No multi-org support in app flows
- No shared team workflows required for MVP

## Recommended system boundary

- `web` handles browser auth with PropelAuth
- `web` resolves the authenticated user and their current org
- `backend` receives explicit identity context that includes at least:
  - `user_id`
  - `org_id`
  - org role or equivalent permission context

## Non-goals for MVP

- Multi-org switching
- Cross-org access
- Custom membership management UI
- Rich role editor UI
- Complex invite and onboarding flows

## Risks to handle explicitly

### Provisioning race

- If first-login provisioning runs twice, org creation must be idempotent or guarded

### Users without orgs

- The app must not silently continue without an org context
- Missing-org state should trigger provisioning or fail clearly

### Future expansion

- The product should not hardcode "user owns exactly one org" into storage schemas
- MVP UI can assume one org, but backend contracts should remain org-based rather than user-based

## Follow-up questions

- Where does first-login provisioning run: `web` app, `backend`, or a dedicated auth webhook flow?
- What is the exact local persistence model for user, org, and membership records?
- What identity contract does `web` send to `backend`?
- Do we want to expose a link to PropelAuth's hosted org management page in MVP, or keep it fully hidden?

## Current recommendation

- Implement default-org provisioning on first login
- Keep max orgs per user at `1`
- Scope everything to `org_id`
- Ship no org UI in Websmasher MVP
