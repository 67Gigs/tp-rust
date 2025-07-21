struct CompteBancaire {
    nom :String,
    solde: f64,
}

impl CompteBancaire{

    fn afficher(&self){
        println!("Compte de {} : {} €:", self.nom, self.solde);
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
    let mut compte = CompteBancaire {
        nom: "Noureddine".to_string(),
        solde: 1000.0,
    };

    compte.afficher();
    
    compte.deposer(500.0);
    compte.afficher();
    
    compte.retirer(200.0);
    compte.afficher();

    compte.fermer();
}