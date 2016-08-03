use nom::{IResult, digit, alpha, multispace};
use std::str;
use std::str::FromStr;



#[derive(Debug)]
pub enum Token {
    Label(Vec<u8>),
    Address(u16),
    Loop(Vec<u8>),
    Operator(u8),
    Multiplier(u16),
}

named!(number<u16>,
  map_res!(
    map_res!(
      digit,
      str::from_utf8
    ),
    FromStr::from_str
  )
);

named!(string<u8>,
    map_res!(
        map_res!(
            alpha,
            str::from_utf8
        ),
        FromStr::from_str
    )
);

named!(address<Token>,
    chain!(
        blanks? ~
        tag!("@") ~
        blanks? ~
        n: number,
        || Token::Address(n)
    )
);

named!(label<Token>,
  chain!(
      blanks? ~
      tag!("^") ~
      blanks? ~
      s: many0!(string),
      || Token::Label(s)
  )
);

named!(operator<Token>,
    chain!(
        blanks? ~
        o: one_of!("><+-,.") ~
        blanks?,
        || Token::Operator(o as u8)
     )
);

named!(multiplier<Token>,
     chain!(
         blanks? ~
         n: preceded!(one_of!("><+-,."), number),
         || Token::Multiplier(n)
     )
);

named!(brackets<Token>,
  chain!(
      blanks? ~
      c: delimited!(char!('['), is_not!("]"), char!(']')),
      || Token::Loop(c.to_vec())
  )
);

named!(blanks,
    chain!(
        many0!(alt!(multispace | eol)),
        || { &b""[..] }
    )
);

named!(eol,
    chain!(
        alt!(tag!("\n") | tag!("\r\n") | tag!("\u{2028}") | tag!("\u{2029}")),
        || { &b""[..] }
    )
);

named!(token<Vec<Token> >,
    many0!(
        alt!(label | address | multiplier | operator | brackets)
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
