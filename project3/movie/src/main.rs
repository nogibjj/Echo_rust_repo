use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use csv::ReaderBuilder;

#[derive(Debug)]
struct Movie {
    title: String,
    movie_type: String,
}

fn main() {
    // Read in the CSV file
    let mut movies = vec![];
    let file_path = Path::new("movies.csv");
    let file = File::open(file_path).expect("Error opening file");
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);

    for result in reader.records() {
        let record = result.expect("Error reading CSV record");
        let title = record[0].to_string();
        let movie_type = record[1].to_string();
        let movie = Movie { title, movie_type };
        movies.push(movie);
    }

    // Prompt the user to select a movie type
    let mut movie_type = String::new();
    println!("What type of movie would you like to watch?");
    io::stdin().read_line(&mut movie_type).expect("Error reading line");

    // Filter the movies by type
    let mut filtered_movies: Vec<&Movie> = movies
        .iter()
        .filter(|movie| movie.movie_type.trim() == movie_type.trim())
        .collect();

    // Randomly recommend a movie from the filtered list
    let movie = filtered_movies
        .choose(&mut rand::thread_rng())
        .expect("No movies found for selected type");

    println!("Recommendation: {}", movie.title);
}