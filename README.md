# TEMPLATE: YOUR PROJECT NAME



## To Build From Source
see https://github.com/lineality/rust_compile_optimizations_cheatsheet
#### or for optimal performance (~6 mb)
```bash
cargo build --profile release-performance
```
#### For smallest size, build (~0.48 mb)
```bash
cargo build --profile release-small 
```


## To ~Install
Call your program by name from command line by using an alias:

Put your executable-binary somewhere, and set the file-name of that file
as a keyword for your command line interface (CLI) 
so that entering that keyword calls the executable (starts the program):

1. Make or get the binary executable and put it somewhere: e.g. for posix
```path
/home/YOURCOMPUTERNAME/YOURDIRECTORYNAME/BINARYFILENAME
```
2. Open the bash shell configuration file in a text editor. The configuration file is usually located at ~/.bashrc or ~/.bash_profile. (use whatever editor: vim, nano, hx (helix), gedit, lapce, teehee, lapce, etc.)
```bash
hx ~/.bashrc
```
(or in some systems it may be called 'bash_profile')

3. Add an "alias" for your executable at the end of your bash file. Replace /path/to/your_executable with the path of your executable. And replace "your_keyword" with whatever you want to call File Fantastic by typing into your terminal. Add this line (with your details put in):
```text
alias name_to_call_in_terminal='/path/to/your_executable'
```
e.g. specific example showing Lovelace's computer installing 'File Fantastic'
```text
alias ff='/home/lovelace/ff_file_browser/ff'
```

4. Save and close the text editor. 
- If you used nano, you can do this by pressing: Ctrl x s (control key, x key, s key)
- If you use Helix(hx), Vim(vi), or Teehee: 'i' to type, then esc for normal mode, then :wq to write and quit

4. Reload the bash-shell configuration file, and maybe open a new terminal, to apply and use the changes.
```bash
source ~/.bashrc
```
or bash_profile

Now you should be able to call File Fantastic by typing 'ff' (or whatever you choose) into a terminal.


# text of NAME_OF_DOT_RS_FILE.rs
```rust
// src/lib.rs (or src/NAME_OF_DOT_RS_FILE.rs)
/// PROJECT_NAME - A ... in Rust
/// use -> cargo build --profile release-performance
/// or, use -> cargo build --profile release-small 

/* Docs:
DESCRIPTION

Sample main file to use this module:
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

```



