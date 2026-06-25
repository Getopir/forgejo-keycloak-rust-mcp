param(
    [string]$ApiBase = "http://192.168.87.56:8080/api/m2m",
    [Parameter(Mandatory = $true)]
    [string]$UserId,
    [string]$Token = "",
    [switch]$AllowDuplicate
)

$ErrorActionPreference = "Stop"

if ($Token.Trim().Length -eq 0) {
    $Token = $env:OPIR_O_M2M_TOKEN
}
if ($Token.Trim().Length -eq 0) {
    throw "Provide -Token or set OPIR_O_M2M_TOKEN. Do not commit token values."
}

function New-OpirHeaders {
    return @{
        "Content-Type" = "application/json"
        "Authorization" = "Bearer $Token"
        "X-OPIR-User-Id" = $UserId
    }
}

function Invoke-OpirJson {
    param(
        [Parameter(Mandatory = $true)]
        [ValidateSet("GET", "POST", "PATCH", "PUT", "DELETE")]
        [string]$Method,
        [Parameter(Mandatory = $true)]
        [string]$Path,
        [object]$Body = $null
    )
    $uri = "$($ApiBase.TrimEnd('/'))$Path"
    $headers = New-OpirHeaders
    if ($null -eq $Body) {
        return Invoke-RestMethod -Method $Method -Uri $uri -Headers $headers
    }
    $json = $Body | ConvertTo-Json -Depth 30
    return Invoke-RestMethod -Method $Method -Uri $uri -Headers $headers -Body $json
}

$projectTitle = "Forgejo Keycloak Rust MCP Gateway"

if (-not $AllowDuplicate) {
    $existingProjects = Invoke-OpirJson -Method GET -Path "/projects?limit=200&offset=0"
    $existing = @($existingProjects.data) | Where-Object { $_.title -eq $projectTitle } | Select-Object -First 1
    if ($existing) {
        Write-Host "Project already exists: $($existing.id)"
        exit 0
    }
}

$description = @"
Build a clean-room Rust MCP gateway for Forgejo access through Keycloak.

Authority chain: Keycloak authenticates humans and agents; the Rust gateway authorizes operation class and policy; Forgejo authorizes repository and organization access through native ACLs.

Initial proof target: use local Keycloak and VM166 Neutrino agent against an isolated blank Forgejo lab VM.
"@

$created = Invoke-OpirJson -Method POST -Path "/projects" -Body @{
    title = $projectTitle
    description = $description
    visibility = "public"
    user_id = $UserId
}
$project = $created.project
Write-Host "Created project: $($project.id)"

$loops = @(
    @{
        code = "FKRM-L001"
        title = "Clean-room foundation and identity chain"
        date = "2026-07-03"
        done = @("Rust workspace builds and tests", "Protected-resource metadata is served", "Missing and wrong-audience tokens are rejected", "Policy registry denies missing scopes")
        tasks = @("Preserve source brief as original input", "Implement health and authenticated MCP probe", "Implement JWT validation against Keycloak JWKS", "Add policy and audit tests")
    },
    @{
        code = "FKRM-L002"
        title = "Forgejo delegation and principal mapping"
        date = "2026-07-10"
        done = @("Immutable subject mapping exists", "Trusted-header backend is isolated", "Caller-supplied identity headers are ignored", "Forgejo ACL is final")
        tasks = @("Add SQLite mapping storage", "Implement Forgejo trusted-header client", "Add header stripping tests", "Verify mapped user permissions in lab")
    },
    @{
        code = "FKRM-L003"
        title = "Curated MCP and CLI surface"
        date = "2026-07-17"
        done = @("Core repo, file, issue, PR and Actions operations exist", "Large responses are bounded", "CLI and MCP share registry")
        tasks = @("Define semantic operation schemas", "Implement resource URIs", "Add cursor and truncation metadata", "Add CLI wrappers")
    },
    @{
        code = "FKRM-L004"
        title = "Generated Forgejo API coverage"
        date = "2026-07-24"
        done = @("Pinned Forgejo specs are normalized", "Every endpoint is classified", "Build fails on unclassified endpoint")
        tasks = @("Pin Forgejo Swagger specs", "Build normalizer", "Add semantic overlay", "Generate coverage report")
    },
    @{
        code = "FKRM-L005"
        title = "Lab VM and Neutrino verification"
        date = "2026-07-31"
        done = @("Blank Forgejo VM is installed", "Gateway uses local Keycloak", "VM166 agent proves Forgejo access via gateway", "OPIR-O evidence is updated")
        tasks = @("Provision lab VM", "Install Forgejo and forgejo-mcpd", "Create Keycloak clients and service account", "Run VM166 agent smoke and record evidence")
    }
)

$sort = 1
foreach ($loop in $loops) {
    $outcome = Invoke-OpirJson -Method POST -Path "/projects/$($project.id)/outcomes" -Body @{
        title = "$($loop.code): $($loop.title)"
        description = "Implementation loop for $projectTitle."
        definition_of_done = $loop.done
        desired_end_date = $loop.date
        loop_count = 1
        sort_order = $sort
        responsible_owner_type = "person"
        responsible_user_id = $UserId
        user_id = $UserId
    }
    Write-Host "Created loop: $($outcome.id) $($loop.code)"
    $taskIndex = 1
    foreach ($task in $loop.tasks) {
        $taskCode = "{0}-T{1:D2}" -f $loop.code, $taskIndex
        $taskDef = Invoke-OpirJson -Method POST -Path "/outcomes/$($outcome.id)/tasks" -Body @{
            description = "$taskCode - $task"
            estimate_hours = 4
            introduced_loop_index = 1
            user_id = $UserId
        }
        Write-Host "  Created task: $($taskDef.id) $taskCode"
        $taskIndex += 1
    }
    $sort += 1
}

Write-Host "OPIR-O project setup complete."
Write-Host "Project id: $($project.id)"
