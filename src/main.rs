mod base;
mod carte;
mod interface_user;
mod placement;
mod robot;

use base::Base;
use carte::{TypeCase, generate_carte, trouver_position_base};
use crossterm::terminal::size;
use interface_user::afficher_interface_jeu;
use std::sync::Arc;

fn main() {
    let (width, height) = size().unwrap();
    let largeur_carte = (width / 2) as usize;
    let hauteur_carte = height as usize;

    let seed = 577679768;
    let (carte, known_carte) = generate_carte(largeur_carte, hauteur_carte, seed);

    let (base_x, base_y) = trouver_position_base(&carte);

    let base = Base::new(
        largeur_carte,
        hauteur_carte,
        base_x,
        base_y,
        carte,
        known_carte.clone(),
    );

    let robots = if let Ok(base_guard) = base.lock() {
        Arc::clone(&base_guard.robots)
    } else {
        panic!("Impossible d'accéder à la base")
    };

    let known_carte_ref = if let Ok(base_guard) = base.lock() {
        Arc::clone(&base_guard.known_carte)
    } else {
        panic!("Impossible d'accéder à la base")
    };

    Base::demarrer_thread_base(Arc::clone(&base), largeur_carte, hauteur_carte);

    println!("Système démarré ! Base en ({}, {})", base_x, base_y);
    println!("Appuyez sur Ctrl+C pour arrêter le programme");

    loop {
        std::thread::sleep(std::time::Duration::from_millis(500));

        if let Ok(carte) = known_carte_ref.lock() {
            let stats = if let Ok(base_guard) = base.lock() {
                base_guard.get_ressources_string()
            } else {
                "Erreur d'accès aux ressources".to_string()
            };

            if let Err(e) = afficher_interface_jeu(&carte, &stats, &robots) {
                eprintln!("Erreur d'affichage: {}", e);
            }
        }
    }
}
