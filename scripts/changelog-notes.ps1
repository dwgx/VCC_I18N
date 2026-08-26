# Print the CHANGELOG section for a version (v0.2.0 or 0.2.0).
param(
    [Parameter(Mandatory = $true)]
    [string]$Version
)
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$root = Split-Path -Parent $PSScriptRoot
$ver = $Version.Trim()
if ($ver.StartsWith('v') -or $ver.StartsWith('V')) {
    $ver = $ver.Substring(1)
}
$path = Join-Path $root 'CHANGELOG.md'
$text = Get-Content -LiteralPath $path -Raw
$pattern = '(?ms)^## ' + [regex]::Escape($ver) + '(?: +-[^\r\n]+)?\r?\n(?<body>.*?)(?=^## |\z)'
$match = [regex]::Match($text, $pattern)
if (-not $match.Success) {
    throw "no CHANGELOG section for $ver"
}
$body = $match.Groups['body'].Value.Trim()
if ([string]::IsNullOrWhiteSpace($body)) {
    throw "empty CHANGELOG section for $ver"
}
Write-Output $body
