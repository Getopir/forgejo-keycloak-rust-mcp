# Forgejo API Coverage

Generated from the pinned Forgejo Swagger document.

- Source version: `16.0.0`
- Pinned spec: `vendor/forgejo-api/forgejo-16.0.0-swagger.v1.json`
- SHA-256: `a41f976f1d616e273c0a1855a625928e59e758f324f0b02fc247a25a5469be84`
- Refresh review: `docs/generated/forgejo-api-coverage-review-16.0.0.md`
- Total operations: `506`
- Semantic overlay operations: `18`
- Disabled metadata-only operations: `488`
- Approval-required operations: `282`
- Destructive operations: `74`
- Admin operations: `51`

## Policy

Generated coverage does not mean generic execution. Only endpoints with
`semantic_overlay` exposure are reachable through named MCP tools. Every other
endpoint remains disabled until it receives a reviewed semantic operation,
scope, risk class, output limit, and approval policy.

## Risk Counts

| Risk | Count |
| --- | ---: |
| `destructive` | 74 |
| `network_egress` | 5 |
| `read_private` | 217 |
| `secret` | 38 |
| `site_admin` | 51 |
| `write_additive` | 7 |
| `write_mutating` | 114 |

## Target Counts

| Target | Count |
| --- | ---: |
| `activity_pub` | 11 |
| `admin` | 51 |
| `issue` | 58 |
| `notification` | 7 |
| `organization` | 56 |
| `pull_request` | 26 |
| `release` | 13 |
| `repository` | 176 |
| `settings` | 5 |
| `unknown` | 28 |
| `user` | 75 |

## Semantic Overlay

| MCP operation | Method | Path | Forgejo operationId | Risk | Approval |
| --- | --- | --- | --- | --- | --- |
| `list_notifications` | `GET` | `/notifications` | `notifyGetList` | `read_private` | `no` |
| `list_repository_metadata` | `GET` | `/repos/{owner}/{repo}` | `repoGet` | `read_private` | `no` |
| `list_repository_issues` | `GET` | `/repos/{owner}/{repo}/issues` | `issueListIssues` | `read_private` | `no` |
| `create_issue` | `POST` | `/repos/{owner}/{repo}/issues` | `issueCreateIssue` | `write_mutating` | `yes` |
| `create_issue_comment` | `POST` | `/repos/{owner}/{repo}/issues/{index}/comments` | `issueCreateComment` | `write_additive` | `no` |
| `list_pull_requests` | `GET` | `/repos/{owner}/{repo}/pulls` | `repoListPullRequests` | `read_private` | `no` |
| `create_pull_request` | `POST` | `/repos/{owner}/{repo}/pulls` | `repoCreatePullRequest` | `write_mutating` | `yes` |
| `get_pull_request_diff` | `GET` | `/repos/{owner}/{repo}/pulls/{index}.{diffType}` | `repoDownloadPullDiffOrPatch` | `read_private` | `no` |
| `merge_pull_request` | `POST` | `/repos/{owner}/{repo}/pulls/{index}/merge` | `repoMergePullRequest` | `write_mutating` | `yes` |
| `list_pull_request_reviews` | `GET` | `/repos/{owner}/{repo}/pulls/{index}/reviews` | `repoListPullReviews` | `read_private` | `no` |
| `submit_pull_request_review` | `POST` | `/repos/{owner}/{repo}/pulls/{index}/reviews` | `repoCreatePullReview` | `write_mutating` | `yes` |
| `submit_pull_request_review` | `POST` | `/repos/{owner}/{repo}/pulls/{index}/reviews/{id}` | `repoSubmitPullReview` | `write_mutating` | `yes` |
| `list_releases` | `GET` | `/repos/{owner}/{repo}/releases` | `repoListReleases` | `read_private` | `no` |
| `create_release` | `POST` | `/repos/{owner}/{repo}/releases` | `repoCreateRelease` | `write_mutating` | `yes` |
| `create_wiki_page` | `POST` | `/repos/{owner}/{repo}/wiki/new` | `repoCreateWikiPage` | `write_mutating` | `yes` |
| `get_wiki_page` | `GET` | `/repos/{owner}/{repo}/wiki/page/{pageName}` | `repoGetWikiPage` | `read_private` | `no` |
| `update_wiki_page` | `PATCH` | `/repos/{owner}/{repo}/wiki/page/{pageName}` | `repoEditWikiPage` | `write_mutating` | `yes` |
| `list_wiki_pages` | `GET` | `/repos/{owner}/{repo}/wiki/pages` | `repoGetWikiPages` | `read_private` | `no` |

## Disabled Destructive/Admin Examples

| Method | Path | operationId | Risk | Target |
| --- | --- | --- | --- | --- |
| `GET` | `/admin/actions/runners` | `getAdminRunners` | `site_admin` | `admin` |
| `POST` | `/admin/actions/runners` | `registerAdminRunner` | `site_admin` | `admin` |
| `GET` | `/admin/actions/runners/jobs` | `adminGetActionRunJobs` | `site_admin` | `admin` |
| `GET` | `/admin/actions/runners/registration-token` | `adminGetRunnerRegistrationToken` | `site_admin` | `admin` |
| `DELETE` | `/admin/actions/runners/{runner_id}` | `deleteAdminRunner` | `site_admin` | `admin` |
| `GET` | `/admin/actions/runners/{runner_id}` | `getAdminRunner` | `site_admin` | `admin` |
| `GET` | `/admin/cron` | `adminCronList` | `site_admin` | `admin` |
| `POST` | `/admin/cron/{task}` | `adminCronRun` | `site_admin` | `admin` |
| `GET` | `/admin/emails` | `adminGetAllEmails` | `site_admin` | `admin` |
| `GET` | `/admin/emails/search` | `adminSearchEmails` | `site_admin` | `admin` |
| `GET` | `/admin/hooks` | `adminListHooks` | `site_admin` | `admin` |
| `POST` | `/admin/hooks` | `adminCreateHook` | `site_admin` | `admin` |
| `DELETE` | `/admin/hooks/{id}` | `adminDeleteHook` | `site_admin` | `admin` |
| `GET` | `/admin/hooks/{id}` | `adminGetHook` | `site_admin` | `admin` |
| `PATCH` | `/admin/hooks/{id}` | `adminEditHook` | `site_admin` | `admin` |
| `GET` | `/admin/orgs` | `adminGetAllOrgs` | `site_admin` | `admin` |
| `GET` | `/admin/quota/groups` | `adminListQuotaGroups` | `site_admin` | `admin` |
| `POST` | `/admin/quota/groups` | `adminCreateQuotaGroup` | `site_admin` | `admin` |
| `DELETE` | `/admin/quota/groups/{quotagroup}` | `adminDeleteQuotaGroup` | `site_admin` | `admin` |
| `GET` | `/admin/quota/groups/{quotagroup}` | `adminGetQuotaGroup` | `site_admin` | `admin` |
| `DELETE` | `/admin/quota/groups/{quotagroup}/rules/{quotarule}` | `adminRemoveRuleFromQuotaGroup` | `site_admin` | `admin` |
| `PUT` | `/admin/quota/groups/{quotagroup}/rules/{quotarule}` | `adminAddRuleToQuotaGroup` | `site_admin` | `admin` |
| `GET` | `/admin/quota/groups/{quotagroup}/users` | `adminListUsersInQuotaGroup` | `site_admin` | `admin` |
| `DELETE` | `/admin/quota/groups/{quotagroup}/users/{username}` | `adminRemoveUserFromQuotaGroup` | `site_admin` | `admin` |
| `PUT` | `/admin/quota/groups/{quotagroup}/users/{username}` | `adminAddUserToQuotaGroup` | `site_admin` | `admin` |
| `GET` | `/admin/quota/rules` | `adminListQuotaRules` | `site_admin` | `admin` |
| `POST` | `/admin/quota/rules` | `adminCreateQuotaRule` | `site_admin` | `admin` |
| `DELETE` | `/admin/quota/rules/{quotarule}` | `adminDeleteQuotaRule` | `site_admin` | `admin` |
| `GET` | `/admin/quota/rules/{quotarule}` | `adminGetQuotaRule` | `site_admin` | `admin` |
| `PATCH` | `/admin/quota/rules/{quotarule}` | `adminEditQuotaRule` | `site_admin` | `admin` |
| `GET` | `/admin/runners/jobs` | `adminSearchRunJobs` | `site_admin` | `admin` |
| `GET` | `/admin/runners/registration-token` | `adminGetRegistrationToken` | `site_admin` | `admin` |
| `GET` | `/admin/unadopted` | `adminUnadoptedList` | `site_admin` | `admin` |
| `DELETE` | `/admin/unadopted/{owner}/{repo}` | `adminDeleteUnadoptedRepository` | `site_admin` | `admin` |
| `POST` | `/admin/unadopted/{owner}/{repo}` | `adminAdoptRepository` | `site_admin` | `admin` |
| `GET` | `/admin/users` | `adminSearchUsers` | `site_admin` | `admin` |
| `POST` | `/admin/users` | `adminCreateUser` | `site_admin` | `admin` |
| `DELETE` | `/admin/users/{username}` | `adminDeleteUser` | `site_admin` | `admin` |
| `PATCH` | `/admin/users/{username}` | `adminEditUser` | `site_admin` | `admin` |
| `DELETE` | `/admin/users/{username}/emails` | `adminDeleteUserEmails` | `site_admin` | `admin` |
