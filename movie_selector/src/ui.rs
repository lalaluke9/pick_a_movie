// use crate::add_movie::add_new_movie;
use crate::movie::{filter_and_pick_movie, Movie};
use crossterm::{
    execute,
    event::{self, Event, KeyCode, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Terminal,
};
use ratatui::{
    layout::Alignment,
    text::{Span, Text},
};
use std::fs;
use std::io::{self, stdout};
use std::error::Error;


fn clear_terminal() {
    // let mut stdout = stdout();
    // execute!(stdout, Clear(ClearType::All)).unwrap();
    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        crossterm::cursor::MoveTo(0, 0)
    )
    .unwrap();
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn load_header(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|_| "V + L Movie Selector".to_string())
}

pub fn run_ui(movies: &mut Vec<Movie>, json_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Clear terminal
    clear_terminal();

    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = setup_terminal()?;
    // let mut terminal = Terminal::new(backend)?;

    // Load the ASCII art header
    let header_text = load_header("Data/header_two.txt");

    let categories = vec![
        "comedy", "drama", "sci-fi", "thriller", "romance", "adventure", "show", "anime", "cartoon",
        "horror", "action", "fantasy", "documentary", "musical", "mystery", "western", "war",
        "crime", "biography", "nick cage",
    ];

    let mut list_state = ListState::default();
    list_state.select(Some(0)); // Start at the first category
    let mut selected_movie: Option<Movie> = None; // State to hold the selected movie

    loop {
        // Prepare data for rendering
        let immutable_movies = movies.clone(); // Clone to avoid borrow conflicts
        let total_movies = immutable_movies.len();
        let display_movie = selected_movie.clone();

        terminal.draw(|f| {
            // clear_terminal();

            let size = f.area();

            // Divide the layout
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(8),      // Header
                    Constraint::Length(3),      // Total movies count
                    Constraint::Percentage(40), // Categories list
                    Constraint::Percentage(50), // Selected movie details
                ])
                .split(size);

            // Add a colorful header
            let header = Paragraph::new(header_text.clone())
                .block(Block::default().borders(Borders::ALL).title(""))
                .style(
                    Style::default()
                        .fg(Color::White)
                        .bg(Color::Magenta) // Background color
                        .add_modifier(Modifier::BOLD),
                )
                .alignment(Alignment::Center);
            f.render_widget(header, chunks[0]);

            // Display total movies count
            let total_paragraph = Paragraph::new(format!("Total Movies: {}", total_movies))
                .block(Block::default().borders(Borders::ALL).title(""))
                .style(Style::default().fg(Color::Cyan))
                .alignment(Alignment::Center);
            f.render_widget(total_paragraph, chunks[1]);

            // Display categories with scrolling and highlight
            let items: Vec<ListItem> = categories
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    let style = if Some(i) == list_state.selected() {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Green)
                    };
                    ListItem::new(*c).style(style)
                })
                .collect();

            let list =
                List::new(items).block(Block::default().borders(Borders::ALL).title("Categories"));
            f.render_stateful_widget(list, chunks[2], &mut list_state);

            // Result Section
            let result_paragraph = if let Some(movie) = &display_movie {
                let result_text = format!(
                    "Title: {}\nWatched: {}\nRating: {}\nTags: {}\nCategories: {}",
                    movie.title,
                    movie.watched.unwrap_or(false).then(|| "Yes").unwrap_or("No"),
                    movie.rating.clone().unwrap_or_else(|| "n/a".to_string()),
                    movie.tags.clone().unwrap_or_else(Vec::new).join(", "),
                    movie.categories.join(", ")
                );

                Paragraph::new(result_text)
                    .block(Block::default().borders(Borders::ALL).title("Result"))
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true })
                    .style(Style::default().fg(Color::Cyan))
            } else {
                Paragraph::new("Press 'r' to select a random movie or 'a' to add a new one.")
                    .block(Block::default().borders(Borders::ALL).title("Result"))
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(Color::White))
            };
            f.render_widget(result_paragraph, chunks[3]);
        })?;

        // Handle user input
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up => {
                    if let Some(selected) = list_state.selected() {
                        if selected > 0 {
                            list_state.select(Some(selected - 1));
                        }
                    }
                }
                KeyCode::Down => {
                    if let Some(selected) = list_state.selected() {
                        if selected < categories.len() - 1 {
                            list_state.select(Some(selected + 1));
                        }
                    }
                }
                KeyCode::Char('r') => {
                    if let Some(selected_index) = list_state.selected() {
                        let selected_category = categories[selected_index];
                        selected_movie = filter_and_pick_movie(&immutable_movies, selected_category)
                            .cloned();
                    }
                }
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    break;
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}