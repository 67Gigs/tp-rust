mod protocol;
mod server;
mod client;

#[cfg(test)]
mod tests;

use std::env;
use std::io::{self, Write};
use server::MessageServer;
use client::MessageClient;

const DEFAULT_SERVER_ADDR: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 TP 8: Implémentation d'un Protocole Personnalisé");
    println!("═══════════════════════════════════════════════════");
    println!("📡 Protocole de messagerie TCP personnalisé");
    println!("🔧 Développé en Rust avec sérialisation JSON\n");

    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        1 => {
            // Mode interactif
            run_interactive_mode().await
        }
        2 => {
            match args[1].as_str() {
                "server" => {
                    run_server(DEFAULT_SERVER_ADDR).await
                }
                "client" => {
                    run_client_interactive().await
                }
                _ => {
                    print_usage();
                    Ok(())
                }
            }
        }
        3 => {
            match args[1].as_str() {
                "server" => {
                    run_server(&args[2]).await
                }
                "client" => {
                    run_client(&args[2], DEFAULT_SERVER_ADDR).await
                }
                _ => {
                    print_usage();
                    Ok(())
                }
            }
        }
        4 => {
            match args[1].as_str() {
                "client" => {
                    run_client(&args[2], &args[3]).await
                }
                _ => {
                    print_usage();
                    Ok(())
                }
            }
        }
        _ => {
            print_usage();
            Ok(())
        }
    }
}

async fn run_interactive_mode() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("Choisissez un mode:");
        println!("1. Serveur");
        println!("2. Client");
        println!("3. Quitter");
        print!("\nVotre choix (1-3): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => {
                println!("\n🖥️  Mode Serveur");
                print!("Adresse d'écoute (défaut: {}): ", DEFAULT_SERVER_ADDR);
                io::stdout().flush()?;
                
                let mut addr_input = String::new();
                io::stdin().read_line(&mut addr_input)?;
                let addr = if addr_input.trim().is_empty() {
                    DEFAULT_SERVER_ADDR
                } else {
                    addr_input.trim()
                };
                
                return run_server(addr).await;
            }
            "2" => {
                println!("\n💻 Mode Client");
                return run_client_interactive().await;
            }
            "3" => {
                println!("👋 Au revoir!");
                return Ok(());
            }
            _ => {
                println!("❌ Choix invalide, veuillez réessayer.\n");
            }
        }
    }
}

async fn run_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let server = MessageServer::new();
    
    println!("🔧 Configuration du serveur:");
    println!("  • Adresse: {}", addr);
    println!("  • Protocole: TCP");
    println!("  • Format: JSON");
    println!("  • Gestion des états: Sessions multiples");
    println!();
    
    server.start(addr).await
}

async fn run_client_interactive() -> Result<(), Box<dyn std::error::Error>> {
    print!("Nom d'utilisateur: ");
    io::stdout().flush()?;
    
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim().to_string();
    
    if username.is_empty() {
        println!("❌ Le nom d'utilisateur ne peut pas être vide");
        return Ok(());
    }
    
    print!("Adresse du serveur (défaut: {}): ", DEFAULT_SERVER_ADDR);
    io::stdout().flush()?;
    
    let mut server_addr = String::new();
    io::stdin().read_line(&mut server_addr)?;
    let server_addr = if server_addr.trim().is_empty() {
        DEFAULT_SERVER_ADDR
    } else {
        server_addr.trim()
    };
    
    run_client(&username, server_addr).await
}

async fn run_client(username: &str, server_addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔧 Configuration du client:");
    println!("  • Utilisateur: {}", username);
    println!("  • Serveur: {}", server_addr);
    println!("  • Protocole: TCP");
    println!("  • Format: JSON");
    println!();
    
    let mut client = MessageClient::new(username.to_string());
    client.connect_and_run(server_addr).await
}

fn print_usage() {
    println!("Usage:");
    println!("  {} [MODE] [OPTIONS]", env::args().next().unwrap());
    println!();
    println!("Modes:");
    println!("  server [ADDRESS]              - Lance le serveur (défaut: {})", DEFAULT_SERVER_ADDR);
    println!("  client [USERNAME] [ADDRESS]   - Lance le client");
    println!("  (sans arguments)              - Mode interactif");
    println!();
    println!("Exemples:");
    println!("  {} server                     - Serveur sur {}", env::args().next().unwrap(), DEFAULT_SERVER_ADDR);
    println!("  {} server 0.0.0.0:9999       - Serveur sur port 9999", env::args().next().unwrap());
    println!("  {} client Alice               - Client 'Alice' vers {}", env::args().next().unwrap(), DEFAULT_SERVER_ADDR);
    println!("  {} client Bob 192.168.1.100:8080 - Client 'Bob' vers serveur distant", env::args().next().unwrap());
}
