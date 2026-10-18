$ErrorActionPreference = 'Stop'
$installer = (Resolve-Path 'dist/plugins/SAD-PET-Plugins-Windows-x64-Setup.exe').Path
$staged = (Resolve-Path 'dist/plugins/SAD-PET-Plugins-Windows-x64/VST3').Path
$destination = Join-Path $env:CommonProgramFiles 'VST3'
$presetFolder = Join-Path $env:APPDATA 'SAD PET/Presets'
New-Item -ItemType Directory -Force -Path $destination, $presetFolder | Out-Null
$sentinel = Join-Path $destination 'unrelated-installer-test.txt'
$preset = Join-Path $presetFolder 'installer-test.txt'
Set-Content $sentinel 'Keep other plugins'
Set-Content $preset 'Keep user presets'

function Install-Plugins {
    $log = Join-Path $env:RUNNER_TEMP 'sad-pet-setup.log'
    $process = Start-Process $installer -ArgumentList "/VERYSILENT /SUPPRESSMSGBOXES /NORESTART /LOG=`"$log`"" -Wait -PassThru
    if ($process.ExitCode -notin @(0, 3010)) {
        Get-Content $log
        throw "Installer failed: $($process.ExitCode)"
    }
    foreach ($name in @('SAD PET.vst3', 'SAD PET FX.vst3')) {
        $bundle = Join-Path $staged $name
        Get-ChildItem $bundle -File -Recurse | ForEach-Object {
            $relative = [IO.Path]::GetRelativePath($staged, $_.FullName)
            $installed = Join-Path $destination $relative
            if (-not (Test-Path $installed)) { throw "Missing installed file: $relative" }
            if ((Get-FileHash $_.FullName).Hash -ne (Get-FileHash $installed).Hash) {
                throw "Installed bytes differ: $relative"
            }
        }
    }
}

Install-Plugins
foreach ($name in @('SAD PET.vst3', 'SAD PET FX.vst3')) {
    Set-Content (Join-Path $destination "$name/stale-installer-test") 'Old file'
}
Install-Plugins
foreach ($name in @('SAD PET.vst3', 'SAD PET FX.vst3')) {
    if (Test-Path (Join-Path $destination "$name/stale-installer-test")) { throw 'Upgrade left a stale bundle file' }
}
$uninstaller = Join-Path $env:ProgramFiles 'SAD PET/Plugins/unins000.exe'
$process = Start-Process $uninstaller -ArgumentList '/VERYSILENT /SUPPRESSMSGBOXES /NORESTART' -Wait -PassThru
if ($process.ExitCode -ne 0) { throw "Uninstaller failed: $($process.ExitCode)" }
foreach ($name in @('SAD PET.vst3', 'SAD PET FX.vst3')) {
    if (Test-Path (Join-Path $destination $name)) { throw "Uninstall left $name" }
}
if ((Get-Content $sentinel -Raw).Trim() -ne 'Keep other plugins') { throw 'Another plugin was changed' }
if ((Get-Content $preset -Raw).Trim() -ne 'Keep user presets') { throw 'User presets were changed' }
Write-Host 'Windows install, same-version upgrade, payload hashes, uninstall and preset preservation passed.'
