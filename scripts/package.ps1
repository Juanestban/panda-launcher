# Builds the release and everything a GitHub release publishes, into target\dist (ADR 0009, 0010):
#   PandaLauncher-<version>-x64.msi                 per-user installer
#   PandaLauncher-<version>-x64-portable.exe        standalone executable
#   SHA256SUMS.txt                                  checksums the in-app updater verifies
#   <installer>.sig                                 optional ed25519 signatures over each installer's SHA-256
# Needs the repo's local WiX tool (`dotnet tool restore`, see dotnet-tools.json).
# Optional build-time inputs (environment): PANDA_MSA_CLIENT_ID, PANDA_UPDATE_PUBLIC_KEY, PANDA_UPDATE_SIGNING_KEY.
# Without the signing key there are no .sig files; keyless builds update from the SHA-256 checksums alone.

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

cargo build --release --locked -p panda-launcher
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

# The workspace version, e.g. 0.1.0.
$version = (Select-String -Path Cargo.toml -Pattern '^version = "(.+)"').Matches[0].Groups[1].Value
$exe = Join-Path $root "target\release\panda-launcher.exe"
$icon = Join-Path $root "crates\panda-launcher\assets\icon\panda.ico"

$dist = Join-Path $root "target\dist"
Remove-Item $dist -Recurse -Force -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $dist | Out-Null
$msi = Join-Path $dist "PandaLauncher-$version-x64.msi"
$portable = Join-Path $dist "PandaLauncher-$version-x64-portable.exe"

dotnet tool restore | Out-Null
dotnet tool run wix build wix\main.wxs -arch x64 -d Version=$version -d ExePath=$exe -d IconPath=$icon -o $msi
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Remove-Item (Join-Path $dist "*.wixpdb") -ErrorAction SilentlyContinue
Copy-Item $exe $portable

# sha256sum format ("<hex>  <name>"), LF line endings.
$sums = Get-ChildItem $dist -File | Sort-Object Name | ForEach-Object { "$((Get-FileHash $_.FullName -Algorithm SHA256).Hash.ToLower())  $($_.Name)" }
[System.IO.File]::WriteAllText((Join-Path $dist "SHA256SUMS.txt"), (($sums -join "`n") + "`n"))

if ($env:PANDA_UPDATE_SIGNING_KEY) {
  cargo run --release --locked -p panda-core --example sign -- $msi $portable
  if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}

Get-ChildItem $dist | Select-Object Name, Length | Format-Table -AutoSize
