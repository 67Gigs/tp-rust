use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, Mutex};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::{info, error};

// Structure pour représenter un message dans notre chat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub user_id: String,
    pub username: String,
    pub content: String,
    pub timestamp: u64,
    pub message_type: MessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    Binary,
    UserJoined,
    UserLeft,
    System,
}

// Structure pour gérer les informations d'un client connecté
#[derive(Debug, Clone)]
pub struct Client {
    pub id: String,
    pub username: String,
    pub addr: SocketAddr,
}

// État partagé du serveur
pub type Clients = Arc<Mutex<HashMap<String, Client>>>;
pub type MessageSender = broadcast::Sender<ChatMessage>;

pub struct WebSocketServer {
    clients: Clients,
    tx: MessageSender,
}

impl WebSocketServer {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1000);
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            tx,
        }
    }

    pub async fn run(&self, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(addr).await?;
        info!("Serveur WebSocket démarré sur {}", addr);

        while let Ok((stream, addr)) = listener.accept().await {
            let clients = self.clients.clone();
            let tx = self.tx.clone();
            
            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, addr, clients, tx).await {
                    error!("Erreur lors de la gestion de la connexion {}: {}", addr, e);
                }
            });
        }

        Ok(())
    }

    pub fn get_message_sender(&self) -> MessageSender {
        self.tx.clone()
    }
}

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    clients: Clients,
    tx: MessageSender,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Nouvelle connexion depuis {}", addr);

    // Effectuer le handshake WebSocket
    let ws_stream = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // Générer un ID unique pour ce client
    let client_id = Uuid::new_v4().to_string();
    let username = format!("User_{}", &client_id[..8]);

    // Créer un récepteur pour les messages broadcast
    let mut rx = tx.subscribe();

    // Envoyer un message de bienvenue
    let welcome_msg = ChatMessage {
        id: Uuid::new_v4().to_string(),
        user_id: "system".to_string(),
        username: "Système".to_string(),
        content: format!("Bienvenue ! Votre ID est: {}", client_id),
        timestamp: chrono::Utc::now().timestamp() as u64,
        message_type: MessageType::System,
    };

    let welcome_json = serde_json::to_string(&welcome_msg)?;
    ws_sender.send(Message::Text(welcome_json)).await?;

    // Tâche pour recevoir les messages du client
    let clients_for_receiver = clients.clone();
    let tx_for_receiver = tx.clone();
    let client_id_for_receiver = client_id.clone();
    let mut username_for_receiver = username.clone(); // Mutable pour pouvoir le mettre à jour

    let receiver_task = tokio::spawn(async move {
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    // Tenter de parser le message comme JSON
                    if let Ok(mut chat_msg) = serde_json::from_str::<ChatMessage>(&text) {
                        chat_msg.user_id = client_id_for_receiver.clone();
                        chat_msg.timestamp = chrono::Utc::now().timestamp() as u64;
                        chat_msg.id = Uuid::new_v4().to_string();

                        // Si c'est un changement de nom d'utilisateur
                        if chat_msg.content.starts_with("/username ") {
                            let new_username = chat_msg.content.replace("/username ", "");
                            if !new_username.is_empty() {
                                let old_username = username_for_receiver.clone();
                                // Mettre à jour le nom d'utilisateur localement ET dans la liste des clients
                                username_for_receiver = new_username.clone();
                                
                                let mut clients_guard = clients_for_receiver.lock().await;
                                if let Some(client) = clients_guard.get_mut(&client_id_for_receiver) {
                                    client.username = new_username.clone();
                                }
                                drop(clients_guard);

                                let system_msg = ChatMessage {
                                    id: Uuid::new_v4().to_string(),
                                    user_id: "system".to_string(),
                                    username: "Système".to_string(),
                                    content: format!("{} a changé son nom en {}", old_username, new_username),
                                    timestamp: chrono::Utc::now().timestamp() as u64,
                                    message_type: MessageType::System,
                                };

                                let _ = tx_for_receiver.send(system_msg);
                            }
                            continue;
                        }

                        // Pour les messages normaux, utiliser le nom d'utilisateur local
                        chat_msg.username = username_for_receiver.clone();
                        info!("Message reçu de {}: {}", chat_msg.username, chat_msg.content);
                        let _ = tx_for_receiver.send(chat_msg);
                    } else {
                        // Message texte simple
                        let chat_msg = ChatMessage {
                            id: Uuid::new_v4().to_string(),
                            user_id: client_id_for_receiver.clone(),
                            username: username_for_receiver.clone(),
                            content: text,
                            timestamp: chrono::Utc::now().timestamp() as u64,
                            message_type: MessageType::Text,
                        };

                        info!("Message texte de {}: {}", chat_msg.username, chat_msg.content);
                        let _ = tx_for_receiver.send(chat_msg);
                    }
                }
                Ok(Message::Binary(data)) => {
                    let chat_msg = ChatMessage {
                        id: Uuid::new_v4().to_string(),
                        user_id: client_id_for_receiver.clone(),
                        username: username_for_receiver.clone(),
                        content: format!("Données binaires ({} octets)", data.len()),
                        timestamp: chrono::Utc::now().timestamp() as u64,
                        message_type: MessageType::Binary,
                    };

                    info!("Données binaires reçues de {}: {} octets", chat_msg.username, data.len());
                    let _ = tx_for_receiver.send(chat_msg);
                }
                Ok(Message::Close(_)) => {
                    info!("Connexion fermée par le client {}", client_id_for_receiver);
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

    // Ajouter le client à la liste
    {
        let mut clients_guard = clients.lock().await;
        clients_guard.insert(client_id.clone(), Client {
            id: client_id.clone(),
            username: username.clone(),
            addr,
        });
    }

    // Annoncer l'arrivée du nouvel utilisateur
    let join_msg = ChatMessage {
        id: Uuid::new_v4().to_string(),
        user_id: "system".to_string(),
        username: "Système".to_string(),
        content: format!("{} a rejoint le chat", username),
        timestamp: chrono::Utc::now().timestamp() as u64,
        message_type: MessageType::UserJoined,
    };
    let _ = tx.send(join_msg);

    // Tâche pour diffuser les messages aux clients
    let client_id_for_sender = client_id.clone();
    let sender_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // Ne pas renvoyer le message à son expéditeur (sauf messages système)
            if msg.user_id == client_id_for_sender && msg.user_id != "system" {
                continue;
            }

            let json_msg = match serde_json::to_string(&msg) {
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
    });

    // Attendre qu'une des tâches se termine
    tokio::select! {
        _ = receiver_task => {},
        _ = sender_task => {},
    }

    // Nettoyer : retirer le client de la liste
    {
        let mut clients_guard = clients.lock().await;
        if let Some(client) = clients_guard.remove(&client_id) {
            // Annoncer le départ de l'utilisateur
            let leave_msg = ChatMessage {
                id: Uuid::new_v4().to_string(),
                user_id: "system".to_string(),
                username: "Système".to_string(),
                content: format!("{} a quitté le chat", client.username),
                timestamp: chrono::Utc::now().timestamp() as u64,
                message_type: MessageType::UserLeft,
            };
            let _ = tx.send(leave_msg);
        }
    }

    info!("Connexion {} fermée", addr);
    Ok(())
}
