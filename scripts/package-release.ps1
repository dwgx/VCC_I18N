# Build the Windows release binary and checksums. Do not commit dist/.
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

cargo build --release
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$dist = Join-Path $root 'dist'
New-Item -ItemType Directory -Force -Path $dist | Out-Null
$exe = Join-Path $root 'target\release\vcc-i18n.exe'
if (-not (Test-Path -LiteralPath $exe)) {
    throw "missing $exe"
}
Copy-Item -LiteralPath $exe -Destination (Join-Path $dist 'vcc-i18n.exe') -Force
$hash = (Get-FileHash -LiteralPath (Join-Path $dist 'vcc-i18n.exe') -Algorithm SHA256).Hash.ToLowerInvariant()
$sums = Join-Path $dist 'SHA256SUMS.txt'
[System.IO.File]::WriteAllText($sums, "$hash  vcc-i18n.exe`n", [System.Text.Encoding]::ASCII)
Write-Output "wrote dist\vcc-i18n.exe"
Write-Output "sha256 $hash"
