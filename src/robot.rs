use crate::carte::TypeCase;

pub trait Robot {
    fn get_type(&self) -> TypeCase;
    fn get_position_x(&self) -> usize;
    fn get_position_y(&self) -> usize;
}

pub struct Explorateur {
    position_x: usize,
    position_y: usize,
}

impl Explorateur {
    pub fn new(x: usize, y: usize, largeur_carte: usize, hauteur_carte: usize) -> Self {
        println!(
            "Création explorateur à ({}, {}), carte de taille {}x{} (utile pour valider la position).",
            x, y, largeur_carte, hauteur_carte
        );
        
        Explorateur {
            position_x: x,
            position_y: y,
        }
    }
}

impl Robot for Explorateur {
    fn get_type(&self) -> TypeCase {
        TypeCase::Explorer
    }

    fn get_position_x(&self) -> usize {
        self.position_x
    }

    fn get_position_y(&self) -> usize {
        self.position_y
    }
}

pub struct Collecteur {
    position_x: usize,
    position_y: usize,
}

impl Collecteur {
    pub fn new(x: usize, y: usize) -> Self {
        Collecteur {
            position_x: x,
            position_y: y,
        }
    }
}

impl Robot for Collecteur {
    fn get_type(&self) -> TypeCase {
        TypeCase::Collector
    }

    fn get_position_x(&self) -> usize {
        self.position_x
    }

    fn get_position_y(&self) -> usize {
        self.position_y
    }
}
