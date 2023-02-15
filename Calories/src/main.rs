// This is a Calories program that calculates the number of calories burned
// after 10, 15, 20, 25, and 30 minutes of running on a treadmill.

use std::io;

fn main() {
    println!("Please enter your weight in kilograms:");
    let mut weight = String::new();

    io::stdin()
        .read_line(&mut weight)
        .expect("Failed to read weight.");

    let weight: f64 = weight.trim().parse().expect("Invalid weight value.");

    println!("Please enter the time in minutes:");
    let mut time = String::new();

    io::stdin()
        .read_line(&mut time)
        .expect("Failed to read time.");

    let time: f64 = time.trim().parse().expect("Invalid time value.");

    let calories_burned = 0.0175 * weight * time;

    println!("Calories burned: {}", calories_burned);
}
