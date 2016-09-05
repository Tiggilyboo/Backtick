use std::collections::HashMap;
use std::io;

use parser;
use parser::Token;
use comparator::Comparator;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct BacktickExpression {
    identifier: String,
    operations: Option<Vec<Token>>,
    start: Option<u16>,
    end: Option<u16>,
}

#[derive(Debug, Hash, Eq, ParialEq)]
pub struct State {
    position: u16,
    expressions: HashMap<BacktickExpression>,
    memory: Vec<u8>,
    stack: Vec<Token>,
}

impl State {
    fn current(&self) -> u8 {
        return self.memory[self.position as usize];
    }

    fn set(&self, value: u8) {
        self.memory[self.position as usize] = value;
    }

    fn populateStack(&self, expr: Option<Vec<Token>>) {
        while expr.is_some() && !expr.unwrap().is_empty(){
            let e = expr.unwrap().pop();
            while(e.unwrap())
                self.stack.push(expr.unwrap());
            }
        }
    }

    pub fn validLogic(c: Token) -> bool {
        let cmp = Comparator::new(c);
        match c { &Token::Comparator((logic, value, texp, fexp)) => {
            (Comparator::is_eq(cmp) && self.current() == value)
            || (Comparator::is_neq(cmp) && self.current() != value)
            || (Comparator::is_lt(cmp) && self.current() < value)
            || (Comparator::is_gt(cmp) && self.current() > value)
        }, _ => {}}
    }

    fn processOperator(&self, c: char, multiplier: u16){
        match operator as char {
            '>' => self.position += multiplier,
            '<' => self.position -= multiplier,
            '+' => self.memory[self.position as usize] += multiplier,
            '-' => self.memory[self.position as usize] -= multiplier,
            ',' => {
                let stdin = io::stdin();
                for l in stdin.lock().lines() {
                    self.memory.extend_from_slice(line.unwrap());
                    break;
                }
            },
            '.' => {
                let l = self.memory.len();
                for m = 0..multiplier {
                    if(l < self.position + m) { break; }
                    print!("{}", self.memory[self.position+m as usize] as char);
                }
            },
            '~' => self.stack.clear(),
            _ => {},
        }
    }

    pub fn run(&self, start: Token){
        self.stack.push(start);
        while(!self.stack.is_empty()){
            self.next(self.stack.pop());
        }
    }

    fn next(&mut self, t: Token) {
        match t {
            Token::Address(ref address) => {
                self.position = *address;
            },
            Token::Condition(ref comparators) => {
                let valid: bool = true;
                for c in comparators {
                    let cmp = Comparator::new(c);
                    match c { &Token::Comparator((logic, value, texp, fexp)) => {
                        if(Comparator::is_or(cmp)){
                            valid = State::validLogic(c);
                            if(valid){
                                self.populateStack(texp);
                                break;
                            } else {
                                self.populateStack(fexp);
                            }
                        } else if(Comparator::is_and(cmp) && valid) {
                            valid = State::validLogic(c);
                            if(valid){
                                self.populateStack(texp);
                                break;
                            } else {
                                self.populateStack(fexp);
                            }
                        }}, _ => {}
                    }
                }
            },
            Token::Execute(ref label) => {
                if(self.expressions.has(label) && self.expressions[label].operations.is_some()){
                    self.populateStack(self.expressions[label].operations);
                }
            },
            Token::Function((ref label, start, end, ref expression)) => {
                self.expressions.push(BacktickExpression {
                    identifer: Some(label),
                    expressions: Some(expression),
                    start: Some(start),
                    end: Some(end),
                });
            },
            Token::Label(ref label) => {
                self.expressions.push(BacktickExpression {
                    identifer: Some(label),
                    expressions: None,
                    start: Some(self.position),
                    end: None,
                });
            },
            Token::Loop(ref expression) => {
                self.extend_from_slice(expression.as_slice());
            },
            Token::Multiplier((operator, multiplier)) => {
                self.processOperator(operator, multiplier);
            },
            Token::Operator(operator) => {
                self.processOperator(operator, 1u16);
            },
            Token::Set(value) => {
                for c = 0..(self.memory.len() - self.position) {
                    self.memory.push(0u8);
                }
                self.memory[self.position as usize] = value as u8;
            },
            Token::Array(contents) => {
                for c = self.position..(self.position + contents.len()){
                    if(c > self.memory.len()){
                        self.memory.push(contents[c - self.position as usize]);
                    } else {
                        self.memory[c] = contents[c - self.position as usize];
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
        expressions: vec![],
        memory: vec![],
        stack: vec![],
    };

    while !tokens.is_empty(){
        s.next(tokens.pop().unwrap());
    }
}
