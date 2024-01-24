use std::fmt::Display;

use phf::Map;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token<'a> {
    Eof,
    Lit(f64),
    Ident(&'a str),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Lparen,
    Rparen,
}

impl<'a> Token<'a> {
    pub fn ident_or_reserved(s: &'a str) -> Token<'a> {
        const PI: f64 = std::f64::consts::PI;
        const E: f64 = std::f64::consts::E;
        static RESERVED: Map<&'static str, f64> = phf::phf_map! {
            "pi" => PI,
            "e" => E,
        };
        RESERVED
            .get(s)
            .map(|&num| Token::Lit(num))
            .unwrap_or_else(|| Token::Ident(s))
    }
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Eof => write!(f, "eof"),
            Token::Lit(num) => write!(f, "{}", num),
            Token::Ident(s) => write!(f, "{}", s),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Percent => write!(f, "%"),
            Token::Caret => write!(f, "^"),
            Token::Lparen => write!(f, "("),
            Token::Rparen => write!(f, ")"),
        }
    }
}
