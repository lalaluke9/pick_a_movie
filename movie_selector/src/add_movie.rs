// use std::io::{self, Write};
// use crate::movie::Movie;
// use crate::movie::save_movies_to_json;
// use std::fs;


// pub fn add_new_movie(movies: &mut Vec<Movie>, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let mut title = String::new();
//     let mut categories = String::new();
//     let mut rating = String::new();
//     let mut tags = String::new();
    
//     println!("Enter the movie title:");
//     io::stdout().flush()?;
//     io::stdin().read_line(&mut title)?;
//     title = title.trim().to_string();

//     println!("Enter categories (comma-separated):");
//     io::stdout().flush()?;
//     io::stdin().read_line(&mut categories)?;
//     let categories: Vec<String> = categories
//         .trim()
//         .split(',')
//         .map(|s| s.trim().to_string())
//         .collect();

//     println!("Enter a rating (or 'n/a'):");
//     io::stdout().flush()?;
//     io::stdin().read_line(&mut rating)?;
//     let rating = rating.trim().to_string();

//     println!("Enter tags (comma-separated):");
//     io::stdout().flush()?;
//     io::stdin().read_line(&mut tags)?;
//     let tags: Vec<String> = tags
//         .trim()
//         .split(',')
//         .map(|s| s.trim().to_string())
//         .collect();

//     // Add the new movie to the list
//     let new_movie = Movie {
//         title,
//         categories,
//         rating: if rating == "n/a" { None } else { Some(rating) },
//         watched: Some(false),
//         tags: if tags.is_empty() { None } else { Some(tags) },
//     };

//     // let new_movie = Movie {
//     //     title,
//     //     categories: categories.split(',').map(|s| s.trim().to_string()).collect(),
//     //     rating: Some(rating),
//     //     tags: Some(tags.split(',').map(|s| s.trim().to_string()).collect()),
//     //     watched: Some(false),
//     // };

//     movies.push(new_movie);

//     // Save updated movies to JSON
//     let updated_json = serde_json::to_string_pretty(&movies)?;
//     fs::write(file_path, updated_json)?;
//     // save_movies_to_json(file_path, movies)?;
//     // println!("Movie added successfully!");

//     Ok(())
// }