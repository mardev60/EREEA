use noise::{NoiseFn, Perlin};
use rand::Rng;

#[derive(Clone, PartialEq, Debug)]
pub enum TypeCase {
    Vide,
    Base,
    Mur,
    Mineral,
    Energy,
    Science,
    Collector,
    Explorer
}

pub fn generate_carte(largeur_carte: usize, hauteur_carte: usize, seed: u32) -> Vec<Vec<TypeCase>> {
    let generateur_bruit = Perlin::new(seed);
    let mut generateur_aleatoire = rand::thread_rng();
    let mut carte = vec![vec![TypeCase::Vide; largeur_carte]; hauteur_carte];

    let position_base_x = generateur_aleatoire.gen_range(0..largeur_carte);
    let position_base_y = generateur_aleatoire.gen_range(0..hauteur_carte);
    carte[position_base_y][position_base_x] = TypeCase::Base;

    for ligne in 0..hauteur_carte {
        for colonne in 0..largeur_carte {
            if carte[ligne][colonne] == TypeCase::Vide {
                let valeur_bruit = generateur_bruit.get([colonne as f64 / 10.0, ligne as f64 / 10.0]);
                carte[ligne][colonne] = match valeur_bruit {
                    v if v < -0.34 => TypeCase::Mur,
                    v if v < -0.2 => TypeCase::Mineral,
                    _ => TypeCase::Vide,
                };
            }
        }
    }
    carte
}