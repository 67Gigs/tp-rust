use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Code d'opération pour définir le type de message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OpCode {
    // Messages du client vers le serveur
    Connect,        // Demande de connexion
    Disconnect,     // Demande de déconnexion
    SendMessage,    // Envoi d'un message
    ListUsers,      // Demande la liste des utilisateurs connectés
    
    // Messages du serveur vers le client
    ConnectAck,     // Confirmation de connexion
    DisconnectAck,  // Confirmation de déconnexion
    MessageReceived,// Message reçu d'un autre utilisateur
    UserList,       // Liste des utilisateurs connectés
    Error,          // Message d'erreur
}

/// Structure principale du protocole
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Identifiant unique du message
    pub id: Uuid,
    /// Code d'opération
    pub op_code: OpCode,
    /// Horodatage du message
    pub timestamp: DateTime<Utc>,
    /// Expéditeur du message
    pub sender: Option<String>,
    /// Destinataire du message (optionnel pour les messages broadcast)
    pub recipient: Option<String>,
    /// Corps du message
    pub payload: MessagePayload,
}

/// Contenu du message selon le type d'opération
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessagePayload {
    Connect { username: String },
    Disconnect,
    Text { content: String },
    UserList { users: Vec<String> },
    Error { code: u16, message: String },
    Empty,
}

/// Codes d'erreur du protocole
pub mod error_codes {
    pub const INVALID_MESSAGE: u16 = 400;
    pub const UNAUTHORIZED: u16 = 401;
    pub const USER_EXISTS: u16 = 409;
    pub const INTERNAL_ERROR: u16 = 500;
}

impl Message {
    /// Crée un nouveau message
    pub fn new(op_code: OpCode, sender: Option<String>, payload: MessagePayload) -> Self {
        Self {
            id: Uuid::new_v4(),
            op_code,
            timestamp: Utc::now(),
            sender,
            recipient: None,
            payload,
        }
    }

    /// Crée un message avec un destinataire spécifique
    pub fn new_with_recipient(
        op_code: OpCode,
        sender: Option<String>,
        recipient: String,
        payload: MessagePayload,
    ) -> Self {
        let mut msg = Self::new(op_code, sender, payload);
        msg.recipient = Some(recipient);
        msg
    }

    /// Crée un message de connexion
    pub fn connect(username: String) -> Self {
        Self::new(
            OpCode::Connect,
            Some(username.clone()),
            MessagePayload::Connect { username },
        )
    }

    /// Crée un message de déconnexion
    pub fn disconnect(username: String) -> Self {
        Self::new(
            OpCode::Disconnect,
            Some(username),
            MessagePayload::Disconnect,
        )
    }

    /// Crée un message texte
    pub fn text_message(sender: String, content: String) -> Self {
        Self::new(
            OpCode::SendMessage,
            Some(sender),
            MessagePayload::Text { content },
        )
    }

    /// Crée un message d'erreur
    pub fn error(code: u16, message: String) -> Self {
        Self::new(
            OpCode::Error,
            None,
            MessagePayload::Error { code, message },
        )
    }

    /// Crée un accusé de réception de connexion
    pub fn connect_ack() -> Self {
        Self::new(OpCode::ConnectAck, None, MessagePayload::Empty)
    }

    /// Crée une liste d'utilisateurs
    pub fn user_list(users: Vec<String>) -> Self {
        Self::new(OpCode::UserList, None, MessagePayload::UserList { users })
    }

    /// Sérialise le message en JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Désérialise un message depuis JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Valide la cohérence du message
    pub fn validate(&self) -> Result<(), String> {
        match &self.op_code {
            OpCode::Connect => {
                if let MessagePayload::Connect { username } = &self.payload {
                    if username.is_empty() {
                        return Err("Username cannot be empty".to_string());
                    }
                } else {
                    return Err("Invalid payload for Connect message".to_string());
                }
            },
            OpCode::SendMessage => {
                if let MessagePayload::Text { content } = &self.payload {
                    if content.is_empty() {
                        return Err("Message content cannot be empty".to_string());
                    }
                } else {
                    return Err("Invalid payload for SendMessage".to_string());
                }
                if self.sender.is_none() {
                    return Err("SendMessage must have a sender".to_string());
                }
            },
            _ => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_serialization() {
        let msg = Message::connect("test_user".to_string());
        let json = msg.to_json().unwrap();
        let deserialized = Message::from_json(&json).unwrap();
        assert_eq!(msg.op_code, deserialized.op_code);
    }

    #[test]
    fn test_message_validation() {
        let valid_msg = Message::connect("user".to_string());
        assert!(valid_msg.validate().is_ok());

        let invalid_msg = Message::connect("".to_string());
        assert!(invalid_msg.validate().is_err());
    }
}
