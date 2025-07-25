mod server;
mod client;

use std::env;
use tracing::{info, Level};
use tracing_subscriber;

use server::WebSocketServer;
use client::{WebSocketClient, create_simple_client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialiser le système de logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("server") => {
            info!("Démarrage du serveur WebSocket...");
            let server = WebSocketServer::new();
            server.run("127.0.0.1:8080").await?;
        }
        Some("client") => {
            info!("Démarrage du client WebSocket...");
            let server_url = args.get(2)
                .unwrap_or(&"ws://127.0.0.1:8080".to_string())
                .clone();
            
            let client = WebSocketClient::new(server_url);
            client.run().await?;
        }
        Some("simple-client") => {
            create_simple_client().await?;
        }
        Some("demo") => {
            info!("Démarrage de la démonstration complète...");
            run_demo().await?;
        }
        _ => {
            println!("=== TP 9: Serveur et Client WebSocket ===");
            println!();
            println!("Usage:");
            println!("  {} server                    - Démarre le serveur WebSocket", args[0]);
            println!("  {} client [url]              - Démarre un client WebSocket", args[0]);
            println!("  {} simple-client             - Client simple vers localhost", args[0]);
            println!("  {} demo                      - Démonstration complète", args[0]);
            println!();
            println!("Exemples:");
            println!("  {} server", args[0]);
            println!("  {} client ws://127.0.0.1:8080", args[0]);
            println!("  {} simple-client", args[0]);
            println!();
            println!("Concepts illustrés:");
            println!("  • Protocole WebSocket et handshake");
            println!("  • Communication bidirectionnelle persistante");
            println!("  • Gestion de multiples connexions avec Tokio");
            println!("  • Messages texte et binaires");
            println!("  • Broadcasting de messages");
            println!("  • Gestion d'état partagé avec Arc<Mutex>");
        }
    }

    Ok(())
}

async fn run_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== DÉMONSTRATION WEBSOCKET ===");
    println!();
    println!("Cette démonstration va :");
    println!("1. Démarrer un serveur WebSocket");
    println!("2. Simuler plusieurs clients qui se connectent");
    println!("3. Échanger des messages entre les clients");
    println!("4. Démontrer les fonctionnalités du protocole");
    println!();
    println!("Appuyez sur Entrée pour continuer...");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    // Démarrer le serveur dans une tâche séparée
    let server = WebSocketServer::new();
    let server_handle = {
        let server = server;
        tokio::spawn(async move {
            if let Err(e) = server.run("127.0.0.1:8080").await {
                eprintln!("Erreur serveur: {}", e);
            }
        })
    };

    // Attendre un peu que le serveur démarre
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    println!("Serveur démarré ! Vous pouvez maintenant :");
    println!("1. Ouvrir d'autres terminaux et lancer: cargo run client");
    println!("2. Ou utiliser un client WebSocket de votre choix");
    println!("3. Connectez-vous à ws://127.0.0.1:8080");
    println!();
    println!("Le serveur reste actif. Appuyez sur Ctrl+C pour arrêter.");

    // Garder le serveur en vie
    server_handle.await?;

    Ok(())
}
