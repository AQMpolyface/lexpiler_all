use crate::tokenizer::Token;
use std::collections::HashSet;
#[derive(Debug, Clone, PartialEq)] // Ensure ValueType is comparable
pub enum ValueType {
    Number,
    Negation,
    String,
    Boolean,
    Group,
    Operation,
    Nil,
    Bang,
    Identifier,
    Sign,
    //Keyword,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypedToken {
    pub value: String,
    pub value_type: ValueType,
    pub parenthesis: Option<Vec<TypedToken>>,
    //    pub line: usize,
}
#[derive(Debug)]
pub struct ParserError {
    pub message: String,
    pub line: usize,
}

impl ParserError {
    pub fn new(message: &str, line: usize) -> Self {
        Self {
            message: message.to_string(),
            line,
        }
    }
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
//maybe should do differen fn to parse, and make this fn just turn token into typedtoken?
pub fn parse(mut tokens: Vec<Token>) -> Result<Vec<TypedToken>, ParserError> {
    if tokens[tokens.len() - 1].token_type == "EOF" {
        tokens.pop();
    }

    /*tokens
    .clone()
    .into_iter()
    .for_each(|uwu| println!("uwu {}", uwu.lexeme));*/
    //println!("received {:?}", tokens);
    let minus_keywords: HashSet<&str> = [
        "PLUS",
        "MINUS",
        "STAR",
        "SLASH",
        "LEFT_BRACE",
        "LEFT_PAREN",
        "GREATER_EQUAL",
        "GREATER",
        "LESS_EQUAL",
        "LESS",
        "BANG_EQUAL",
        "BANG",
        "EQUAL_EQUAL",
        "EQUAL",
    ]
    .into_iter()
    .collect();

    let keywords: HashSet<&str> = [
        "AND", "CLASS", "ELSE", "FALSE", "FOR", "FUN", "IF", "NIL", "OR", "PRINT", "RETURN",
        "SUPER", "THIS", "TRUE", "VAR", "WHILE",
    ]
    .into_iter()
    .collect();
    let mut typed_vector: Vec<TypedToken> = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        let current_token = tokens[i].clone();
        /*println!(
            "current token: {}, position: {}",
            current_token.token_type, i
        );*/
        match current_token.token_type.as_str() {
            "TRUE" | "FALSE" => {
                typed_vector.push(TypedToken::new(
                    current_token.lexeme,
                    ValueType::Boolean,
                    None,
                ));
                //println!("UWU: {:?}", typed_vector);
                i += 1;
            }

            "NIL" => {
                typed_vector.push(TypedToken::new(current_token.lexeme, ValueType::Nil, None));
                i += 1;
            }
            "NUMBER" => {
                typed_vector.push(TypedToken::new(
                    current_token.literal,
                    ValueType::Number,
                    None,
                ));
                i += 1;
            }
            "STRING" => {
                typed_vector.push(TypedToken::new(
                    current_token.literal,
                    ValueType::String,
                    None,
                ));
                i += 1;
            }
            "IDENTIFIER" => {
                typed_vector.push(TypedToken::new(
                    current_token.literal,
                    ValueType::Identifier,
                    None,
                ));
                i += 1;
            }
            "MINUS" => {
                if i == 0 && tokens[i + 1].token_type == "NUMBER" {
                    typed_vector.push(TypedToken::new(
                        format!("(- {})", tokens[i + 1].literal),
                        ValueType::Negation,
                        None,
                    ));
                    //skip next number
                    i += 2;
                } else if minus_keywords.contains(tokens[i - 1].token_type.as_str())
                    && tokens[i + 1].token_type == "NUMBER"
                {
                    typed_vector.push(TypedToken::new(
                        format!("(- {})", tokens[i + 1].literal),
                        ValueType::Negation,
                        None,
                    ));
                    //skip next number
                    i += 1;
                } else {
                    typed_vector.push(TypedToken::new(current_token.lexeme, ValueType::Sign, None));

                    i += 1;
                }
            }
            "PLUS" | "SLASH" | "STAR" => {
                typed_vector.push(TypedToken::new(current_token.lexeme, ValueType::Sign, None));
                i += 1;
            }
            "BANG" => {
                let mut inside: Vec<Token> = Vec::new();
                if i >= tokens.len() - 1 {
                    return Err(ParserError::new(
                        "cannot end with a BANG token",
                        current_token.line,
                    ));
                }
                i += 1;
                while i < tokens.len() {
                    // Changed condition
                    let inner_token: Token = tokens[i].clone();
                    if inner_token.token_type == "BANG" {
                        inside.push(inner_token);
                        i += 1;
                        //continue;
                    } else if inner_token.token_type == "LEFT_PAREN" {
                        let mut paren_count = 1;
                        inside.push(inner_token.clone());
                        i += 1;

                        while i < tokens.len() && paren_count > 0 {
                            let inner_inner_token = tokens[i].clone();
                            if inner_inner_token.token_type == "LEFT_PAREN" {
                                paren_count += 1;
                            } else if inner_inner_token.token_type == "RIGHT_PAREN" {
                                paren_count -= 1;
                            }
                            inside.push(inner_inner_token);
                            i += 1;
                        }
                        if paren_count > 0 {
                            return Err(ParserError::new("unclosed parenthesis", inner_token.line));
                        }
                    } else {
                        inside.push(inner_token);
                        i += 1;
                    }
                }

                //println!("inside bang {:?}, len: {}", inside, inside.len());
                let uwu: Vec<TypedToken> = parse(inside)?;

                // println!("Parsed token: {:?}", uwu);
                let formatted_output = uwu
                    .iter()
                    .map(|t| t.value.clone()) // Extract values from parsed tokens
                    .collect::<Vec<String>>() // Collect as Vec<String>
                    .join(" "); // Join with spaces

                typed_vector.push(TypedToken::new(
                    format!("(! {})", formatted_output),
                    ValueType::Bang,
                    Some(uwu),
                ));
            }
            "LEFT_PAREN" => {
                let mut inside: Vec<Token> = Vec::new();
                if i >= tokens.len() - 1 {
                    return Err(ParserError::new(
                        "cannot end with a LEFT_PAREN token",
                        current_token.line,
                    ));
                }
                i += 1;
                let mut paren_count = 1;

                while i < tokens.len() {
                    let inner_token: Token = tokens[i].clone();
                    if inner_token.token_type == "LEFT_PAREN" {
                        paren_count += 1;
                        //println!("+1 paren: {}", paren_count);
                        inside.push(inner_token);
                    } else if inner_token.token_type == "RIGHT_PAREN" {
                        paren_count -= 1;
                        //println!("-1 paren: {}", paren_count);

                        //weird quirk: i dont know what the fuck is happening, somite i have to push the last paren and sometimes not
                        if paren_count == 0 {
                            i += 1; // Move past this closing paren
                            break;
                        }

                        inside.push(inner_token);
                    } else {
                        inside.push(inner_token);
                    }
                    i += 1;
                }

                if paren_count > 0 {
                    return Err(ParserError::new("unclosed parenthesis", current_token.line));
                }

                if inside.is_empty() {
                    typed_vector.push(TypedToken::new(
                        "(group )".to_string(),
                        ValueType::Group,
                        None,
                    ));
                } else {
                    //println!("inside pre_uwu: {:?}", inside);
                    let uwu: Vec<TypedToken> = parse(inside)?;

                    //println!("output of uwu inside paren : {:?}", uwu);

                    let formatted_output = uwu
                        .iter()
                        .map(|t| t.value.clone()) // Extract values from parsed tokens
                        .collect::<Vec<String>>() // Collect as Vec<String>
                        .join(" "); // Join with spaces
                    typed_vector.push(TypedToken::new(
                        format!("(group {})", formatted_output),
                        ValueType::Group,
                        Some(uwu),
                    ));
                }
            }
            e if keywords.contains(e) => {
                typed_vector.push(TypedToken::new(
                    current_token.lexeme,
                    ValueType::Group,
                    None,
                ));

                i += 1;
            }
            _ => {
                return Err(ParserError::new(
                    format!(
                        "Error: Unkwown character while parsing: {:?}",
                        current_token
                    )
                    .as_str(),
                    current_token.line,
                ))
            }
        }
        //i += 1;
    }

    /*typed_vector
    .clone()
    .into_iter()
    .for_each(|uwu| println!("typed uwu value uwu {}", uwu.value));*/

    let new_vec: Vec<TypedToken> = parse_expressions(typed_vector)?;
    /*new_vec
    .clone()
    .into_iter()
    .for_each(|uwu| println!("new vec uwu value{}", uwu.value));*/
    Ok(new_vec)
}

fn parse_expressions(mut result: Vec<TypedToken>) -> Result<Vec<TypedToken>, ParserError> {
    //let mut new_vec: Vec<TypedToken> = Vec::new();

    //println!("received in parse_expression: typed_vec: {:?}", result);
    //let mut i = 0;
    let mut i = 0;
    while i < result.len() {
        if matches!(result[i].value.as_str(), "*" | "/") {
            if i > 0 && i < result.len() - 1 {
                let operator = result[i].value.clone();
                let lhs = &result[i - 1];
                let rhs = &result[i + 1];

                let new_expr = TypedToken::new(
                    format!("({} {} {})", operator, lhs.value, rhs.value),
                    ValueType::Operation,
                    Some(vec![lhs.to_owned(), rhs.to_owned()]),
                );
                result.splice(i - 1..=i + 1, vec![new_expr]);
                continue;
            }
        }
        i += 1;
    }
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

                if lhs.value_type == ValueType::String {
                    let new_expr = TypedToken::new(
                        format!("({} {} {})", operator, lhs.value, rhs.value),
                        ValueType::Operation,
                        None,
                    );

                    result.splice(i - 1..=i + 1, vec![new_expr]);
                } else {
                    let new_expr = TypedToken::new(
                        format!("({} {} {})", operator, lhs.value, rhs.value),
                        ValueType::Operation,
                        None,
                    );

                    result.splice(i - 1..=i + 1, vec![new_expr]);
                }
                continue;
            }
        }
        i += 1;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bang_operator() {
        let input = vec![
            TypedToken::new("!".to_string(), ValueType::Bang, None),
            TypedToken::new("true".to_string(), ValueType::Boolean, None),
        ];

        let expected = vec![TypedToken::new(
            "(! true)".to_string(),
            ValueType::Bang,
            None,
        )];

        let result = parse_expressions(input).unwrap();
        assert_eq!(result, expected);
    }
}
