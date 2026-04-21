param(
    [ValidateSet("dev", "build")]
    [string]$Mode = "dev"
)

$ErrorActionPreference = "Stop"

function Get-VsDevCmdPath {
    $vsWhere = Join-Path ${env:ProgramFiles(x86)} "Microsoft Visual Studio\Installer\vswhere.exe"
    if (Test-Path $vsWhere) {
        $installationPath = & $vsWhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
        if ($LASTEXITCODE -eq 0 -and $installationPath) {
            $candidate = Join-Path $installationPath "Common7\Tools\VsDevCmd.bat"
            if (Test-Path $candidate) {
                return $candidate
            }
        }
    }

    $fallbacks = @(
        "C:\Program Files\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat",
        "C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat",
        "C:\Program Files\Microsoft Visual Studio\2022\Professional\Common7\Tools\VsDevCmd.bat",
        "C:\Program Files\Microsoft Visual Studio\2022\Enterprise\Common7\Tools\VsDevCmd.bat"
    )

    foreach ($candidate in $fallbacks) {
        if (Test-Path $candidate) {
            return $candidate
        }
    }

    return $null
}

$vsDevCmd = Get-VsDevCmdPath
if (-not $vsDevCmd) {
    Write-Error "Visual Studio C++ Build Tools were not detected. Install 'Desktop development with C++' and retry."
}

Write-Host "Using Visual Studio developer environment: $vsDevCmd" -ForegroundColor Cyan

$tauriAction = if ($Mode -eq "dev") { "dev" } else { "build" }
$command = "call `"$vsDevCmd`" -arch=x64 -host_arch=x64 >nul && pnpm tauri $tauriAction"

cmd /d /s /c $command
exit $LASTEXITCODE
