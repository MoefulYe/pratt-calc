use std::process::exit;

use anyhow::{bail, Result};
use rustyline::{error::ReadlineError, DefaultEditor};

use crate::parser::{Context, Parser};

pub struct Repl {
    ctx: Context,
    rl: DefaultEditor,
}

impl Repl {
    pub fn new() -> anyhow::Result<Repl> {
        Ok(Repl {
            ctx: Context::new(),
            rl: DefaultEditor::new()?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            match self.rl.readline(">>> ") {
                Ok(line) => {
                    if let Err(err) = self.handle_line(&line) {
                        println!("{err}");
                    }
                    self.rl.add_history_entry(&line).unwrap();
                }
                Err(ReadlineError::Eof) | Err(ReadlineError::Interrupted) => {
                    println!("Bye!");
                    exit(0);
                }
                Err(err) => {
                    bail!("{:?}", err)
                }
            }
        }
    }

    fn handle_line(&mut self, line: &str) -> Result<()> {
        let line = line.trim();
        if line.is_empty() {
            Ok(())
        } else if line == "vars" {
            if self.ctx.is_empty() {
                println!("no variables defined")
            } else {
                for (key, val) in self.ctx.iter() {
                    println!("{} = {}", key, val);
                }
            }
            Ok(())
        } else if line == "clear" {
            self.ctx.clear();
            Ok(())
        } else if let Some(leftover) = line.strip_prefix("let") {
            if let Some((key, val)) = leftover.split_once("=") {
                let key = key.trim();
                if key.chars().any(|c| !c.is_ascii_alphabetic()) {
                    bail!("expected identifier")
                }
                let val = Parser::new(val)?.parse(self.ctx())?;
                match self.ctx.insert(key.trim().to_owned(), val) {
                    Some(old) => {
                        println!("{key} = {old} -> {val}");
                    }
                    None => {
                        println!("{key} = {val}");
                    }
                }
                Ok(())
            } else {
                bail!("expected `let <ident> = <expr>`")
            }
        } else {
            let res = Parser::new(line)?.parse(self.ctx())?;
            println!("{res}");
            Ok(())
        }
    }

    fn ctx(&self) -> &Context {
        &self.ctx
    }
}
