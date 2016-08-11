#[macro_use]
extern crate nom;

mod cfg;
mod parser;

fn main(){
    let p = b"
        @0,^in
        @1^out
        ^copy @0:1 !`
            @in[->+>+<2]
            @out.
            ~
        `
        @in!copy
    ";

    let tokens = parser::parse(p);
    let graph = cfg::process(tokens.unwrap());
}
