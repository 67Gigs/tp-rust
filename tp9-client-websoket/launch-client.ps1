# Script pour lancer le client dans une nouvelle fenêtre
$scriptPath = Join-Path $PSScriptRoot "start-client.ps1"
Start-Process powershell -ArgumentList "-ExecutionPolicy", "Bypass", "-File", "`"$scriptPath`""
