#[derive(Debug)]
pub struct Base {
    pub ressources: Ressources,
    pub position: Position,
}

#[derive(Debug)]
pub struct Ressources {
    pub energy: usize,
    pub mineral: usize,
    pub science: usize,
}

#[derive(Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Base {
    pub fn init(largeur: usize, hauteur: usize, pos_x: usize, pos_y: usize) -> Self {
        Base {
            ressources: Ressources {
                energy: 0,
                mineral: 0,
                science: 0,
            },
            position: Position {
                x: pos_x,
                y: pos_y,
            },
        }
    }
}