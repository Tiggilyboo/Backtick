use nom::{IResult, digit, alphanumeric, multispace, not_line_ending};
use std::str;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Token {
    Address(u16),
    Label(String),
    Comment(bool),
    Comparator((Vec<u8>, u16, Option<Vec<Token>>, Option<Vec<Token>>)),
    Condition(Vec<Token>),
    Execute(String),
    Function((String, Option<u16>, Option<u16>, Vec<Token>)),
    Loop(Vec<Token>),
    Multiplier((u8, u16)),
    Operator(u8),
    Set(u16),
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

named!(string<String>,
    chain!(
        a: alphanumeric,
        || String::from_utf8(a.to_vec()).unwrap()
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

named!(instruction<u8>,
    chain!(
        o: one_of!("><+-,.~"),
        || o as u8
    )
);

// @number
named!(address<Token>,
    chain!(
        n: preceded!(tag!("@"), number),
        || Token::Address(n)
    )
);

// ^label
named!(label<Token>,
  chain!(
      s: preceded!(tag!("^"), string),
      || Token::Label(s)
  )
);

// ><+-,.[]
named!(operator<Token>,
    chain!(
        o: instruction,
        || Token::Operator(o)
     )
);

// <3
named!(multiplier<Token>,
     chain!(
         o: instruction ~
         n: number,
         || Token::Multiplier((o, n))
     )
);

// [operators~]
named!(brackets<Token>,
  chain!(
      c: delimited!(char!('['), many0!(expression), char!(']')),
      || Token::Loop(c)
  )
);

named!(blank<Token>,
    chain!(
        alt!(multispace | eol),
        || Token::Comment(false)
    )
);

// ```comment ``comment``
named!(comment<Token>,
    chain!(
        blanks? ~
        alt!(
            preceded!(tag!("```"), not_line_ending) |
            delimited!(tag!("``"), is_not!("``"), tag!("``"))
        ) ~
        blanks?,
        || Token::Comment(true)
    )
);

// !function
named!(execute<Token>,
    chain!(
        l: preceded!(tag!("!"), string),
        || Token::Execute(l)
    )
);

// =number
named!(set<Token>,
    chain!(
        v: preceded!(tag!("="), number),
        || Token::Set(v)
    )
);

named!(comparator<Token>,
    chain!(
        j: alt!(    // condition start, or, and,
            tag!("?") | tag!("|") | tag!("&")
        ) ~ blanks? ~
        c: alt!(    // eq, neq lt, gt, lte, gte,
            tag!("=") | tag!("'=") | tag!("\\") | tag!("/") | tag!("\\=") | tag!("/=")
        ) ~ blanks? ~
        a: opt!(tag!("@")) ~
        n: number ~ blanks? ~
        ft: opt!(backtick_expression) ~ blanks? ~
        fe: opt!(preceded!(tag!(":"), backtick_expression)),
        || {
            let mut v = Vec::new();
            v.extend_from_slice(j);
            v.extend_from_slice(c);
            match a {
                Some(_) => v.insert(0, 1u8),
                None => v.insert(0, 0u8),
            }

            return Token::Comparator((v, n, ft, fe));
        }
    )
);

named!(condition<Token>,
    chain!(
        c: many1!(comparator),
        || Token::Condition(c)
    )
);

named!(expression<Token>,
    alt!(blank | comment | label | address | multiplier | brackets |
        operator | condition | execute | set)
);

named!(backtick_expression<Vec<Token> >,
    delimited!(char!('`'), many0!(expression), char!('`'))
);

// ^label @start : end `expressions`
named!(backtick<Token>,
    chain!(
        l: preceded!(tag!("^"), string) ~
        blanks? ~
        s: opt!(preceded!(tag!("@"), number)) ~ blanks? ~
        o: opt!(preceded!(tag!(":"), number)) ~ blanks? ~
        c: preceded!(tag!("!"), backtick_expression),
        || Token::Function((l, s, o, c))
    )
);

named!(token<Vec<Token> >,
    many0!(alt_complete!(backtick | expression))
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
