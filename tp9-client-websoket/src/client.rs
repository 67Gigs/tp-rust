use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use std::io::{self, Write};
use tracing::{info, error};
use uuid::Uuid;

use crate::server::{ChatMessage, MessageType};

pub struct WebSocketClient {
    server_url: String,
}

impl WebSocketClient {
    pub fn new(server_url: String) -> Self {
        Self { server_url }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Connexion au serveur WebSocket: {}", self.server_url);

        // Se connecter au serveur
        let (ws_stream, _) = connect_async(&self.server_url).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        info!("Connecté avec succès au serveur !");
        println!("\n=== CLIENT WEBSOCKET ===");
        println!("Connecté au serveur: {}", self.server_url);
        println!("Commandes disponibles:");
        println!("  - Tapez votre message et appuyez sur Entrée");
        println!("  - /username <nom> - Changer votre nom d'utilisateur");
        println!("  - /binary <texte> - Envoyer des données binaires");
        println!("  - /quit - Quitter le client");
        println!("========================\n");

        // Tâche pour recevoir les messages du serveur
        let receiver_task = tokio::spawn(async move {
            while let Some(msg) = ws_receiver.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        // Tenter de parser comme ChatMessage
                        if let Ok(chat_msg) = serde_json::from_str::<ChatMessage>(&text) {
                            print_chat_message(&chat_msg);
                        } else {
                            println!("Message reçu: {}", text);
                        }
                        // Réafficher le prompt
                        print!("> ");
                        io::stdout().flush().unwrap();
                    }
                    Ok(Message::Binary(data)) => {
                        println!("Données binaires reçues: {} octets", data.len());
                        print!("> ");
                        io::stdout().flush().unwrap();
                    }
                    Ok(Message::Close(_)) => {
                        println!("Connexion fermée par le serveur");
                        break;
                    }
                    Err(e) => {
                        error!("Erreur WebSocket: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });

        // Tâche pour lire l'entrée utilisateur et envoyer des messages
        let sender_task = tokio::spawn(async move {
            let stdin = tokio::io::stdin();
            let mut reader = tokio::io::BufReader::new(stdin);
            let mut line = String::new();

            loop {
                print!("> ");
                io::stdout().flush().unwrap();

                line.clear();
                match tokio::io::AsyncBufReadExt::read_line(&mut reader, &mut line).await {
                    Ok(0) => break, // EOF
                    Ok(_) => {
                        let input = line.trim().to_string();
                        
                        if input.is_empty() {
                            continue;
                        }

                        // Traiter les commandes spéciales
                        if input == "/quit" {
                            println!("Déconnexion...");
                            break;
                        } else if input.starts_with("/binary ") {
                            // Envoyer des données binaires
                            let content = input.replace("/binary ", "");
                            if let Err(e) = ws_sender.send(Message::Binary(content.into_bytes())).await {
                                error!("Erreur lors de l'envoi de données binaires: {}", e);
                                break;
                            }
                        } else if input.starts_with("/username ") {
                            // Changer le nom d'utilisateur
                            let chat_msg = ChatMessage {
                                id: Uuid::new_v4().to_string(),
                                user_id: String::new(), // Sera rempli par le serveur
                                username: String::new(), // Sera rempli par le serveur
                                content: input,
                                timestamp: 0, // Sera rempli par le serveur
                                message_type: MessageType::System,
                            };

                            let json_msg = match serde_json::to_string(&chat_msg) {
                                Ok(json) => json,
                                Err(e) => {
                                    error!("Erreur de sérialisation: {}", e);
                                    continue;
                                }
                            };

                            if let Err(e) = ws_sender.send(Message::Text(json_msg)).await {
                                error!("Erreur lors de l'envoi du message: {}", e);
                                break;
                            }
                        } else {
                            // Message texte normal
                            let chat_msg = ChatMessage {
                                id: Uuid::new_v4().to_string(),
                                user_id: String::new(), // Sera rempli par le serveur
                                username: String::new(), // Sera rempli par le serveur
                                content: input,
                                timestamp: 0, // Sera rempli par le serveur
                                message_type: MessageType::Text,
                            };

                            let json_msg = match serde_json::to_string(&chat_msg) {
                                Ok(json) => json,
                                Err(e) => {
                                    error!("Erreur de sérialisation: {}", e);
                                    continue;
                                }
                            };

                            if let Err(e) = ws_sender.send(Message::Text(json_msg)).await {
                                error!("Erreur lors de l'envoi du message: {}", e);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        error!("Erreur lors de la lecture de l'entrée: {}", e);
                        break;
                    }
                }
            }

            // Fermer la connexion proprement
            let _ = ws_sender.close().await;
        });

        // Attendre qu'une des tâches se termine
        tokio::select! {
            _ = receiver_task => {},
            _ = sender_task => {},
        }

        Ok(())
    }
}

fn print_chat_message(msg: &ChatMessage) {
    let timestamp = chrono::DateTime::from_timestamp(msg.timestamp as i64, 0)
        .unwrap_or_else(|| chrono::Utc::now())
        .format("%H:%M:%S");

    match msg.message_type {
        MessageType::Text => {
            println!("[{}] {}: {}", timestamp, msg.username, msg.content);
        }
        MessageType::Binary => {
            println!("[{}] {} a envoyé: {}", timestamp, msg.username, msg.content);
        }
        MessageType::System => {
            println!("[{}] 🔔 {}", timestamp, msg.content);
        }
        MessageType::UserJoined => {
            println!("[{}] ➡️  {}", timestamp, msg.content);
        }
        MessageType::UserLeft => {
            println!("[{}] ⬅️  {}", timestamp, msg.content);
        }
    }
}

// Fonction utilitaire pour créer un client simple
pub async fn create_simple_client() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CLIENT WEBSOCKET SIMPLE ===");
    println!("Connexion au serveur local...");

    let client = WebSocketClient::new("ws://127.0.0.1:8080".to_string());
    client.run().await?;

    Ok(())
}
