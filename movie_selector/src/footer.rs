use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub fn create_footer() -> Paragraph<'static> {
    Paragraph::new("Press 'h' for Help | 'q' to Quit")
        .block(Block::default().borders(Borders::ALL).title("Footer"))
        .style(Style::default().fg(Color::White).bg(Color::DarkGray))
        .alignment(Alignment::Center)
}