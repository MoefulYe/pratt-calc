use std::env::args;

use anyhow::Result;
use parser::{Context, Parser};
use repl::Repl;

pub mod lexer;
pub mod parser;
pub mod repl;
pub mod token;

fn main() -> Result<()> {
    let args = args();
    if args.len() > 1 {
        let expr = args.skip(1).fold("".to_owned(), |mut acc, arg| {
            acc.push_str(&arg);
            acc.push(' ');
            acc
        });
        let ctx = Context::new();
        let res = Parser::new(&expr)?.parse(&ctx)?;
        println!("{res}");
        Ok(())
    } else {
        Repl::new()?.run()
    }
}
