use std::fs::{self, File};
use std::io::{self, Write, Read, BufReader};
use std::path::Path;
use chrono::Utc;

// Structure pour gérer les fichiers avec ownership et membership
struct GestionnaireFichier {
    chemin_courant: String,
    nom_fichier: String,
    contenu: String,
    date_creation: String,
}

impl GestionnaireFichier {
    // Constructeur pour créer un nouveau gestionnaire
    fn nouveau(chemin: String, nom: String) -> Self {
        let maintenant = Utc::now();
        GestionnaireFichier {
            chemin_courant: chemin,
            nom_fichier: nom,
            contenu: String::new(),
            date_creation: maintenant.format("%d/%m/%Y %H:%M:%S").to_string(),
        }
    }

    // Afficher les informations du fichier
    fn afficher_info(&self) {
        println!("=== INFORMATIONS DU FICHIER ===");
        println!("Nom: {}", self.nom_fichier);
        println!("Chemin: {}", self.chemin_courant);
        println!("Date de création: {}", self.date_creation);
        println!("Taille du contenu: {} caractères", self.contenu.len());
        println!("===============================");
    }

    // Lire un fichier existant
    fn lire_fichier(&mut self, chemin_fichier: &str) -> io::Result<()> {
        let fichier = File::open(chemin_fichier)?;
        let mut lecteur = BufReader::new(fichier);
        
        self.contenu.clear();
        lecteur.read_to_string(&mut self.contenu)?;
        
        self.chemin_courant = chemin_fichier.to_string();
        self.nom_fichier = Path::new(chemin_fichier)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        
        println!("Fichier '{}' lu avec succès!", self.nom_fichier);
        Ok(())
    }

    // Écrire dans un fichier (création ou écrasement)
    fn ecrire_fichier(&mut self, chemin_fichier: &str, contenu: String) -> io::Result<()> {
        let mut fichier = File::create(chemin_fichier)?;
        fichier.write_all(contenu.as_bytes())?;
        
        self.contenu = contenu;
        self.chemin_courant = chemin_fichier.to_string();
        self.nom_fichier = Path::new(chemin_fichier)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        
        println!("Fichier '{}' écrit avec succès!", self.nom_fichier);
        Ok(())
    }

    // Modifier le contenu du fichier actuel
    fn modifier_contenu(&mut self, nouveau_contenu: String) {
        self.contenu = nouveau_contenu;
        println!("Contenu modifié en mémoire. N'oubliez pas de sauvegarder!");
    }

    // Ajouter du contenu au fichier existant
    fn ajouter_contenu(&mut self, contenu_additionnel: &str) {
        self.contenu.push_str(contenu_additionnel);
        println!("Contenu ajouté en mémoire. N'oubliez pas de sauvegarder!");
    }

    // Sauvegarder les modifications
    fn sauvegarder(&self) -> io::Result<()> {
        let mut fichier = File::create(&self.chemin_courant)?;
        fichier.write_all(self.contenu.as_bytes())?;
        println!("Modifications sauvegardées dans '{}'!", self.nom_fichier);
        Ok(())
    }

    // Supprimer définitivement un fichier
    fn supprimer_fichier(chemin_fichier: &str) -> io::Result<()> {
        fs::remove_file(chemin_fichier)?;
        println!("Fichier '{}' supprimé définitivement!", chemin_fichier);
        Ok(())
    }

    // Afficher le contenu du fichier
    fn afficher_contenu(&self) {
        if self.contenu.is_empty() {
            println!("Le fichier est vide ou aucun fichier n'est chargé.");
        } else {
            println!("=== CONTENU DU FICHIER ===");
            println!("{}", self.contenu);
            println!("=========================");
        }
    }

    // Lister les fichiers dans un répertoire
    fn lister_fichiers(repertoire: &str) -> io::Result<()> {
        println!("=== FICHIERS DANS '{}' ===", repertoire);
        let entrees = fs::read_dir(repertoire)?;
        
        for (index, entree) in entrees.enumerate() {
            let entree = entree?;
            let chemin = entree.path();
            
            if chemin.is_file() {
                println!("{}. {}", index + 1, chemin.display());
            }
        }
        println!("========================");
        Ok(())
    }

    // Consommer le gestionnaire (exemple d'ownership)
    fn fermer(self) {
        println!("Gestionnaire fermé pour le fichier: {}", self.nom_fichier);
        println!("Date de création était: {}", self.date_creation);
        // self est consommé ici, ne peut plus être utilisé
    }
}

fn main() {
    println!("=== GESTIONNAIRE DE FICHIERS ===");
    println!("Date et heure: {}", Utc::now().format("%d/%m/%Y %H:%M:%S"));
    println!("=================================\n");

    let mut gestionnaire = GestionnaireFichier::nouveau(
        String::from("./"),
        String::from("nouveau_fichier.txt")
    );

    // Boucle principale du programme
    loop {
        afficher_menu();
        
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        
        let choix: u32 = match choix.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez saisir un numéro valide!");
                continue;
            }
        };

        // Utilisation de match pour gérer les choix
        match choix {
            1 => {
                // Lire un fichier
                println!("Entrez le chemin du fichier à lire:");
                let mut chemin = String::new();
                io::stdin().read_line(&mut chemin).expect("Erreur de lecture");
                let chemin = chemin.trim();
                
                match gestionnaire.lire_fichier(chemin) {
                    Ok(()) => gestionnaire.afficher_info(),
                    Err(e) => println!("Erreur lors de la lecture: {}", e),
                }
            },
            2 => {
                // Créer/Écrire un fichier
                println!("Entrez le nom du fichier à créer:");
                let mut nom = String::new();
                io::stdin().read_line(&mut nom).expect("Erreur de lecture");
                let nom = nom.trim();
                
                println!("Entrez le contenu du fichier:");
                let mut contenu = String::new();
                io::stdin().read_line(&mut contenu).expect("Erreur de lecture");
                
                match gestionnaire.ecrire_fichier(nom, contenu) {
                    Ok(()) => gestionnaire.afficher_info(),
                    Err(e) => println!("Erreur lors de l'écriture: {}", e),
                }
            },
            3 => {
                // Modifier le contenu
                println!("Contenu actuel:");
                gestionnaire.afficher_contenu();
                
                println!("Entrez le nouveau contenu:");
                let mut nouveau_contenu = String::new();
                io::stdin().read_line(&mut nouveau_contenu).expect("Erreur de lecture");
                
                gestionnaire.modifier_contenu(nouveau_contenu);
            },
            4 => {
                // Ajouter du contenu
                println!("Entrez le contenu à ajouter:");
                let mut contenu_additionnel = String::new();
                io::stdin().read_line(&mut contenu_additionnel).expect("Erreur de lecture");
                
                gestionnaire.ajouter_contenu(&contenu_additionnel);
            },
            5 => {
                // Sauvegarder
                match gestionnaire.sauvegarder() {
                    Ok(()) => println!("Sauvegarde réussie!"),
                    Err(e) => println!("Erreur lors de la sauvegarde: {}", e),
                }
            },
            6 => {
                // Afficher le contenu
                gestionnaire.afficher_contenu();
            },
            7 => {
                // Afficher les informations
                gestionnaire.afficher_info();
            },
            8 => {
                // Lister les fichiers
                println!("Entrez le répertoire à lister (ou '.' pour le répertoire courant):");
                let mut repertoire = String::new();
                io::stdin().read_line(&mut repertoire).expect("Erreur de lecture");
                let repertoire = repertoire.trim();
                
                match GestionnaireFichier::lister_fichiers(repertoire) {
                    Ok(()) => {},
                    Err(e) => println!("Erreur lors du listage: {}", e),
                }
            },
            9 => {
                // Supprimer un fichier
                println!("ATTENTION: Cette action est irréversible!");
                println!("Entrez le chemin du fichier à supprimer:");
                let mut chemin = String::new();
                io::stdin().read_line(&mut chemin).expect("Erreur de lecture");
                let chemin = chemin.trim();
                
                println!("Êtes-vous sûr? (oui/non):");
                let mut confirmation = String::new();
                io::stdin().read_line(&mut confirmation).expect("Erreur de lecture");
                
                if confirmation.trim().to_lowercase() == "oui" {
                    match GestionnaireFichier::supprimer_fichier(chemin) {
                        Ok(()) => println!("Fichier supprimé!"),
                        Err(e) => println!("Erreur lors de la suppression: {}", e),
                    }
                } else {
                    println!("Suppression annulée.");
                }
            },
            10 => {
                // Quitter
                println!("Fermeture du gestionnaire...");
                gestionnaire.fermer(); // Consommation du gestionnaire
                break;
            },
            _ => println!("Choix invalide! Veuillez choisir entre 1 et 10."),
        }
        
        // Pause avant de continuer
        println!("\nAppuyez sur Entrée pour continuer...");
        let mut pause = String::new();
        io::stdin().read_line(&mut pause).expect("Erreur");
        println!();
    }
    
    println!("Au revoir!");
}

fn afficher_menu() {
    let options = [
        "Lire un fichier",
        "Créer/Écrire un fichier", 
        "Modifier le contenu en mémoire",
        "Ajouter du contenu",
        "Sauvegarder les modifications",
        "Afficher le contenu",
        "Afficher les informations du fichier",
        "Lister les fichiers du répertoire",
        "Supprimer définitivement un fichier",
        "Quitter"
    ];
    
    println!("=== MENU GESTIONNAIRE DE FICHIERS ===");
    
    // Utilisation d'enumerate dans une boucle for
    for (index, option) in options.iter().enumerate() {
        println!("{}. {}", index + 1, option);
    }
    
    println!("=====================================");
    println!("Votre choix:");
}
