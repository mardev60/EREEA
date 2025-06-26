use crate::base::Base;
use crate::carte::TypeCase;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct RobotState {
    pub position_x: usize,
    pub position_y: usize,
    pub at_base: bool,
    pub carrying_resource: Option<TypeCase>,
}

impl RobotState {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            position_x: x,
            position_y: y,
            at_base: true,
            carrying_resource: None,
        }
    }
}

pub trait Robot: Send {
    /// Détermine le prochain mouvement du robot
    fn next_move(&self);
    /// Retourne le type du robot (Explorateur ou Collecteur)
    fn get_type(&self) -> TypeCase;
    /// Retourne la position X actuelle du robot
    fn get_position_x(&self) -> Result<usize, String>;
    /// Retourne la position Y actuelle du robot
    fn get_position_y(&self) -> Result<usize, String>;
    /// Indique si le robot est à la base
    fn is_at_base(&self) -> Result<bool, String>;
}

pub struct Explorateur {
    state: Arc<Mutex<RobotState>>,
}

impl Explorateur {
    /// Crée un nouvel explorateur et lance son thread d'exploration autonome
    pub fn new(
        map_width: usize,
        map_height: usize,
        x: usize,
        y: usize,
        base_ref: Arc<Mutex<Base>>,
    ) -> Self {
        let explorateur = Explorateur {
            state: Arc::new(Mutex::new(RobotState::new(x, y))),
        };

        let state = Arc::clone(&explorateur.state);
        let base = Arc::clone(&base_ref);

        thread::spawn(move || {
            println!("[EXPLORATEUR] Démarrage de l'exploration...");

            loop {
                let (x, y) = match state.lock() {
                    Ok(robot_state) => (robot_state.position_x, robot_state.position_y),
                    Err(_) => {
                        println!("[EXPLORATEUR] Erreur d'accès à l'état du robot");
                        thread::sleep(Duration::from_millis(150));
                        continue;
                    }
                };
                let mut rng = rand::thread_rng();
                let direction = rng.gen_range(0..4);

                let new_x;
                let new_y;
                match direction {
                    0 => {
                        new_x = x;
                        new_y = if y > 0 { y - 1 } else { y };
                    }
                    1 => {
                        new_x = x;
                        new_y = if y < map_height - 1 { y + 1 } else { y };
                    }
                    2 => {
                        new_x = if x > 0 { x - 1 } else { x };
                        new_y = y;
                    }
                    _ => {
                        new_x = if x < map_width - 1 { x + 1 } else { x };
                        new_y = y;
                    }
                }

                let can_move = if let Ok(base_guard) = base.try_lock() {
                    if let Ok(known_carte) = base_guard.known_carte.try_lock() {
                        !matches!(known_carte[new_y][new_x], TypeCase::Mur | TypeCase::Inconnu)
                    } else {
                        false
                    }
                } else {
                    false
                };

                if can_move {
                    if let Ok(mut robot_state) = state.lock() {
                        robot_state.position_x = new_x;
                        robot_state.position_y = new_y;
                    }
                }

                let (current_x, current_y) = match state.lock() {
                    Ok(robot_state) => (robot_state.position_x, robot_state.position_y),
                    Err(_) => {
                        thread::sleep(Duration::from_millis(150));
                        continue;
                    }
                };

                if let Ok(base_guard) = base.try_lock() {
                    let carte_data: Vec<(usize, usize, TypeCase)> =
                        if let Ok(carte_reelle) = base_guard.carte_reelle.try_lock() {
                            let mut data = Vec::new();
                            for dy in -2..=2 {
                                for dx in -2..=2 {
                                    let new_x = current_x as i32 + dx;
                                    let new_y = current_y as i32 + dy;

                                    if (dx.abs() + dy.abs()) <= 2
                                        && new_x >= 0
                                        && new_y >= 0
                                        && new_x < map_width as i32
                                        && new_y < map_height as i32
                                    {
                                        let new_x = new_x as usize;
                                        let new_y = new_y as usize;

                                        if let Some(case_type) =
                                            carte_reelle.get(new_y).and_then(|row| row.get(new_x))
                                        {
                                            data.push((new_x, new_y, case_type.clone()));
                                        }
                                    }
                                }
                            }
                            data
                        } else {
                            Vec::new()
                        };

                    drop(base_guard);

                    if let Ok(base_guard) = base.try_lock() {
                        for (x, y, case_type) in carte_data {
                            base_guard.mettre_a_jour_carte(x, y, case_type);
                        }
                    }
                }

                thread::sleep(Duration::from_millis(150));
            }
        });

        explorateur
    }
}

impl Robot for Explorateur {
    /// Implémentation vide car le mouvement est géré dans le thread
    fn next_move(&self) {}

    /// Retourne le type Explorateur
    fn get_type(&self) -> TypeCase {
        TypeCase::Explorer
    }

    /// Retourne la position X actuelle de l'explorateur
    fn get_position_x(&self) -> Result<usize, String> {
        self.state
            .lock()
            .map(|state| state.position_x)
            .map_err(|_| "Failed to lock robot state".to_string())
    }

    /// Retourne la position Y actuelle de l'explorateur
    fn get_position_y(&self) -> Result<usize, String> {
        self.state
            .lock()
            .map(|state| state.position_y)
            .map_err(|_| "Failed to lock robot state".to_string())
    }

    /// Indique si l'explorateur est à la base
    fn is_at_base(&self) -> Result<bool, String> {
        self.state
            .lock()
            .map(|state| state.at_base)
            .map_err(|_| "Failed to lock robot state".to_string())
    }
}

pub struct Collecteur {
    state: Arc<Mutex<RobotState>>,
    base_x: usize,
    base_y: usize,
}

impl Collecteur {
    /// Crée un nouveau collecteur sans thread (version simple)
    pub fn new(x: usize, y: usize) -> Self {
        Collecteur {
            state: Arc::new(Mutex::new(RobotState::new(x, y))),
            base_x: x,
            base_y: y,
        }
    }

    /// Crée un nouveau collecteur et lance son thread de collecte autonome
    pub fn new_with_base(x: usize, y: usize, base_ref: Arc<Mutex<Base>>) -> Self {
        let collecteur = Collecteur {
            state: Arc::new(Mutex::new(RobotState::new(x, y))),
            base_x: x,
            base_y: y,
        };

        let robot_state = Arc::clone(&collecteur.state);
        let base = Arc::clone(&base_ref);

        thread::spawn(move || {
            println!("[COLLECTEUR] Démarrage de la collecte...");
            let mut rng = rand::thread_rng();

            loop {
                let (x, y, is_at_base, carrying_res) = match robot_state.lock() {
                    Ok(state) => (
                        state.position_x,
                        state.position_y,
                        state.at_base,
                        state.carrying_resource.clone(),
                    ),
                    Err(_) => {
                        println!("[COLLECTEUR] Erreur d'accès à l'état du robot");
                        thread::sleep(Duration::from_millis(300));
                        continue;
                    }
                };

                let base_access_result = base.try_lock();

                if let Ok(mut base_guard) = base_access_result {
                    if carrying_res.is_some() && is_at_base {
                        if let Some(resource) = carrying_res {
                            base_guard.ajouter_ressource(resource);
                            if let Ok(mut state) = robot_state.lock() {
                                state.carrying_resource = None;
                            }
                            println!("[COLLECTEUR] Ressource déposée à la base");
                        }
                    } else if carrying_res.is_none() {
                        let mut resource_pos: Option<(usize, usize, TypeCase)> = None;

                        if let Ok(known_carte) = base_guard.known_carte.try_lock() {
                            for dy in -2..=2 {
                                for dx in -2..=2 {
                                    let new_x = x as i32 + dx;
                                    let new_y = y as i32 + dy;

                                    if new_x >= 0
                                        && new_y >= 0
                                        && new_x < known_carte[0].len() as i32
                                        && new_y < known_carte.len() as i32
                                    {
                                        let new_x = new_x as usize;
                                        let new_y = new_y as usize;

                                        match known_carte[new_y][new_x] {
                                            TypeCase::Energy
                                            | TypeCase::Mineral
                                            | TypeCase::Science => {
                                                resource_pos = Some((
                                                    new_x,
                                                    new_y,
                                                    known_carte[new_y][new_x].clone(),
                                                ));
                                                break;
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                if resource_pos.is_some() {
                                    break;
                                }
                            }
                        }

                        if let Some((res_x, res_y, resource_type)) = resource_pos {
                            if let Ok(mut state) = robot_state.lock() {
                                state.position_x = res_x;
                                state.position_y = res_y;
                                state.carrying_resource = Some(resource_type);
                                state.at_base = false;
                            }

                            drop(base_guard);

                            if let Ok(base_guard) = base.try_lock() {
                                if let Ok(mut carte_reelle) = base_guard.carte_reelle.try_lock() {
                                    carte_reelle[res_y][res_x] = TypeCase::Vide;
                                }
                                if let Ok(mut known_carte_mut) = base_guard.known_carte.try_lock() {
                                    known_carte_mut[res_y][res_x] = TypeCase::Vide;
                                }
                            }

                            println!(
                                "[COLLECTEUR] Ressource collectée en ({}, {}) - Retour à la base",
                                res_x, res_y
                            );
                        } else if let Ok(known_carte) = base_guard.known_carte.try_lock() {
                            let direction = rng.gen_range(0..4);
                            let new_x;
                            let new_y;

                            match direction {
                                0 => {
                                    new_x = x;
                                    new_y = if y > 0 { y - 1 } else { y };
                                }
                                1 => {
                                    new_x = x;
                                    new_y = if y < known_carte.len() - 1 { y + 1 } else { y };
                                }
                                2 => {
                                    new_x = if x > 0 { x - 1 } else { x };
                                    new_y = y;
                                }
                                _ => {
                                    new_x = if x < known_carte[0].len() - 1 {
                                        x + 1
                                    } else {
                                        x
                                    };
                                    new_y = y;
                                }
                            }

                            let can_move = !matches!(
                                known_carte[new_y][new_x],
                                TypeCase::Mur | TypeCase::Inconnu
                            );

                            if can_move {
                                if let Ok(mut state) = robot_state.lock() {
                                    state.position_x = new_x;
                                    state.position_y = new_y;
                                }
                            }
                        }
                    } else if carrying_res.is_some() {
                        let base_x = base_guard.position.x;
                        let base_y = base_guard.position.y;

                        println!(
                            "[COLLECTEUR] Retour à la base: position actuelle ({}, {}), base en ({}, {})",
                            x, y, base_x, base_y
                        );

                        let mut new_x = x;
                        let mut new_y = y;

                        if x < base_x {
                            new_x = x + 1;
                        } else if x > base_x {
                            new_x = x - 1;
                        } else if y < base_y {
                            new_y = y + 1;
                        } else if y > base_y {
                            new_y = y - 1;
                        }

                        if let Ok(known_carte) = base_guard.known_carte.try_lock() {
                            let can_move =
                                if new_x < known_carte[0].len() && new_y < known_carte.len() {
                                    !matches!(
                                        known_carte[new_y][new_x],
                                        TypeCase::Mur | TypeCase::Inconnu
                                    )
                                } else {
                                    false
                                };

                            if can_move {
                                if let Ok(mut state) = robot_state.lock() {
                                    state.position_x = new_x;
                                    state.position_y = new_y;
                                    if new_x == base_x && new_y == base_y {
                                        state.at_base = true;
                                        println!("[COLLECTEUR] Arrivé à la base !");
                                    }
                                }
                                println!("[COLLECTEUR] Déplacement vers ({}, {})", new_x, new_y);
                            } else {
                                println!("[COLLECTEUR] Chemin bloqué, tentative de contournement");
                                let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];
                                for (dx, dy) in directions.iter() {
                                    let alt_x_i32 = x as i32 + dx;
                                    let alt_y_i32 = y as i32 + dy;

                                    if alt_x_i32 >= 0
                                        && alt_y_i32 >= 0
                                        && alt_x_i32 < known_carte[0].len() as i32
                                        && alt_y_i32 < known_carte.len() as i32
                                    {
                                        let alt_x = alt_x_i32 as usize;
                                        let alt_y = alt_y_i32 as usize;

                                        match known_carte[alt_y][alt_x] {
                                            TypeCase::Mur | TypeCase::Inconnu => continue,
                                            _ => {
                                                if let Ok(mut state) = robot_state.lock() {
                                                    state.position_x = alt_x;
                                                    state.position_y = alt_y;
                                                }
                                                println!(
                                                    "[COLLECTEUR] Contournement vers ({}, {})",
                                                    alt_x, alt_y
                                                );
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    println!("[COLLECTEUR] Impossible d'accéder à la base, attente...");
                }

                thread::sleep(Duration::from_millis(300));
            }
        });

        collecteur
    }
}

impl Robot for Collecteur {
    /// Implémentation vide car le mouvement est géré dans le thread
    fn next_move(&self) {}

    /// Retourne le type Collecteur
    fn get_type(&self) -> TypeCase {
        TypeCase::Collector
    }

    /// Retourne la position X actuelle du collecteur
    fn get_position_x(&self) -> Result<usize, String> {
        self.state
            .lock()
            .map(|state| state.position_x)
            .map_err(|_| "Failed to lock robot state".to_string())
    }

    /// Retourne la position Y actuelle du collecteur
    fn get_position_y(&self) -> Result<usize, String> {
        self.state
            .lock()
            .map(|state| state.position_y)
            .map_err(|_| "Failed to lock robot state".to_string())
    }

    /// Indique si le collecteur est à la base
    fn is_at_base(&self) -> Result<bool, String> {
        self.state
            .lock()
            .map(|state| state.at_base)
            .map_err(|_| "Failed to lock robot state".to_string())
    }
}
