// src/movie.rs
use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;
use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Movie {
    pub title: String,
    pub categories: Vec<String>,
    pub watched: Option<bool>, // Use Option if not all movies are watched yet
    pub rating: Option<String>, // Use Option for movies without ratings
    pub tags: Option<Vec<String>>, // Use Option for movies without tags
}

pub fn load_movies_from_json(file_path: &str) -> Vec<Movie> {
    let file_content = fs::read_to_string(file_path).expect("Failed to read JSON file");
    serde_json::from_str(&file_content).expect("Failed to parse JSON")
}

pub fn save_movies_to_json(file_path: &str, movies: &Vec<Movie>) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(movies)?;
    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn filter_and_pick_movie<'a>(movies: &'a [Movie], category: &str) -> Option<&'a Movie> {
    let filtered: Vec<&Movie> = movies
        .iter()
        .filter(|movie| movie.categories.contains(&category.to_string()))
        .collect();

    filtered.choose(&mut rand::thread_rng()).copied()
}
