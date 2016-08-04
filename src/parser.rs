use nom::{IResult, digit, alphanumeric, multispace, not_line_ending};
use std::str;
use std::str::FromStr;

#[derive(Debug)]
pub enum Token {
    Label(String),
    Address(u16),
    Loop(Vec<u8>),
    Operator(u8),
    Multiplier((u8, u16)),
    Comment(bool),
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
            alphanumeric,
            str::from_utf8
        ),
        FromStr::from_str
    )
);

named!(address<Token>,
    chain!(
        n: preceded!(tag!("@"), number),
        || Token::Address(n)
    )
);

named!(label<Token>,
  chain!(
      s: preceded!(tag!("^"), many0!(string)),
      || Token::Label(String::from_utf8(s.to_vec()).unwrap())
  )
);

named!(multoperator<u8>,
    chain!(
        o: one_of!("><+-,.[]"),
        || o as u8
    )
);

named!(operator<Token>,
    chain!(
        o: multoperator,
        || Token::Operator(o)
     )
);

named!(multiplier<Token>,
     chain!(
         o: multoperator ~
         n: number,
         || Token::Multiplier((o, n))
     )
);

named!(brackets<Token>,
  chain!(
      c: delimited!(char!('['), is_not!("]"), char!(']')),
      || Token::Loop(c.to_vec())
  )
);

named!(blank<Token>,
    chain!(
        alt!(multispace | eol),
        || Token::Comment(false)
    )
);

named!(eol,
    chain!(
        alt!(tag!("\n") | tag!("\r\n") | tag!("\u{2028}") | tag!("\u{2029}")),
        || { &b""[..] }
    )
);

named!(blanks,
    chain!(
        many0!(alt!(multispace | eol)),
        || { &b""[..] }
    )
);

named!(comment<Token>,
    chain!(
        blanks? ~
        alt!(
            delimited!(tag!("``"), is_not!("``"), tag!("``")) |
            preceded!(tag!("```"), not_line_ending)
        ) ~
        blanks?,
        || Token::Comment(true)
    )
);

named!(token<Vec<Token> >,
    many0!(
        alt!(blank | label | address | multiplier | operator | brackets | comment)
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
