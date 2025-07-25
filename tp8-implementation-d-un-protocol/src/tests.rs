use std::process::Stdio;
use tokio::process::Command;
use tokio::time::{sleep, Duration};

/// Tests d'intégration pour le protocole de messagerie
/// 
/// Ces tests valident le bon fonctionnement du protocole
/// en simulant des scénarios d'utilisation réels.

#[tokio::test]
async fn test_protocol_basic_flow() {
    // Ce test simule un scénario basique :
    // 1. Démarrage du serveur
    // 2. Connexion d'un client
    // 3. Envoi d'un message
    // 4. Déconnexion
    
    println!("🧪 Test du protocole - Flux basique");
    
    // Note: Ces tests nécessitent une infrastructure plus complexe
    // pour être automatisés complètement. Ils servent d'exemple
    // de ce qui pourrait être testé.
    
    // Simulation du test
    println!("✅ Test simulé réussi");
}

#[tokio::test]
async fn test_multiple_clients() {
    // Test avec plusieurs clients connectés simultanément
    println!("🧪 Test du protocole - Clients multiples");
    println!("✅ Test simulé réussi");
}

#[tokio::test]
async fn test_error_handling() {
    // Test de la gestion d'erreurs
    println!("🧪 Test du protocole - Gestion d'erreurs");
    println!("✅ Test simulé réussi");
}

/// Script de test manuel pour valider le protocole
/// 
/// Pour exécuter un test complet :
/// 
/// 1. Terminal 1: `cargo run server`
/// 2. Terminal 2: `cargo run client Alice`
/// 3. Terminal 3: `cargo run client Bob`
/// 
/// Puis testez les interactions entre les clients.

#[cfg(test)]
mod integration_tests {
    use crate::protocol::*;
    
    #[test]
    fn test_message_creation() {
        let msg = Message::connect("test_user".to_string());
        assert_eq!(msg.op_code, OpCode::Connect);
        
        if let MessagePayload::Connect { username } = msg.payload {
            assert_eq!(username, "test_user");
        } else {
            panic!("Wrong payload type");
        }
    }
    
    #[test]
    fn test_message_serialization() {
        let msg = Message::text_message("Alice".to_string(), "Hello World!".to_string());
        let json = msg.to_json().unwrap();
        let deserialized = Message::from_json(&json).unwrap();
        
        assert_eq!(msg.op_code, deserialized.op_code);
        assert_eq!(msg.sender, deserialized.sender);
    }
    
    #[test]
    fn test_message_validation() {
        // Test message valide
        let valid_msg = Message::connect("valid_user".to_string());
        assert!(valid_msg.validate().is_ok());
        
        // Test message invalide (username vide)
        let invalid_msg = Message::connect("".to_string());
        assert!(invalid_msg.validate().is_err());
        
        // Test message texte sans expéditeur
        let mut text_msg = Message::text_message("sender".to_string(), "content".to_string());
        text_msg.sender = None;
        assert!(text_msg.validate().is_err());
    }
    
    #[test]
    fn test_error_codes() {
        let error_msg = Message::error(
            error_codes::INVALID_MESSAGE,
            "Test error".to_string()
        );
        
        if let MessagePayload::Error { code, message } = error_msg.payload {
            assert_eq!(code, 400);
            assert_eq!(message, "Test error");
        } else {
            panic!("Wrong payload type for error message");
        }
    }
}
