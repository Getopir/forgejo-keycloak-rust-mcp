# SPDX-License-Identifier: AGPL-3.0-or-later

[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [ValidatePattern('^\d+\.\d+\.\d+$')]
    [string]$Version,

    [Parameter(Mandatory = $true)]
    [string]$SigningKey,

    [Parameter(Mandatory = $true)]
    [string]$OutputDirectory
)

$ErrorActionPreference = 'Stop'
$tag = "v$Version"
$prefix = "forgejo-keycloak-rust-mcp-$Version/"
$resolvedKey = (Resolve-Path -LiteralPath $SigningKey).Path

& git rev-parse --verify "$tag^{commit}" | Out-Null
if ($LASTEXITCODE -ne 0) {
    throw "Git tag $tag does not resolve to a commit"
}

if (Test-Path -LiteralPath $OutputDirectory) {
    if (Get-ChildItem -LiteralPath $OutputDirectory -Force) {
        throw "Output directory must be empty: $OutputDirectory"
    }
} else {
    New-Item -ItemType Directory -Path $OutputDirectory | Out-Null
}

$output = (Resolve-Path -LiteralPath $OutputDirectory).Path
$tarName = "forgejo-keycloak-rust-mcp-$Version.tar.gz"
$zipName = "forgejo-keycloak-rust-mcp-$Version.zip"
$tarPath = Join-Path $output $tarName
$zipPath = Join-Path $output $zipName
$checksumsPath = Join-Path $output 'SHA256SUMS'

& git archive --format=tar.gz --prefix=$prefix --output=$tarPath $tag
if ($LASTEXITCODE -ne 0) { throw 'Failed to create tar.gz source archive' }

& git archive --format=zip --prefix=$prefix --output=$zipPath $tag
if ($LASTEXITCODE -ne 0) { throw 'Failed to create zip source archive' }

$checksumLines = @($tarPath, $zipPath) | ForEach-Object {
    $hash = (Get-FileHash -Algorithm SHA256 -LiteralPath $_).Hash.ToLowerInvariant()
    "$hash  $([IO.Path]::GetFileName($_))"
}
[IO.File]::WriteAllLines($checksumsPath, $checksumLines, [Text.Encoding]::ASCII)

& ssh-keygen -Y sign -f $resolvedKey -n file $checksumsPath
if ($LASTEXITCODE -ne 0) { throw 'Failed to sign SHA256SUMS' }

Get-ChildItem -LiteralPath $output | Select-Object Name, Length
