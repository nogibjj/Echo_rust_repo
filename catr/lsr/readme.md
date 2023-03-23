README: lsr

lsr is a Rust clone of the Unix ls command, which lists the contents of directories or lists of files along with their permissions, sizes, and modification times. It supports a subset of the options available in the original program.

## Installation
You can install lsr using Cargo, the Rust package manager:

```
cargo install lsr
```
Usage
css
```
lsr [OPTIONS] [FILES]...
```
## Options
-h, --help: Prints help information
-l: Use a long listing format
-a, --all: Do not ignore entries starting with .
-r, --reverse: Reverse order while sorting
-s, --size: Print the allocated size of each file, in blocks
-t: Sort by modification time, newest first
## Arguments
FILES: List of files or directories to list. If no files or directories are provided, lsr lists the contents of the current directory.
## Examples
List files and directories in the current directory:

```
lsr
```
List files and directories in a specific directory:

bash
```
lsr /path/to/directory
```
List all files, including hidden files:

css
```
lsr -a
```
List files and directories sorted by modification time:


```
lsr -t
```
## Development
lsr relies on Unix-specific file and ownership concepts and will not work on Windows. It uses a text table to create aligned columns of output and includes documentation comments to help users understand how to use the program.

The code is organized into modules in separate files to help keep it organized, and a custom type is used to query and visually represent a file's permissions. An implementation is used to add a method to this custom type.