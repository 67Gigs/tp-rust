cargo build  : compile le projet mode debug par defaut
cargo check   : vérifie le code sans produire de binaire
cargo update  met à jour les dépendances
cargo doc --open  : gènère la documentation et l'ouvre dans le navigateur Web
cargo run : compile et exécute le projet

u32 = entier non signé sur 32 bits ( valeurs positives)
rust comprend que c'est un entier par défaut i32 quand on affecte a une variable une valeur comme 72

pour déclarer les variables il faut utiliser les snake_case ( par convention de RUST ) et surtout ne jamais commencer par chiffre, pas d'espaces ni tirets 


i32    32   signé   -2xxx  à 2xxxxxxx
u32     32   non signé       0 à 4 xxxxxxx
i64     64     signé         très grand intervalle
u8    8     non signé      à à 255 

les fonctions  : 
       fn définit une fonction 
       &str est de type de chaine de caractères ( référence)
       on cree une fonction addtion() qui retourne une somme et on l'appelle depuis le main

Les conditions :
         let nombre = 16;
          if nombre %2 == 0 {
            println!("Pair");
          } else {
             println!("Impair");
          }

Les boucles :
         for i in 1..=10{
            println!(" i vaut {}", i);
         }

         // A noter que  1..5
         //  ..  intervalle exculsif ( fin exclue ) : 1,2,3,4
         // ..=  intervalle inclusif ( fin incluse ) : 1,2,3,4,5

Tableaux :
        iterer sur un tableau :
            let  voitures = ["jeep", "renault", "bmw"];
            for voiture in voitures {
                println!("Voiture : {}", voiture);
            }

        Utiliser tab.iter().enumerate() pour parcourir par index et valeur (key and value)
            for (i,voiture) in voitures.iter().enumerate(){
                println!("Index {} : {}", i, voiture);
            }
            iter(): crée un itérateur sur la collection sans le consommer
            enumerate: transforme l'itérateur en une séquence de index,valeur
        
Vecteurs :
        let noms = vec![String::from("Kevin"), String::from("Nourdine")];
        for(i,nom) in noms.iter().enumerate(){
            println!("Nom {} :{}", i, nom);
        }


    Exemple d'utilisation de la focntion enumerate dans un cas reel :
        - afficher un menu de choix


