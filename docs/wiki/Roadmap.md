# Project Status

The original Phase 0–3 roadmap is complete as a product-baseline plan. The
gateway is on the stable `1.2.9` line; current capability and security posture
are documented in the pages linked below rather than repeated as historical
phase prose.

## Shipped

- Keycloak authentication, explicit Keycloak-to-Forgejo principal mapping,
  policy enforcement, OAuth protected-resource metadata, and token-free audit
  events.
- Bounded Forgejo repository, issue, pull-request, review, release,
  notification, comment, and wiki operations, including PR diff inspection and
  evidence-backed review submission.
- Exact-payload, single-use approval gates for reviewed high-risk operations:
  issue creation, PR creation and merge, release creation, PR review
  submission, and wiki publication.
- Pinned Forgejo API classification: all 506 upstream operations are assessed;
  only 18 reviewed semantic-overlay operations are executable.

## Intentional boundaries

This is not a generic Forgejo API proxy. Generated endpoints without a
reviewed semantic overlay remain metadata-only and disabled. Instance admin,
destructive actions, release deletion or replacement, release-asset upload,
and comparable high-risk paths remain disabled.

## Remaining work

The concise, actionable maintainer backlog is in
[Further Planned Improvements](Further-Planned-Improvements.md). It contains
the security, release-process, and deliberately deferred capability work that
is still relevant.

## Historical material

The detailed Phase 0–3 plan and the obsolete pre-1.0 Codeberg issue drafts
are retained under [Documentation Archive](Documentation-Archive.md). Release
notes are the authoritative feature-by-feature history.
