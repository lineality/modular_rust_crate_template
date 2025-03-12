// src/lib.rs (or src/NAME_OF_DOT_RS_FILE.rs)
/// PROJECT_NAME - A ... in Rust
/// use -> cargo build --profile release-performance
/// or, use -> cargo build --profile release-small 

/* Docs:
# DESCRIPTION

### Sample main file to use this module
```rust
// src/main.rs

// import YOUR_MODULE_NAME module w/ these 2 lines
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
*/

pub fn NAME_OF_FUNCTION_YOU_ARE_IMPORTING() {
    println!("Hello, world!");
}
