mod movie;
mod ui;
mod add_movie;
mod footer;
mod header;

use ui::run_ui;
use movie::load_movies_from_json;
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use std::io;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let json_file = "Data/movies.json";
//     let mut movies = load_movies_from_json(json_file);
//     run_ui(&mut movies, json_file)?;
//     Ok(())
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the movies from the JSON file
    let json_file = "Data/movies.json";
    let mut movies = load_movies_from_json(json_file);

    // Enable raw mode and run the TUI
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;

    // Run the TUI and ensure cleanup in case of an error or normal exit
    let result = run_ui(&mut movies, json_file);

    // Always cleanup the terminal
    disable_raw_mode()?;
    execute!(stdout, crossterm::terminal::LeaveAlternateScreen)?;
    if let Err(err) = result {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}