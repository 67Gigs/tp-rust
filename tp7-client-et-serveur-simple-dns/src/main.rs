use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::UdpSocket;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;
use rand::Rng;

// Structure pour l'en-tête DNS selon RFC 1035
#[derive(Debug, Clone)]
struct DnsHeader {
    id: u16,           // Identifiant de la requête
    flags: u16,        // Flags (QR, OPCODE, AA, TC, RD, RA, Z, RCODE)
    qdcount: u16,      // Nombre de questions
    ancount: u16,      // Nombre de réponses
    nscount: u16,      // Nombre d'enregistrements d'autorité
    arcount: u16,      // Nombre d'enregistrements additionnels
}

impl DnsHeader {
    fn new_query(id: u16) -> Self {
        DnsHeader {
            id,
            flags: 0x0100, // RD (Recursion Desired) = 1
            qdcount: 1,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }

    fn new_response(id: u16, answer_count: u16) -> Self {
        DnsHeader {
            id,
            flags: 0x8180, // QR=1, RD=1, RA=1
            qdcount: 1,
            ancount: answer_count,
            nscount: 0,
            arcount: 0,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.write_u16::<BigEndian>(self.id).unwrap();
        bytes.write_u16::<BigEndian>(self.flags).unwrap();
        bytes.write_u16::<BigEndian>(self.qdcount).unwrap();
        bytes.write_u16::<BigEndian>(self.ancount).unwrap();
        bytes.write_u16::<BigEndian>(self.nscount).unwrap();
        bytes.write_u16::<BigEndian>(self.arcount).unwrap();
        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let mut cursor = Cursor::new(bytes);
        Ok(DnsHeader {
            id: cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?,
            flags: cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?,
            qdcount: cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?,
            ancount: cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?,
            nscount: cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?,
            arcount: cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?,
        })
    }
}

// Structure pour une question DNS
#[derive(Debug, Clone)]
struct DnsQuestion {
    name: String,
    qtype: u16,    // Type de requête (A = 1, AAAA = 28, etc.)
    qclass: u16,   // Classe (IN = 1 pour Internet)
}

impl DnsQuestion {
    fn new(name: String) -> Self {
        DnsQuestion {
            name,
            qtype: 1,  // Type A (IPv4)
            qclass: 1, // Classe IN (Internet)
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Encoder le nom de domaine
        for part in self.name.split('.') {
            bytes.push(part.len() as u8);
            bytes.extend_from_slice(part.as_bytes());
        }
        bytes.push(0); // Fin du nom
        
        bytes.write_u16::<BigEndian>(self.qtype).unwrap();
        bytes.write_u16::<BigEndian>(self.qclass).unwrap();
        bytes
    }

    fn from_bytes(bytes: &[u8], offset: &mut usize) -> Result<Self, String> {
        let mut name_parts = Vec::new();
        
        while *offset < bytes.len() {
            let len = bytes[*offset] as usize;
            *offset += 1;
            
            if len == 0 {
                break;
            }
            
            let part = String::from_utf8(bytes[*offset..*offset + len].to_vec())
                .map_err(|e| e.to_string())?;
            name_parts.push(part);
            *offset += len;
        }
        
        let name = name_parts.join(".");
        
        let mut cursor = Cursor::new(&bytes[*offset..]);
        let qtype = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
        let qclass = cursor.read_u16::<BigEndian>().map_err(|e| e.to_string())?;
        *offset += 4;
        
        Ok(DnsQuestion { name, qtype, qclass })
    }
}

// Structure pour une réponse DNS
#[derive(Debug, Clone)]
struct DnsAnswer {
    name: String,
    atype: u16,    // Type de l'enregistrement
    aclass: u16,   // Classe
    ttl: u32,      // Time To Live
    rdlength: u16, // Longueur des données
    rdata: Vec<u8>, // Données de la réponse
}

impl DnsAnswer {
    fn new_a_record(name: String, ip: Ipv4Addr, ttl: u32) -> Self {
        DnsAnswer {
            name,
            atype: 1,  // Type A
            aclass: 1, // Classe IN
            ttl,
            rdlength: 4,
            rdata: ip.octets().to_vec(),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Encoder le nom (même format que dans la question)
        for part in self.name.split('.') {
            bytes.push(part.len() as u8);
            bytes.extend_from_slice(part.as_bytes());
        }
        bytes.push(0);
        
        bytes.write_u16::<BigEndian>(self.atype).unwrap();
        bytes.write_u16::<BigEndian>(self.aclass).unwrap();
        bytes.write_u32::<BigEndian>(self.ttl).unwrap();
        bytes.write_u16::<BigEndian>(self.rdlength).unwrap();
        bytes.extend_from_slice(&self.rdata);
        
        bytes
    }
}

// Structure complète d'un message DNS
#[derive(Debug)]
struct DnsMessage {
    header: DnsHeader,
    questions: Vec<DnsQuestion>,
    answers: Vec<DnsAnswer>,
}

impl DnsMessage {
    fn new_query(domain: String) -> Self {
        let id = rand::thread_rng().gen::<u16>();
        DnsMessage {
            header: DnsHeader::new_query(id),
            questions: vec![DnsQuestion::new(domain)],
            answers: vec![],
        }
    }

    fn new_response(query_id: u16, question: DnsQuestion, answers: Vec<DnsAnswer>) -> Self {
        DnsMessage {
            header: DnsHeader::new_response(query_id, answers.len() as u16),
            questions: vec![question],
            answers,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        bytes.extend_from_slice(&self.header.to_bytes());
        
        for question in &self.questions {
            bytes.extend_from_slice(&question.to_bytes());
        }
        
        for answer in &self.answers {
            bytes.extend_from_slice(&answer.to_bytes());
        }
        
        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let header = DnsHeader::from_bytes(&bytes[0..12])?;
        let mut offset = 12;
        
        let mut questions = Vec::new();
        for _ in 0..header.qdcount {
            let question = DnsQuestion::from_bytes(bytes, &mut offset)?;
            questions.push(question);
        }
        
        // Pour simplifier, on ne parse pas les réponses dans ce TP
        let answers = Vec::new();
        
        Ok(DnsMessage {
            header,
            questions,
            answers,
        })
    }
}

// Client DNS
struct DnsClient;

impl DnsClient {
    async fn resolve(domain: &str, dns_server: &str) -> Result<Option<Ipv4Addr>, String> {
        let socket = UdpSocket::bind("0.0.0.0:0").await.map_err(|e| e.to_string())?;
        
        // Gérer le cas où le serveur DNS inclut déjà le port
        let server_addr: SocketAddr = if dns_server.contains(':') {
            dns_server.parse().map_err(|e: std::net::AddrParseError| e.to_string())?
        } else {
            format!("{}:53", dns_server).parse()
                .map_err(|e: std::net::AddrParseError| e.to_string())?
        };
        
        let query = DnsMessage::new_query(domain.to_string());
        let query_bytes = query.to_bytes();
        
        println!("📤 Envoi de la requête DNS pour {} vers {}", domain, dns_server);
        socket.send_to(&query_bytes, server_addr).await.map_err(|e| e.to_string())?;
        
        let mut buf = [0u8; 512];
        let (size, _) = socket.recv_from(&mut buf).await.map_err(|e| e.to_string())?;
        
        // Parse simple de la réponse (extraction de l'IP depuis les données brutes)
        if size > 12 {
            let _response = DnsMessage::from_bytes(&buf[..size])?;
            
            // Recherche de l'IP dans les données brutes (position approximative)
            // Cette implémentation est simplifiée pour le TP
            if size >= 40 && buf[3] & 0x0F == 0 { // Pas d'erreur
                // L'IP se trouve généralement vers la fin du paquet
                let ip_offset = size - 4;
                if ip_offset >= 4 {
                    let ip = Ipv4Addr::new(
                        buf[ip_offset - 4],
                        buf[ip_offset - 3],
                        buf[ip_offset - 2],
                        buf[ip_offset - 1]
                    );
                    println!("✅ Résolution réussie: {} -> {}", domain, ip);
                    return Ok(Some(ip));
                }
            }
        }
        
        println!("❌ Échec de la résolution pour {}", domain);
        Ok(None)
    }
}

// Serveur DNS simple
struct DnsServer {
    records: HashMap<String, Ipv4Addr>,
}

impl DnsServer {
    fn new() -> Self {
        let mut records = HashMap::new();
        
        // Ajouter quelques enregistrements prédéfinis
        records.insert("example.com".to_string(), Ipv4Addr::new(93, 184, 216, 34));
        records.insert("test.local".to_string(), Ipv4Addr::new(127, 0, 0, 1));
        records.insert("server.local".to_string(), Ipv4Addr::new(192, 168, 1, 100));
        records.insert("dns.local".to_string(), Ipv4Addr::new(192, 168, 1, 1));
        records.insert("67gigs.com".to_string(), Ipv4Addr::new(67, 67, 67, 67));
        
        DnsServer { records }
    }

    async fn start(&self, bind_addr: &str) -> Result<(), String> {
        let socket = UdpSocket::bind(bind_addr).await.map_err(|e| e.to_string())?;
        println!("🚀 Serveur DNS démarré sur {}", bind_addr);
        println!("📋 Domaines disponibles:");
        for (domain, ip) in &self.records {
            println!("   {} -> {}", domain, ip);
        }
        println!();

        let mut buf = [0u8; 512];
        
        loop {
            let (size, src) = socket.recv_from(&mut buf).await.map_err(|e| e.to_string())?;
            println!("📨 Requête reçue de {} ({} bytes)", src, size);
            
            match self.handle_query(&buf[..size]).await {
                Ok(response) => {
                    socket.send_to(&response, src).await.map_err(|e| e.to_string())?;
                    println!("📤 Réponse envoyée à {}", src);
                }
                Err(e) => {
                    eprintln!("❌ Erreur lors du traitement de la requête: {}", e);
                }
            }
        }
    }

    async fn handle_query(&self, query_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let query = DnsMessage::from_bytes(query_bytes)?;
        
        if query.questions.is_empty() {
            return Err("Aucune question dans la requête".to_string());
        }
        
        let question = &query.questions[0];
        println!("🔍 Recherche de: {} (type: {})", question.name, question.qtype);
        
        let mut answers = Vec::new();
        
        if question.qtype == 1 { // Type A
            if let Some(ip) = self.records.get(&question.name) {
                let answer = DnsAnswer::new_a_record(
                    question.name.clone(),
                    *ip,
                    300 // TTL de 5 minutes
                );
                answers.push(answer);
                println!("✅ Réponse trouvée: {} -> {}", question.name, ip);
            } else {
                println!("❌ Domaine non trouvé: {}", question.name);
            }
        }
        
        let response = DnsMessage::new_response(
            query.header.id,
            question.clone(),
            answers
        );
        
        Ok(response.to_bytes())
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    println!("🌐 TP 7: Client et Serveur DNS Simples");
    println!("=====================================");
    
    // Démarrer le serveur DNS en arrière-plan
    let server = DnsServer::new();
    let server_task = tokio::spawn(async move {
        // Utiliser un port plus élevé pour éviter les problèmes de permissions
        if let Err(e) = server.start("0.0.0.0:8053").await {
            eprintln!("Erreur serveur DNS: {}", e);
        }
    });
    
    // Attendre un peu que le serveur démarre
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    println!("\n🔍 Test du client DNS:");
    println!("----------------------");
    
    // Tester avec notre serveur local
    let domains_to_test = vec![
        "test.local",
        "server.local",
        "dns.local",
        "example.com",
        "67gigs.com",
        "nonexistent.local"
    ];
    
    for domain in domains_to_test {
        match DnsClient::resolve(domain, "127.0.0.1:8053").await {
            Ok(Some(ip)) => println!("✅ {} résolu en {}", domain, ip),
            Ok(None) => println!("❌ {} non résolu", domain),
            Err(e) => println!("❌ Erreur pour {}: {}", domain, e),
        }
        println!();
    }
    
    println!("\n🌍 Test avec des serveurs DNS publics:");
    println!("--------------------------------------");
    
    // Tester avec des serveurs DNS réels (Google, Cloudflare)
    let public_tests = vec![
        ("google.com", "8.8.8.8"),
        ("cloudflare.com", "1.1.1.1"),
    ];
    
    for (domain, dns_server) in public_tests {
        match DnsClient::resolve(domain, dns_server).await {
            Ok(Some(ip)) => println!("✅ {} résolu en {} via {}", domain, ip, dns_server),
            Ok(None) => println!("❌ {} non résolu via {}", domain, dns_server),
            Err(e) => println!("❌ Erreur pour {} via {}: {}", domain, dns_server, e),
        }
        println!();
    }
    
    println!("💡 Le serveur DNS continue de fonctionner sur 127.0.0.1:8053");
    println!("   Vous pouvez tester avec: nslookup test.local 127.0.0.1 -port=8053");
    
    // Maintenir le serveur en vie
    server_task.await.map_err(|e| e.to_string())?;
    
    Ok(())
}
