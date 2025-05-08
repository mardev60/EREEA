mod carte;
mod base;
mod robot;  
mod interface_user;
mod placement;

use carte::{generate_carte, TypeCase, trouver_position_base};
use base::Base;
use interface_user::afficher_interface_jeu;
use crossterm::terminal::size;

fn main() {
    // Taille de la carte
    let (width, height) = size().unwrap();
    let largeur_carte = width as usize;
    let hauteur_carte = height as usize;

    let seed = 1243;
    let (carte, known_carte) = generate_carte(largeur_carte, hauteur_carte, seed);

    let (base_x, base_y) = trouver_position_base(&carte);
    let base = Base::init(largeur_carte, hauteur_carte, base_x, base_y, known_carte.clone());
    println!("Base: {:?}", base);

    
    let stats = "Energy: 555 | Mineral: 100 | Science: 100";

    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        afficher_interface_jeu(&known_carte, stats).unwrap();
    }
}
