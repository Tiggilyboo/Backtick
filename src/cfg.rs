use std::hash::{Hash, SipHasher, Hasher};

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

use parser;
use parser::Token;

pub struct Node {
    hash: u32,
    data: Token,
    edges: Vec<Rc<RefCell<&'static Node>>>,
}

fn random(s: &mut u32) -> u32 {
    *s ^= *s << 13;
    *s ^= *s >> 17;
    *s ^= *s << 5;
    *s
}

impl Node {
    fn new(hash : &mut u32, t: Token) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            hash: random(hash),
            data: t,
            edges: Vec::new(),
        }))
    }

    fn traverse<T>(&self, t: &T, seen: &mut HashSet<u32>)
        where T: Fn(Token)
    {
        if seen.contains(&self.hash){
            return;
        }
        t(self.data.clone());
        seen.insert(self.hash);
        for n in &self.edges {
            n.borrow().traverse(t, seen);
        }
    }

    fn root(&self) -> Rc<RefCell<&'static Node>> {
        let r = self.edges[0].clone();
        r
    }

    fn next(&self, t: Token) {
        match t {
            Token::Address(n) => {},
            Token::Comment(b) => {},
            Token::Comparator(cmp) => {},
            Token::Condition(expr) => {},
            Token::Execute(name) => {},
            Token::Function((name, start, end, expr)) => {},
            Token::Label(name) => {},
            Token::Loop(expr) => {},
            Token::Multiplier((op, n)) => {},
            Token::Operator(op) => {},
            Token::Set(n) => {}
        }
    }
}

pub fn process(tokens: Vec<Token>) -> Rc<RefCell<Node>> {
    let mut tst: Vec<&'static Token> = Vec::new();
    let mut seed = 7u32;
    let mut cfg = Node::new(&mut seed, tokens[0].clone());
    for t in tokens {
        cfg.borrow_mut().next(t)
    }

    cfg
}
