struct CompteBancaire {
    nom :String,
    solde: f64,
}

impl CompteBancaire{

    fn afficher(&self){
        println!("Compte de {} : {} €", self.nom, self.solde);
    }

    fn deposer(&mut self, montant:f64){
        if montant <= 0.0 {
            println!("Le montant à déposer doit être positif.");
            return;
        }
        self.solde += montant;
        println!("+{} € déposés:",montant);
    }

    fn retirer( &mut self, montant:f64){

        if self.solde >= montant{
            self.solde -=montant;
            println!("-{} € retirés.",montant)
        } else{
            println!("Solde insuffisant")
        }
    }

    fn renommer(&mut self, nouveau_nom: String){
        self.nom = nouveau_nom;
        println!("Le compte a été renommé en : {}", self.nom);
    }

    fn fermer(self){
        println!("le compte de {} est fermé, dernier solde : {}€ ", self.nom, self.solde);
    }

    // self ici est consomé ici , on ne peut plus utiliser l'objet ensuite
}



fn main() {
    let mut compte_nb = CompteBancaire {
        nom: "Noureddine".to_string(),
        solde: 1000.0,
    };

    compte_nb.afficher();

    compte_nb.deposer(500.0);
    compte_nb.afficher();

    compte_nb.retirer(200.0);
    compte_nb.renommer("Noureddine B".to_string());
    compte_nb.afficher();

    compte_nb.fermer();
    
    // créer un Vec<CompteBancaire> pour gérer plusieurs comptes ( en utilisant .iter(), .enumerate() )
    let mut comptes: Vec<CompteBancaire> = Vec::new();
    comptes.push(CompteBancaire {
        nom: "Alice".to_string(),
        solde: 1500.0,
    });
    comptes.push(CompteBancaire {
        nom: "Bob".to_string(),
        solde: 2000.0,
    });
    comptes.push(CompteBancaire {
        nom: "Charlie".to_string(),
        solde: 3000.0,
    });
    for (_index, compte) in comptes.iter().enumerate() {
        compte.afficher();
    }

    comptes[0].deposer(100.0);
    comptes[1].retirer(500.0);
    comptes[2].renommer("Charlie Brown".to_string());
    for (_index, compte) in comptes.iter().enumerate() {
        compte.afficher();
    }
}