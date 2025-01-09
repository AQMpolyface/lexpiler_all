use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: String,
    pub lexeme: String,
    pub literal: String,
}

pub struct Parser {
    bad: bool,
    line: u32,
    valid_chars: HashSet<char>,
    keywords: HashSet<String>,
    tokens: Vec<Token>,
}

impl Parser {
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

        Parser {
            bad: false,
            line: 1,
            valid_chars,
            keywords,
            tokens: Vec::new(),
        }
    }

    pub fn parse(&mut self, content: &str) -> (u8, Vec<Token>) {
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
                        self.add_token("EQUAL_EQUAL", "==", "null");
                        i += 2;
                    } else {
                        self.add_token("EQUAL", "=", "null");
                        i += 1;
                    }
                }
                '!' => {
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        self.add_token("BANG_EQUAL", "!=", "null");
                        i += 2;
                    } else {
                        self.add_token("BANG", "!", "null");
                        i += 1;
                    }
                }
                '<' => {
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        self.add_token("LESS_EQUAL", "<=", "null");
                        i += 2;
                    } else {
                        self.add_token("LESS", "<", "null");
                        i += 1;
                    }
                }
                '>' => {
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        self.add_token("GREATER_EQUAL", ">=", "null");
                        i += 2;
                    } else {
                        self.add_token("GREATER", ">", "null");
                        i += 1;
                    }
                }
                '/' => {
                    if i + 1 < chars.len() && chars[i + 1] == '/' {
                        while i < chars.len() && chars[i] != '\n' {
                            i += 1;
                        }
                    } else {
                        self.add_token("SLASH", "/", "null");
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
                        self.add_token("STRING", &format!("\"{}\"", string_vec), &string_vec);
                    } else {
                        eprintln!("[line {}] Error: Unterminated string.", self.line);
                        self.bad = true;
                    }
                }
                c if self.valid_chars.contains(&c) => {
                    if let Some(token) = self.tokenize_more(c) {
                        self.tokens.push(token);
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
                        if parts.len() == 2 && parts[1].chars().all(|c| c == '0') {
                            self.add_token("NUMBER", &numbers, &format!("{}.0", parts[0]));
                        } else if parts.len() == 2 && parts[1].ends_with("0") {
                            let end = trim_zero(parts[1]);
                            self.add_token("NUMBER", &numbers, &format!("{}.{}", parts[0], end));
                        } else {
                            self.add_token("NUMBER", &numbers, &numbers);
                        }
                    } else {
                        self.add_token("NUMBER", &numbers, &format!("{}.0", numbers));
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
                        if let Some(token) = self.check_word(&identifier) {
                            self.tokens.push(token);
                        }
                    } else {
                        self.add_token("IDENTIFIER", &identifier, "null");
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

        self.add_token("EOF", "", "null");
        let exit_code = if self.bad { 65 } else { 0 };
        (exit_code, self.tokens.clone())
    }

    fn add_token(&mut self, token_type: &str, lexeme: &str, literal: &str) {
        self.tokens.push(Token {
            token_type: token_type.to_string(),
            lexeme: lexeme.to_string(),
            literal: literal.to_string(),
        });
    }

    fn tokenize_more(&mut self, c: char) -> Option<Token> {
        match c {
            '(' => Some(Token {
                token_type: "LEFT_PAREN".to_string(),
                lexeme: "(".to_string(),
                literal: "null".to_string(),
            }),
            ')' => Some(Token {
                token_type: "RIGHT_PAREN".to_string(),
                lexeme: ")".to_string(),
                literal: "null".to_string(),
            }),
            '{' => Some(Token {
                token_type: "LEFT_BRACE".to_string(),
                lexeme: "{".to_string(),
                literal: "null".to_string(),
            }),
            '}' => Some(Token {
                token_type: "RIGHT_BRACE".to_string(),
                lexeme: "}".to_string(),
                literal: "null".to_string(),
            }),
            '*' => Some(Token {
                token_type: "STAR".to_string(),
                lexeme: "*".to_string(),
                literal: "null".to_string(),
            }),
            '.' => Some(Token {
                token_type: "DOT".to_string(),
                lexeme: ".".to_string(),
                literal: "null".to_string(),
            }),
            ',' => Some(Token {
                token_type: "COMMA".to_string(),
                lexeme: ",".to_string(),
                literal: "null".to_string(),
            }),
            '+' => Some(Token {
                token_type: "PLUS".to_string(),
                lexeme: "+".to_string(),
                literal: "null".to_string(),
            }),
            '-' => Some(Token {
                token_type: "MINUS".to_string(),
                lexeme: "-".to_string(),
                literal: "null".to_string(),
            }),
            ';' => Some(Token {
                token_type: "SEMICOLON".to_string(),
                lexeme: ";".to_string(),
                literal: "null".to_string(),
            }),
            '/' => Some(Token {
                token_type: "SLASH".to_string(),
                lexeme: "/".to_string(),
                literal: "null".to_string(),
            }),
            _ => {
                if !c.is_whitespace() {
                    eprintln!("[line {}] Error: Unexpected character: {}", self.line, c);
                    self.bad = true;
                }
                None
            }
        }
    }

    fn check_word(&self, word: &str) -> Option<Token> {
        let (token_type, lexeme) = match word {
            "and" => ("AND", "and"),
            "class" => ("CLASS", "class"),
            "else" => ("ELSE", "else"),
            "false" => ("FALSE", "false"),
            "for" => ("FOR", "for"),
            "fun" => ("FUN", "fun"),
            "if" => ("IF", "if"),
            "nil" => ("NIL", "nil"),
            "or" => ("OR", "or"),
            "print" => ("PRINT", "print"),
            "return" => ("RETURN", "return"),
            "super" => ("SUPER", "super"),
            "this" => ("THIS", "this"),
            "true" => ("TRUE", "true"),
            "var" => ("VAR", "var"),
            "while" => ("WHILE", "while"),
            _ => return None,
        };

        Some(Token {
            token_type: token_type.to_string(),
            lexeme: lexeme.to_string(),
            literal: "null".to_string(),
        })
    }
}
fn trim_zero(number: &str) -> String {
    let trimmed = number.trim_end_matches('0');
    if trimmed.is_empty() {
        "0".to_string()
    } else {
        if trimmed.ends_with('.') {
            trimmed[..trimmed.len() - 1].to_string()
        } else {
            trimmed.to_string()
        }
    }
}
