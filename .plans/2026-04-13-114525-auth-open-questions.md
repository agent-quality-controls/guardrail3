# Auth Open Questions

## Decided

- Auth provider: PropelAuth
- Current goal: define how auth flows through Websmasher, not which vendor to use

## What still needs design

### 1. Session boundary

Questions:
- Which code is allowed to call PropelAuth directly?
- Where do `require_user()` and `require_org()` live?
- What is the single app-level API for "current user", "current org", and "current session"?

Decision to make:
- Keep PropelAuth usage behind a narrow boundary, likely in `web/app` or a small auth glue layer, not spread across `logic/` and `components/`

### 2. Route protection

Questions:
- Which routes are public?
- Which routes require login?
- Which routes require org membership?
- Do we protect in middleware, route/layout guards, or both?
- What is the redirect behavior for logged-out users?

Decision to make:
- One canonical route protection pattern. No mixed ad hoc checks.

### 3. Org model

Questions:
- Is every report, crawl, and dashboard view scoped to an org?
- Can a user belong to multiple orgs?
- How is the current org selected?
- Is org context carried in the URL, session, or both?
- What happens when a user has zero orgs?

Decision to make:
- Define one canonical meaning of "current org" before building dashboard flows

### 4. Roles and permissions

Questions:
- What product roles exist?
- What can each role do?
- Are permissions enforced in `web`, `backend`, or both?
- Are permissions checked at the org level, report level, or both?

Decision to make:
- Permission checks need a typed policy layer, not scattered `if` statements

### 5. Web to backend trust contract

Questions:
- Does `backend` verify PropelAuth tokens directly?
- Or does `web` authenticate the browser session and forward trusted identity to `backend`?
- What exact user and org fields are passed across service boundaries?
- What headers or token format does `backend` accept?

Decision to make:
- Define one canonical identity contract between services before backend APIs are built out

### 6. Service-to-service auth

Questions:
- How does `web` authenticate to `backend`?
- How does `backend` authenticate to `crawler`?
- How are internal callbacks or job updates authenticated?
- Which credentials are user-scoped vs machine-scoped?

Decision to make:
- Separate browser auth from service auth. Do not reuse user session material for internal service calls.

### 7. Local app data model

Questions:
- Do we persist local `users`, `orgs`, and `memberships` tables?
- Which fields come from PropelAuth and which are local-only?
- When is local state provisioned: first login, first org join, background sync, or lazy create?
- What is the source of truth for names, roles, and membership?

Decision to make:
- Decide whether PropelAuth is only identity/auth, or also the source of truth for org membership inside the app

### 8. Onboarding flow

Questions:
- What happens on first signup?
- Must a user create or join an org before they can use the product?
- Who can invite users?
- What does the empty state look like for a logged-in user with no org access?

Decision to make:
- Define first-run flow before dashboard UI is finalized

### 9. Auth-related error handling

Questions:
- What is the standard response for unauthenticated access?
- What is the standard response for unauthorized access?
- What happens when the session expires mid-flow?
- How are org access errors shown in the UI?

Decision to make:
- Authentication and authorization errors need standard server shapes and standard UI behavior

### 10. Ownership model

Questions:
- Are crawls and reports owned by the user, the org, or both?
- Can any org member view every report?
- What happens to ownership and access when a user leaves an org?

Decision to make:
- Ownership rules must be explicit before report APIs and dashboard filters are finalized

### 11. API keys and machine access

Questions:
- Will Websmasher expose APIs for customers?
- If yes, are API keys tied to a user, an org, or both?
- How do API keys interact with org roles and permissions?

Decision to make:
- Keep machine auth as a separate design track from browser session auth

## Recommended order

1. Session boundary
2. Org model
3. Web to backend trust contract
4. Roles and permissions
5. Route protection
6. Local app data model
7. Onboarding flow
8. Auth-related error handling
9. Ownership model
10. Service-to-service auth
11. API keys and machine access

## What should come out of this

- A short auth architecture doc
- One canonical session API in the `web` app
- One canonical identity contract between `web` and `backend`
- A clear org model
- A clear permission model
- A clear route protection pattern
