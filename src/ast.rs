use parser;
use parser::Token;

pub fn process(tokens: Vec<parser::Token>){
    for t in tokens {
        match t {
            Token::Address(addr) => println!("Address"),
            Token::Label(name) => println!("Label"),
            Token::Comment(ws) => println!("Comment"),
            Token::Comparator((opcmp, value, tbranch, fbranch)) => println!("Comparator"),
            Token::Condition(comps) => println!("Condition"),
            Token::Execute(name) => println!("Execute"),
            Token::Function((name, start, end, body)) => println!("Function"),
            Token::Loop(body) => println!("Loop"),
            Token::Multiplier((op, mult)) => println!("Multiplier"),
            Token::Operator(op) => println!("Operator"),
            Token::Set(value) => println!("Value")
        }
    }
}
