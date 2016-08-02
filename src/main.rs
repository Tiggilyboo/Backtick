#[macro_use]
extern crate nom;

mod parser;

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
