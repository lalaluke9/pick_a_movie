mod movie;
mod ui;
mod add_movie;

use ui::run_ui;
use movie::load_movies_from_json;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // load movies from JSON
//     let movies = load_movies_from_json("Data/movies.json");
//      // Run the UI
//     run_ui(movies)?;
//     Ok(())
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_file = "Data/movies.json";
    let mut movies = load_movies_from_json(json_file);
    run_ui(&mut movies, json_file)?;
    Ok(())
}