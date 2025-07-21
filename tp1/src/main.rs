use std::io;

fn main() {
    // Variables pour le compte bancaire
    let mut solde: f32 = 1500.50;
    let titulaire = "Kevin Dupont";
    let numero_compte = "FR123456789";
    
    println!("=== Système de Gestion Bancaire ===");
    println!("Titulaire: {}", titulaire);
    println!("Numéro: {}", numero_compte);
    println!("Solde initial: {:.2} €", solde);
    println!("===================================\n");

    loop {
        // Menu principal avec enumerate
        let options = ["Afficher solde", "Retrait", "Liste comptes", "Quitter"];
        println!("Menu:");
        for (i, option) in options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }
        
        println!("Veuillez saisir un numéro de votre choix:");
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        
        let choix: usize = match choix.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez saisir un numéro valide");
                continue;
            }
        };
        
        if choix == 0 || choix > options.len() {
            println!("Choix hors limite!");
            continue;
        }
        
        println!("Vous avez sélectionné: {}\n", options[choix - 1]);
        
        // Actions selon le choix
        if choix == 1 {
            // Afficher solde
            afficher_solde(titulaire, numero_compte, solde);
        } else if choix == 2 {
            // Retrait
            println!("Entrez le montant à retirer:");
            let mut montant_str = String::new();
            io::stdin().read_line(&mut montant_str).expect("Erreur de lecture");
            
            let montant: f32 = match montant_str.trim().parse() {
                Ok(m) => m,
                Err(_) => {
                    println!("Montant invalide!");
                    continue;
                }
            };
            
            solde = retrait(solde, montant);
        } else if choix == 3 {
            // Liste comptes
            liste_comptes();
        } else if choix == 4 {
            // Quitter
            println!("Au revoir!");
            break;
        }
        
        println!("Appuyez sur Entrée pour continuer...");
        let mut pause = String::new();
        io::stdin().read_line(&mut pause).expect("Erreur");
        println!();
    }
}

fn afficher_solde(titulaire: &str, numero: &str, solde: f32) {
    println!("=== SOLDE DU COMPTE ===");
    println!("Titulaire: {}", titulaire);
    println!("Numéro: {}", numero);
    println!("Solde: {:.2} €", solde);
    println!("=======================");
}

fn retrait(solde_actuel: f32, montant: f32) -> f32 {
    if montant <= 0.0 {
        println!("Le montant doit être positif!");
        return solde_actuel;
    }
    
    if solde_actuel >= montant {
        let nouveau_solde = solde_actuel - montant;
        println!("Retrait de {:.2} € effectué!", montant);
        println!("Nouveau solde: {:.2} €", nouveau_solde);
        nouveau_solde
    } else {
        println!("Solde insuffisant! Solde actuel: {:.2} €", solde_actuel);
        solde_actuel
    }
}

fn liste_comptes() {
    println!("=== LISTE DES COMPTES ===");
    let comptes = [
        ("Kevin Dupont", "FR123456789", 1500.50),
        ("Marie Martin", "FR987654321", 2750.00),
        ("Paul Bernard", "FR555666777", 890.25),
    ];
    
    for (i, (nom, numero, solde)) in comptes.iter().enumerate() {
        println!("{}. {} - {} - {:.2} €", i + 1, nom, numero, solde);
    }
    println!("========================");
}