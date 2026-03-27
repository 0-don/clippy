$ErrorActionPreference = 'Stop'

$packageArgs = @{
  packageName    = 'clippy-clipboard'
  fileType       = 'exe'
  url64bit       = 'https://github.com/0-don/clippy/releases/download/v{{VERSION}}/clippy_{{VERSION}}_x64-setup.exe'
  checksum64     = '{{SHA256}}'
  checksumType64 = 'sha256'
  silentArgs     = '/S'
  validExitCodes = @(0)
}

Install-ChocolateyPackage @packageArgs
