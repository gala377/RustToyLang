use crate::token;

pub fn is_beg_of_ident(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

pub fn is_part_of_ident(ch: char) -> bool {
    is_beg_of_ident(ch) || ch.is_numeric()
}

pub fn is_part_of_op(ch: char) -> bool {
    match ch {
        '+' => true,
        '(' => true,
        ')' => true,
        ',' => true,
        '-' => true,
        _ => false,
    }
}

pub fn is_operator(symbol: &str) -> Option<token::Kind> {
    match symbol {
        "+" => Some(token::Kind::Addition),
        "," => Some(token::Kind::Comma),
        "-" => Some(token::Kind::Substraction),
        "(" => Some(token::Kind::LeftParenthesis),
        ")" => Some(token::Kind::RightParenthesis),
        "++" => Some(token::Kind::Increment),
        "--" => Some(token::Kind::Decrement),
        _ => None,
    }
}

pub fn is_keyword(symbol: &str) -> Option<token::Kind> {
    match symbol {
        "def" => Some(token::Kind::FuncDef),
        _ => None,
    }
}