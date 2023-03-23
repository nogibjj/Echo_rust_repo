README: catr

catr is a command-line program that concatenates multiple files and prints the result to STDOUT. It also supports prefixing each line with the line number.

## Installation
You can install catr using Cargo, the Rust package manager:

```cargo install catr```

## Usage
css
```catr [FLAGS] [OPTIONS] [FILES]...```

## Flags
-h, --help: Prints help information
-n, --number: Number all output lines
-b, --number-non-blank: Number non-blank output lines
-v, --version: Prints version information
Note: -n and -b are mutually exclusive.

## Options
-o, --output: Output file. If not specified, output is printed to STDOUT.
## Arguments
FILES: List of files to concatenate. If no files are provided, catr reads from STDIN.
## Examples
Concatenate two files and number all lines:

```catr -n file1.txt file2.txt```

Concatenate files and redirect output to a file:

lua
```catr file1.txt file2.txt -o output.txt```

Read from STDIN and number non-blank lines:

swift
```echo -e "Hello\n\nWorld\n" | catr -b```

## Development
catr is organized into a library and a binary crate. Testing-first development is used to ensure code quality. Public and private variables and functions are defined as appropriate.

In addition, the program tests for the existence of a file and creates a random string for a file that does not exist. It also reads regular files or STDIN and uses eprintln! to print to STDERR and format! to format a string.

## License
catr is licensed under the MIT License. See LICENSE for more information.