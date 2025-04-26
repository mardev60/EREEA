mod carte;
mod base;
mod robot;  
mod placement;
mod interface_user;

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
    let base = Base::init(largeur_carte, hauteur_carte, 10, 10);
    println!("Base: {:?}", base);

    
    let stats = "Energy: 555";

    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        afficher_interface_jeu(&carte, stats).unwrap();
    }
}
