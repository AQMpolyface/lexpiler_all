use crate::parser::Token;
use regex::Regex;
use std::fmt;
#[derive(Clone, Debug, PartialEq)]
pub enum ValueType {
    Number,
    Negation,
    String,
    Boolean,
    Group,
    Operation,
    Nil,
    Bang,
    MulDiv,
    AddSub,
    Comparison,
}

#[derive(Clone, Debug)]
pub struct TypedToken {
    pub value: String,
    pub value_type: ValueType,
    pub parenthesis: Option<Vec<TypedToken>>,
}

impl TypedToken {
    pub fn new(value: String, value_type: ValueType, parenthesis: Option<Vec<TypedToken>>) -> Self {
        Self {
            value,
            value_type,
            parenthesis,
        }
    }
}
pub fn ev_parse_more(tokens: Vec<Token>) -> Vec<TypedToken> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut send_sign = false;
    let mut has_shitty_sign = false;
    let debug = false;
    let mut has_comparison = false;
    while i < tokens.len() {
        let token = &tokens[i];
        match token.token_type.as_str() {
            "NUMBER" => {
                result.push(TypedToken::new(
                    token.literal.clone(),
                    ValueType::Number,
                    None,
                ));
            }
            "STRING" => {
                result.push(TypedToken::new(
                    token.literal.clone(),
                    ValueType::String,
                    None,
                ));
            }
            "MINUS" => {
                result.push(TypedToken::new("-".to_string(), ValueType::Operation, None));

                send_sign = true;
            }
            "PLUS" => {
                result.push(TypedToken::new("+".to_string(), ValueType::Operation, None));

                send_sign = true;
            }
            "TRUE" | "FALSE" => {
                result.push(TypedToken::new(
                    token.lexeme.clone(),
                    ValueType::Boolean,
                    None,
                ));
            }
            "NIL" => {
                result.push(TypedToken::new(token.lexeme.clone(), ValueType::Nil, None));
            }
            "LEFT_PAREN" => {
                // Handle grouped expressions similar to before
                let mut inner_tokens = Vec::new();
                let mut paren_count = 1;
                i += 1;

                while i < tokens.len() && paren_count > 0 {
                    let inner_token = &tokens[i];
                    if inner_token.token_type == "LEFT_PAREN" {
                        paren_count += 1;
                    } else if inner_token.token_type == "RIGHT_PAREN" {
                        paren_count -= 1;
                        if paren_count == 0 {
                            break;
                        }
                    }
                    inner_tokens.push(inner_token.clone());
                    i += 1;
                }

                let inner_result = ev_parse_more(inner_tokens);
                if inner_result.is_empty() {
                    continue;
                }

                let inner_values: Vec<String> =
                    inner_result.iter().map(|t| t.value.clone()).collect();
                result.push(TypedToken::new(
                    format!("(group {})", inner_values.join(" ")),
                    ValueType::Group,
                    Some(inner_result),
                ));
                /*
                result.push(TypedToken::new(
                    format!("(group )"),
                    ValueType::Group,
                    Some(inner_tokens),
                ));
                */
            }
            "SLASH" => {
                result.push(TypedToken::new("/".to_string(), ValueType::Operation, None));
                send_sign = true;
            }
            "STAR" => {
                result.push(TypedToken::new("*".to_string(), ValueType::Operation, None));
                send_sign = true;
            }

            "BANG_EQUAL" | "EQUAL_EQUAL" => {
                //println!("bang equal or equal equal");
                result.push(TypedToken::new(
                    token.lexeme.clone(),
                    ValueType::Operation,
                    None,
                ));
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
                let inner_result = ev_parse_more(inner_tokens);
                result.push(TypedToken::new(
                    format!(
                        "(! {})",
                        inner_result
                            .iter()
                            .map(|token| format!("{}", token))
                            .collect::<Vec<String>>()
                            .join(", ")
                    ),
                    ValueType::Bang,
                    Some(inner_result),
                ));
            }
            "GREATER_EQUAL" | "LESS_EQUAL" | "GREATER" | "LESS" => {
                has_comparison = true;
                //println!("{}", token.lexeme);
                result.push(TypedToken::new(
                    token.lexeme.clone(),
                    ValueType::Comparison,
                    None,
                ));
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
    if has_comparison {
        if debug {
            for (num, tokensuwu) in result.clone().iter().enumerate() {
                println!(
                    "pars sign hit: result from parssign:, number: {} {}",
                    num, tokensuwu
                );
            }
        }
        result = ev_parse_equals(result);
    } else {
        if debug {
            for (num, tokensuwu) in result.clone().iter().enumerate() {
                println!("result from result1:, number: {} {}", num, tokensuwu);
            }
        }
    }
    if send_sign {
        if debug {
            println!("it has sign");
            for (num, tokensuwu) in result.clone().iter().enumerate() {
                println!(
                    "pars sign hit: result from parssign:, number: {} {}",
                    num, tokensuwu
                );
            }
        }
        result = ev_parse_signs(result);
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
        result = ev_parse_equal_equal(result);
    } else {
        if debug {
            for (num, tokensuwu) in result.clone().iter().enumerate() {
                println!("result from result1:, number: {} {}", num, tokensuwu);
            }
        }
    }

    result
}
pub fn ev_parse_equals(tokens: Vec<TypedToken>) -> Vec<TypedToken> {
    for tokensuwu in tokens.clone() {
        println!("received on ev_parse_equals: {}", tokensuwu);
    }
    let mut result = tokens.clone();
    let mut i = 1;
    while i < result.len() {
        match result[i].value.as_str() {
            "<" | ">" | ">=" | "<=" => {
                if result[i + 1].value.as_str() != "-" {
                    if i > 0 && i < result.len() - 1 {
                        let operator = result[i].clone();
                        let lhs = result[i - 1].clone();
                        let rhs = result[i + 1].clone();

                        let new_expr = format!("({} {} {})", operator, lhs, rhs);
                        result.splice(
                            i - 1..=i + 1,
                            vec![TypedToken::new(new_expr, ValueType::Comparison, None)],
                        );
                        continue;
                    } else if result[i + 1].value.as_str() == "-" {
                        let operator = result[i].clone();
                        let rhs = format!("(- {})", result[i + 2].clone());
                        let lhs = result[i - 1].clone();
                        let new_expr = format!("({} {} {})", operator, lhs, rhs);
                        result.splice(
                            i - 1..=i + 1,
                            vec![TypedToken::new(new_expr, ValueType::Comparison, None)],
                        );
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
fn ev_parse_signs(tokens: Vec<TypedToken>) -> Vec<TypedToken> {
    let debug = false;
    let mut result = tokens.clone();
    // Handle comparisons first
    let mut i = 1;
    while i < result.len() {
        let current_token = &result[i];
        if matches!(current_token.value.as_str(), "<" | ">" | ">=" | "<=") {
            if i > 0 && i < result.len() - 1 {
                let operator = current_token.value.clone();
                let lhs = &result[i - 1];
                let rhs = &result[i + 1];

                // Validate types for comparison
                if ev_validate_types(lhs, rhs, operator.clone()) {
                    let new_expr = TypedToken::new(
                        format!("({} {} {})", operator, lhs.value, rhs.value),
                        ValueType::Comparison,
                        None,
                    );
                    //println!("new exp: {:?}", new_expr);
                    result.splice(i - 1..=i + 1, vec![new_expr]);
                    continue;
                }
            }
        }
        i += 1;
    }

    // Handle unary negation
    let mut i = 0;
    while i < result.len() {
        if result[i].value == "-" {
            if i == 0
                || matches!(
                    result[i - 1].value.as_str(),
                    "+" | "-" | "*" | "/" | "(" | ">" | "<" | ">=" | "<="
                )
            {
                if i < result.len() - 1 {
                    let rhs = &result[i + 1];

                    // Only allow negation of numbers
                    if rhs.value_type == ValueType::Number {
                        let new_expr = TypedToken::new(
                            format!("(- {})", rhs.value),
                            ValueType::Negation,
                            None,
                        );
                        result.splice(i..=i + 1, vec![new_expr]);
                        continue;
                    }
                }
            }
        }
        i += 1;
    }

    // Handle multiplication and division
    let mut i = 0;
    while i < result.len() {
        if matches!(result[i].value.as_str(), "*" | "/") {
            if i > 0 && i < result.len() - 1 {
                let operator = result[i].value.clone();
                let lhs = &result[i - 1];
                let rhs = &result[i + 1];

                if ev_validate_types(lhs, rhs, operator.clone()) {
                    let new_expr = TypedToken::new(
                        format!("({} {} {})", operator, lhs.value, rhs.value),
                        ValueType::MulDiv,
                        Some(vec![lhs.to_owned(), rhs.to_owned()]),
                    );
                    result.splice(i - 1..=i + 1, vec![new_expr]);
                    continue;
                }
            }
        }
        i += 1;
    }

    // Handle addition and subtraction
    let mut i = 0;
    while i < result.len() {
        if matches!(result[i].value.as_str(), "+" | "-") {
            let has_error = false;

            // Check position errors
            if i == 0 || i == result.len() - 1 {
                true;
            }

            if !has_error && i > 0 && i < result.len() - 1 {
                let operator = result[i].value.clone();
                let lhs = &result[i - 1];
                let rhs = &result[i + 1];

                if ev_validate_types(lhs, rhs, operator.clone()) {
                    let new_expr = TypedToken::new(
                        format!("({} {} {})", operator, lhs.value, rhs.value),
                        ValueType::Operation,
                        None,
                    );
                    result.splice(i - 1..=i + 1, vec![new_expr]);
                    continue;
                }
            }
        }
        i += 1;
    }

    if debug {
        println!("Final result: {:?}", result);
    }

    result
}
impl fmt::Display for TypedToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
pub fn ev_validate_types(lhs: &TypedToken, rhs: &TypedToken, operator: String) -> bool {
    let mut has_error = false;

    if lhs.value.trim().is_empty() || ev_is_valid_operand(&lhs.value) {
        has_error = true;
    }

    if rhs.value.trim().is_empty() || ev_is_valid_operand(&rhs.value) {
        has_error = true;
    }

    has_error
}
pub fn ev_is_valid_operand(operand: &str) -> bool {
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
pub fn ev_parse_equal_equal(tokens: Vec<TypedToken>) -> Vec<TypedToken> {
    let mut result = tokens.clone();
    let mut i = 1;

    while i < result.len() {
        match result[i].value.as_str() {
            "==" | "!=" => {
                if i > 0 && i < result.len() - 1 {
                    let operator = result[i].clone();
                    let lhs = result[i - 1].clone();
                    let rhs = result[i + 1].clone();
                    let new_expr = format!("({} {} {})", operator, lhs, rhs);
                    result.splice(
                        i - 1..=i + 1,
                        vec![TypedToken::new(new_expr, ValueType::Operation, None)],
                    );
                    continue;
                }
            }
            _ => {}
        }
        i += 1;
    }
    result
}
