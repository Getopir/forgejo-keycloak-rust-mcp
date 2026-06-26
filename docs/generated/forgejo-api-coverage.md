# Forgejo API Coverage

Generated from the pinned Forgejo Swagger document.

- Source version: `15.0.3+gitea-1.22.0`
- Pinned spec: `vendor/forgejo-api/forgejo-15.0.3-gitea-1.22.0-swagger.v1.json`
- SHA-256: `a90f2fe1266a7a08dfcf682cd28db96c364e18a7de2a4e559a26afe3485bb26f`
- Total operations: `491`
- Semantic overlay operations: `10`
- Disabled metadata-only operations: `481`
- Approval-required operations: `275`
- Destructive operations: `72`
- Admin operations: `48`

## Policy

Generated coverage does not mean generic execution. Only endpoints with
`semantic_overlay` exposure are reachable through named MCP tools. Every other
endpoint remains disabled until it receives a reviewed semantic operation,
scope, risk class, output limit, and approval policy.

## Risk Counts

| Risk | Count |
| --- | ---: |
| `destructive` | 72 |
| `network_egress` | 5 |
| `read_private` | 209 |
| `secret` | 38 |
| `site_admin` | 48 |
| `write_additive` | 7 |
| `write_mutating` | 112 |

## Target Counts

| Target | Count |
| --- | ---: |
| `activity_pub` | 11 |
| `admin` | 48 |
| `issue` | 58 |
| `notification` | 7 |
| `organization` | 56 |
| `pull_request` | 26 |
| `release` | 13 |
| `repository` | 166 |
| `settings` | 5 |
| `unknown` | 27 |
| `user` | 74 |

## Semantic Overlay

| MCP operation | Method | Path | Forgejo operationId | Risk | Approval |
| --- | --- | --- | --- | --- | --- |
| `list_notifications` | `GET` | `/notifications` | `notifyGetList` | `read_private` | `no` |
| `list_repository_metadata` | `GET` | `/repos/{owner}/{repo}` | `repoGet` | `read_private` | `no` |
| `list_repository_issues` | `GET` | `/repos/{owner}/{repo}/issues` | `issueListIssues` | `read_private` | `no` |
| `create_issue_comment` | `POST` | `/repos/{owner}/{repo}/issues/{index}/comments` | `issueCreateComment` | `write_additive` | `no` |
| `list_pull_requests` | `GET` | `/repos/{owner}/{repo}/pulls` | `repoListPullRequests` | `read_private` | `no` |
| `create_pull_request` | `POST` | `/repos/{owner}/{repo}/pulls` | `repoCreatePullRequest` | `write_mutating` | `yes` |
| `merge_pull_request` | `POST` | `/repos/{owner}/{repo}/pulls/{index}/merge` | `repoMergePullRequest` | `write_mutating` | `yes` |
| `list_pull_request_reviews` | `GET` | `/repos/{owner}/{repo}/pulls/{index}/reviews` | `repoListPullReviews` | `read_private` | `no` |
| `list_releases` | `GET` | `/repos/{owner}/{repo}/releases` | `repoListReleases` | `read_private` | `no` |
| `create_release` | `POST` | `/repos/{owner}/{repo}/releases` | `repoCreateRelease` | `write_mutating` | `yes` |

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
