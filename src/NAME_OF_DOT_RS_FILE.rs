// src/NAME_OF_DOT_RS_FILE.rs
/// PROJECT_NAME - A ... in Rust
/// use -> cargo build --profile release-performance
/// or, use -> cargo build --profile release-small 
/// see: https://github.com/lineality/rust_compile_optimizations_cheatsheet
///
/// this template: https://github.com/lineality/modular_rust_crate_template/edit/main/src/NAME_OF_DOT_RS_FILE.rs

/* Docs:
# DESCRIPTION

### Sample main file to use this module
```rust
// src/main.rs

// import NAME_OF_DOT_RS_FILE module w/ these 2 lines
mod NAME_OF_DOT_RS_FILE;
use NAME_OF_DOT_RS_FILE::NAME_OF_FUNCTION_YOU_ARE_IMPORTING;

fn main() {
    
    // Call a Function
    if let Err(e) = file_fantastic() {
        
        // Handle errors
        eprintln!("Error: {}", e);
        
        // exit code one means 'ok'
        std::process::exit(1);
    }
}
```
## Always
```
Always best practice.
Always extensive doc strings.
Always comments.
Always clear, meaningful, unique names.
Always absolute file paths.
Always error handling.
Never unsafe code.
Never use unwrap.
```
*/

pub fn NAME_OF_FUNCTION_YOU_ARE_IMPORTING() {
    println!("Hello, world!");
}
