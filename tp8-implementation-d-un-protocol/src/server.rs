use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, broadcast};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use uuid::Uuid;

use crate::protocol::{Message, OpCode, MessagePayload, error_codes};

/// État d'un client connecté
#[derive(Debug, Clone)]
pub struct ClientSession {
    pub id: Uuid,
    pub username: String,
    pub connected_at: chrono::DateTime<chrono::Utc>,
}

/// État global du serveur
pub struct ServerState {
    /// Sessions actives (ID de session -> information client)
    pub sessions: HashMap<Uuid, ClientSession>,
    /// Mapping nom d'utilisateur -> ID de session
    pub username_to_session: HashMap<String, Uuid>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            username_to_session: HashMap::new(),
        }
    }

    /// Ajoute une nouvelle session
    pub fn add_session(&mut self, username: String) -> Result<Uuid, String> {
        if self.username_to_session.contains_key(&username) {
            return Err(format!("User '{}' already connected", username));
        }

        let session_id = Uuid::new_v4();
        let session = ClientSession {
            id: session_id,
            username: username.clone(),
            connected_at: chrono::Utc::now(),
        };

        self.sessions.insert(session_id, session);
        self.username_to_session.insert(username, session_id);
        
        Ok(session_id)
    }

    /// Supprime une session
    pub fn remove_session(&mut self, session_id: &Uuid) -> Option<String> {
        if let Some(session) = self.sessions.remove(session_id) {
            self.username_to_session.remove(&session.username);
            Some(session.username)
        } else {
            None
        }
    }

    /// Obtient la liste des utilisateurs connectés
    pub fn get_connected_users(&self) -> Vec<String> {
        self.sessions.values().map(|s| s.username.clone()).collect()
    }
}

/// Serveur de messagerie
pub struct MessageServer {
    state: Arc<Mutex<ServerState>>,
    broadcast_tx: broadcast::Sender<Message>,
}

impl MessageServer {
    /// Crée un nouveau serveur
    pub fn new() -> Self {
        let (broadcast_tx, _) = broadcast::channel(1000);
        
        Self {
            state: Arc::new(Mutex::new(ServerState::new())),
            broadcast_tx,
        }
    }

    /// Lance le serveur sur l'adresse spécifiée
    pub async fn start(&self, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(addr).await?;
        println!("🚀 Serveur de messagerie démarré sur {}", addr);
        println!("📋 Protocole de messagerie personnalisé v1.0");
        println!("🔄 En attente de connexions...\n");

        loop {
            let (stream, addr) = listener.accept().await?;
            println!("🔗 Nouvelle connexion depuis: {}", addr);

            let state = Arc::clone(&self.state);
            let broadcast_tx = self.broadcast_tx.clone();
            let broadcast_rx = self.broadcast_tx.subscribe();

            tokio::spawn(async move {
                if let Err(e) = Self::handle_client(stream, state, broadcast_tx, broadcast_rx).await {
                    eprintln!("❌ Erreur avec le client {}: {}", addr, e);
                }
            });
        }
    }

    /// Gère un client connecté
    async fn handle_client(
        mut stream: TcpStream,
        state: Arc<Mutex<ServerState>>,
        broadcast_tx: broadcast::Sender<Message>,
        mut broadcast_rx: broadcast::Receiver<Message>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (reader, mut writer) = stream.split();
        let mut buf_reader = BufReader::new(reader);
        let mut session_id: Option<Uuid> = None;
        let mut username: Option<String> = None;

        let mut line = String::new();
        loop {
            tokio::select! {
                // Lecture des messages du client
                result = buf_reader.read_line(&mut line) => {
                    match result {
                        Ok(0) => break, // Connexion fermée
                        Ok(_) => {
                            let trimmed = line.trim();
                            if !trimmed.is_empty() {
                                match Message::from_json(trimmed) {
                                    Ok(msg) => {
                                        if let Err(e) = msg.validate() {
                                            let error_msg = Message::error(
                                                error_codes::INVALID_MESSAGE,
                                                format!("Invalid message: {}", e)
                                            );
                                            Self::send_message_to_writer(&mut writer, &error_msg).await?;
                                        } else {
                                            Self::process_message(
                                                msg,
                                                &mut session_id,
                                                &mut username,
                                                &state,
                                                &broadcast_tx,
                                                &mut writer
                                            ).await?;
                                        }
                                    }
                                    Err(e) => {
                                        let error_msg = Message::error(
                                            error_codes::INVALID_MESSAGE,
                                            format!("JSON parse error: {}", e)
                                        );
                                        Self::send_message_to_writer(&mut writer, &error_msg).await?;
                                    }
                                }
                            }
                            line.clear();
                        }
                        Err(e) => {
                            eprintln!("Erreur de lecture: {}", e);
                            break;
                        }
                    }
                }
                
                // Messages broadcast
                result = broadcast_rx.recv() => {
                    match result {
                        Ok(msg) => {
                            // Ne pas renvoyer les messages à l'expéditeur
                            if let Some(ref current_username) = username {
                                if msg.sender.as_ref() == Some(current_username) {
                                    continue;
                                }
                            }

                            let json = format!("{}\n", msg.to_json().unwrap_or_default());
                            if let Err(_) = writer.write_all(json.as_bytes()).await {
                                break;
                            }
                        }
                        Err(_) => break, // Canal fermé
                    }
                }
            }
        }

        // Nettoyage lors de la déconnexion
        if let Some(id) = session_id {
            let mut state_guard = state.lock().await;
            if let Some(disconnected_user) = state_guard.remove_session(&id) {
                println!("👋 Utilisateur déconnecté: {}", disconnected_user);
                
                // Notifier les autres utilisateurs
                let disconnect_msg = Message::new(
                    OpCode::MessageReceived,
                    Some("Serveur".to_string()),
                    MessagePayload::Text {
                        content: format!("{} a quitté le chat", disconnected_user)
                    }
                );
                let _ = broadcast_tx.send(disconnect_msg);
            }
        }

        Ok(())
    }

    /// Traite un message reçu du client
    async fn process_message(
        msg: Message,
        session_id: &mut Option<Uuid>,
        username: &mut Option<String>,
        state: &Arc<Mutex<ServerState>>,
        broadcast_tx: &broadcast::Sender<Message>,
        writer: &mut tokio::net::tcp::WriteHalf<'_>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match msg.op_code {
            OpCode::Connect => {
                if let MessagePayload::Connect { username: new_username } = msg.payload {
                    let mut state_guard = state.lock().await;
                    match state_guard.add_session(new_username.clone()) {
                        Ok(new_session_id) => {
                            *session_id = Some(new_session_id);
                            *username = Some(new_username.clone());
                            
                            println!("✅ Nouvel utilisateur connecté: {}", new_username);
                            
                            // Confirmation de connexion
                            let ack = Message::connect_ack();
                            Self::send_message_to_writer(writer, &ack).await?;
                            
                            // Envoie la liste des utilisateurs
                            let users = state_guard.get_connected_users();
                            let user_list = Message::user_list(users);
                            Self::send_message_to_writer(writer, &user_list).await?;
                            
                            // Notifie les autres utilisateurs
                            let join_msg = Message::new(
                                OpCode::MessageReceived,
                                Some("Serveur".to_string()),
                                MessagePayload::Text {
                                    content: format!("{} a rejoint le chat", new_username)
                                }
                            );
                            let _ = broadcast_tx.send(join_msg);
                        }
                        Err(e) => {
                            let error_msg = Message::error(error_codes::USER_EXISTS, e);
                            Self::send_message_to_writer(writer, &error_msg).await?;
                        }
                    }
                }
            }
            
            OpCode::Disconnect => {
                if let Some(id) = session_id {
                    let mut state_guard = state.lock().await;
                    if let Some(disconnected_user) = state_guard.remove_session(id) {
                        let ack = Message::new(
                            OpCode::DisconnectAck,
                            None,
                            MessagePayload::Empty
                        );
                        Self::send_message_to_writer(writer, &ack).await?;
                        
                        // Notifier les autres
                        let disconnect_msg = Message::new(
                            OpCode::MessageReceived,
                            Some("Serveur".to_string()),
                            MessagePayload::Text {
                                content: format!("{} a quitté le chat", disconnected_user)
                            }
                        );
                        let _ = broadcast_tx.send(disconnect_msg);
                    }
                }
            }
            
            OpCode::SendMessage => {
                if session_id.is_some() {
                    // Diffuser le message à tous les clients connectés
                    let broadcast_msg = Message::new(
                        OpCode::MessageReceived,
                        msg.sender,
                        msg.payload
                    );
                    let _ = broadcast_tx.send(broadcast_msg);
                } else {
                    let error_msg = Message::error(
                        error_codes::UNAUTHORIZED,
                        "Not connected".to_string()
                    );
                    Self::send_message_to_writer(writer, &error_msg).await?;
                }
            }
            
            OpCode::ListUsers => {
                if session_id.is_some() {
                    let state_guard = state.lock().await;
                    let users = state_guard.get_connected_users();
                    let user_list = Message::user_list(users);
                    Self::send_message_to_writer(writer, &user_list).await?;
                } else {
                    let error_msg = Message::error(
                        error_codes::UNAUTHORIZED,
                        "Not connected".to_string()
                    );
                    Self::send_message_to_writer(writer, &error_msg).await?;
                }
            }
            
            _ => {
                let error_msg = Message::error(
                    error_codes::INVALID_MESSAGE,
                    "Unsupported operation".to_string()
                );
                Self::send_message_to_writer(writer, &error_msg).await?;
            }
        }
        
        Ok(())
    }

    /// Envoie un message au client
    async fn send_message_to_writer(
        writer: &mut tokio::net::tcp::WriteHalf<'_>,
        msg: &Message,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json = format!("{}\n", msg.to_json()?);
        writer.write_all(json.as_bytes()).await?;
        Ok(())
    }
}
