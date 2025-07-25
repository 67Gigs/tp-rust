# Script de test pour le TP 9 - WebSocket

Write-Host "=== TP 9: Test WebSocket Serveur et Client ===" -ForegroundColor Green
Write-Host ""

# Fonction pour afficher les instructions
function Show-Instructions {
    Write-Host "Instructions de test:" -ForegroundColor Yellow
    Write-Host "1. Compile le projet avec: cargo build" -ForegroundColor Cyan
    Write-Host "2. Terminal 1: cargo run server" -ForegroundColor Cyan
    Write-Host "3. Terminal 2: cargo run client" -ForegroundColor Cyan
    Write-Host "4. Terminal 3: cargo run client (pour tester multi-client)" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Commandes client disponibles:" -ForegroundColor Yellow
    Write-Host "  - Tapez un message normal et appuyez sur Entrée" -ForegroundColor White
    Write-Host "  - /username <nom> : Changer votre nom" -ForegroundColor White
    Write-Host "  - /binary <texte> : Envoyer des données binaires" -ForegroundColor White
    Write-Host "  - /quit : Quitter le client" -ForegroundColor White
    Write-Host ""
}

# Fonction pour compiler le projet
function Build-Project {
    Write-Host "Compilation du projet..." -ForegroundColor Yellow
    
    try {
        $buildOutput = cargo build 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ Compilation réussie!" -ForegroundColor Green
            return $true
        } else {
            Write-Host "❌ Erreur de compilation:" -ForegroundColor Red
            Write-Host $buildOutput -ForegroundColor Red
            return $false
        }
    } catch {
        Write-Host "❌ Erreur lors de la compilation: $_" -ForegroundColor Red
        return $false
    }
}

# Fonction pour tester la compilation
function Test-Compilation {
    Write-Host "Test de compilation..." -ForegroundColor Yellow
    
    if (Build-Project) {
        Write-Host "Test de compilation: PASSÉ ✅" -ForegroundColor Green
    } else {
        Write-Host "Test de compilation: ÉCHOUÉ ❌" -ForegroundColor Red
        return $false
    }
    
    return $true
}

# Fonction pour afficher l'aide
function Show-Help {
    Write-Host "Usage: ./test-websocket.ps1 [option]" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Options disponibles:" -ForegroundColor Yellow
    Write-Host "  help        - Affiche cette aide" -ForegroundColor White
    Write-Host "  build       - Compile le projet" -ForegroundColor White
    Write-Host "  server      - Démarre le serveur WebSocket" -ForegroundColor White
    Write-Host "  client      - Démarre un client WebSocket" -ForegroundColor White
    Write-Host "  demo        - Démonstration interactive" -ForegroundColor White
    Write-Host "  test        - Tests automatisés" -ForegroundColor White
    Write-Host "  instructions - Affiche les instructions détaillées" -ForegroundColor White
    Write-Host ""
}

# Fonction pour démarrer le serveur
function Start-Server {
    Write-Host "Démarrage du serveur WebSocket..." -ForegroundColor Yellow
    Write-Host "Le serveur écoute sur 127.0.0.1:8080" -ForegroundColor Cyan
    Write-Host "Appuyez sur Ctrl+C pour arrêter le serveur" -ForegroundColor Cyan
    Write-Host ""
    
    try {
        cargo run server
    } catch {
        Write-Host "❌ Erreur lors du démarrage du serveur: $_" -ForegroundColor Red
    }
}

# Fonction pour démarrer un client
function Start-Client {
    Write-Host "Démarrage d'un client WebSocket..." -ForegroundColor Yellow
    Write-Host "Connexion à ws://127.0.0.1:8080" -ForegroundColor Cyan
    Write-Host ""
    
    try {
        cargo run client
    } catch {
        Write-Host "❌ Erreur lors du démarrage du client: $_" -ForegroundColor Red
    }
}

# Fonction pour la démonstration
function Start-Demo {
    Write-Host "=== DÉMONSTRATION WEBSOCKET ===" -ForegroundColor Green
    Write-Host ""
    Write-Host "Cette démonstration va montrer:" -ForegroundColor Yellow
    Write-Host "1. Compilation du projet" -ForegroundColor White
    Write-Host "2. Démarrage du serveur" -ForegroundColor White
    Write-Host "3. Instructions pour les clients" -ForegroundColor White
    Write-Host ""
    
    $continue = Read-Host "Appuyez sur Entrée pour continuer ou 'q' pour quitter"
    if ($continue -eq 'q') { return }
    
    # Test de compilation
    if (-not (Test-Compilation)) {
        Write-Host "Impossible de continuer sans compilation réussie." -ForegroundColor Red
        return
    }
    
    Write-Host ""
    Write-Host "Projet compilé avec succès!" -ForegroundColor Green
    Write-Host ""
    
    Show-Instructions
    
    Write-Host "Voulez-vous démarrer le serveur maintenant? (y/N)" -ForegroundColor Yellow
    $startServer = Read-Host
    
    if ($startServer -eq 'y' -or $startServer -eq 'Y') {
        Start-Server
    } else {
        Write-Host "Pour démarrer manuellement:" -ForegroundColor Cyan
        Write-Host "  Serveur: cargo run server" -ForegroundColor White
        Write-Host "  Client:  cargo run client" -ForegroundColor White
    }
}

# Fonction pour les tests automatisés
function Run-Tests {
    Write-Host "=== TESTS AUTOMATISÉS ===" -ForegroundColor Green
    Write-Host ""
    
    $allPassed = $true
    
    # Test 1: Compilation
    Write-Host "Test 1: Compilation" -ForegroundColor Yellow
    if (Test-Compilation) {
        Write-Host "✅ PASSÉ" -ForegroundColor Green
    } else {
        Write-Host "❌ ÉCHOUÉ" -ForegroundColor Red
        $allPassed = $false
    }
    Write-Host ""
    
    # Test 2: Vérification des fichiers sources
    Write-Host "Test 2: Vérification des fichiers sources" -ForegroundColor Yellow
    $requiredFiles = @("src/main.rs", "src/server.rs", "src/client.rs", "Cargo.toml")
    $filesMissing = $false
    
    foreach ($file in $requiredFiles) {
        if (Test-Path $file) {
            Write-Host "  ✅ $file existe" -ForegroundColor Green
        } else {
            Write-Host "  ❌ $file manquant" -ForegroundColor Red
            $filesMissing = $true
        }
    }
    
    if (-not $filesMissing) {
        Write-Host "✅ PASSÉ" -ForegroundColor Green
    } else {
        Write-Host "❌ ÉCHOUÉ" -ForegroundColor Red
        $allPassed = $false
    }
    Write-Host ""
    
    # Test 3: Vérification des dépendances
    Write-Host "Test 3: Vérification des dépendances" -ForegroundColor Yellow
    if (Test-Path "Cargo.toml") {
        $cargoContent = Get-Content "Cargo.toml" -Raw
        $requiredDeps = @("tokio", "tokio-tungstenite", "futures-util", "serde", "uuid")
        $depsMissing = $false
        
        foreach ($dep in $requiredDeps) {
            if ($cargoContent -match $dep) {
                Write-Host "  ✅ $dep trouvé" -ForegroundColor Green
            } else {
                Write-Host "  ❌ $dep manquant" -ForegroundColor Red
                $depsMissing = $true
            }
        }
        
        if (-not $depsMissing) {
            Write-Host "✅ PASSÉ" -ForegroundColor Green
        } else {
            Write-Host "❌ ÉCHOUÉ" -ForegroundColor Red
            $allPassed = $false
        }
    } else {
        Write-Host "❌ ÉCHOUÉ - Cargo.toml introuvable" -ForegroundColor Red
        $allPassed = $false
    }
    Write-Host ""
    
    # Résumé des tests
    Write-Host "=== RÉSUMÉ DES TESTS ===" -ForegroundColor Green
    if ($allPassed) {
        Write-Host "🎉 Tous les tests sont passés!" -ForegroundColor Green
        Write-Host "Le projet est prêt pour la démonstration." -ForegroundColor Cyan
    } else {
        Write-Host "⚠️  Certains tests ont échoué." -ForegroundColor Yellow
        Write-Host "Veuillez corriger les problèmes avant de continuer." -ForegroundColor Yellow
    }
    Write-Host ""
}

# Point d'entrée principal
$command = $args[0]

switch ($command) {
    "help" { Show-Help }
    "build" { Build-Project }
    "server" { Start-Server }
    "client" { Start-Client }
    "demo" { Start-Demo }
    "test" { Run-Tests }
    "instructions" { Show-Instructions }
    default {
        Write-Host "=== TP 9: Serveur et Client WebSocket ===" -ForegroundColor Green
        Write-Host ""
        Write-Host "Choisissez une option:" -ForegroundColor Yellow
        Write-Host "1. Démonstration interactive (demo)" -ForegroundColor White
        Write-Host "2. Tests automatisés (test)" -ForegroundColor White
        Write-Host "3. Compiler le projet (build)" -ForegroundColor White
        Write-Host "4. Démarrer le serveur (server)" -ForegroundColor White
        Write-Host "5. Démarrer un client (client)" -ForegroundColor White
        Write-Host "6. Afficher l'aide (help)" -ForegroundColor White
        Write-Host ""
        
        $choice = Read-Host "Votre choix (1-6)"
        
        switch ($choice) {
            "1" { Start-Demo }
            "2" { Run-Tests }
            "3" { Build-Project }
            "4" { Start-Server }
            "5" { Start-Client }
            "6" { Show-Help }
            default { 
                Write-Host "Option invalide. Utilisez './test-websocket.ps1 help' pour voir les options." -ForegroundColor Red
            }
        }
    }
}
