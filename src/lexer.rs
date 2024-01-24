use anyhow::{bail, Result};

use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    peek: Token<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Result<Self> {
        let mut lexer = Lexer {
            input,
            peek: Token::Eof,
        };
        lexer._next()?;
        Ok(lexer)
    }

    fn _next(&mut self) -> Result<()> {
        self.skip_whitespace();
        let token = match self.input.chars().next() {
            Some('+') => {
                self.input = &self.input[1..];
                Token::Plus
            }
            Some('-') => {
                self.input = &self.input[1..];
                Token::Minus
            }
            Some('*') => {
                self.input = &self.input[1..];
                Token::Star
            }
            Some('/') => {
                self.input = &self.input[1..];
                Token::Slash
            }
            Some('%') => {
                self.input = &self.input[1..];
                Token::Percent
            }
            Some('^') => {
                self.input = &self.input[1..];
                Token::Caret
            }
            Some('(') => {
                self.input = &self.input[1..];
                Token::Lparen
            }
            Some(')') => {
                self.input = &self.input[1..];
                Token::Rparen
            }
            Some(ch) if ch.is_ascii_digit() => {
                let mut i = self
                    .input
                    .chars()
                    .enumerate()
                    .skip_while(|(_, c)| c.is_ascii_digit());
                match i.next() {
                    Some((_, '.')) => match i.clone().next() {
                        Some((_, ch)) if ch.is_ascii_digit() => {
                            let i = i
                                .skip_while(|(_, c)| c.is_ascii_digit())
                                .map(|(i, _)| i)
                                .next()
                                .unwrap_or(self.input.len());
                            let num = self.input[..i].parse()?;
                            self.input = &self.input[i..];
                            Token::Lit(num)
                        }
                        Some((_, ch)) => {
                            bail!("expected digit after decimal point, found {ch}")
                        }
                        None => {
                            bail!("expected digit after decimal point, found EOF")
                        }
                    },
                    Some((i, _)) => {
                        let num = self.input[..i].parse()?;
                        self.input = &self.input[i..];
                        Token::Lit(num)
                    }
                    None => {
                        let num = self.input.parse()?;
                        self.input = &self.input[self.input.len()..];
                        Token::Lit(num)
                    }
                }
            }
            Some(ch) if ch.is_alphabetic() => {
                let i = self
                    .input
                    .chars()
                    .enumerate()
                    .skip_while(|(_, c)| c.is_ascii_alphabetic())
                    .map(|(i, _)| i)
                    .next()
                    .unwrap_or(self.input.len());
                let ident = &self.input[..i];
                self.input = &self.input[i..];
                Token::ident_or_reserved(ident)
            }
            Some(ch) => bail!("unexpected character {ch}"),
            None => Token::Eof,
        };
        self.peek = token;
        Ok(())
    }

    fn skip_whitespace(&mut self) {
        self.input = self.input.trim_start();
    }

    pub fn next(&mut self) -> Result<Token<'a>> {
        let token = self.peek;
        self._next()?;
        Ok(token)
    }

    pub fn peek(&self) -> Token<'a> {
        self.peek
    }

    pub fn eat(&mut self, token: Token) -> Result<bool> {
        if self.peek == token {
            self._next()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn expect(&self, token: Token) -> bool {
        self.peek == token
    }

    pub fn eof(&self) -> bool {
        self.expect(Token::Eof)
    }
}
