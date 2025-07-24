# Rust - Guide de Référence

## Commandes Cargo

| Commande | Description |
|----------|-------------|
| `cargo build` | Compile le projet en mode debug par défaut |
| `cargo check` | Vérifie le code sans produire de binaire |
| `cargo update` | Met à jour les dépendances |
| `cargo doc --open` | Génère la documentation et l'ouvre dans le navigateur Web |
| `cargo run` | Compile et exécute le projet |

## Types de Données

### Types Entiers

| Type | Taille | Signé | Plage |
|------|--------|-------|--------|
| `i32` | 32 bits | Oui | -2,147,483,648 à 2,147,483,647 |
| `u32` | 32 bits | Non | 0 à 4,294,967,295 |
| `i64` | 64 bits | Oui | Très grand intervalle |
| `u8` | 8 bits | Non | 0 à 255 |

**Note :** Rust utilise `i32` par défaut pour les entiers. Utilisez `snake_case` pour nommer les variables (convention Rust).

### Types Flottants

| Type | Taille | Plage |
|------|--------|-------|
| `f32` | 32 bits | ±3.4 × 10^38 (précision de 6-7 chiffres) |
| `f64` | 64 bits | ±1.7 × 10^308 (précision de 15-16 chiffres) |
**Note :** Utilisez `f64` par défaut pour les flottants.

### Booléens

| Type | Valeurs |
|------|---------|
| `bool` | `true` ou `false` |

### Caractères

| Type | Description |
|------|-------------|
| `char` | Représente un caractère Unicode (4 octets) |
| Exemple | `'a'`, `'1'`, `'\n'` |

### Chaînes de Caractères

| Type | Description |
|------|-------------|
| `String` | Chaîne de caractères mutable, allouée dynamiquement |
| `&str` | Chaîne de caractères immuable, référence à une chaîne |
| Exemple | `String::from("Hello")`, `"Hello"` |

### Tuples

| Type | Description |
|------|-------------|
| Tuple | Groupe de valeurs de types différents |
| Exemple | `(1, "Hello", 3.14)` |
| Accès | `let (x, y, z) = tuple; println!("x: {}, y: {}, z: {}", x, y, z);` |

## Variables et Immutabilité

En Rust, les variables sont immuables par défaut. Pour déclarer une variable mutable, utilisez le mot-clé `mut` :

```rust
let mut x = 5; // x est mutable
x = 10; // On peut modifier x
println!("x: {}", x);
```

```rust
let y:[i32;4] = [1,2,3,4]; // y est un tableau immuable
let mut z:[i32;4] = [1,2,3,4]; // z est un tableau mutable
z[0] = 10; // On peut modifier z
println!("z: {:?}", z);
```

## Fonctions

```rust
fn addition(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let resultat = addition(5, 3);
    println!("Résultat: {}", resultat);
}
```

- `fn` définit une fonction
- `&str` est un type de chaîne de caractères (référence)

## Structures de Contrôle

### Conditions

```rust
let nombre = 16;
if nombre % 2 == 0 {
    println!("Pair");
} else {
    println!("Impair");
}
```

### Boucles

#### Boucle for classique

```rust
for i in 1..=10 {
    println!("i vaut {}", i);
}
```

**Intervalles :**

- `1..5` : intervalle exclusif (fin exclue) → 1, 2, 3, 4
- `1..=5` : intervalle inclusif (fin incluse) → 1, 2, 3, 4, 5

#### Boucle Loop

```rust
let mut compteur = 0;
loop {
    println!(" Compteur: {}", compteur);
    compteur+=1;
    if compteur == ! {
        break; // On sort de la boucle quand le compteur atteint 3
    }
}
```

#### Boucle While

```rust
let mut compteur2 = 0;
while compteur2 < 4 {
    println!(" Compteur 2: {}", compteur2);
    compteur+=1;
}
```

## Collections

### Tableaux

#### Itération simple

```rust
let voitures = ["jeep", "renault", "bmw"];
for voiture in voitures {
    println!("Voiture : {}", voiture);
}
```

#### Itération avec index

```rust
for (i, voiture) in voitures.iter().enumerate() {
    println!("Index {} : {}", i, voiture);
}
```

#### Itération sur les références aux elements du tableau

```rust
for &elt in &tab {
    println!("l'element est {}", elt);
}
```

**Méthodes importantes :**

- `iter()` : crée un itérateur sur la collection sans la consommer
- `enumerate()` : transforme l'itérateur en séquence (index, valeur)

#### Warning lors de l'utilisation des tableaux quand un tableau n'est pas utilisé (pour debug)

Ajouter _ devant la variable :

```rust
let tab:[i32;4] = [1,2,3,4];
let _tab2:[i32;4] = [1,2,3,4];
```

### Vecteurs

```rust
let noms = vec![String::from("Kevin"), String::from("Nourdine")];
for (i, nom) in noms.iter().enumerate() {
    println!("Nom {} : {}", i, nom);
}
```

## Exemple Pratique

La fonction `enumerate()` est particulièrement utile pour :

- Afficher un menu de choix
- Créer des listes numérotées
- Associer des indices à des valeurs

## Les structure (struct)

```rust
struct Salarie {
    nom: String,
    ville: String,
    age: u32
}
```

### Usage d'une structure => on crée une instance de la structure

```rust
let kevin = Salarie {
    nom:String::from("kevin"),
    ville:String::from("Lyon"),
    age:25
};

println!("Nom :{}, Ville :{}, Age :{}", kevin.nom, kevin.ville, kevin.age);
```

### Fonctions associées aux structures

pour utiliser des fonctions associées à une structure, on utilise `impl` :

```rust
impl Salarie {
    fn afficher(&self) {
        println!("Nom :{}, Ville :{}, Age :{}", self.nom, self.ville, self.age);
    }
}
fn main() {
    let kevin = Salarie {
        nom: String::from("Kevin"),
        ville: String::from("Lyon"),
        age: 25,
    };
    
    kevin.afficher();
}
```

il faut ajouter `&self` pour accéder aux attributs de la structure.
ceci est similaire à `this` en JavaScript ou `self` en Python.
ceci est un emprunt immuable, donc on ne peut pas modifier les attributs de la structure.

#### Exemples

```rust
struct Compteur {
    value: u32
}

impl Compteur {
    fn afficher(&self) {
        println!("Compteur : {}", self.value);
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn deplacer(self) {
        println!("Déplacement du compteur : {}", self.value);
    }
}

fn main() {
    let mut compteur = Compteur { value: 0 };
    
    compteur.afficher(); // Affiche 0
    compteur.increment(); // Incrémente de 1
    compteur.afficher(); // Affiche 1
    
    compteur.deplacer(); // Déplace le compteur

    // compteur.afficher(); // Erreur : `compteur` n'est plus valide après `deplacer`
}
```

&variable permet de faire un emprunt immuable, ce qui signifie que la fonction peut lire les données mais ne peut pas les modifier.
&mut variable permet de faire un emprunt mutable, ce qui signifie que la fonction peut lire et modifier les données.
variable permet de prendre possession des données, ce qui signifie que la fonction peut lire et modifier les données, mais la variable d'origine n'est plus valide.

## Equivalent du switch en rust : match

```rust
let nombre = 5;

match nombre {
    1 => println!("1"),
    2 => println!("2"),
    3 => println!("3"),
    4 => println!("4"),
    _ => println!("nada"),
}
```

## Fichiers

### Ecriture dans un fichier

```rust
use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut file = File::create("output.txt")?;

    file.write_all(b"Hello, world!")?;
    file.write_all(b"\nThis is a test file.")?;

    println!("File written successfully!");

    Ok(())
}
```

ne pas utiliser "é" dans le texte, car cela peut causer des problèmes d'encodage si le fichier est ouvert dans un éditeur qui ne supporte pas l'UTF-8. (code ASCII seul)

`Ok(());` signifie que la fonction `main` s'est exécutée avec succès et a retourné un résultat vide.

`Err(e);` signifie que la fonction `main` s'est terminée avec une erreur et a retourné un résultat contenant l'erreur.

io::Result<()> est un type de résultat qui peut être soit Ok(()), soit Err(e). Il est utilisé pour gérer les erreurs lors de l'écriture dans un fichier.

### Lecture d'un fichier

```rust
use std::fs::File;
use std::io::{self, BufReader, Read};
fn main() -> io::Result<()> {
    let file = File::open("output.txt")?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();

    reader.read_to_string(&mut contents)?;

    println!("File contents:\n{}", contents);

    Ok(())
}
```

## Heures et Dates

```rust
let now = Utc::now();
println!("Current time: {}", now);
```

l'heure affichée est en UTC, donc dans le format : `2023-10-01 12:34:56.45454545 UTC`.

pour le format FR ou FR + heure locale, on peut le formater de telle manière :

```rust
let now = Utc::now();
println!("Current time: {}", now.format("%Y-%m-%d %H:%M:%S").to_string());
```

## Ownership et membership

## Ownership et Membership

### 1. Ownership (Propriété)

- Chaque valeur a un propriétaire unique, responsable de libérer la mémoire
- Lorsqu'elle sort du scope
- Quand le propriétaire est déplacé, l'ancien propriétaire ne peut plus y accéder
- Quand le propriétaire sort du scope, la valeur est automatiquement libérée

**Exemple :**

```rust
let prenom = String::from("Noureddine"); // prenom est propriétaire de la String
let secu = String::from("1897272824252");
let prenom2 = prenom.clone();

greetings(prenom); // propriétaire est transféré à la fonction greetings()
println!("{}", prenom2); 

greetings2(&secu);  // emprunt immuable 
println!("{}", secu); 
```

### 2. Membership (Appartenance à une structure)

Décrit quelles sont les données contenues dans une structure `Struct`.

**Exemple :**

```rust
struct User {
    nom: String,
    secu: String,
}

let user = User {
    nom: String::from("Alexandre"),
    secu: String::from("1825678290 55")
};

println!("nom {}", user.nom);
display(user);
```

### Fonctions associées

```rust
// Fonction display qui prend possession
fn display(user: User) -> User {
    println!("Nom: {}, num secu : {}", user.nom, user.secu);
    user
}

// Avec emprunt & 
fn greetings2(msg: &String) {
    println!("Hello Mister {}", msg);
}   

// Sans emprunt
fn greetings(msg: String) {
    println!("Hello Mister {}", msg);
}
```

