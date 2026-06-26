#!/usr/bin/env python3
# SPDX-License-Identifier: AGPL-3.0-or-later

"""Generate the Forgejo API coverage report from the pinned Swagger spec."""

from __future__ import annotations

import collections
import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SPEC_PATH = ROOT / "vendor" / "forgejo-api" / "forgejo-15.0.3-gitea-1.22.0-swagger.v1.json"
REPORT_PATH = ROOT / "docs" / "generated" / "forgejo-api-coverage.md"
HTTP_METHODS = {"get", "post", "put", "patch", "delete", "head"}


SEMANTIC_OVERLAY = {
    ("GET", "/repos/{owner}/{repo}"): "list_repository_metadata",
    ("GET", "/repos/{owner}/{repo}/issues"): "list_repository_issues",
    ("POST", "/repos/{owner}/{repo}/issues/{index}/comments"): "create_issue_comment",
    ("GET", "/repos/{owner}/{repo}/pulls"): "list_pull_requests",
    ("POST", "/repos/{owner}/{repo}/pulls"): "create_pull_request",
    ("GET", "/repos/{owner}/{repo}/pulls/{index}/reviews"): "list_pull_request_reviews",
    ("GET", "/repos/{owner}/{repo}/releases"): "list_releases",
    ("POST", "/repos/{owner}/{repo}/releases"): "create_release",
    ("GET", "/notifications"): "list_notifications",
    ("POST", "/repos/{owner}/{repo}/pulls/{index}/merge"): "merge_pull_request",
}


def target_type(path: str, tags: list[str]) -> str:
    if "/admin" in path or "admin" in tags:
        return "admin"
    if path.startswith("/activitypub") or "activitypub" in tags:
        return "activity_pub"
    if "/pulls" in path:
        return "pull_request"
    if "/issues" in path:
        return "issue"
    if "/releases" in path:
        return "release"
    if "/notifications" in path:
        return "notification"
    if "/orgs" in path:
        return "organization"
    if "/repos" in path or "repository" in tags:
        return "repository"
    if "/settings" in path:
        return "settings"
    if "/user" in path or "/users" in path:
        return "user"
    return "unknown"


def risk(method: str, path: str, operation_id: str, kind: str) -> str:
    operation_id = operation_id.lower()
    path = path.lower()
    if kind == "admin" or "/sudo" in path:
        return "site_admin"
    if method == "DELETE" or "delete" in operation_id or "purge" in operation_id:
        return "destructive"
    if any(token in path for token in ("token", "secret", "/keys", "/hooks", "/oauth2")):
        return "secret"
    if kind == "activity_pub" and method != "GET":
        return "network_egress"
    if method in {"GET", "HEAD"}:
        return "read_private"
    if "/comments" in path or "comment" in operation_id:
        return "write_additive"
    return "write_mutating"


def approval_required(risk_name: str) -> bool:
    return risk_name in {
        "write_mutating",
        "destructive",
        "network_egress",
        "secret",
        "site_admin",
        "long_running",
    }


def endpoints(spec: dict) -> list[dict]:
    rows = []
    for path, item in spec["paths"].items():
        for method, operation in item.items():
            if method not in HTTP_METHODS:
                continue
            method_name = method.upper()
            tags = operation.get("tags") or []
            op_id = operation.get("operationId") or ""
            kind = target_type(path, tags)
            risk_name = risk(method_name, path, op_id, kind)
            semantic = SEMANTIC_OVERLAY.get((method_name, path))
            rows.append(
                {
                    "method": method_name,
                    "path": path,
                    "operation_id": op_id,
                    "target_type": kind,
                    "risk": risk_name,
                    "approval_required": approval_required(risk_name),
                    "exposure": "semantic_overlay" if semantic else "disabled",
                    "semantic_operation": semantic or "",
                }
            )
    return sorted(rows, key=lambda row: (row["path"], row["method"]))


def main() -> None:
    raw = SPEC_PATH.read_bytes()
    spec = json.loads(raw.decode("utf-8"))
    rows = endpoints(spec)
    by_risk = collections.Counter(row["risk"] for row in rows)
    by_target = collections.Counter(row["target_type"] for row in rows)
    semantic = [row for row in rows if row["exposure"] == "semantic_overlay"]
    destructive = [row for row in rows if row["risk"] == "destructive"]
    admin = [row for row in rows if row["target_type"] == "admin"]
    sha256 = hashlib.sha256(raw).hexdigest()

    lines = [
        "# Forgejo API Coverage",
        "",
        "Generated from the pinned Forgejo Swagger document.",
        "",
        f"- Source version: `{spec.get('info', {}).get('version', 'unknown')}`",
        f"- Pinned spec: `{SPEC_PATH.relative_to(ROOT).as_posix()}`",
        f"- SHA-256: `{sha256}`",
        f"- Total operations: `{len(rows)}`",
        f"- Semantic overlay operations: `{len(semantic)}`",
        f"- Disabled metadata-only operations: `{len(rows) - len(semantic)}`",
        f"- Approval-required operations: `{sum(1 for row in rows if row['approval_required'])}`",
        f"- Destructive operations: `{len(destructive)}`",
        f"- Admin operations: `{len(admin)}`",
        "",
        "## Policy",
        "",
        "Generated coverage does not mean generic execution. Only endpoints with",
        "`semantic_overlay` exposure are reachable through named MCP tools. Every other",
        "endpoint remains disabled until it receives a reviewed semantic operation,",
        "scope, risk class, output limit, and approval policy.",
        "",
        "## Risk Counts",
        "",
        "| Risk | Count |",
        "| --- | ---: |",
    ]
    lines.extend(f"| `{name}` | {count} |" for name, count in sorted(by_risk.items()))
    lines.extend(["", "## Target Counts", "", "| Target | Count |", "| --- | ---: |"])
    lines.extend(f"| `{name}` | {count} |" for name, count in sorted(by_target.items()))
    lines.extend(
        [
            "",
            "## Semantic Overlay",
            "",
            "| MCP operation | Method | Path | Forgejo operationId | Risk | Approval |",
            "| --- | --- | --- | --- | --- | --- |",
        ]
    )
    for row in semantic:
        lines.append(
            "| `{semantic_operation}` | `{method}` | `{path}` | `{operation_id}` | `{risk}` | `{approval}` |".format(
                **row,
                approval="yes" if row["approval_required"] else "no",
            )
        )
    lines.extend(
        [
            "",
            "## Disabled Destructive/Admin Examples",
            "",
            "| Method | Path | operationId | Risk | Target |",
            "| --- | --- | --- | --- | --- |",
        ]
    )
    for row in sorted(destructive + admin, key=lambda row: (row["path"], row["method"]))[:40]:
        lines.append(
            f"| `{row['method']}` | `{row['path']}` | `{row['operation_id']}` | `{row['risk']}` | `{row['target_type']}` |"
        )

    REPORT_PATH.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"wrote {REPORT_PATH.relative_to(ROOT)} from {len(rows)} operations")


if __name__ == "__main__":
    main()
