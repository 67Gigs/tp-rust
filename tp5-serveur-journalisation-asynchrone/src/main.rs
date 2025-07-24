use std::fs::create_dir_all;
use std::path::Path;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use chrono::{DateTime, Local};

#[derive(Clone)]
struct LogServer {
    log_file: Arc<Mutex<tokio::fs::File>>,
    client_counter: Arc<AtomicU32>,
}

impl LogServer {
    /// Initialise le serveur de logs
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Créer le dossier logs s'il n'existe pas
        if !Path::new("logs").exists() {
            create_dir_all("logs")?;
        }

        // Ouvrir le fichier de log en mode append
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("logs/server.log")
            .await?;

        Ok(LogServer {
            log_file: Arc::new(Mutex::new(log_file)),
            client_counter: Arc::new(AtomicU32::new(1)),
        })
    }

    /// Écrit un message dans le fichier de log avec horodatage
    async fn write_log(&self, client_id: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp: DateTime<Local> = Local::now();
        let formatted_timestamp = timestamp.format("%Y-%m-%d %H:%M:%S%.3f");
        
        let log_entry = format!("[{}] Client-{}: {}\n", formatted_timestamp, client_id, message.trim());
        
        let mut file = self.log_file.lock().await;
        file.write_all(log_entry.as_bytes()).await?;
        file.flush().await?;
        
        // Afficher aussi dans la console pour le debug
        print!("{}", log_entry);
        
        Ok(())
    }

    /// Traite une connexion client
    async fn handle_client(&self, mut stream: TcpStream, addr: std::net::SocketAddr) {
        let client_id = self.client_counter.fetch_add(1, Ordering::SeqCst);
        let client_id_str = format!("{:08}", client_id);
        
        // Message de connexion
        if let Err(e) = self.write_log(&client_id_str, &format!("Connexion établie depuis {}", addr)).await {
            eprintln!("Erreur lors de l'écriture du log de connexion: {}", e);
            return;
        }

        // Envoyer un message de bienvenue au client
        if let Err(e) = stream.write_all(format!("Bienvenue! Votre ID client: {}\n", client_id_str).as_bytes()).await {
            eprintln!("Erreur lors de l'envoi du message de bienvenue: {}", e);
            return;
        }

        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        loop {
            line.clear();
            
            match reader.read_line(&mut line).await {
                Ok(0) => {
                    // Connexion fermée par le client
                    if let Err(e) = self.write_log(&client_id_str, "Connexion fermée par le client").await {
                        eprintln!("Erreur lors de l'écriture du log de déconnexion: {}", e);
                    }
                    break;
                }
                Ok(_) => {
                    // Message reçu
                    let message = line.trim();
                    
                    if message.is_empty() {
                        continue;
                    }

                    // Commandes spéciales
                    if message.eq_ignore_ascii_case("quit") || message.eq_ignore_ascii_case("exit") {
                        if let Err(e) = self.write_log(&client_id_str, "Déconnexion demandée par le client").await {
                            eprintln!("Erreur lors de l'écriture du log de déconnexion: {}", e);
                        }
                        
                        if let Err(e) = writer.write_all(b"Au revoir!\n").await {
                            eprintln!("Erreur lors de l'envoi du message d'au revoir: {}", e);
                        }
                        break;
                    }

                    // Enregistrer le message dans les logs
                    if let Err(e) = self.write_log(&client_id_str, message).await {
                        eprintln!("Erreur lors de l'écriture du message dans les logs: {}", e);
                        break;
                    }

                    // Confirmer la réception au client
                    let confirmation = format!("Message reçu et enregistré: {}\n", message);
                    if let Err(e) = writer.write_all(confirmation.as_bytes()).await {
                        eprintln!("Erreur lors de l'envoi de la confirmation: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Erreur lors de la lecture depuis le client {}: {}", client_id_str, e);
                    if let Err(log_err) = self.write_log(&client_id_str, &format!("Erreur de lecture: {}", e)).await {
                        eprintln!("Erreur lors de l'écriture du log d'erreur: {}", log_err);
                    }
                    break;
                }
            }
        }

        println!("Client {} déconnecté", client_id_str);
    }

    /// Démarre le serveur
    async fn start(&self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&addr).await?;
        
        println!("🚀 Serveur de journalisation démarré sur {}", addr);
        println!("📝 Les logs sont enregistrés dans logs/server.log");
        println!("ℹ️  Les clients peuvent se connecter avec: telnet 127.0.0.1 {}", port);
        println!("ℹ️  Tapez 'quit' ou 'exit' pour fermer une connexion client");
        println!("---");

        // Message de démarrage dans les logs
        self.write_log("SYSTEM", &format!("Serveur démarré sur {}", addr)).await?;

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    let server_clone = self.clone();
                    
                    // Traiter chaque client dans une tâche séparée
                    tokio::spawn(async move {
                        server_clone.handle_client(stream, addr).await;
                    });
                }
                Err(e) => {
                    eprintln!("Erreur lors de l'acceptation de connexion: {}", e);
                    self.write_log("SYSTEM", &format!("Erreur d'acceptation: {}", e)).await?;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Port par défaut
    let port = 8080;
    
    // Initialiser le serveur
    let server = LogServer::new().await?;
    
    // Gérer l'arrêt propre avec Ctrl+C
    let server_clone = server.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Erreur lors de l'écoute du signal Ctrl+C");
        println!("\n🛑 Arrêt du serveur demandé...");
        
        if let Err(e) = server_clone.write_log("SYSTEM", "Arrêt du serveur").await {
            eprintln!("Erreur lors de l'écriture du log d'arrêt: {}", e);
        }
        
        std::process::exit(0);
    });
    
    // Démarrer le serveur
    server.start(port).await?;
    
    Ok(())
}