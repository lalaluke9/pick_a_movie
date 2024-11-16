// use crate::add_movie::add_new_movie;
use crate::footer::create_footer;
use crate::header::create_header;

use crate::movie::{filter_and_pick_movie, Movie};
use crossterm::{
    event::{self, EnableMouseCapture, DisableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
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
    text::Span,
};
use std::error::Error;
use std::fs;
use std::io::{self};


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
    let mut terminal = setup_terminal()?;
    // let mut terminal = Terminal::new(backend)?;

    let categories = vec![
        "comedy",
        "drama",
        "sci-fi",
        "thriller",
        "romance",
        "adventure",
        "show",
        "anime",
        "cartoon",
        "horror",
        "action",
        "fantasy",
        "documentary",
        "musical",
        "mystery",
        "western",
        "war",
        "crime",
        "biography",
        "nick cage",
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
                    Constraint::Length(12),      // Header
                    Constraint::Length(3),      // Total movies count
                    Constraint::Percentage(40), // Categories list
                    Constraint::Percentage(50), // Selected movie details
                    Constraint::Length(3),      // Footer
                ])
                .split(size);

            // let header = Paragraph::new(header_text.clone())
            //     .block(
            //         Block::default().borders(Borders::ALL).title(Span::styled(
            //             "V + L Movie Selector",
            //             Style::default()
            //                 .fg(Color::Yellow)
            //                 .add_modifier(Modifier::BOLD),
            //         )),
            //     )
            //     .style(Style::default().bg(Color::Magenta))
            //     .alignment(Alignment::Center);
            
            let header = create_header();
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
                            .fg(Color::Black)
                            .bg(Color::Yellow)
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
                    "{:<10} {}\n{:<10} {}\n{:<10} {}\n{:<10} {}\n{:<10} {}",
                    "Title:",
                    movie.title,
                    "Watched:",
                    movie
                        .watched
                        .unwrap_or(false)
                        .then(|| "Yes")
                        .unwrap_or("No"),
                    "Rating:",
                    movie.rating.clone().unwrap_or_else(|| "n/a".to_string()),
                    "Tags:",
                    movie.tags.clone().unwrap_or_else(Vec::new).join(", "),
                    "Categories:",
                    movie.categories.join(", ")
                );

                Paragraph::new(result_text)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Result")
                            .style(Style::default().fg(Color::Blue)), // Card-like border color
                    )
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true })
                    .style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD), // Make the text bold
                    )
            } else {
                Paragraph::new("Press 'r' to select a random movie or 'a' to add a new one.")
                    .block(Block::default().borders(Borders::ALL).title("Result"))
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(Color::White))
            };

            f.render_widget(result_paragraph, chunks[3]);

            // Footer Section
            let footer = create_footer();
            f.render_widget(footer, chunks[4]);
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
                        selected_movie =
                            filter_and_pick_movie(&immutable_movies, selected_category).cloned();
                    }
                }

                KeyCode::Char('g') => {
                    // Pick a random movie globally
                    use rand::seq::SliceRandom;

                    if let Some(movie) = immutable_movies.choose(&mut rand::thread_rng()) {
                        selected_movie = Some(movie.clone()); // Clone to convert &Movie to Movie
                    }
                }

                KeyCode::Char('h') => {
                    let help_text = vec![
                        "Key Bindings:",
                        "  ↑ / ↓ : Navigate categories",
                        "  r     : Random movie in selected category",
                        "  g     : Global random movie",
                        "  a     : Add a new movie",
                        "  q     : Quit",
                        "  h     : Show this help",
                    ]
                    .join("\n");

                    // Temporarily display the help screen
                    terminal.draw(|f| {
                        let size = f.area(); // Get the full screen size
                        let help_block = Paragraph::new(help_text)
                            .block(
                                Block::default()
                                    .borders(Borders::ALL)
                                    .title("Help")
                                    .style(Style::default().fg(Color::Yellow)),
                            )
                            .alignment(Alignment::Left)
                            .wrap(Wrap { trim: true });
                        f.render_widget(help_block, size); // Render help text over the entire terminal
                    })?;

                    // Wait for user to press a key to exit help
                    loop {
                        if let Event::Key(key) = event::read()? {
                            if key.code == KeyCode::Char('h') || key.code == KeyCode::Char('q') {
                                break; // Exit the help screen when 'h' or 'q' is pressed
                            }
                        }
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

    // disable_raw_mode()?;
    // Cleanup terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
