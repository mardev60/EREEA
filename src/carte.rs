use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::placement::placer_elements_aleatoires;

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
    let mut generateur_aleatoire = StdRng::seed_from_u64(seed as u64); // extension du seed u32 en u64 car la lib ne prend pas en charge u32
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

    let ressources = [
        (TypeCase::Energy, "Énergie"),
        (TypeCase::Science, "Science"),
    ];
    
    for (type_case, nom) in ressources.iter() {
        let quantite = generateur_aleatoire.gen_range(10..=20);
        println!("Génération de {} {}...", quantite, nom);
        
        placer_elements_aleatoires(
            &mut carte,
            quantite,
            type_case.clone(),
            largeur_carte,
            hauteur_carte,
            &mut generateur_aleatoire
        );
    }
    
    carte
}

pub fn trouver_position_base(carte: &Vec<Vec<TypeCase>>) -> (usize, usize) {
    for (i, ligne) in carte.iter().enumerate() {
        for (j, case) in ligne.iter().enumerate() {
            if *case == TypeCase::Base {
                return (i, j);
            }
        }
    }
    (0, 0)
}
