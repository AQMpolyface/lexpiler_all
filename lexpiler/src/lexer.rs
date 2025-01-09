use std::collections::HashSet;

pub struct Lexer {
    bad: bool,
    line: u32,
    valid_chars: HashSet<char>,
    keywords: HashSet<String>,
}

impl Lexer {
    pub fn new() -> Self {
        let valid_chars: HashSet<char> =
            ['(', ')', '{', '}', '*', '.', ',', '+', '-', ';', '/', '\n']
                .iter()
                .cloned()
                .collect();

        let keywords: HashSet<String> = [
            "and", "class", "else", "false", "for", "fun", "if", "nil", "or", "print", "return",
            "super", "this", "true", "var", "while",
        ]
        .iter()
        .map(|&s| s.to_string())
        .collect();

        Lexer {
            bad: false,
            line: 1,
            valid_chars,
            keywords,
        }
    }

    pub fn tokenize(&mut self, content: &str) -> u8 {
        let mut token: String;
        let chars: Vec<char> = content.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if chars[i] == '\n' {
                self.line += 1;
            }
            if chars[i].is_whitespace() {
                i += 1;
                continue;
            }

            match chars[i] {
                '=' => {
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        println!("EQUAL_EQUAL == null");
                        i += 2;
                    } else {
                        println!("EQUAL = null");
                        i += 1;
                    }
                }
                '!' => {
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        println!("BANG_EQUAL != null");
                        i += 2;
                    } else {
                        println!("BANG ! null");
                        i += 1;
                    }
                }
                '<' => {
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        println!("LESS_EQUAL <= null");
                        i += 2;
                    } else {
                        println!("LESS < null");
                        i += 1;
                    }
                }
                '>' => {
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        println!("GREATER_EQUAL >= null");
                        i += 2;
                    } else {
                        println!("GREATER > null");
                        i += 1;
                    }
                }
                '/' => {
                    if i + 1 < chars.len() && chars[i + 1] == '/' {
                        while i < chars.len() && chars[i] != '\n' {
                            i += 1;
                        }
                    } else {
                        println!("SLASH / null");
                        i += 1;
                    }
                }
                '"' => {
                    let mut string_vec = String::new();
                    i += 1;
                    let mut is_terminated = false;

                    while i < chars.len() && chars[i] != '"' {
                        if chars[i] == '\n' {
                            eprintln!("[line {}] Error: Unterminated string.", self.line);
                            self.bad = true;
                            break;
                        }
                        string_vec.push(chars[i]);
                        i += 1;
                    }
                    if i < chars.len() && chars[i] == '"' {
                        i += 1;
                        is_terminated = true;
                    }

                    if is_terminated {
                        println!("STRING \"{}\" {}", string_vec, string_vec);
                    } else {
                        eprintln!("[line {}] Error: Unterminated string.", self.line);
                        self.bad = true;
                    }
                }
                c if self.valid_chars.contains(&c) => {
                    token = self.tokenize_more(c);
                    if !token.is_empty() {
                        println!("{}", token);
                    }
                    i += 1;
                }
                c if c.is_numeric() => {
                    let mut numbers = String::new();
                    while i < chars.len() && (chars[i].is_numeric() || chars[i] == '.') {
                        numbers.push(chars[i]);
                        i += 1;
                    }
                    if numbers.contains('.') {
                        let parts: Vec<&str> = numbers.split('.').collect();
                        if parts.len() == 2 {
                            // Remove trailing zeros from decimal part
                            let decimal_part = parts[1].trim_end_matches('0').to_string();
                            if decimal_part.is_empty() {
                                println!("NUMBER {} {}.0", numbers, parts[0]);
                            } else {
                                println!("NUMBER {} {}.{}", numbers, parts[0], decimal_part);
                            }
                        } else {
                            println!("NUMBER {} {}", numbers, numbers);
                        }
                    } else {
                        println!("NUMBER {} {}.0", numbers, numbers);
                    }
                }
                c if c.is_alphabetic() || c == '_' => {
                    let mut identifier = String::new();
                    while i < chars.len()
                        && !chars[i].is_whitespace()
                        && !self.valid_chars.contains(&chars[i])
                    {
                        identifier.push(chars[i]);
                        i += 1;
                    }
                    if self.keywords.contains(&identifier) {
                        let temp_token = self.check_word(&identifier);
                        println!("{}", temp_token);
                    } else {
                        println!("IDENTIFIER {} null", identifier);
                    }
                }
                _ => {
                    eprintln!(
                        "[line {}] Error: Unexpected character: {}",
                        self.line, chars[i]
                    );
                    self.bad = true;
                    i += 1;
                }
            }
        }

        println!("EOF  null");
        if self.bad {
            65
        } else {
            0
        }
    }

    fn tokenize_more(&mut self, c: char) -> String {
        match c {
            '(' => String::from("LEFT_PAREN ( null"),
            ')' => String::from("RIGHT_PAREN ) null"),
            '{' => String::from("LEFT_BRACE { null"),
            '}' => String::from("RIGHT_BRACE } null"),
            '*' => String::from("STAR * null"),
            '.' => String::from("DOT . null"),
            ',' => String::from("COMMA , null"),
            '+' => String::from("PLUS + null"),
            '-' => String::from("MINUS - null"),
            ';' => String::from("SEMICOLON ; null"),
            '/' => String::from("SLASH / null"),
            _ => {
                if !c.is_whitespace() {
                    eprintln!("[line {}] Error: Unexpected character: {}", self.line, c);
                    self.bad = true;
                }
                String::new()
            }
        }
    }

    fn check_word(&self, word: &str) -> String {
        match word {
            "and" => "AND and null",
            "class" => "CLASS class null",
            "else" => "ELSE else null",
            "false" => "FALSE false null",
            "for" => "FOR for null",
            "fun" => "FUN fun null",
            "if" => "IF if null",
            "nil" => "NIL nil null",
            "or" => "OR or null",
            "print" => "PRINT print null",
            "return" => "RETURN return null",
            "super" => "SUPER super null",
            "this" => "THIS this null",
            "true" => "TRUE true null",
            "var" => "VAR var null",
            "while" => "WHILE while null",
            _ => "",
        }
        .to_string()
    }
}
