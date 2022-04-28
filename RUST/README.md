### Installing rust
- use the following commands
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
### Rust should now be installed
- next you need to install the rust compiler
- this can be done with the following command
- note that I am working on fedora linux
$ sudo dnf install rustc
### Things to notice
- all rust source files have a .rs file extension
- an example of this is main.rs or hello_world.rs
## Compiling a rust source file
- to comple a rust file use the rustc compiler 
- the following command is as shown bellow
$ rustc main.rs
## Note 
- Makefiles can also be used in rust
### What is Cargo
- Cargo is RUSTs package manager and can be used to download libaries
- In RUST libaries are also called dependencies
- cargo should come installed with rust, but the following command can be used to install it
$ sudo dnf install cargo
### Cargo commands 
$ cargo new hello_cargo
- the command above is used to create a new project using cargo
- it will create a new directory with the name of the project
- inside of the file is a "Cargo.toml" file and a directory
- the directory has a main.rs file
- TOML def:  TOML (Tom’s Obvious, Minimal Language)
### What is TOML
- toml is cargos configeration format
- the first line [package] is the heading 
- it indicates the follwing statmetns are configurating a package
- under the [dependencies] heading is where you would list your dependencies of libaries, in rust packages of code are called crates
### Building a program using cargo
- inside of the directory created by the command "new cargo name_used"
- run the following command
$ cargo build
- this command creates an executable in target/debug/hello_cargo
### Runing with cargo
- the following command can be used to run the program after cargo build 
$ cargo run
- the command "cargo build --release" can be used for final compilation and will put the execurtable in the target directory 