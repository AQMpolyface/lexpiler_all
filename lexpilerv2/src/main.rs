mod parser;
mod tokenizer;

use crate::parser::*;
use crate::tokenizer::Tokenizer;
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[1]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" | "t" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let mut tokeniz = Tokenizer::new();
            let (exit_code, tokens) = tokeniz.tokenize(&file_contents);
            for tokensuwu in tokens {
                println!(
                    "{} {} {}",
                    tokensuwu.token_type, tokensuwu.lexeme, tokensuwu.literal
                );
            }

            std::process::exit(exit_code as i32);
        }
        "parse" | "p" => {
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let mut tokeniz = Tokenizer::new();
            let (exit_code, tokens) = tokeniz.tokenize(&file_contents);

            let typed_token = parse(tokens);
            match typed_token {
                Ok(v) => {
                    v.iter().for_each(|uwu| println!("{}", uwu.value));
                    /*for tokensuwu in typed_token {
                        println!("{}", tokensuwu.value);
                    }*/

                    std::process::exit(exit_code as i32);
                }
                Err(e) => {
                    eprintln!("[line {}] {}", e.line, e.message);
                    std::process::exit(65)
                }
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
