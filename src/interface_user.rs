use crate::carte::TypeCase;
use crate::robot::Robot;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;
use std::sync::{Arc, Mutex};

pub fn afficher_interface_jeu(
    carte_jeu: &Vec<Vec<TypeCase>>,
    stats_ressources: &str,
    robots: &Arc<Mutex<Vec<Box<dyn Robot + Send>>>>,
) -> Result<(), io::Error> {
    let sortie_standard = io::stdout();
    let backend_terminal = CrosstermBackend::new(sortie_standard);
    let mut terminal = Terminal::new(backend_terminal)?;

    terminal.draw(|frame| {
        let dimensions = frame.area();
        let hauteur_carte = dimensions.height.saturating_sub(5);
        
        let zones = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(hauteur_carte)
            ].as_ref())
            .split(dimensions);

        let widget_ressources = Paragraph::new(stats_ressources)
            .block(Block::default().borders(Borders::ALL).title("Statistiques"))
            .style(
                Style::default()
                    .fg(Color::Rgb(208, 191, 154))
                    .bg(Color::Rgb(27, 27, 34)),
            );

        frame.render_widget(widget_ressources, zones[0]);

        let mut carte_affichage = carte_jeu.clone();

        if let Ok(robots_guard) = robots.lock() {
            for robot in robots_guard.iter() {
                let x = robot.get_position_x();
                let y = robot.get_position_y();
                if y < carte_affichage.len() && x < carte_affichage[0].len() {
                    carte_affichage[y][x] = robot.get_type();
                }
            }
        }

        let mut representation_carte = String::new();
        for ligne in carte_affichage.iter() {
            for case in ligne {
                let symbole = match case {
                    TypeCase::Vide => "  ",
                    TypeCase::Base => "🏠",
                    TypeCase::Mur => "🪨",
                    TypeCase::Energy => "⚡",
                    TypeCase::Mineral => "💎",
                    TypeCase::Science => "🔬",
                    TypeCase::Explorer => "🛸",
                    TypeCase::Collector => "🤖",
                    TypeCase::Inconnu => "▒▒",
                };
                representation_carte.push_str(symbole);
            }
            representation_carte.push('\n');
        }

        let widget_carte = Paragraph::new(representation_carte)
            .block(Block::default().borders(Borders::ALL).title("Territoire"))
            .style(
                Style::default()
                    .fg(Color::Rgb(208, 191, 154))
                    .bg(Color::Rgb(27, 27, 34)),
            );

        frame.render_widget(widget_carte, zones[1]);
    })?;

    Ok(())
}