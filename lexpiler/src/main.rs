mod evaluate_utils;
mod lexer;
mod parser;
mod parser_utils;

use crate::evaluate_utils::*;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::parser_utils::*;
use regex::Regex;
use std::env;
use std::fs;
use std::i32;
use std::io::{self, Write};
#[quit::main]
fn main() {
    let debug = false;
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} <command> <filename>", args[0]).unwrap();
        quit::with_code(64);
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" | "t" => {
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                quit::with_code(64);
            });

            let mut lexer = Lexer::new();
            let exit_code = lexer.tokenize(&file_contents);

            quit::with_code(exit_code);
        }
        "parse" | "p" => {
            writeln!(io::stderr(), "Parsing file...").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                quit::with_code(64);
            });

            let mut parser = Parser::new();
            let (exit_code, tokens) = parser.parse(&file_contents);
            let result = parse_more(tokens);

            if debug {
                println!("pre-parse equal: {:#?}", result);
            }

            let result2 = parse_equals(result);
            if debug {
                println!("post-parse equal: {:#?}", result2);
            }

            let result3 = parse_equal_equal(result2);
            if debug {
                println!("post-parse equal_equal: {:#?}", result3);
            }

            let mut exit_code_1 = exit_code;
            unsafe {
                if BAD {
                    let errors = ERROR_VECTOR.lock().unwrap();
                    for error in errors.iter() {
                        eprintln!("{}", error);
                    }
                    exit_code_1 = 65;
                } else {
                    for uwu in result3 {
                        println!("{}", uwu);
                    }
                }
            }

            quit::with_code(exit_code_1);
        }
        "evaluate" | "e" => {
            writeln!(io::stderr(), "Parsing file...").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                quit::with_code(64);
            });
            let mut parser = Parser::new();
            let (exit_code, tokens) = parser.parse(&file_contents);
            let result = ev_parse_more(tokens);
            let debug = false;
            if debug {
                println!("pre-parse equal: {:#?}", result);
            }

            let result2 = ev_parse_equals(result);
            if debug {
                println!("post-parse equal: {:#?}", result2);
            }

            let result3 = ev_parse_equal_equal(result2);
            if debug {
                println!("post-parse equal_equal: {:#?}", result3);
            }

            let mut exit_code_1 = exit_code;

            /*for things in result3.clone() {
                println!("value: {}, type: {:?}", things.value, things.value_type);
            }*/

            unsafe {
                if BAD {
                    let errors = ERROR_VECTOR.lock().unwrap();
                    for error in errors.iter() {
                        eprintln!("{}", error);
                    }
                    exit_code_1 = 65;
                } else {
                    evaluate(result3);
                }
            }

            quit::with_code(exit_code_1);
        }

        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            quit::with_code(64);
        }
    }
}
/*fn evaluate_token(str: TypedToken) {
    let a = str.value_type.clone();
    match a {
        ValueType::String => println!("{}", str.value),
        ValueType::Boolean => handle_bool(str.value.as_str(), false),
        ValueType::Number => handle_numbers(str, false),
        ValueType::Operation => handle_muldiv(str),
        //handle_operation(str.value),
        ValueType::Negation => handle_numbers(str, true),
        ValueType::Nil => println!("nil"),
        ValueType::Group => handle_group(str),
        ValueType::Bang => {
            handle_bang(str);
        }
        ValueType::MulDiv => handle_muldiv(str),
        ValueType::AddSub => {}
        ValueType::Comparison => handle_comparison(str),
    }
}*/
fn evaluate(strings: Vec<TypedToken>) {
    for str in strings {
        let a = str.value_type.clone();
        match a {
            ValueType::String => println!("{}", str.value),
            ValueType::Boolean => handle_bool(str.value.as_str(), false),
            ValueType::Number => handle_numbers(str, false),
            ValueType::Operation => handle_muldiv(str),
            //handle_operation(str.value),
            ValueType::Negation => handle_numbers(str, true),
            ValueType::Nil => println!("nil"),
            ValueType::Group => handle_group(str),
            ValueType::Bang => {
                handle_bang(str);
            }
            ValueType::MulDiv => handle_muldiv(str),
            ValueType::AddSub => {}
            ValueType::Comparison => {
                println!("saas {:?}", str);
                handle_comparison(str.value)
            }
        }
    }
}
fn handle_comparison(operation: String) {
    println!("op {}", operation);
}
fn handle_operations(operation: String) -> String {
    let debug = false;
    if debug {
        println!("Processing: {}", operation);
    }

    // Remove 'group' keywords and trim spaces
    let operation = operation.replace("group", "").trim().to_string();

    // Handle lone negative number case early
    if operation.starts_with('-') && !operation.contains(' ') {
        if let Ok(_) = operation.parse::<f32>() {
            return operation;
        }
    }

    // If no parentheses, evaluate simple expression
    if !operation.contains('(') {
        let parts: Vec<String> = operation
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        if parts.is_empty() {
            return "0".to_string();
        }

        if parts.len() == 1 {
            return parts[0].to_string();
        }

        // Handle unary minus
        if parts.len() == 2 && parts[0] == "-" {
            let num = parts[1].parse::<f32>().unwrap();
            return (-num).to_string();
        }
        if parts.len() == 3 {
            // Handle binary operations
            let op = &parts[0];
            let arg1 = &parts[1];
            let arg2 = &parts[2];

            // Handle string concatenation
            if op == "+" && (arg1.parse::<f32>().is_err() || arg2.parse::<f32>().is_err()) {
                return format!("{}{}", arg1, arg2); // Concatenate strings
            }

            // Handle numeric operations
            let num1 = arg1.parse::<f32>().unwrap();
            let num2 = arg2.parse::<f32>().unwrap();
            let result = match op.as_str() {
                "+" => num1 + num2,
                "-" => num1 - num2,
                "*" => num1 * num2,
                "/" => num1 / num2,
                _ => panic!("Unknown operator: {}", op),
            };
            return result.to_string();
        }

        panic!("Invalid expression format: {}", operation);
    }

    // Find innermost parentheses
    let mut depth = 0;
    let mut max_depth = 0;
    let mut innermost_start = 0;
    let mut innermost_end = 0;

    let chars: Vec<char> = operation.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        if c == '(' {
            depth += 1;
            if depth > max_depth {
                max_depth = depth;
                innermost_start = i;
            }
        } else if c == ')' {
            if depth == max_depth {
                innermost_end = i;
                break;
            }
            depth -= 1;
        }
    }

    // Extract innermost expression
    let inner_expr = operation[innermost_start..=innermost_end].to_string();
    if debug {
        println!("Inner expression: {}", inner_expr);
    }

    // Remove outer parentheses for processing
    let inner_expr = if inner_expr.starts_with('(') && inner_expr.ends_with(')') {
        inner_expr[1..inner_expr.len() - 1].to_string()
    } else {
        inner_expr
    };

    // Recursively evaluate inner expression
    let inner_result = handle_operations(inner_expr);
    if debug {
        println!("Inner result: {}", inner_result);
    }

    // Substitute result back and continue processing
    let mut new_operation = operation[..innermost_start].to_string();
    new_operation.push_str(&inner_result);
    new_operation.push_str(&operation[innermost_end + 1..]);
    if debug {
        println!("After substitution: {}", new_operation);
    }

    handle_operations(new_operation)
}

fn handle_muldiv(opeuwu: TypedToken) {
    //println!("handle muldiv WEE WOO {}", opeuwu.value);
    println!("{}", handle_operations(opeuwu.value))
}
fn handle_numbers(number1: TypedToken, is_negative: bool) {
    //println!("NUMBER WEE WOO {:?}", number1);
    let mut number = number1.value.as_str();

    // Remove any leading/trailing parentheses
    number = number.trim_start_matches('(').trim_end_matches(')');

    if number.contains('.') {
        let parts: Vec<&str> = number.split('.').collect();
        if parts.len() == 2 {
            if let Ok(decimal) = parts[1].parse::<u32>() {
                if decimal == 0 {
                    if is_negative {
                        println!("-{}", parts[0].trim_start_matches('-').trim());
                    } else {
                        println!("{}", parts[0].trim());
                    }
                } else {
                    if is_negative {
                        println!("-{}", number.trim_start_matches('-').trim());
                    } else {
                        println!("{}", number.trim());
                    }
                }
            }
        }
    } else {
        if is_negative {
            println!("-{}", number.trim_start_matches('-').trim());
        } else {
            println!("{}", number.trim());
        }
    }
}
fn handle_bang(token: TypedToken) /*-> bool*/
{
    //println!("WEE WOO BANG");

    match token.parenthesis {
        None => {
            eprintln!("Can't have an empty bang");
        }
        Some(_) => handle_bool(token.value.as_str(), true),
    }
}

fn handle_bool(mut str: &str, should_negate: bool) {
    str = str.trim_start_matches('(').trim_end_matches(')');

    str = str.trim_start_matches('!');
    //println!("WEE WOO BOOL {}", str);
    let base_value = match str.trim() {
        "true" => true,
        "false" => false,
        _ => false,
    };

    let final_value = if should_negate {
        !base_value
    } else {
        base_value
    };

    println!("{}", final_value);
}
fn handle_group(str: TypedToken) {
    //println!("GROUP WEE WOO {}", str.value);
    let mut number_of_paren = 0;
    for chars in str.value.chars() {
        if chars == '(' {
            number_of_paren += 1;
        }
    }

    /*let newstr = format!("{}{}", ' ', &str.value[7..str.value.len() - 1],);

    let newstr = format!("{}{}", newstr, ' '); */
    match str.parenthesis {
        None => eprintln!("parenthesis empty, error"),
        Some(a) => {
            evaluate(a);
        }
    }
}
