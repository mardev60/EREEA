mod carte;
mod base;
mod robot;  
mod interface_user;
mod placement;

use carte::{generate_carte, TypeCase};
use base::Base;
use interface_user::afficher_interface_jeu;
use crossterm::terminal::size;

fn main() {
    // Taille de la carte
    let (width, height) = size().unwrap();
    let largeur_carte =(width as usize) - 5;
    let hauteur_carte = (height as usize) - 5;

    let seed = 123;
    let carte = generate_carte(largeur_carte, hauteur_carte, seed);

    // todo : trouver la position de la base (qui a été generée par le generateur de carte)
    let base = Base::init(largeur_carte, hauteur_carte, 10, 10);
    println!("Base: {:?}", base);

    
    let stats = "Energy: 555 | Mineral: 100 | Science: 100";

    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        afficher_interface_jeu(&carte, stats).unwrap();
    }
}
