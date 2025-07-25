use std::io::{self, Write};
use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc;

use crate::protocol::{Message, OpCode, MessagePayload};

/// Client de messagerie
pub struct MessageClient {
    username: String,
    connected: bool,
}

impl MessageClient {
    /// Crée un nouveau client
    pub fn new(username: String) -> Self {
        Self {
            username,
            connected: false,
        }
    }

    /// Se connecte au serveur et lance l'interface utilisateur
    pub async fn connect_and_run(&mut self, server_addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔗 Connexion au serveur {}...", server_addr);
        
        let mut stream = TcpStream::connect(server_addr).await?;
        let (reader, mut writer) = stream.split();
        let mut buf_reader = BufReader::new(reader);

        println!("✅ Connecté au serveur!");
        println!("📝 Protocole de messagerie personnalisé v1.0");
        println!("ℹ️  Tapez /help pour voir les commandes disponibles\n");

        // Canal pour les messages utilisateur
        let (tx, mut rx) = mpsc::channel::<String>(100);

        // Task pour lire l'entrée utilisateur
        let tx_clone = tx.clone();
        let input_task = tokio::spawn(async move {
            let stdin = io::stdin();
            loop {
                print!("> ");
                io::stdout().flush().unwrap();
                
                let mut input = String::new();
                match stdin.read_line(&mut input) {
                    Ok(_) => {
                        let trimmed = input.trim().to_string();
                        if !trimmed.is_empty() {
                            if tx_clone.send(trimmed).await.is_err() {
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Erreur de lecture: {}", e);
                        break;
                    }
                }
            }
        });

        // Envoyer le message de connexion
        let connect_msg = Message::connect(self.username.clone());
        self.send_message_to_writer(&mut writer, &connect_msg).await?;

        let mut line = String::new();
        loop {
            tokio::select! {
                // Messages du serveur
                result = buf_reader.read_line(&mut line) => {
                    match result {
                        Ok(0) => {
                            println!("\n❌ Connexion fermée par le serveur");
                            break;
                        }
                        Ok(_) => {
                            let trimmed = line.trim();
                            if !trimmed.is_empty() {
                                self.handle_server_message(trimmed).await;
                            }
                            line.clear();
                        }
                        Err(e) => {
                            eprintln!("Erreur de lecture du serveur: {}", e);
                            break;
                        }
                    }
                }
                
                // Messages de l'utilisateur
                user_input = rx.recv() => {
                    if let Some(input) = user_input {
                        if input == "/quit" || input == "/exit" {
                            if self.connected {
                                let disconnect_msg = Message::disconnect(self.username.clone());
                                let _ = self.send_message_to_writer(&mut writer, &disconnect_msg).await;
                            }
                            break;
                        }
                        
                        if let Some(msg) = self.process_user_input(&input).await {
                            if let Err(e) = self.send_message_to_writer(&mut writer, &msg).await {
                                eprintln!("Erreur d'envoi: {}", e);
                                break;
                            }
                        }
                    }
                }
            }
        }

        input_task.abort();
        println!("👋 Déconnexion...");
        Ok(())
    }

    /// Traite les messages reçus du serveur
    async fn handle_server_message(&mut self, json: &str) {
        match Message::from_json(json) {
            Ok(msg) => {
                match msg.op_code {
                    OpCode::ConnectAck => {
                        self.connected = true;
                        println!("✅ Connexion établie avec succès!");
                        println!("💬 Vous pouvez maintenant envoyer des messages");
                    }
                    
                    OpCode::DisconnectAck => {
                        self.connected = false;
                        println!("✅ Déconnexion confirmée");
                    }
                    
                    OpCode::MessageReceived => {
                        if let MessagePayload::Text { content } = msg.payload {
                            let timestamp = msg.timestamp.format("%H:%M:%S");
                            match msg.sender {
                                Some(sender) => {
                                    println!("[{}] {}: {}", timestamp, sender, content);
                                }
                                None => {
                                    println!("[{}] {}", timestamp, content);
                                }
                            }
                        }
                    }
                    
                    OpCode::UserList => {
                        if let MessagePayload::UserList { users } = msg.payload {
                            println!("👥 Utilisateurs connectés:");
                            for user in users {
                                if user == self.username {
                                    println!("  • {} (vous)", user);
                                } else {
                                    println!("  • {}", user);
                                }
                            }
                        }
                    }
                    
                    OpCode::Error => {
                        if let MessagePayload::Error { code, message } = msg.payload {
                            println!("❌ Erreur {}: {}", code, message);
                        }
                    }
                    
                    _ => {
                        println!("⚠️  Message non géré: {:?}", msg.op_code);
                    }
                }
            }
            Err(e) => {
                eprintln!("Erreur de parsing du message: {}", e);
            }
        }
    }

    /// Traite l'entrée utilisateur et crée les messages appropriés
    async fn process_user_input(&self, input: &str) -> Option<Message> {
        if input.starts_with('/') {
            // Commandes spéciales
            match input {
                "/help" => {
                    self.show_help();
                    None
                }
                "/users" | "/list" => {
                    if self.connected {
                        Some(Message::new(
                            OpCode::ListUsers,
                            Some(self.username.clone()),
                            MessagePayload::Empty
                        ))
                    } else {
                        println!("❌ Vous devez être connecté pour utiliser cette commande");
                        None
                    }
                }
                "/disconnect" => {
                    if self.connected {
                        Some(Message::disconnect(self.username.clone()))
                    } else {
                        println!("❌ Vous n'êtes pas connecté");
                        None
                    }
                }
                _ => {
                    println!("❌ Commande inconnue. Tapez /help pour voir les commandes disponibles");
                    None
                }
            }
        } else {
            // Message texte normal
            if self.connected {
                if input.trim().is_empty() {
                    println!("❌ Le message ne peut pas être vide");
                    None
                } else {
                    Some(Message::text_message(
                        self.username.clone(),
                        input.to_string()
                    ))
                }
            } else {
                println!("❌ Vous devez être connecté pour envoyer des messages");
                None
            }
        }
    }

    /// Affiche l'aide
    fn show_help(&self) {
        println!("\n📋 Commandes disponibles:");
        println!("  /help          - Affiche cette aide");
        println!("  /users, /list  - Liste les utilisateurs connectés");
        println!("  /disconnect    - Se déconnecte du serveur");
        println!("  /quit, /exit   - Quitte l'application");
        println!("  <message>      - Envoie un message à tous les utilisateurs");
        println!();
    }

    /// Envoie un message au serveur
    async fn send_message_to_writer(
        &self,
        writer: &mut tokio::net::tcp::WriteHalf<'_>,
        msg: &Message,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json = format!("{}\n", msg.to_json()?);
        writer.write_all(json.as_bytes()).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = MessageClient::new("test_user".to_string());
        assert_eq!(client.username, "test_user");
        assert!(!client.connected);
    }
}
