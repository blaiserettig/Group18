use std::process::exit;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    TokenTypeEntryPoint,
    TokenTypeExit,
    TokenTypeIntegerLiteral,
    TokenTypeSemicolon,
    TokenTypeEquals,
    TokenTypeIdentifier,
    TokenTypeTypeI32S,
    TokenTypeTypeF32S,
    TokenTypeTypeBool,
    TokenTypeTypeChar,
    TokenTypeTypeString,
    TokenTypeFloatLiteral,
    TokenTypeCharLiteral,
    TokenTypeBooleanLiteral,
    TokenTypeStringLiteral,
    TokenTypeFor,
    TokenTypeForIn,
    TokenTypeForTo,
    TokenTypeIf,
    TokenTypeElse,
    TokenTypeLeftCurlyBrace,
    TokenTypeRightCurlyBrace,
    TokenTypePlus,
    TokenTypeMinus,
    TokenTypeMultiply,
    TokenTypeDivide,
    TokenTypeLessThan,
    TokenTypeLessThanOrEqual,
    TokenTypeGreaterThan,
    TokenTypeGreaterThanOrEqual,
    TokenTypeEqualsEquals,
    TokenTypeNotEquals,
    TokenTypeLeftParen,
    TokenTypeRightParen,
    TokenTypeFn,
    TokenTypeReturn,
    TokenTypeTypeVoid,
    TokenTypeArrow,
    TokenTypeComma,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
    pub line: usize,
    pub column: usize,
}

pub struct Tokenizer {
    chars: Vec<char>,
    index: usize,
    line: usize,
    column: usize,
}

impl Tokenizer {
    pub fn new(input_string: String) -> Self {
        Self {
            chars: input_string.chars().collect(),
            index: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut buffer: Vec<char> = Vec::new();

        tokens.push(Token {
            token_type: TokenType::TokenTypeEntryPoint,
            value: None,
            line: 1,
            column: 1,
        });

        while !self.is_at_end() {
            if self.current().unwrap().is_ascii_whitespace() {
                self.consume();
                continue;
            }

            let start_line = self.line;
            let start_column = self.column;

            if self.current().unwrap().is_ascii_alphabetic() || self.current().unwrap() == '_' {
                buffer.push(self.consume());
                while self.current() != None && (self.current().unwrap().is_ascii_alphanumeric() || self.current().unwrap() == '_') {
                    buffer.push(self.consume());
                }
                if buffer == ['e', 'x', 'i', 't'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeExit,
                        value: None,
                        line: start_line,
                        column: start_column,
                    });
                } else if buffer == ['i', '3', '2', 's'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeTypeI32S,
                        value: None,
                        line: start_line,
                        column: start_column,
                    });
                } else if buffer == ['f', '3', '2', 's'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeTypeF32S,
                        value: None,
                        line: start_line,
                        column: start_column,
                    });
                } else if buffer == ['b', 'o', 'o', 'l'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeTypeBool,
                        value: None,
                        line: start_line,
                        column: start_column,
                    });
                } else if buffer == ['c', 'h', 'a', 'r'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeTypeChar,
                        value: None,
                        line: start_line,
                        column: start_column,
                    });
                } else if buffer == ['t', 'r', 'u', 'e'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeBooleanLiteral,
                        value: Some("true".to_string()),
                        line: start_line,
                        column: start_column,
                    });
                } else if buffer == ['f', 'a', 'l', 's', 'e'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeBooleanLiteral,
                        value: Some("false".to_string()),
                        line: start_line,
                        column: start_column,
                    });
                } else if buffer == ['f', 'o', 'r'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeFor,
                        value: None,
                        line: start_line,
                        column: start_column,
                    })
                } else if buffer == ['i', 'n'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeForIn,
                        value: None,
                        line: start_line,
                        column: start_column,
                    })
                } else if buffer == ['t', 'o'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeForTo,
                        value: None,
                        line: start_line,
                        column: start_column,
                    })
                } else if buffer == ['i', 'f'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeIf,
                        value: None,
                        line: start_line,
                        column: start_column,
                    })
                } else if buffer == ['e', 'l', 's', 'e'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeElse,
                        value: None,
                        line: start_line,
                        column: start_column,
                    })
                } else if buffer == ['f', 'n'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeFn,
                        value: None,
                        line: start_line,
                        column: start_column,
                    })
                } else if buffer == ['r', 'e', 't', 'u', 'r', 'n'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeReturn,
                        value: None,
                        line: start_line,
                        column: start_column,
                    })
                } else if buffer == ['v', 'o', 'i', 'd'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeTypeVoid,
                        value: None,
                        line: start_line,
                        column: start_column,
                    })
                } else if buffer == ['s', 't', 'r', 'i', 'n', 'g'] {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeTypeString,
                        value: None,
                        line: start_line,
                        column: start_column,
                    })
                } else {
                    // If not a keyword, it is an identifier
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeIdentifier,
                        value: Some(buffer.iter().collect()),
                        line: start_line,
                        column: start_column,
                    });
                }
            } else if self.current().unwrap().is_ascii_digit() {
                buffer.push(self.consume());
                while self.current() != None && self.current().unwrap().is_ascii_digit() {
                    buffer.push(self.consume());
                }
                if self.current() != None && self.current().unwrap() == '.' {
                    buffer.push(self.consume());
                    while self.current() != None && self.current().unwrap().is_ascii_digit() {
                        buffer.push(self.consume());
                    }
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeFloatLiteral,
                        value: Some(buffer.iter().collect()),
                        line: start_line,
                        column: start_column,
                    });
                } else {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeIntegerLiteral,
                        value: Some(buffer.iter().collect()),
                        line: start_line,
                        column: start_column,
                    });
                }
            } else if self.current().unwrap() == ';' {
                self.consume();
                tokens.push(Token {
                    token_type: TokenType::TokenTypeSemicolon,
                    value: None,
                    line: start_line,
                    column: start_column,
                });
            } else if self.current().unwrap() == '=' {
                self.consume();
                if self.current() == Some('=') {
                    self.consume();
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeEqualsEquals,
                        value: None,
                        line: start_line,
                        column: start_column,
                    });
                } else {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeEquals,
                        value: None,
                        line: start_line,
                        column: start_column,
                    });
                }
            } else if self.current().unwrap() == '!' {
                self.consume();
                if self.current() == Some('=') {
                    self.consume();
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeNotEquals,
                        value: None,
                        line: start_line,
                        column: start_column,
                    });
                } else {
                    eprintln!("Tokenization Error at line {}, column {}: '!' must be followed by '='", start_line, start_column);
                    exit(1);
                }
            } else if self.current().unwrap() == '<' {
                self.consume();
                if self.current() == Some('=') {
                    self.consume();
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeLessThanOrEqual,
                        value: None,
                        line: start_line,
                        column: start_column,
                    });
                } else {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeLessThan,
                        value: None,
                        line: start_line,
                        column: start_column,
                    });
                }
            } else if self.current().unwrap() == '>' {
                self.consume();
                if self.current() == Some('=') {
                    self.consume();
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeGreaterThanOrEqual,
                        value: None,
                        line: start_line,
                        column: start_column,
                    });
                } else {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeGreaterThan,
                        value: None,
                        line: start_line,
                        column: start_column,
                    });
                }
            } else if self.current().unwrap() == '+' {
                self.consume();
                tokens.push(Token {
                    token_type: TokenType::TokenTypePlus,
                    value: None,
                    line: start_line,
                    column: start_column,
                });
            } else if self.current().unwrap() == '-' {
                self.consume();
                if self.current() == Some('>') {
                    self.consume();
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeArrow, 
                        value: None,
                        line: start_line,
                        column: start_column,
                    });
                } else {
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeMinus,
                        value: None,
                        line: start_line,
                        column: start_column,
                    });
                }
            } else if self.current().unwrap() == ',' {
                self.consume();
                tokens.push(Token {
                    token_type: TokenType::TokenTypeComma,
                    value: None,
                    line: start_line,
                    column: start_column,
                });
            } else if self.current().unwrap() == '*' {
                self.consume();
                tokens.push(Token {
                    token_type: TokenType::TokenTypeMultiply,
                    value: None,
                    line: start_line,
                    column: start_column,
                });
            } else if self.current().unwrap() == '/' {
                self.consume();
                tokens.push(Token {
                    token_type: TokenType::TokenTypeDivide,
                    value: None,
                    line: start_line,
                    column: start_column,
                });
            } else if self.current().unwrap() == '(' {
                self.consume();
                tokens.push(Token {
                    token_type: TokenType::TokenTypeLeftParen,
                    value: None,
                    line: start_line,
                    column: start_column,
                });
            } else if self.current().unwrap() == ')' {
                self.consume();
                tokens.push(Token {
                    token_type: TokenType::TokenTypeRightParen,
                    value: None,
                    line: start_line,
                    column: start_column,
                });
            } else if self.current().unwrap() == '{' {
                self.consume();
                tokens.push(Token {
                    token_type: TokenType::TokenTypeLeftCurlyBrace,
                    value: None,
                    line: start_line,
                    column: start_column,
                });
            } else if self.current().unwrap() == '}' {
                self.consume();
                tokens.push(Token {
                    token_type: TokenType::TokenTypeRightCurlyBrace,
                    value: None,
                    line: start_line,
                    column: start_column,
                });
            } else if self.current().unwrap() == '\'' {
                self.consume(); // opening quote
                let char_val = self.consume();
                if self.current().unwrap() == '\'' {
                    self.consume(); // closing quote
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeCharLiteral,
                        value: Some(char_val.to_string()),
                        line: start_line,
                        column: start_column,
                    });
                } else {
                    eprintln!("Tokenization Error at line {}, column {}: Expected closing quote for char literal", start_line, start_column);
                    exit(1);
                }
            } else if self.current().unwrap() == '"' {
                self.consume();
                let mut string_content = String::new();
                while self.current() != None && self.current().unwrap() != '"' {
                    let ch = self.consume();
                    if ch == '\\' && self.current() != None {
                        // Handle escape sequences
                        let next = self.consume();
                        match next {
                            'n' => string_content.push('\n'),
                            't' => string_content.push('\t'),
                            'r' => string_content.push('\r'),
                            '\\' => string_content.push('\\'),
                            '"' => string_content.push('"'),
                            _ => {
                                string_content.push('\\');
                                string_content.push(next);
                            }
                        }
                    } else {
                        string_content.push(ch);
                    }
                }
                if self.current() != None && self.current().unwrap() == '"' {
                    self.consume();
                    tokens.push(Token {
                        token_type: TokenType::TokenTypeStringLiteral,
                        value: Some(string_content),
                        line: start_line,
                        column: start_column,
                    });
                } else {
                    eprintln!("Tokenization Error at line {}, column {}: Expected closing quote for string literal", start_line, start_column);
                    exit(1);
                }
            } else if self.current().unwrap().is_ascii_whitespace() {
                self.consume();
            } else {
                eprintln!("Tokenization Error at line {}, column {}: Unrecognized character '{}'", start_line, start_column, self.current().unwrap());
                exit(1);
            }
            buffer.clear();
        }
        tokens
    }

    pub fn current(&mut self) -> Option<char> {
        if self.index < self.chars.len() {
            Some(self.chars[self.index])
        } else {
            None
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.index >= self.chars.len()
    }

    pub fn consume(&mut self) -> char {
        let c: char = self.chars[self.index];
        self.index += 1;
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        c
    }
}
