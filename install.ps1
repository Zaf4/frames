$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

$Repository = "datavil/framex"
$Version = if ($env:FX_VERSION) { $env:FX_VERSION } else { "latest" }

$Architecture = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString()
$Target = switch ($Architecture) {
    "X64" { "x86_64-pc-windows-msvc" }
    "Arm64" { "aarch64-pc-windows-msvc" }
    default { throw "framex: unsupported Windows architecture: $Architecture" }
}

$Archive = "fx-$Target.zip"
if ($Version -eq "latest") {
    $ReleaseUrl = "https://github.com/$Repository/releases/latest/download"
}
else {
    $Version = "v$($Version.TrimStart('v'))"
    $ReleaseUrl = "https://github.com/$Repository/releases/download/$Version"
}

$InstallDir = if ($env:FX_INSTALL_DIR) {
    $env:FX_INSTALL_DIR
}
else {
    Join-Path $HOME ".local\bin"
}

$TempDir = Join-Path ([System.IO.Path]::GetTempPath()) "framex-$([guid]::NewGuid())"

try {
    New-Item -ItemType Directory -Path $TempDir -Force | Out-Null
    $ArchivePath = Join-Path $TempDir $Archive
    $ChecksumPath = "$ArchivePath.sha256"

    Write-Host "framex: downloading $Archive"
    Invoke-WebRequest -UseBasicParsing -Uri "$ReleaseUrl/$Archive" -OutFile $ArchivePath
    Invoke-WebRequest -UseBasicParsing -Uri "$ReleaseUrl/$Archive.sha256" -OutFile $ChecksumPath

    $ExpectedChecksum = ((Get-Content $ChecksumPath -Raw).Trim() -split "\s+")[0]
    $ActualChecksum = (Get-FileHash $ArchivePath -Algorithm SHA256).Hash
    if ($ExpectedChecksum -ne $ActualChecksum) {
        throw "framex: checksum verification failed"
    }

    Expand-Archive -Path $ArchivePath -DestinationPath $TempDir -Force
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    Copy-Item (Join-Path $TempDir "fx.exe") (Join-Path $InstallDir "fx.exe") -Force

    Write-Host "framex: installed fx to $(Join-Path $InstallDir 'fx.exe')"
    if (($env:PATH -split ";") -notcontains $InstallDir) {
        Write-Host "framex: add $InstallDir to PATH to run fx"
    }
}
finally {
    if (Test-Path $TempDir) {
        Remove-Item $TempDir -Recurse -Force
    }
}
