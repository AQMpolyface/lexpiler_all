// parser_utils.rs
use crate::parser::Token;
use regex::Regex;
use std::sync::Mutex;

// Import the static variables from main
extern crate lazy_static;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ERROR_VECTOR: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub static mut BAD: bool = false;

pub fn parse_equal_equal(tokens: Vec<String>) -> Vec<String> {
    let mut result = tokens.clone();
    let mut i = 1;

    while i < result.len() {
        match result[i].as_str() {
            "==" | "!=" => {
                if i > 0 && i < result.len() - 1 {
                    let operator = result[i].clone();
                    let lhs = result[i - 1].clone();
                    let rhs = result[i + 1].clone();
                    let new_expr = format!("({} {} {})", operator, lhs, rhs);
                    result.splice(i - 1..=i + 1, vec![new_expr]);
                    continue;
                }
            }
            _ => {}
        }
        i += 1;
    }
    result
}

pub fn parse_equals(tokens: Vec<String>) -> Vec<String> {
    let mut result = tokens.clone();
    let mut i = 1;
    while i < result.len() {
        match result[i].as_str() {
            "<" | ">" | ">=" | "<=" => {
                if result[i + 1].as_str() != "-" {
                    if i > 0 && i < result.len() - 1 {
                        let operator = result[i].clone();
                        let lhs = result[i - 1].clone();
                        let rhs = result[i + 1].clone();

                        let new_expr = format!("({} {} {})", operator, lhs, rhs);
                        result.splice(i - 1..=i + 1, vec![new_expr]);
                        continue;
                    } else if result[i + 1].as_str() == "-" {
                        let operator = result[i].clone();
                        let rhs = format!("(- {})", result[i + 2].clone());
                        let lhs = result[i - 1].clone();
                        let new_expr = format!("({} {} {})", operator, lhs, rhs);
                        result.splice(i - 1..=i + 1, vec![new_expr]);
                        continue;
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    result
}

pub fn validate_expression(lhs: &str, rhs: &str, operator: String) -> bool {
    let mut has_error = false;

    if lhs.trim().is_empty() || is_valid_operand(lhs) {
        ERROR_VECTOR.lock().unwrap().push(format!(
            "error: {} needs to have a group or a number before",
            operator
        ));

        has_error = true;
    }

    if rhs.trim().is_empty() || is_valid_operand(rhs) {
        ERROR_VECTOR.lock().unwrap().push(format!(
            "error: {} needs to have a group or a number after",
            operator
        ));
        //have to see if it errors with code 65 when there is a parsing erroe

        has_error = true;
    }

    has_error
}

pub fn is_valid_operand(operand: &str) -> bool {
    let debug = false;

    if operand.parse::<f64>().is_ok() {
        if debug {
            println!(" {} is an f64", operand);
        }
        return true;
    }

    if operand.starts_with('(') && operand.ends_with(')') {
        if debug {
            println!("{} is a group, OK", operand);
        }
        return true;
    }

    let string_pattern = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
    if string_pattern.is_match(operand) {
        if debug {
            println!("{} is a string", operand);
        }
        return true;
    }

    false
}
fn parse_signs(tokens: Vec<String>) -> Vec<String> {
    let debug = false;
    let mut result = tokens.clone();

    if debug {
        for (num, tokenaa) in tokens.iter().enumerate() {
            println!("token: {} number {}", tokenaa, num);
        }
    }
    // first pass: handle comparison operators

    let mut i = 1;
    while i < result.len() {
        match result[i].as_str() {
            "<" | ">" | ">=" | "<=" => {
                if result[i + 1].as_str() != "-" {
                    if i > 0 && i < result.len() - 1 {
                        let operator = result[i].clone();
                        let lhs = result[i - 1].clone();
                        let rhs = result[i + 1].clone();
                        let new_expr = format!("({} {} {})", operator, lhs, rhs);
                        result.splice(i - 1..=i + 1, vec![new_expr]);
                        continue;
                    } else if result[i + 1].as_str() == "-" {
                        let operator = result[i].clone();

                        let rhs = format!("(- {})", result[i + 2].clone());
                        let lhs = result[i - 1].clone();
                        //let rhs = result[i + 1].clone();
                        let new_expr = format!("({} {} {})", operator, lhs, rhs);
                        result.splice(i - 1..=i + 1, vec![new_expr]);
                        continue;
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }

    //unary minus i hate this thing so much omg
    let mut i = 0;
    while i < result.len() {
        //println!("{}", result[i]);
        if result[i] == "-" {
            //println!("checking a -");
            if i == 0
                || matches!(result[i - 1].as_str(), "+" | "-" | "*" | "/" | "(")
                || matches!(result[i - 1].as_str(), ">" | "<" | ">=" | "<=")
            {
                if i < result.len() - 1 {
                    let rhs = result[i + 1].clone();
                    let new_expr = format!("(- {})", rhs);
                    if debug {
                        println!("Creating unary negation of group: {}", new_expr);
                    }
                    result.splice(i..=i + 1, vec![new_expr]);
                    continue;
                }
            }
        }
        i += 1;
    }
    // Third pass: handle multiplication and division
    let mut i = 0;
    while i < result.len() {
        match result[i].as_str() {
            "*" | "/" => {
                if i == 0 {
                    unsafe {
                        BAD = true;
                        ERROR_VECTOR
                            .lock()
                            .unwrap()
                            .push(String::from("error: expression cannot start with + or -"));
                    }
                }
                if i == result.len() - 1 {
                    unsafe {
                        BAD = true;
                        ERROR_VECTOR
                            .lock()
                            .unwrap()
                            .push(String::from("error: expression cannot end with + or -"));
                    }
                }
                if result[i + 1].as_str() != "-" {
                    if i > 0 && i < result.len() - 1 {
                        let operator = result[i].clone();
                        let lhs = result[i - 1].clone();
                        let rhs = result[i + 1].clone();
                        let new_expr = format!("({} {} {})", operator, lhs, rhs);
                        result.splice(i - 1..=i + 1, vec![new_expr]);
                        continue;
                    } else if result[i + 1].as_str() == "-" {
                        let operator = result[i].clone();

                        let rhs = format!("(- {})", result[i + 2].clone());
                        let lhs = result[i - 1].clone();
                        //let rhs = result[i + 1].clone();
                        let new_expr = format!("({} {} {})", operator, lhs, rhs);
                        result.splice(i - 1..=i + 1, vec![new_expr]);
                        continue;
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }

    // Fourth pass: handle addition and subtraction
    let mut i = 0;
    while i < result.len() {
        match result[i].as_str() {
            "+" | "-" => {
                // Flag to track if we encountered any errors
                let mut has_error = false;

                // Check if we're at the start of the expression
                if i == 0 {
                    unsafe {
                        BAD = true;
                        ERROR_VECTOR
                            .lock()
                            .unwrap()
                            .push(String::from("error: expression cannot start with + or -"));
                    }
                    has_error = true;
                }

                // Check if we're at the end of the expression
                if i == result.len() - 1 {
                    unsafe {
                        BAD = true;
                        ERROR_VECTOR
                            .lock()
                            .unwrap()
                            .push(String::from("error: expression cannot end with + or -"));
                    }
                    has_error = true;
                }

                // Check for out of bounds access
                if tokens.get(i - 1).is_none() {
                    unsafe {
                        BAD = true;
                        ERROR_VECTOR
                            .lock()
                            .unwrap()
                            .push(String::from("error: index is out of bounds"));
                    }
                    has_error = true;
                }

                if i > 0 && i < result.len() - 1 && !has_error {
                    let operator = result[i].clone();

                    let lhs = result[i - 1].clone();
                    let rhs = result[i + 1].clone();

                    //println!(
                    //"checking for the second time for a {}: lhs = {} rhs = {}",
                    //operator, lhs, rhs
                    //);
                    // Check if lhs is empty or whitespace
                    if lhs.trim().is_empty() {
                        unsafe {
                            BAD = true;
                            ERROR_VECTOR
                                .lock()
                                .unwrap()
                                .push(String::from("error: empty expression before + or -"));
                        }
                        has_error = true;
                    }

                    // Check if rhs is empty or whitespace
                    if rhs.trim().is_empty() {
                        unsafe {
                            BAD = true;
                            ERROR_VECTOR
                                .lock()
                                .unwrap()
                                .push(String::from("error: empty expression after + or -"));
                        }
                        has_error = true;
                    }

                    // Only proceed if no errors so far
                    if !validate_expression(&lhs, &rhs, operator.clone()) {
                        has_error = true;
                    }

                    if !has_error {
                        let new_expr = format!("({} {} {})", operator, lhs, rhs);
                        if debug {
                            println!("Creating addition/subtraction: {}", new_expr);
                        }
                        result.splice(i - 1..=i + 1, vec![new_expr]);
                        continue;
                    }
                }

                if has_error {
                    i += 1;
                    continue;
                }
            }
            _ => {}
        }
        i += 1;
    }

    result
}

pub fn parse_more(tokens: Vec<Token>) -> Vec<String> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut send_sign = false;
    let mut has_shitty_sign = false;
    let mut has_comparison = false;
    let debug = false;
    //println!("tiken {:?}", tokens);
    while i < tokens.len() {
        let token = &tokens[i];
        let token_type = token.token_type.as_str();
        if debug {
            println!(
                "type: {}, literal: {}, lexme: {}, number {}
                ",
                token.token_type, token.literal, token.lexeme, i
            );
        }
        match token_type {
            "NUMBER" | "STRING" => {
                result.push(token.literal.clone());
            }
            "LEFT_PAREN" => {
                // Handle grouped expressions
                let mut inner_tokens = Vec::new();
                let mut paren_count = 1;
                i += 1;
                while i < tokens.len() && paren_count > 0 {
                    let inner_token = &tokens[i];
                    let inner_type = inner_token.token_type.as_str();

                    if inner_type == "LEFT_PAREN" {
                        paren_count += 1;
                    } else if inner_type == "RIGHT_PAREN" {
                        paren_count -= 1;
                        if paren_count == 0 {
                            break;
                        }
                    }
                    inner_tokens.push(inner_token.clone());
                    i += 1;
                }

                let inner_result = parse_more(inner_tokens);
                if inner_result.len() <= 0 {
                    unsafe {
                        ERROR_VECTOR
                            .lock()
                            .unwrap()
                            .push(String::from("error: empty parenthesis"));
                        BAD = true;
                    }
                    continue;
                }
                result.push(format!("(group {})", inner_result.join(" ")));
            }
            "MINUS" => {
                result.push("-".to_string());
                send_sign = true;
            }
            "PLUS" => {
                result.push("+".to_string());
                send_sign = true;
            }
            "SLASH" => {
                result.push("/".to_string());
                send_sign = true;
            }
            "STAR" => {
                result.push("*".to_string());
                send_sign = true;
            }
            "TRUE" | "FALSE" | "NIL" => {
                result.push(token.lexeme.clone());
            }
            "BANG_EQUAL" | "EQUAL_EQUAL" => {
                //println!("bang equal or equal equal");
                result.push(token.lexeme.clone());
                has_shitty_sign = true;
            }
            "BANG" => {
                i += 1; // Advance to the next token after the first BANG
                let mut inner_tokens: Vec<Token> = Vec::new();

                // Collect consecutive BANG tokens
                while i < tokens.len() && tokens[i].token_type.as_str() == "BANG"
                    || tokens[i].token_type.as_str() == "LEFT_PAREN"
                {
                    inner_tokens.push(tokens[i].clone());
                    i += 1;
                }

                // Ensure there's a non-BANG token after the sequence
                if i < tokens.len() {
                    inner_tokens.push(tokens[i].clone());
                    i += 1; // Consume the final token
                } else {
                    panic!("Unexpected end of input after BANG tokens");
                }

                // Parse the collected tokens
                let inner_result = parse_more(inner_tokens);
                result.push(format!("(! {})", inner_result.join(" ")));
            }
            "GREATER_EQUAL" | "LESS_EQUAL" | "GREATER" | "LESS" => {
                has_comparison = true;
                result.push(token.lexeme.clone());
            }
            _ => {}
        }
        i += 1;
    }
    if debug {
        if send_sign {
            println!("calling parse_sign on{:#?}", result);
        } else {
            println!("not calling parse sign on {:?}", result);
        }
    }
    if send_sign {
        if debug {
            for (num, tokensuwu) in result.clone().iter().enumerate() {
                println!(
                    "pars sign hit: result from parssign:, number: {} {}",
                    num, tokensuwu
                );
            }
        }
        result = parse_signs(result);
    } else {
        if debug {
            for (num, tokensuwu) in result.clone().iter().enumerate() {
                println!("result from result1:, number: {} {}", num, tokensuwu);
            }
        }
    }
    if has_shitty_sign {
        if debug {
            for (num, tokensuwu) in result.clone().iter().enumerate() {
                println!(
                    "pars sign hit: result from parssign:, number: {} {}",
                    num, tokensuwu
                );
            }
        }
        result = parse_equal_equal(result);
    } else {
        if debug {
            for (num, tokensuwu) in result.clone().iter().enumerate() {
                println!("result from result1:, number: {} {}", num, tokensuwu);
            }
        }
    }
    if has_comparison {
        if debug {
            for (num, tokensuwu) in result.clone().iter().enumerate() {
                println!(
                    "pars sign hit: result from parssign:, number: {} {}",
                    num, tokensuwu
                );
            }
        }
        result = parse_equals(result);
    } else {
        if debug {
            for (num, tokensuwu) in result.clone().iter().enumerate() {
                println!("result from result1:, number: {} {}", num, tokensuwu);
            }
        }
    }
    result
}
