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

```rust
for i in 1..=10 {
    println!("i vaut {}", i);
}
```

**Intervalles :**
- `1..5` : intervalle exclusif (fin exclue) → 1, 2, 3, 4
- `1..=5` : intervalle inclusif (fin incluse) → 1, 2, 3, 4, 5

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

**Méthodes importantes :**
- `iter()` : crée un itérateur sur la collection sans la consommer
- `enumerate()` : transforme l'itérateur en séquence (index, valeur)

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
