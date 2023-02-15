# Calories 
This is a Calories program that calculates the number of calories burned

Explanation:

- The io module is imported to allow input and output operations.
- The main function is defined, which is the entry point of the program.
- The println! macro is used to prompt the user to input their weight in kilograms.
- The String::new() function creates a new, empty string to store the weight input.
- The read_line() method of the io module is called to read the weight input from the user. The input is stored in the weight string.
- The trim() method is used to remove any leading or trailing whitespace from the input string.
- The parse() method is called to convert the input string to a floating-point number (f64).
- The weight value is assigned to the weight variable.
- Steps 3-8 are repeated to prompt the user for the time input, which is also stored as an f64.
- The formula for calculating the calories burned is applied to the weight and time values to get the calories_burned variable.
- The println! macro is used to output the calories_burned value to the console.
