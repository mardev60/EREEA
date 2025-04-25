mod carte;
mod base;
mod robot;  
mod placement;

use carte::{generate_carte, TypeCase};
use base::Base;

fn main() {
    let largeur_carte = 20;
    let hauteur_carte = 20;
    let seed = 123;
    let carte = generate_carte(largeur_carte, hauteur_carte, seed);
    let base = Base::init(largeur_carte, hauteur_carte, 10, 10);
    println!("Base: {:?}", base);

    for ligne in carte {
        for case in ligne {
            let symbole = match case {
                TypeCase::Vide => ' ',
                TypeCase::Base => 'H',
                TypeCase::Mur => 'O',
                TypeCase::Mineral => 'M',
                TypeCase::Energy => 'E',
                TypeCase::Science => 'S',
                TypeCase::Collector => 'C',
                TypeCase::Explorer => 'X'
            };
            print!("{}", symbole);
        }
        println!();
    }
}
