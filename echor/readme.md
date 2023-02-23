## Rust implementation of the Unix echo command.

Usage
```

echor [OPTIONS] <TEXT>...
    
```
Arguments
- TEXT: Input text. Must have at least one value.
Options
- -n, --omit_newline: Do not print newline.
Example
```
$ echor hello world
hello world
$ echor -n hello world
hello world$
```
Build and Install
1. Install Rust from https://www.rust-lang.org/tools/install
2. Clone this repository: git clone https://github.com/username/echor.git
3. Build the binary: cargo build --release
4. Install the binary: sudo cp target/release/echor /usr/local/bin/echor
License
This code is released under the MIT License. See LICENSE for details.