# Script pour lancer le client WebSocket
Write-Host "=== Démarrage du Client WebSocket ===" -ForegroundColor Green
Write-Host "Connexion au serveur 127.0.0.1:8080..." -ForegroundColor Cyan
Write-Host ""

# Changer vers le répertoire du projet
Set-Location $PSScriptRoot

# Lancer le client
try {
    cargo run client
} catch {
    Write-Host "Erreur lors du démarrage du client: $_" -ForegroundColor Red
    Read-Host "Appuyez sur Entrée pour continuer"
}
