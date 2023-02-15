// This is a Calories program that calculates the number of calories burned
// after 10, 15, 20, 25, and 30 minutes of running on a treadmill.

fn main() {
   
    // user input
    let mut minutes = 10;
    let mut calories_burned = 0.0;

    // Display the table header
    println!("Minutes\t\tCalories Burned");
    println!("--------------------------------");

    // Display the number of calories burned after 10, 15, 20, 25, and 30 minutes
    while minutes <= 30 {
        calories_burned = 3.9 * minutes as f64;
        println!("{}\t\t{}", minutes, calories_burned);
        minutes += 5;
    }
}