use crate::robot::{Robot, Collecteur, Explorateur};
use std::fmt;

pub struct Base {
    pub ressources: Ressources,
    pub position: Position,
    pub robots: Vec<Box<dyn Robot>>,
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

impl fmt::Debug for Base {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Base")
            .field("ressources", &self.ressources)
            .field("position", &self.position)
            .field("robots", &format!("{} robots", self.robots.len()))
            .finish()
    }
}

impl Base {
    pub fn init(largeur: usize, hauteur: usize, pos_x: usize, pos_y: usize) -> Self {
        let mut base = Base {
            ressources: Ressources {
                energy: 0,
                mineral: 0,
                science: 0,
            },
            position: Position {
                x: pos_x,
                y: pos_y,
            },
            robots: Vec::new(),
        };
        
        // création et ajout d'un robot collecteur
        let collecteur = Box::new(Collecteur::new(pos_x, pos_y));
        base.add_robot(collecteur);

        // création et ajout d'un robot explorateur
        let explorateur = Box::new(Explorateur::new(pos_x, pos_y));
        base.add_robot(explorateur);

        base
    }

    pub fn add_robot(&mut self, robot: Box<dyn Robot>) {
        self.robots.push(robot);
    }
}