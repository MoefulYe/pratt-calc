use std::collections::HashMap;

use anyhow::{bail, Result};
use phf::phf_map;

use crate::{lexer::Lexer, token::Token};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Bin {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

impl Bin {
    fn prec(self) -> u8 {
        match self {
            Bin::Add | Bin::Sub => 1,
            Bin::Mul | Bin::Div | Bin::Mod => 2,
            Bin::Pow => 3,
        }
    }

    fn eval(self, lhs: f64, rhs: f64) -> f64 {
        match self {
            Bin::Add => lhs + rhs,
            Bin::Sub => lhs - rhs,
            Bin::Mul => lhs * rhs,
            Bin::Div => lhs / rhs,
            Bin::Mod => lhs % rhs,
            Bin::Pow => lhs.powf(rhs),
        }
    }
}

impl<'a> TryFrom<Token<'a>> for Bin {
    type Error = Token<'a>;

    fn try_from(value: Token<'a>) -> Result<Self, Self::Error> {
        match value {
            Token::Plus => Ok(Self::Add),
            Token::Minus => Ok(Self::Sub),
            Token::Star => Ok(Self::Mul),
            Token::Slash => Ok(Self::Div),
            Token::Percent => Ok(Self::Mod),
            Token::Caret => Ok(Self::Pow),
            value => Err(value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Un {
    Neg,
    Pos,
}

impl Un {
    fn sgn(self) -> f64 {
        match self {
            Un::Neg => -1.0,
            Un::Pos => 1.0,
        }
    }
}

impl<'a> TryFrom<Token<'a>> for Un {
    type Error = Token<'a>;

    fn try_from(value: Token<'a>) -> Result<Self, Self::Error> {
        match value {
            Token::Plus => Ok(Self::Pos),
            Token::Minus => Ok(Self::Neg),
            value => Err(value),
        }
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

pub type Context = HashMap<String, f64>;

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> anyhow::Result<Self> {
        let lexer = Lexer::new(input)?;
        Ok(Parser { lexer })
    }

    pub fn parse(&mut self, ctx: &Context) -> anyhow::Result<f64> {
        let res = self.parse_expr(ctx)?;
        let peek = self.lexer.peek();
        if peek != Token::Eof {
            bail!("expected EOF, found {}", peek)
        } else {
            Ok(res)
        }
    }

    fn parse_expr(&mut self, ctx: &Context) -> anyhow::Result<f64> {
        self.parse_pratt(ctx, 0)
    }

    fn parse_pratt(&mut self, ctx: &Context, prec: u8) -> anyhow::Result<f64> {
        let mut lhs = self.parse_atom(ctx)?;
        loop {
            let bin: Result<Bin, Token> = self.lexer.peek().try_into();
            let bin = match bin {
                Ok(ok) => ok,
                Err(_) => break,
            };
            let next_prec = bin.prec();
            if next_prec <= prec {
                break;
            }
            self.lexer.next()?;
            lhs = bin.eval(lhs, self.parse_pratt(ctx, next_prec)?);
        }
        Ok(lhs)
    }

    fn parse_atom(&mut self, ctx: &Context) -> anyhow::Result<f64> {
        let mut sgn = 1.0;
        let token = loop {
            let un: Result<Un, Token> = self.lexer.next()?.try_into();
            match un {
                Ok(un) => sgn *= un.sgn(),
                Err(token) => break token,
            }
        };
        match token {
            Token::Lit(num) => Ok(sgn * num),
            Token::Ident(ident) => {
                if self.lexer.peek() == Token::Lparen {
                    self.lexer.next()?;
                    let x = self.parse_expr(ctx)?;
                    if !self.lexer.eat(Token::Rparen)? {
                        let cur = self.lexer.peek();
                        bail!("expected ), found {}", cur)
                    }
                    Self::call_buildin(ident, x)
                } else {
                    match ctx.get(ident) {
                        Some(&num) => Ok(sgn * num),
                        None => bail!("unknown variable {}", ident),
                    }
                }
            }
            Token::Lparen => {
                let expr = self.parse_expr(ctx)?;
                if !self.lexer.eat(Token::Rparen)? {
                    let cur = self.lexer.peek();
                    bail!("expected ), found {}", cur)
                }
                Ok(sgn * expr)
            }
            token => bail!("unexpected token {}, expected `(`, ident or num", token),
        }
    }

    fn call_buildin(func: &str, x: f64) -> anyhow::Result<f64> {
        static BUILDIN: phf::Map<&'static str, fn(f64) -> f64> = phf_map! {
            "sin" => f64::sin,
            "cos" => f64::cos,
            "tan" => f64::tan,
            "asin" => f64::asin,
            "acos" => f64::acos,
            "atan" => f64::atan,
            "sinh" => f64::sinh,
            "cosh" => f64::cosh,
            "tanh" => f64::tanh,
            "asinh" => f64::asinh,
            "acosh" => f64::acosh,
            "atanh" => f64::atanh,
            "sqrt" => f64::sqrt,
            "cbrt" => f64::cbrt,
            "exp" => f64::exp,
            "ln" => f64::ln,
            "log" => f64::log10,
            "abs" => f64::abs,
            "floor" => f64::floor,
            "ceil" => f64::ceil,
            "round" => f64::round,
            "trunc" => f64::trunc,
            "signum" => f64::signum,
            "fract" => f64::fract,
        };
        match BUILDIN.get(func) {
            Some(func) => Ok(func(x)),
            None => bail!("unknown function {}", func),
        }
    }
}
