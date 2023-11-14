# Portfolio Project: First Checki-in 

Donald Whitehead  
CS-339R Fall 2023  

**description:**  

This program is a very simple file manager implimented in Rust and using Slint for the user interface. At the moment it is capable of simple nagivation though the file system and opening files with your system's default application depending on the file type.  

Written in Rust 2021 edition. (v 1.73)  
Tested on MacOS 13.1 (Apple Clang 14.0.0), Fedora Linux (kernel 6.2.0, gcc 13.0.1), Windows 11 (gcc 12.1.0)**   

Assure you have rust installed. In order to do that, follow the instructions for your OS of choice at: https://www.rust-lang.org/tools/install  

Rust also requires a C++ compiler.  
MacOS: $ xcode-select --install  
Linux: install build tools (such as gcc) for your distrobution.  
Windows: Rust recommends installing “Desktop Development with C++” through Visual Studio. I used gcc through MsSys project  

**To compile:**  
  1) clone or download repository 
  2) open your terminal of choice and navigate to the project folder 
  3) This project uses Cargo which comes as a part of rust, compiling is as simple as running 'cargo build --release' 
  4) The binary will be placed at ../target/release/rustfm  

**To run:**  
  Ideal way to run is to use Cargo with the 'cargo run --release' command in the project root, but you can also compile and run the binary.  

  -If launched without a path as an argument, the program will start in the current directory.  
  -If a path is supplied (example: cargo run -- '\home') then the program will start in the specified directory.  

**Usage:**  
  -Navigation is very simple. click once to highlight an directory or file, clicking a highlighted item a second time will either change to that directory or open the file.  
  -You can go back to the parent directory by hitting the back button.  
  -You can navigate to a folder directly if by entering the full path into the navigation bar.

**Bugs**  
  -There are some issues when opening certain directories, such as shortcuts and directories that link elsewhere.  
  -The list tends to shift slightly when highlighting an item, it can be annoying when trying to double click.  
  -Some files may not open if your OS does not have a default application for them.  



