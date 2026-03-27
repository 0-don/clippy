$ErrorActionPreference = 'Stop'

$uninstallPath = Join-Path $env:LOCALAPPDATA 'clippy\uninstall.exe'
if (Test-Path $uninstallPath) {
  Start-Process $uninstallPath -ArgumentList '/S' -Wait
}
