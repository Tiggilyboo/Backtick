use parser::Token;

// Automatic EQ && AND, NEQ && OR bitflags to configure otherwise
pub enum Comparator {
    EQ = 1 << 0,
    NEQ = 1 << 1,
    OR = 1 << 2,
    LT = 1 << 3,
    GT = 1 << 4,
}

impl Comparator {
    pub fn parse(&self, t: Token) -> u8 {
        match t {
            Token::Comparator((ref cmp, rhs, ref t, ref f)) => {
                let mut ct = 0u8;
                let mut c: Vec<u8> = cmp.clone();

                if !c.is_empty(){
                    match c.pop().unwrap() as char {
                        '?' => ct = ct,
                        '|' => ct = ct ^ Comparator::OR as u8,
                        '&' => ct = ct,
                        _ => ct = ct,
                    }
                }
                if !c.is_empty(){
                    match c.pop().unwrap() as char {
                        '=' => ct = ct ^ Comparator::EQ as u8,
                        '\'' => ct = ct ^ Comparator::NEQ as u8,
                        '\\' => ct = ct ^ Comparator::LT as u8,
                        '/' => ct = ct ^ Comparator::GT as u8,
                        _ => ct = ct,
                    }
                }
                if !c.is_empty(){
                    match c.pop().unwrap() as char {
                        '=' => ct = ct ^ Comparator::EQ as u8,
                        _ => ct = ct,
                    }
                }

                ct
            },
            _ => 0
        }
    }

    pub fn is_eq(&self, c: u8) -> bool {
        return c & Comparator::EQ as u8 > 0;
    }
    pub fn is_lt(&self, c: u8) -> bool {
        return c & Comparator::LT as u8 > 0;
    }
    pub fn is_gt(&self, c: u8) -> bool {
        return c & Comparator::GT as u8 > 0;
    }
    pub fn is_or(&self, c: u8) -> bool {
        return c & Comparator::OR as u8 > 0;
    }
}
