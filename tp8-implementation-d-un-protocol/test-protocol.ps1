# Script de test pour le protocole de messagerie
# Usage: .\test-protocol.ps1

Write-Host "🚀 Script de test du protocole de messagerie" -ForegroundColor Green
Write-Host "═══════════════════════════════════════════════" -ForegroundColor Green

# Vérifier que Rust et Cargo sont installés
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Erreur: Cargo n'est pas installé ou pas dans le PATH" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Cargo trouvé" -ForegroundColor Green

# Compiler le projet
Write-Host "`n🔨 Compilation du projet..." -ForegroundColor Yellow
$buildResult = cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Échec de la compilation" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Compilation réussie" -ForegroundColor Green

# Exécuter les tests unitaires
Write-Host "`n🧪 Exécution des tests unitaires..." -ForegroundColor Yellow
$testResult = cargo test
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Échec des tests unitaires" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Tests unitaires réussis" -ForegroundColor Green

Write-Host "`n📋 Instructions pour le test manuel:" -ForegroundColor Cyan
Write-Host "1. Ouvrez 3 terminaux" -ForegroundColor White
Write-Host "2. Terminal 1: cargo run server" -ForegroundColor White
Write-Host "3. Terminal 2: cargo run client Alice" -ForegroundColor White
Write-Host "4. Terminal 3: cargo run client Bob" -ForegroundColor White
Write-Host "5. Testez les interactions entre les clients" -ForegroundColor White

Write-Host "`n🎯 Le protocole est prêt à être testé!" -ForegroundColor Green

# Proposer de lancer le mode interactif
$response = Read-Host "`nVoulez-vous lancer le mode interactif? (y/n)"
if ($response -eq "y" -or $response -eq "Y") {
    Write-Host "`n🚀 Lancement du mode interactif..." -ForegroundColor Yellow
    cargo run
}
