use std::collections::HashMap;
use std::cell::RefCell;
use std::io;

use parser;
use parser::Token;
use comparator;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct BacktickExpression {
    condition: Option<u8>,
    operations: Option<Vec<u8>>,
    start: Option<u16>,
    end: Option<u16>
}

#[derive(Debug)]
pub struct State {
    position: u16,
    expressions: HashMap<&'static str, BacktickExpression>,
    memory: Vec<u8>,
    inloop: bool,
}

impl State {

    // Process each node based on its parent information (store position, declare labels, etc)
    pub fn next(&mut self, t: Token) {
        match t {
            Token::Address(ref address) => {},
            Token::Comment(comment) => {},
            Token::Comparator((ref logic, address, ref texpression, ref fexpression)) => {},
            Token::Condition(ref comparators) => {},
            Token::Execute(ref label) => {},
            Token::Function((ref label, start, end, ref expression)) => {},
            Token::Label(ref label) => {},
            Token::Loop(ref expression) => {},
            Token::Multiplier((operator, factor)) => {},
            Token::Operator(operator) => {
                match operator as char {
                    '>' => self.position += 1,
                    '<' => self.position -= 1,
                    '+' => self.memory[self.position as usize] += 1,
                    '-' => self.memory[self.position as usize] -= 1,
                    ',' => {},
                    '.' => {
                        print!("{}", self.memory[self.position as usize] as char);
                    },
                    '~' => self.inloop = false,
                    _ => {},
                }
            },
            Token::Set(value) => {
                self.memory[self.position as usize] = value as u8;
            },
        }
    }
}

pub fn process(tokens: &mut Vec<Token>) {
    let mut s = State {
        position: 0,
        expressions: HashMap::new(),
        memory: vec![],
        inloop: false,
    };

    while !tokens.is_empty(){
        s.next(tokens.pop().unwrap());
    }
}
