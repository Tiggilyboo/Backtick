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

    if tokens.is_some(){
        for t in tokens {
            println!("{:?}", t);
        }
    } else {
        println!("No tokens.");
    }

    println!("Done.");
}
