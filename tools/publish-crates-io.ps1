# SPDX-License-Identifier: AGPL-3.0-or-later

[CmdletBinding()]
param(
    [switch]$Execute,
    [switch]$SkipTests,
    [int]$IndexWaitSeconds = 300
)

$ErrorActionPreference = "Stop"

$Packages = @(
    "forgejo-keycloak-mcp-policy",
    "forgejo-keycloak-mcp-identity",
    "forgejo-keycloak-mcp-audit",
    "forgejo-keycloak-rust-mcp"
)

$LocalDependencies = @{
    "forgejo-keycloak-mcp-policy" = @()
    "forgejo-keycloak-mcp-identity" = @()
    "forgejo-keycloak-mcp-audit" = @("forgejo-keycloak-mcp-policy")
    "forgejo-keycloak-rust-mcp" = @(
        "forgejo-keycloak-mcp-policy",
        "forgejo-keycloak-mcp-identity",
        "forgejo-keycloak-mcp-audit"
    )
}

function Invoke-Cargo {
    param([string[]]$CargoArgs)

    Write-Host "cargo $($CargoArgs -join ' ')"
    & cargo @CargoArgs
    if ($LASTEXITCODE -ne 0) {
        throw "cargo $($CargoArgs -join ' ') failed with exit code $LASTEXITCODE"
    }
}

function Get-WorkspacePackageMap {
    $metadata = & cargo metadata --no-deps --format-version 1 | ConvertFrom-Json
    if ($LASTEXITCODE -ne 0) {
        throw "cargo metadata failed with exit code $LASTEXITCODE"
    }

    $map = @{}
    foreach ($package in $metadata.packages) {
        if ($Packages -contains $package.name) {
            $map[$package.name] = $package.version
        }
    }

    foreach ($name in $Packages) {
        if (-not $map.ContainsKey($name)) {
            throw "workspace package $name was not found by cargo metadata"
        }
    }

    return $map
}

function Get-CratesIoVersions {
    param([string]$Name)

    try {
        $response = Invoke-RestMethod -Uri "https://crates.io/api/v1/crates/$Name" -ErrorAction Stop
        return @($response.versions | ForEach-Object { $_.num })
    } catch {
        return @()
    }
}

function Test-CratesIoVersion {
    param(
        [string]$Name,
        [string]$Version
    )

    return (Get-CratesIoVersions -Name $Name) -contains $Version
}

function Wait-CratesIoVersion {
    param(
        [string]$Name,
        [string]$Version,
        [int]$TimeoutSeconds
    )

    $deadline = (Get-Date).AddSeconds($TimeoutSeconds)
    while ((Get-Date) -lt $deadline) {
        if (Test-CratesIoVersion -Name $Name -Version $Version) {
            Write-Host "$Name $Version is visible on crates.io"
            return
        }

        Start-Sleep -Seconds 10
    }

    throw "$Name $Version did not become visible on crates.io within $TimeoutSeconds seconds"
}

if (-not $Execute) {
    Write-Host "Dry-run mode. Pass -Execute to publish to crates.io."
}

$versions = Get-WorkspacePackageMap

foreach ($name in $Packages) {
    $version = $versions[$name]
    if (Test-CratesIoVersion -Name $name -Version $version) {
        Write-Host "$name $version is already published"
    } else {
        Write-Host "$name $version is not published"
    }
}

if (-not $SkipTests) {
    Invoke-Cargo @("fmt", "--check")
    Invoke-Cargo @("test", "--workspace")
}

foreach ($name in $Packages) {
    $version = $versions[$name]

    if (Test-CratesIoVersion -Name $name -Version $version) {
        Write-Host "Skipping $name $version because it is already published"
        continue
    }

    $missingDependencies = @()
    foreach ($dependencyName in $LocalDependencies[$name]) {
        $dependencyVersion = $versions[$dependencyName]
        if (-not (Test-CratesIoVersion -Name $dependencyName -Version $dependencyVersion)) {
            $missingDependencies += "$dependencyName $dependencyVersion"
        }
    }

    if ((-not $Execute) -and $missingDependencies.Count -gt 0) {
        Write-Host "Skipping dry-run for $name $version because crates.io does not yet have: $($missingDependencies -join ', ')"
        continue
    }

    Invoke-Cargo @("publish", "--dry-run", "-p", $name)

    if ($Execute) {
        Invoke-Cargo @("publish", "-p", $name)
        Wait-CratesIoVersion -Name $name -Version $version -TimeoutSeconds $IndexWaitSeconds
    }
}

Write-Host "crates.io publish sequence completed"
