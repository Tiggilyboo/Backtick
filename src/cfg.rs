use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::hash::{Hash, Hasher, SipHasher};

use parser;
use parser::Token;
use comparator::Comparator;

#[derive(Debug, PartialEq)]
pub struct BacktickExpression {
    identifier: Option<String>,
    operations: Option<Vec<Token>>,
    start: Option<u16>,
    end: Option<u16>,
}

#[derive(Debug, PartialEq)]
pub struct State {
    position: u16,
    expressions: HashMap<String, BacktickExpression>,
    memory: Vec<u8>,
    stack: Vec<Token>,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.memory.hash(state);
    }
}

impl State {
    fn current(&self) -> u8 {
        return self.memory[self.position as usize];
    }

    fn set(&mut self, value: u16) {
        let s = self.memory.len() as isize - self.position as isize;
        for _ in 0..s {
            self.memory.push(0u8);
        }
        self.memory[self.position as usize] = value as u8;
    }

    fn populate_stack(&mut self, expr: Vec<Token>) {
        let mut ex = expr.clone();

        while !expr.is_empty(){
            let e = ex.pop();
            if e.is_some(){
                self.stack.push(e.unwrap());
            }
        }
    }

    pub fn valid_logic(current: u8, c: &Token) -> bool {
        let cmp = Comparator::new(&c);
        match c {
            &Token::Comparator((ref comp, value, ref t, ref f)) => {
                (Comparator::is_eq(cmp) && current == value)
                || (Comparator::is_neq(cmp) && current != value)
                || (Comparator::is_lt(cmp) && current < value)
                || (Comparator::is_gt(cmp) && current > value)
            },
            _ => false,
        }
    }

    fn processOperator(&mut self, c: char, multiplier: u16){
        match c as char {
            '>' => self.position += multiplier,
            '<' => self.position -= multiplier,
            '+' => self.memory[self.position as usize] += multiplier as u8,
            '-' => self.memory[self.position as usize] -= multiplier as u8,
            ',' => {
                let mut l = String::new();
                let sin = io::stdin();
                sin.lock().read_line(&mut l).unwrap();
                self.memory.extend_from_slice(l.as_bytes());
            },
            '.' => {
                let l = self.memory.len();
                for m in 0..multiplier {
                    if l < (self.position as usize + m as usize) { break; }
                    print!("{}", self.memory[self.position as usize + m as usize] as char);
                }
            },
            '~' => self.stack.clear(),
            _ => {},
        }
    }

    pub fn start(&mut self, start: Token){
        self.stack.push(start);
    }

    fn next(&mut self, t: Token) {
        match t {
            Token::Address(ref address) => {
                self.position = *address;
            },
            Token::Condition(ref comparators) => {
                let mut valid: bool = true;
                for c in comparators {
                    let cmp = &Comparator::new(c);
                    match c { &Token::Comparator((ref logic, value, ref texp, ref fexp)) => {
                        if Comparator::is_or(*cmp){
                            valid = State::valid_logic(self.current(), c);
                            if valid && texp.is_some() {
                                self.populate_stack(texp.clone().unwrap());
                                break;
                            } else if fexp.is_some() {
                                self.populate_stack(fexp.clone().unwrap());
                            }
                        } else if Comparator::is_and(*cmp) && valid {
                            valid = State::valid_logic(self.current(), c);
                            if valid && texp.is_some(){
                                self.populate_stack(texp.clone().unwrap());
                                break;
                            } else if fexp.is_some(){
                                self.populate_stack(fexp.clone().unwrap());
                            }
                        }}, _ => {}
                    }
                }
            },
            Token::Execute(ref label) => {
                match self.expressions.get(label).unwrap().operations.clone().unwrap() {
                    t => {
                        self.populate_stack(t);
                    }
                }
            },
            Token::Function((ref label, start, end, ref expression)) => {
                self.expressions.insert(label.clone(), BacktickExpression{
                    identifier: Some(label.clone()),
                    operations: Some(expression.clone()),
                    start: start,
                    end: end,
                });
            },
            Token::Label(ref label) => {
                self.expressions.insert(label.clone(), BacktickExpression{
                    identifier: Some(label.clone()),
                    operations: None,
                    start: None,
                    end: None,
                });
            },
            Token::Loop(ref expression) => {
                self.populate_stack(expression.clone());
            },
            Token::Multiplier((operator, multiplier)) => {
                self.processOperator(operator as char, multiplier);
            },
            Token::Operator(operator) => {
                self.processOperator(operator as char, 1u16);
            },
            Token::Set(value) => {
                self.set(value);
            },
            Token::Array(contents) => {
                let s = self.position as usize + contents.len() as usize;
                let l = self.memory.len() as usize;

                for c in self.position as usize..s {
                    if c > l {
                        self.memory.push(contents[c as usize - self.position as usize]);
                    } else {
                        self.memory[c] = contents[c as usize - self.position as usize];
                    }
                }
            },
            _ => {}
        }
    }
}

pub fn process(tokens: &mut Vec<Token>) {
    let mut s = State {
        position: 0,
        expressions: HashMap::new(),
        memory: vec![],
        stack: vec![],
    };

    while !tokens.is_empty(){
        s.next(tokens.pop().unwrap());
    }
}
