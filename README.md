# Essaim de Robots pour l'Exploration et l'Étude Astrobiologique

```
 ███████╗██████╗ ███████╗███████╗ █████╗ 
 ██╔════╝██╔══██╗██╔════╝██╔════╝██╔══██╗
 █████╗  ██████╔╝█████╗  █████╗  ███████║
 ██╔══╝  ██╔══██╗██╔══╝  ██╔══╝  ██╔══██║
 ███████╗██║  ██║███████╗███████╗██║  ██║
 ╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝╚═╝  ╚═╝
     Exploration & Étude Astrobiologique                                  
```

EREEA est un jeu de simulation de robots en Rust qui s'exécute dans le terminal. Le jeu utilise une interface interactive basée sur Ratatui et Crossterm pour créer une expérience immersive de gestion de robots.

## 🎮 Description

Dans ce jeu, vous gérez une base qui contrôle différents types de robots (Collecteurs et Explorateurs) sur une carte générée procéduralement. La carte contient différents types de ressources :
- Minéraux
- Énergie
- Points de Science

Les robots peuvent explorer la carte et collecter ces ressources pour développer votre base.

## 🔧 Caractéristiques

- Génération procédurale de carte utilisant l'algorithme de Perlin Noise
- Interface utilisateur interactive dans le terminal
- Système de brouillard de guerre (zones inexplorées)
- Différents types de robots avec des fonctions spécifiques
- Gestion de ressources multiples

## 📦 Dépendances

- `crossterm` (v0.29.0) - Pour la gestion du terminal
- `noise` (v0.9.0) - Pour la génération procédurale de la carte
- `rand` (v0.8) - Pour la génération aléatoire
- `ratatui` (v0.29.0) - Pour l'interface utilisateur dans le terminal

## 🚀 Installation

1. Assurez-vous d'avoir Rust installé sur votre système
2. Clonez ce dépôt
3. Exécutez la commande :
```bash
cargo run
```

## 🎯 Comment jouer

Le jeu démarre avec une base placée aléatoirement sur la carte. Les robots explorent automatiquement les environs et collectent des ressources. L'interface affiche :
- La carte avec les différents éléments (base, robots, ressources)
- Les statistiques de ressources collectées
- Les zones explorées vs. inexplorées

Pour quitter le jeu, appuyez sur `Ctrl+C`.

## 🛠️ Structure du projet

- `src/main.rs` - Point d'entrée du programme et boucle principale
- `src/carte.rs` - Génération et gestion de la carte
- `src/robot.rs` - Logique des robots
- `src/base.rs` - Gestion de la base
- `src/interface_user.rs` - Interface utilisateur
- `src/placement.rs` - Algorithmes de placement des éléments 