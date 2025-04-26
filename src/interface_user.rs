use crate::carte::TypeCase;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;

pub fn afficher_interface_jeu(carte_jeu: &Vec<Vec<TypeCase>>, stats_ressources: &str) -> Result<(), io::Error> {
    let sortie_standard = io::stdout();
    let backend_terminal = CrosstermBackend::new(sortie_standard);
    let mut terminal = Terminal::new(backend_terminal)?;

    terminal.draw(|frame| {
        let dimensions = frame.area();
        let hauteur_carte = dimensions.height.saturating_sub(5);
        
        let zones = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Zone des ressources
                Constraint::Length(hauteur_carte)  // Zone de la carte
            ].as_ref())
            .split(dimensions);

        let widget_ressources = Paragraph::new(stats_ressources)
            .block(Block::default().borders(Borders::ALL).title("Statistiques"))
            .style(
                Style::default()
                    .fg(Color::Rgb(180, 200, 220))  // Bleu clair
                    .bg(Color::Rgb(20, 20, 30)),    // Bleu foncé
            );

        frame.render_widget(widget_ressources, zones[0]);

        let mut representation_carte = String::new();
        for ligne in carte_jeu {
            for case in ligne {
                let symbole = match case {
                    TypeCase::Vide => ' ',
                    TypeCase::Base => 'H',
                    TypeCase::Mur => 'O',
                    TypeCase::Energy => 'E',
                    TypeCase::Mineral => 'M',
                    TypeCase::Science => 'S',
                    TypeCase::Explorer => 'X',
                    TypeCase::Collector => 'C',
                };
                representation_carte.push(symbole);
            }
            representation_carte.push('\n');
        }

        let widget_carte = Paragraph::new(representation_carte)
            .block(Block::default().borders(Borders::ALL).title("Territoire"))
            .style(
                Style::default()
                    .fg(Color::Rgb(180, 200, 220))  // Bleu clair
                    .bg(Color::Rgb(20, 20, 30)),    // Bleu foncé
            );

        frame.render_widget(widget_carte, zones[1]);
    })?;

    Ok(())
}