#[macro_use]
extern crate nom;

mod parser;

// Backtick
// An Extended Brainfuck language transpiling to Brainfuck using bfc
/*
    @                   Put the pointer at the specified position (This means we need to know what our current position is)
    >5                  Move right 5 positions
    =0                  Set the value of the current position to 0
    ~                   Break out of the current loop or exit the program
    ^name               Label the current position with an idenfitier, can be used later with @name f.e.
    !`++>[<->+]`        Declare a function with enclosed backtick expression at the current position
    ^name!`ascii`       Declare a variable with ASCII contents at the current position
    =``                 Set the current position to the current + length memory to the contained backtick expression
*/

fn main(){
    let test = "
        >3
        @0
        +10
        ^test
        [->+<]
        @0.3
    ";
    print!("Parsing: ");
    println!("{}", test);

    let tokens = parser::parse(test.as_bytes());
    let mut i = 0;
    for t in tokens {
        println!("{:?}", t);
        println!("Tokens: {}", i);
        i += 1;
    }
    println!("Done.");
}
