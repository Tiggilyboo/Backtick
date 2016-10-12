use nom::{IResult, digit, alphanumeric, multispace, not_line_ending};
use std::str;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Token {
    Address(u16),
    Label(String),
    Comment(bool),
    Comparator((Vec<u8>, u8, Option<Vec<Token>>, Option<Vec<Token>>)),
    Condition(Vec<Token>),
    Execute(String),
    Function((String, Option<u16>, Option<u16>, Vec<Token>)),
    Loop(Vec<Token>),
    Multiplier((u8, u16)),
    Operator(u8),
    Set(u16),
    Array(Vec<u8>),
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        match (self, other){
            (&Token::Address(ref a), &Token::Address(ref b)) => a == b,
            (&Token::Comment(ref a), &Token::Comment(ref b)) => a == b,
            (&Token::Comparator(ref a), &Token::Comparator(ref b)) => a == b,
            (&Token::Condition(ref a), &Token::Condition(ref b)) => a == b,
            (&Token::Execute(ref a), &Token::Execute(ref b)) => a == b,
            (&Token::Function(ref a), &Token::Function(ref b)) => a == b,
            (&Token::Loop(ref a), &Token::Loop(ref b)) => a == b,
            (&Token::Multiplier(ref a), &Token::Multiplier(ref b)) => a == b,
            (&Token::Operator(ref a), &Token::Operator(ref b)) => a == b,
            (&Token::Set(ref a), &Token::Set(ref b)) => a == b,
            (&Token::Array(ref a), &Token::Array(ref b)) => a == b,
            _ => false,
        }
    }
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

named!(byte<u8>,
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

// = { 1, 2, 3, ... }
named!(setArray<Token>,
    chain!(
        tag!("=") ~ blanks? ~
        tag!("{") ~
        arr: many0!(
            chain!(
                blanks? ~
                a: byte ~
                blanks? ~
                opt!(tag!(",")) ~ blanks?,
                || a as u8
            )
        ) ~
        tag!("}"),
        || Token::Array(arr)
    )
);

// = `some text as u8's`
named!(byteArray<Token>,
    chain!(
        tag!("=") ~ blanks? ~
        tag!("`") ~
        arr: many0!(
            chain!(
                a: byte,
                || a as u8
            )
        ) ~
        tag!("`"),
        || Token::Array(arr)
    )
);

// ? / 3 & \= 1 && '= 2 @0 `=1`:`=0`
named!(comparator<Token>,
    chain!(
        j: alt!(    // condition start, or, and,
            tag!("?") | tag!("|") | tag!("&")
        ) ~ blanks? ~
        c: alt!(    // eq, neq lt, gt, lte, gte,
            tag!("=") | tag!("'") | tag!("\\") | tag!("/") | tag!("\\=") | tag!("/=")
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

            return Token::Comparator((v, n as u8, ft, fe));
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
    alt!(blank | comment | backtick | label | address | multiplier | brackets |
        operator | condition | execute | set | byteArray | setArray)
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
    many0!(expression)
);

pub fn parse(i: &[u8]) -> Option<Vec<Token>> {
    let parsed = token(i);

    if parsed.is_done(){
        Some(parsed.unwrap().1)
    } else {
        None
    }
}
