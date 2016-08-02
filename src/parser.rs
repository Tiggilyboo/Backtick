use nom::{IResult, digit, alpha};
use std::str;

#[derive(Debug)]
pub enum Token {
    Label(Vec<u8>),
    Address(u16),
    Loop(Vec<u8>),
    Operator(u8),
    Multiplier(u16),
}

fn number(n: &[u8]) -> IResult<&[u8], u16> {
    map_res!(
        n,
        digit,
        |d| str::FromStr::from_str(str::from_utf8(d).unwrap())
    )
}

named!(label<&[u8], Token>,
    chain!(
        tag!("^") ~
        s: alpha,
        || Token::Label(s.to_vec())
    )
);

named!(address<&[u8], Token>,
    chain!(
        tag!("@") ~
        n: number,
        || Token::Address(n)
    )
);

named!(operator<&[u8], Token>,
    chain!(
        o: one_of!("><+-,."),
        || Token::Operator(o as u8)
    )
);

named!(multiplier<&[u8], Token>,
    chain!(
        n: preceded!(one_of!("><+-,."), number),
        || Token::Multiplier(n)
    )
);

named!(brackets<&[u8], Token>,
    chain!(
        c: delimited!(char!('['), is_not!("]"), char!(']')),
        || Token::Loop(c.to_vec())
    )
);

named!(token<&[u8], Vec<Token> >,
    many0!(
        alt!(label | address | operator | multiplier | brackets)
    )
);

pub fn parse(i: &[u8]) -> Option<Vec<Token> > {
    let parsed: IResult<&[u8], Vec<Token> > = token(i);

    if parsed.is_done() {
        Some(parsed.unwrap().1)
    } else if parsed.is_incomplete(){
        None
    } else { //if parsed.is_err(){
        None
    }
}
