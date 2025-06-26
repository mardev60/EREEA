use crate::carte::TypeCase;
use crate::robot::{Collecteur, Explorateur, Robot};
use rand::Rng;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Base {
    pub carte_reelle: Arc<Mutex<Vec<Vec<TypeCase>>>>,
    pub known_carte: Arc<Mutex<Vec<Vec<TypeCase>>>>,
    pub robots: Arc<Mutex<Vec<Box<dyn Robot + Send>>>>,
    pub ressources: Ressources,
    pub position: Position,
}

#[derive(Debug)]
pub struct Ressources {
    pub energy: Arc<Mutex<usize>>,
    pub mineral: Arc<Mutex<usize>>,
    pub science: Arc<Mutex<usize>>,
}

#[derive(Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl fmt::Debug for Base {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let energy = self.ressources.energy.lock().unwrap();
        let mineral = self.ressources.mineral.lock().unwrap();
        let science = self.ressources.science.lock().unwrap();
        let robots_count = self.robots.lock().unwrap().len();

        f.debug_struct("Base")
            .field("energy", &*energy)
            .field("mineral", &*mineral)
            .field("science", &*science)
            .field("position", &self.position)
            .field("robots", &format!("{} robots", robots_count))
            .finish()
    }
}

impl Base {
    pub fn new(
        width: usize,
        height: usize,
        pos_x: usize,
        pos_y: usize,
        carte_reelle: Vec<Vec<TypeCase>>,
        known_carte_init: Vec<Vec<TypeCase>>,
    ) -> Arc<Mutex<Self>> {
        let carte_reelle = Arc::new(Mutex::new(carte_reelle));
        let known_carte = Arc::new(Mutex::new(known_carte_init));
        let robots = Arc::new(Mutex::new(Vec::new()));
        let energy = Arc::new(Mutex::new(0));
        let mineral = Arc::new(Mutex::new(0));
        let science = Arc::new(Mutex::new(0));

        let base = Arc::new(Mutex::new(Base {
            carte_reelle: Arc::clone(&carte_reelle),
            known_carte: Arc::clone(&known_carte),
            robots: Arc::clone(&robots),
            ressources: Ressources {
                energy: Arc::clone(&energy),
                mineral: Arc::clone(&mineral),
                science: Arc::clone(&science),
            },
            position: Position { x: pos_x, y: pos_y },
        }));

        if let Ok(mut base_guard) = base.lock() {
            base_guard.add_robot(Box::new(Explorateur::new(
                width,
                height,
                pos_x,
                pos_y,
                Arc::clone(&base),
            )));
            base_guard.add_robot(Box::new(Collecteur::new_with_base(
                pos_x,
                pos_y,
                Arc::clone(&base),
            )));
        }

        base
    }

    pub fn demarrer_thread_base(base: Arc<Mutex<Base>>, map_width: usize, map_height: usize) {
        let base_thread = Arc::clone(&base);
        thread::spawn(move || {
            let mut rng = rand::thread_rng();

            loop {
                if let Ok(base_guard) = base_thread.lock() {
                    let energy_val = *base_guard.ressources.energy.lock().unwrap();
                    let mineral_val = *base_guard.ressources.mineral.lock().unwrap();
                    let science_val = *base_guard.ressources.science.lock().unwrap();
                    let pos_x = base_guard.position.x;
                    let pos_y = base_guard.position.y;

                    if energy_val >= 5 && mineral_val >= 5 && science_val >= 5 {
                        let mut robots = base_guard.robots.lock().unwrap();
                        if rng.gen_range(0..3) == 0 {
                            robots.push(Box::new(Explorateur::new(
                                map_width,
                                map_height,
                                pos_x,
                                pos_y,
                                Arc::clone(&base_thread),
                            )));
                            println!("[BASE] Nouvel explorateur créé !");
                        } else {
                            robots.push(Box::new(Collecteur::new_with_base(
                                pos_x,
                                pos_y,
                                Arc::clone(&base_thread),
                            )));
                            println!("[BASE] Nouveau collecteur créé !");
                        }
                        *base_guard.ressources.energy.lock().unwrap() -= 5;
                        *base_guard.ressources.mineral.lock().unwrap() -= 5;
                        *base_guard.ressources.science.lock().unwrap() -= 5;

                        println!(
                            "[BASE] Ressources restantes: E:{}, M:{}, S:{}",
                            energy_val - 5,
                            mineral_val - 5,
                            science_val - 5
                        );
                    }
                }

                thread::sleep(Duration::from_secs(3));
            }
        });
    }

    pub fn add_robot(&mut self, robot: Box<dyn Robot + Send>) {
        self.robots.lock().unwrap().push(robot);
    }

    pub fn ajouter_ressource(&self, ressource: TypeCase) {
        match ressource {
            TypeCase::Energy => {
                *self.ressources.energy.lock().unwrap() += 1;
                println!(
                    "[BASE] +1 Énergie (Total: {})",
                    *self.ressources.energy.lock().unwrap()
                );
            }
            TypeCase::Mineral => {
                *self.ressources.mineral.lock().unwrap() += 1;
                println!(
                    "[BASE] +1 Minerais (Total: {})",
                    *self.ressources.mineral.lock().unwrap()
                );
            }
            TypeCase::Science => {
                *self.ressources.science.lock().unwrap() += 1;
                println!(
                    "[BASE] +1 Science (Total: {})",
                    *self.ressources.science.lock().unwrap()
                );
            }
            _ => (),
        }
    }

    pub fn mettre_a_jour_carte(&self, x: usize, y: usize, case: TypeCase) {
        let mut carte = self.known_carte.lock().unwrap();
        if x < carte[0].len() && y < carte.len() {
            carte[y][x] = case;
        }
    }

    pub fn get_ressources_string(&self) -> String {
        let energy = *self.ressources.energy.lock().unwrap();
        let mineral = *self.ressources.mineral.lock().unwrap();
        let science = *self.ressources.science.lock().unwrap();
        let robots_count = self.robots.lock().unwrap().len();
        format!(
            "Energy: {} | Mineral: {} | Science: {} | Robots: {}",
            energy, mineral, science, robots_count
        )
    }
}
