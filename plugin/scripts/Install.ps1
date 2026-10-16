#Requires -RunAsAdministrator
$ErrorActionPreference = "Stop"
$pluginDestination = Join-Path $env:CommonProgramFiles "VST3"
New-Item -ItemType Directory -Force -Path $pluginDestination | Out-Null
Get-ChildItem (Join-Path $PSScriptRoot "VST3") -Directory -Filter "*.vst3" | ForEach-Object {
    Copy-Item $_.FullName -Destination $pluginDestination -Recurse -Force
}
Write-Host "SAD PET and SAD PET FX installed. Restart your DAW and rescan plug-ins."
