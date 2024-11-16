use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
};
use std::fs;

fn load_header(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|_| "V + L Movie Selector".to_string())
}

pub fn create_header() -> Paragraph<'static> {
    // Load the ASCII art header
    let header_text = load_header("Data/header_three.txt");

    Paragraph::new(header_text.to_string())
        .block(
            Block::default().borders(Borders::ALL).title(Span::styled(
                "V + L Movie Selector",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
        )
        .style(Style::default().bg(Color::Magenta))
        .alignment(Alignment::Center)
}