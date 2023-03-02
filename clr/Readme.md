# Rascalry

## How cal works

The cal command is a command-line utility that displays a calendar for a given month and year. It is a part of the GNU coreutils package, which is a collection of basic command-line utilities for Unix-like operating systems.

The cal command is used to display a calendar for a given month and year. It is a part of the GNU coreutils package, which is a collection of basic command-line utilities for Unix-like operating systems.

## How to use cal


The following code is written in Rust programming language and its purpose is to execute a program that performs some calculations based on the arguments passed to it via the command line.

Function Description:
The main() function is the entry point of the program, which takes no arguments and returns nothing. It is responsible for handling the control flow of the program.

Code Logic:
The code first calls the get_args() function from the calr module, which returns a Result type with a value of Ok(args) if the command-line arguments were successfully parsed or Err(e) if an error occurred during parsing.

The and_then() method is then called on the Result value returned by get_args() function to perform an operation on the parsed arguments, which is the run() function from the calr module. If the result of the and_then() method is an Err(e) value, it means that an error occurred during the execution of the program, so the code then prints the error message to the standard error stream by calling eprintln!("{}", e) and exits the program with an exit code of 1 by calling std::process::exit(1).

Parameters:
None

Return:
None, the function just executes the program and exits or prints an error message and exits.

Example:
rust
```
fn main() {
    if let Err(e) = calr::get_args().and_then(calr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
```
In this example, the main() function calls the get_args() function from the calr module to parse the command-line arguments, then calls the run() function to execute the program based on the parsed arguments. If an error occurs during the execution, the error message is printed to the standard error stream and the program exits with an exit code of 1.

## Test 
```
#!/usr/bin/env bash

OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

cal 2020 > $OUTDIR/2020.txt
cal 2 2020 > $OUTDIR/2-2020.txt
cal 4 2020 > $OUTDIR/4-2020.txt
cal 5 2020 > $OUTDIR/5-2020.txt
```