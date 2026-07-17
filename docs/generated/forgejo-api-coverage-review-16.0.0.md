# Forgejo 16.0.0 API Coverage Review

Regenerated for the `2.0.0` compatibility baseline from the unchanged pinned
document with SHA-256
`a41f976f1d616e273c0a1855a625928e59e758f324f0b02fc247a25a5469be84`.
Contract tests verify that all 17 existing named semantic operations map to the
18 reviewed endpoint overlays below. No disabled endpoint became executable.

This review compares the pinned Forgejo `16.0.0` Swagger document with the
previous `15.0.3+gitea-1.22.0` pin.

- Added operations: `15`
- Removed operations: `0`
- Existing semantic overlays still present: `18` of `18`
- Newly exposed semantic overlays: `0`
- New approval-required operations: `7`
- Review result: all new operations remain `disabled` metadata

| Method | Path | operationId | Risk | Approval | Target |
| --- | --- | --- | --- | --- | --- |
| `GET` | `/actions/run` | `getActionsRun` | `read_private` | no | `unknown` |
| `GET` | `/admin/users/{username}/tokens` | `adminListUserAccessTokens` | `site_admin` | yes | `admin` |
| `POST` | `/admin/users/{username}/tokens` | `adminCreateUserAccessToken` | `site_admin` | yes | `admin` |
| `DELETE` | `/admin/users/{username}/tokens/{token}` | `adminDeleteUserAccessToken` | `site_admin` | yes | `admin` |
| `GET` | `/repos/{owner}/{repo}/actions/artifacts` | `ListActionArtifacts` | `read_private` | no | `repository` |
| `DELETE` | `/repos/{owner}/{repo}/actions/artifacts/{artifact_id}` | `DeleteActionArtifact` | `destructive` | yes | `repository` |
| `GET` | `/repos/{owner}/{repo}/actions/artifacts/{artifact_id}` | `GetActionArtifact` | `read_private` | no | `repository` |
| `GET` | `/repos/{owner}/{repo}/actions/artifacts/{artifact_id}/zip` | `DownloadActionArtifact` | `read_private` | no | `repository` |
| `GET` | `/repos/{owner}/{repo}/actions/jobs/{job_id}/logs` | `repoGetActionJobLogs` | `read_private` | no | `repository` |
| `DELETE` | `/repos/{owner}/{repo}/actions/runs/{run_id}` | `DeleteActionRun` | `destructive` | yes | `repository` |
| `GET` | `/repos/{owner}/{repo}/actions/runs/{run_id}/artifacts` | `ListActionRunArtifacts` | `read_private` | no | `repository` |
| `POST` | `/repos/{owner}/{repo}/actions/runs/{run_id}/cancel` | `CancelActionRun` | `write_mutating` | yes | `repository` |
| `GET` | `/repos/{owner}/{repo}/actions/runs/{run_id}/jobs` | `ListActionRunJobs` | `read_private` | no | `repository` |
| `GET` | `/repos/{owner}/{repo}/actions/runs/{run_id}/logs` | `repoGetActionRunLogs` | `read_private` | no | `repository` |
| `POST` | `/user/activitypub/follow` | `userCurrentActivityPubFollow` | `write_mutating` | yes | `user` |

No new endpoint is executable through the MCP gateway as a result of this
refresh. Enabling any of these operations requires a separate semantic review,
scope assignment, output bound, audit contract, and approval decision.
