use crate::TypeCase;
use rand::Rng;

pub fn placer_elements_aleatoires(
    carte: &mut Vec<Vec<TypeCase>>,
    nombre_elements: usize,
    element: TypeCase,
    largeur_carte: usize,
    hauteur_carte: usize,
    rng: &mut impl Rng,
) {
    let mut elements_places = 0;
    
    while elements_places < nombre_elements {
        let x = rng.gen_range(0..largeur_carte);
        let y = rng.gen_range(0..hauteur_carte);
        
        if carte[y][x] == TypeCase::Vide {
            carte[y][x] = element.clone(); // Place l'élément sur la carte
            elements_places += 1;
        }
    }
}