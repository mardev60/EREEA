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
    pub fn new(x: usize, y: usize) -> Self {
        Explorateur {
            position_x: x,
            position_y: y,
        }
    }
}

impl Robot for Explorateur {
    fn get_type(&self) -> TypeCase {
        TypeCase::Explorateur
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
        TypeCase::Collecteur
    }

    fn get_position_x(&self) -> usize {
        self.position_x
    }

    fn get_position_y(&self) -> usize {
        self.position_y
    }
}
