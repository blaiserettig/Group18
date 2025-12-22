use crate::tokenize::{Token, TokenType};
use std::collections::HashMap;
use std::vec;

#[derive(Debug)]
pub enum AbstractSyntaxTreeSymbol {
    AbstractSyntaxTreeSymbolEntry,
    AbstractSyntaxTreeSymbolFunctionDec {
        name: String,
        params: Vec<(Type, String)>,
        return_type: Type,
        body: Vec<AbstractSyntaxTreeNode>,
    },
    AbstractSyntaxTreeSymbolFunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    AbstractSyntaxTreeSymbolReturn(Option<Expr>),
    AbstractSyntaxTreeSymbolExit(Expr),
    AbstractSyntaxTreeSymbolVariableDeclaration {
        name: String,
        type_: Type,
        value: Expr,
    },
    AbstractSyntaxTreeSymbolVariableAssignment {
        name: String,
        value: Expr,
    },
    AbstractSyntaxTreeSymbolFor {
        iterator_name: String,
        iterator_begin: Expr,
        iterator_end: Expr,
        body: Vec<AbstractSyntaxTreeNode>,
    },
    AbstractSyntaxTreeSymbolIf {
        condition: Expr,
        body: Vec<AbstractSyntaxTreeNode>,
        else_body: Option<Box<AbstractSyntaxTreeNode>>,
    },
    AbstractSyntaxTreeSymbolBlock {
        body: Vec<AbstractSyntaxTreeNode>,
    },
}

#[derive(Debug)]
pub struct AbstractSyntaxTreeNode {
    pub symbol: AbstractSyntaxTreeSymbol,
    pub children: Vec<AbstractSyntaxTreeNode>,
}

#[derive(Debug, PartialEq)]
pub enum ParseTreeSymbol {
    ParseTreeSymbolNodeEntryPoint,
    ParseTreeSymbolNodeStatement,
    ParseTreeSymbolNodeExpression,
    ParseTreeSymbolNodeExit,
    ParseTreeSymbolNodeVariableDeclaration,
    ParseTreeSymbolNodeVariableAssignment,
    ParseTreeSymbolNodeType,
    ParseTreeSymbolNodeFor,
    ParseTreeSymbolNodeIf,
    ParseTreeSymbolNodeElse,
    ParseTreeSymbolNodeBlock,
    ParseTreeSymbolNodeFunctionDec,
    ParseTreeSymbolNodeFunctionCall,
    ParseTreeSymbolNodeReturn,
    ParseTreeSymbolNodeEquality,
    ParseTreeSymbolNodeComparison,
    ParseTreeSymbolNodeAdd,
    ParseTreeSymbolNodeMul,
    ParseTreeSymbolNodePrimary,
    ParseTreeSymbolTerminalExit,
    ParseTreeSymbolTerminalSemicolon,
    ParseTreeSymbolTerminalIntegerLiteral,
    ParseTreeSymbolTerminalEquals,
    ParseTreeSymbolTerminalI32S,
    ParseTreeSymbolTerminalF32S,
    ParseTreeSymbolTerminalBool,
    ParseTreeSymbolTerminalChar,
    ParseTreeSymbolTerminalString,
    ParseTreeSymbolTerminalFloatLiteral,
    ParseTreeSymbolTerminalCharLiteral,
    ParseTreeSymbolTerminalBooleanLiteral,
    ParseTreeSymbolTerminalStringLiteral,
    ParseTreeSymbolTerminalIdentifier,
    ParseTreeSymbolTerminalFor,
    ParseTreeSymbolTerminalForIn,
    ParseTreeSymbolTerminalForTo,
    ParseTreeSymbolTerminalIf,
    ParseTreeSymbolTerminalElse,
    ParseTreeSymbolTerminalLeftCurlyBrace,
    ParseTreeSymbolTerminalRightCurlyBrace,
    ParseTreeSymbolTerminalPlus,
    ParseTreeSymbolTerminalMinus,
    ParseTreeSymbolTerminalStar,
    ParseTreeSymbolTerminalSlash,
    ParseTreeSymbolTerminalLessThan,
    ParseTreeSymbolTerminalLessThanOrEqual,
    ParseTreeSymbolTerminalGreaterThan,
    ParseTreeSymbolTerminalGreaterThanOrEqual,
    ParseTreeSymbolTerminalEqualsEquals,
    ParseTreeSymbolTerminalNotEquals,
    ParseTreeSymbolTerminalLeftParen,
    ParseTreeSymbolTerminalRightParen,
    ParseTreeSymbolTerminalFn,
    ParseTreeSymbolTerminalReturn,
    ParseTreeSymbolTerminalVoid,
    ParseTreeSymbolTerminalArrow,
    ParseTreeSymbolTerminalComma,
    ParseTreeSymbolNodeArrayLiteral,
    ParseTreeSymbolNodeArrayIndex,
    ParseTreeSymbolTerminalLeftBracket,
    ParseTreeSymbolTerminalRightBracket,
}

#[derive(Debug)]
pub struct ParseTreeNode {
    symbol: ParseTreeSymbol,
    children: Vec<ParseTreeNode>,
    value: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I32S,
    F32S,
    Bool,
    Char,
    String,
    Void,
    Array(Box<Type>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i32),
    Float(f32),
    Bool(bool),
    Char(char),
    String(String),
    Ident(String),
    BinaryOp {
        left: Box<Expr>,
        op: BinOpType,
        right: Box<Expr>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    ArrayLiteral(Vec<Expr>),
    ArrayIndex {
        array: Box<Expr>,
        index: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum BinOpType {
    Multiply,
    Divide,
    Add,
    Subtract,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

struct VarEntry {
    var_type: Type,
    var_value: Expr,
}

pub struct Parser {
    tokens: Vec<Token>,
    token_index: usize,
    scopes: Vec<HashMap<String, VarEntry>>,
    functions: HashMap<String, (Vec<Type>, Type)>,
    return_type_stack: Vec<Type>,
    source_lines: Vec<String>,
    filename: String,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, source_code: String, filename: String) -> Self {
        let source_lines = source_code.lines().map(|s| s.to_string()).collect();
        Self {
            tokens,
            token_index: 0,
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
            return_type_stack: Vec::new(),
            source_lines,
            filename,
        }
    }

    // Assume the tokens are given to us starting from the entry point
    pub fn parse(&mut self) -> ParseTreeNode {
        self.parse_entry()
    }

    pub fn print_tree(&mut self, node: &ParseTreeNode, indent: usize) {
        for _i in 0..indent {
            print!("    ");
        }
        println!("{:?}", node.symbol);

        for _i in 0..indent {
            print!("    ");
        }
        println!("{:?}", node.value);

        for child in &node.children {
            self.print_tree(child, indent + 1);
        }
    }

    fn is_at_end(&self) -> bool {
        self.token_index >= self.tokens.len()
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.token_index)
    }

    fn format_error(&self, token: &Token, message: &str) -> String {
        let red = "\x1b[31m";
        let reset = "\x1b[0m";
        let mut error = format!(
            "{red}Fatal -- Parse Error{reset}\n{}\nAT\n{}:{}:{}\n---\n",
            message, self.filename, token.line, token.column
        );

        if token.line > 0 && token.line <= self.source_lines.len() {
            let line_content = &self.source_lines[token.line - 1];
            error.push_str(line_content);
            error.push('\n');
            for _ in 0..(token.column - 1) {
                error.push(' ');
            }
            error.push('^');
        }
        error
    }

    fn error_at_current(&self, message: &str) -> String {
        if let Some(token) = self.current() {
            self.format_error(token, message)
        } else if let Some(last_token) = self.tokens.last() {
            let red = "\x1b[31m";
            let reset = "\x1b[0m";
            format!(
                "{red}Fatal -- Parse Error{reset}\n{} (at end of input)\nAT\n{}:{}:{}\n---\n",
                message, self.filename, last_token.line, last_token.column
            )
        } else {
            format!("\x1b[31mFatal -- Error\x1b[0m: {}", message)
        }
    }

    fn consume(&mut self) -> &Token {
        let token = &self.tokens[self.token_index];
        self.token_index += 1;
        token
    }

    fn parse_entry(&mut self) -> ParseTreeNode {

        let mut entry_node = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeEntryPoint,
            children: Vec::new(),
            value: None,
        };

        while !self.is_at_end() {
            match self.parse_statement() {
                Ok(stmt) => entry_node.children.push(stmt),
                Err(e) => {
                    eprintln!("{}", e);
                    break;
                }
            }
        }
        entry_node
    }

    fn parse_statement(&mut self) -> Result<ParseTreeNode, String> {
        let token = &self.current().unwrap();

        let mut statement_node = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeStatement,
            children: Vec::new(),
            value: None,
        };

        match token.token_type {
            TokenType::TokenTypeExit => {
                statement_node.children.push(self.parse_exit()?);
                Ok(statement_node)
            }
            TokenType::TokenTypeTypeI32S => {
                statement_node
                    .children
                    .push(self.parse_variable_declaration()?);
                Ok(statement_node)
            }
            TokenType::TokenTypeTypeF32S => {
                statement_node
                    .children
                    .push(self.parse_variable_declaration()?);
                Ok(statement_node)
            }
            TokenType::TokenTypeTypeBool => {
                statement_node
                    .children
                    .push(self.parse_variable_declaration()?);
                Ok(statement_node)
            }
            TokenType::TokenTypeTypeChar => {
                statement_node
                    .children
                    .push(self.parse_variable_declaration()?);
                Ok(statement_node)
            }
            TokenType::TokenTypeTypeString => {
                statement_node
                    .children
                    .push(self.parse_variable_declaration()?);
                Ok(statement_node)
            }
            TokenType::TokenTypeIdentifier => {
                // if next token is '(', it's a function call
                // else assignment
                if self.tokens.get(self.token_index + 1).map(|t| t.token_type) == Some(TokenType::TokenTypeLeftParen) {
                     let call_node = self.parse_function_call_expr()?;
                     statement_node.children.push(call_node);
                     if self.consume().token_type != TokenType::TokenTypeSemicolon {
                         return Err("Expected ';' after function call statement".to_string());
                     }
                } else {
                    statement_node
                        .children
                        .push(self.parse_variable_assignment()?);
                }
                Ok(statement_node)
            }
            TokenType::TokenTypeFor => {
                statement_node.children.push(self.parse_for()?);
                Ok(statement_node)
            }
            TokenType::TokenTypeIf => {
                statement_node.children.push(self.parse_if()?);
                Ok(statement_node)
            }
            TokenType::TokenTypeLeftCurlyBrace => {
                self.push_scope();
                statement_node.children.push(self.parse_block()?);
                self.pop_scope();
                Ok(statement_node)
            }
            TokenType::TokenTypeFn => {
                statement_node.children.push(self.parse_function_dec()?);
                Ok(statement_node)
            }
            TokenType::TokenTypeReturn => {
                statement_node.children.push(self.parse_return()?);
                Ok(statement_node)
            }
            TokenType::TokenTypeIntegerLiteral
            | TokenType::TokenTypeFloatLiteral
            | TokenType::TokenTypeBooleanLiteral
            | TokenType::TokenTypeCharLiteral
            | TokenType::TokenTypeStringLiteral => {
                Err(self.format_error(token, &format!(
                    "unexpected literal at start of statement: {:?}",
                    token.token_type
                )))
            }
            _ => Err(self.format_error(token, &format!(
                "unrecognized token type: {:?}",
                token.token_type
            ))),
        }
    }

    fn parse_function_dec(&mut self) -> Result<ParseTreeNode, String> {
        // "fn" Ident "(" Ident* ")" "->" Type Block
        let fn_token = self.consume();
        let fn_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalFn,
            children: vec![],
            value: None,
        };

        let ident_token = self.current().ok_or_else(|| self.error_at_current("Expected identifier after fn"))?;
        if ident_token.token_type != TokenType::TokenTypeIdentifier {
            return Err(self.format_error(ident_token, "Expected identifier after fn"));
        }
        let ident_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalIdentifier,
            children: vec![],
            value: ident_token.value.clone(),
        };
        self.consume();

        if self.consume().token_type != TokenType::TokenTypeLeftParen {
            return Err(self.error_at_current("Expected '(' after function name"));
        }
        let lparen_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalLeftParen,
            children: vec![],
            value: None,
        };

        // PARAMS
        // (Type Ident, Type Ident, ...)
        let mut param_children = Vec::new();

        if self.current().map(|t| t.token_type) != Some(TokenType::TokenTypeRightParen) {
            loop {
                let type_node = self.parse_type()?;
                param_children.push(type_node);

                let p_ident = self.current().ok_or_else(|| self.error_at_current("Expected identifier in param"))?;
                if p_ident.token_type != TokenType::TokenTypeIdentifier {
                    return Err(self.format_error(p_ident, "Expected identifier in param"));
                }
                param_children.push(ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalIdentifier,
                    children: vec![],
                    value: p_ident.value.clone(),
                });
                self.consume();

                if self.current().map(|t| t.token_type) == Some(TokenType::TokenTypeComma) {
                    self.consume();
                } else {
                    break;
                }
            }
        }

        if self.consume().token_type != TokenType::TokenTypeRightParen {
            return Err(self.error_at_current("Expected ')' after parameters"));
        }
        let rparen_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalRightParen,
            children: vec![],
            value: None,
        };

        if self.consume().token_type != TokenType::TokenTypeArrow {
            return Err(self.error_at_current("Expected '->' after parameters"));
        }
        let arrow_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalArrow,
            children: vec![],
            value: None,
        };

        let return_type_node = self.parse_type()?;

        self.push_scope();

        // params MUST be inserted into scope before parsing block
        {
            // hacky reconstruction of params from param_children
            let mut i = 0;
            while i < param_children.len() {
                // [TypeNode, IdentNode] pairs
                let type_node = &param_children[i];
                let ident_node = &param_children[i+1];
                let type_val = self.match_type_in_scope(type_node);
                let name = ident_node.value.as_ref().unwrap().clone();
                // value is unknown at compile time, but we just need entry for lookup
                // we'll give it a dummy value
                self.insert_in_scope(name, VarEntry { var_type: type_val, var_value: Expr::Int(0)});
                i += 2;
            }
        }

        let block_node = self.parse_block()?;
        self.pop_scope();

        let mut children = vec![fn_terminal, ident_terminal, lparen_terminal];
        children.extend(param_children);
        children.push(rparen_terminal);
        children.push(arrow_terminal);
        children.push(return_type_node);
        children.push(block_node);

        Ok(ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeFunctionDec,
            children,
            value: None,
        })
    }

    fn parse_function_call_expr(&mut self) -> Result<ParseTreeNode, String> {
        // Ident "(" Expr* ")"
        let ident_token = self.consume();
        let ident_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalIdentifier,
            children: vec![],
            value: ident_token.value.clone(),
        };

        if self.consume().token_type != TokenType::TokenTypeLeftParen {
            return Err(self.error_at_current("Expected '(' after function name"));
        }
        let lparen_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalLeftParen,
            children: vec![],
            value: None,
        };

        let mut args_children = Vec::new();
        if self.current().map(|t| t.token_type) != Some(TokenType::TokenTypeRightParen) {
            loop {
                let expr = self.parse_expression()?;
                args_children.push(expr);
                if self.current().map(|t| t.token_type) == Some(TokenType::TokenTypeComma) {
                    self.consume();
                } else {
                    break;
                }
            }
        }

        if self.consume().token_type != TokenType::TokenTypeRightParen {
            return Err(self.error_at_current("Expected ')' after args"));
        }
        let rparen_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalRightParen,
            children: vec![],
            value: None,
        };

        let mut children = vec![ident_terminal, lparen_terminal];
        children.extend(args_children);
        children.push(rparen_terminal);

        Ok(ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeFunctionCall,
            children,
            value: None,
        })
    }

    fn parse_array_literal(&mut self) -> Result<ParseTreeNode, String> {
        let left_bracket = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalLeftBracket,
            children: vec![],
            value: None,
        };
        self.consume(); 

        let mut children = vec![left_bracket];
        if self.current().map(|t| t.token_type) != Some(TokenType::TokenTypeRightBracket) {
            loop {
                children.push(self.parse_expression()?);
                if self.current().map(|t| t.token_type) == Some(TokenType::TokenTypeComma) {
                    let comma = ParseTreeNode {
                        symbol: ParseTreeSymbol::ParseTreeSymbolTerminalComma,
                        children: vec![],
                        value: None,
                    };
                    self.consume();
                    children.push(comma);
                } else {
                    break;
                }
            }
        }

        if self.current().map(|t| t.token_type) != Some(TokenType::TokenTypeRightBracket) {
            return Err(self.error_at_current("Expected ']' at end of array literal"));
        }
        let right_bracket = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalRightBracket,
            children: vec![],
            value: None,
        };
        self.consume();
        children.push(right_bracket);

        Ok(ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeArrayLiteral,
            children,
            value: None,
        })
    }

    fn parse_array_index_expr(&mut self, ident_terminal: ParseTreeNode) -> Result<ParseTreeNode, String> {
        let left_bracket = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalLeftBracket,
            children: vec![],
            value: None,
        };
        self.consume(); // [

        let index_expr = self.parse_expression()?;

        if self.current().map(|t| t.token_type) != Some(TokenType::TokenTypeRightBracket) {
            return Err(self.error_at_current("Expected ']' after array index"));
        }
        let right_bracket = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalRightBracket,
            children: vec![],
            value: None,
        };
        self.consume();

        Ok(ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeArrayIndex,
            children: vec![ident_terminal, left_bracket, index_expr, right_bracket],
            value: None,
        })
    }

    fn parse_return(&mut self) -> Result<ParseTreeNode, String> {
        // "return" Expr ";"
        let ret_token = self.consume();
        let ret_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalReturn,
            children: vec![],
            value: None,
        };

        if self.current().map(|t| t.token_type) == Some(TokenType::TokenTypeSemicolon) {
            let semi_terminal = ParseTreeNode {
                symbol: ParseTreeSymbol::ParseTreeSymbolTerminalSemicolon,
                children: vec![],
                value: None,
            };
            self.consume();
            Ok(ParseTreeNode {
                symbol: ParseTreeSymbol::ParseTreeSymbolNodeReturn,
                children: vec![ret_terminal, semi_terminal],
                value: None,
            })
        } else {
            let expr = self.parse_expression()?;

            if self.consume().token_type != TokenType::TokenTypeSemicolon {
                return Err(self.error_at_current("Expected ';' after return"));
            }
            let semi_terminal = ParseTreeNode {
                symbol: ParseTreeSymbol::ParseTreeSymbolTerminalSemicolon,
                children: vec![],
                value: None,
            };

            Ok(ParseTreeNode {
                symbol: ParseTreeSymbol::ParseTreeSymbolNodeReturn,
                children: vec![ret_terminal, expr, semi_terminal],
                value: None,
            })
        }
    }

    fn parse_exit(&mut self) -> Result<ParseTreeNode, String> {
        let exit_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalExit,
            children: Vec::new(),
            value: None,
        };
        self.consume();

        let expr_node = self.parse_expression()?;

        let semi_terminal = if self
            .current()
            .map_or(false, |t| t.token_type == TokenType::TokenTypeSemicolon)
        {
            let node = ParseTreeNode {
                symbol: ParseTreeSymbol::ParseTreeSymbolTerminalSemicolon,
                children: Vec::new(),
                value: None,
            };
            self.consume();
            node
        } else {
            return Err(self.error_at_current(&format!(
                "expected Semicolon, found: {:?}",
                self.current().map(|t| &t.token_type)
            )));
        };

        Ok(ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeExit,
            children: vec![exit_terminal, expr_node, semi_terminal],
            value: None,
        })
    }

    fn parse_expression(&mut self) -> Result<ParseTreeNode, String> {
        let expr_content = self.parse_equality()?;
        Ok(ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeExpression,
            children: vec![expr_content],
            value: None,
        })
    }

    // Equality → Comparison (("==" | "!=") Comparison)*
    fn parse_equality(&mut self) -> Result<ParseTreeNode, String> {
        let mut left = self.parse_comparison()?;

        while let Some(token) = self.current() {
            match token.token_type {
                TokenType::TokenTypeEqualsEquals | TokenType::TokenTypeNotEquals => {
                    let op_type = token.token_type;
                    let op_terminal = ParseTreeNode {
                        symbol: match op_type {
                            TokenType::TokenTypeEqualsEquals => {
                                ParseTreeSymbol::ParseTreeSymbolTerminalEqualsEquals
                            }
                            TokenType::TokenTypeNotEquals => {
                                ParseTreeSymbol::ParseTreeSymbolTerminalNotEquals
                            }
                            _ => unreachable!(),
                        },
                        children: Vec::new(),
                        value: None,
                    };
                    self.consume();

                    let right = self.parse_comparison()?;

                    left = ParseTreeNode {
                        symbol: ParseTreeSymbol::ParseTreeSymbolNodeEquality,
                        children: vec![left, op_terminal, right],
                        value: None,
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // Comparison → Add (("<" | "<=" | ">" | ">=") Add)*
    fn parse_comparison(&mut self) -> Result<ParseTreeNode, String> {
        let mut left = self.parse_add()?;

        while let Some(token) = self.current() {
            match token.token_type {
                TokenType::TokenTypeLessThan
                | TokenType::TokenTypeLessThanOrEqual
                | TokenType::TokenTypeGreaterThan
                | TokenType::TokenTypeGreaterThanOrEqual => {
                    let op_type = token.token_type;
                    let op_terminal = ParseTreeNode {
                        symbol: match op_type {
                            TokenType::TokenTypeLessThan => {
                                ParseTreeSymbol::ParseTreeSymbolTerminalLessThan
                            }
                            TokenType::TokenTypeLessThanOrEqual => {
                                ParseTreeSymbol::ParseTreeSymbolTerminalLessThanOrEqual
                            }
                            TokenType::TokenTypeGreaterThan => {
                                ParseTreeSymbol::ParseTreeSymbolTerminalGreaterThan
                            }
                            TokenType::TokenTypeGreaterThanOrEqual => {
                                ParseTreeSymbol::ParseTreeSymbolTerminalGreaterThanOrEqual
                            }
                            _ => unreachable!(),
                        },
                        children: Vec::new(),
                        value: None,
                    };
                    self.consume();

                    let right = self.parse_add()?;

                    left = ParseTreeNode {
                        symbol: ParseTreeSymbol::ParseTreeSymbolNodeComparison,
                        children: vec![left, op_terminal, right],
                        value: None,
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // Add → → Mul (("+" | "-") Mul)*
    fn parse_add(&mut self) -> Result<ParseTreeNode, String> {
        let mut left = self.parse_mul()?;

        while let Some(token) = self.current() {
            match token.token_type {
                TokenType::TokenTypePlus | TokenType::TokenTypeMinus => {
                    let op_type = token.token_type;
                    let op_terminal = ParseTreeNode {
                        symbol: match op_type {
                            TokenType::TokenTypePlus => {
                                ParseTreeSymbol::ParseTreeSymbolTerminalPlus
                            }
                            TokenType::TokenTypeMinus => {
                                ParseTreeSymbol::ParseTreeSymbolTerminalMinus
                            }
                            _ => unreachable!(),
                        },
                        children: Vec::new(),
                        value: None,
                    };
                    self.consume();

                    let right = self.parse_mul()?;

                    left = ParseTreeNode {
                        symbol: ParseTreeSymbol::ParseTreeSymbolNodeAdd,
                        children: vec![left, op_terminal, right],
                        value: None,
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // Mul → Primary (("*" | "/") Primary)*
    fn parse_mul(&mut self) -> Result<ParseTreeNode, String> {
        let mut left = self.parse_primary()?;

        while let Some(token) = self.current() {
            match token.token_type {
                TokenType::TokenTypeMultiply | TokenType::TokenTypeDivide => {
                    let op_type = token.token_type;
                    let op_terminal = ParseTreeNode {
                        symbol: match op_type {
                            TokenType::TokenTypeMultiply => {
                                ParseTreeSymbol::ParseTreeSymbolTerminalStar
                            }
                            TokenType::TokenTypeDivide => {
                                ParseTreeSymbol::ParseTreeSymbolTerminalSlash
                            }
                            _ => unreachable!(),
                        },
                        children: Vec::new(),
                        value: None,
                    };
                    self.consume();

                    let right = self.parse_primary()?;

                    left = ParseTreeNode {
                        symbol: ParseTreeSymbol::ParseTreeSymbolNodeMul,
                        children: vec![left, op_terminal, right],
                        value: None,
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // Primary → Int_Lit | Float_Lit | Bool_Lit | Ident | "(" Expr ")"
    fn parse_primary(&mut self) -> Result<ParseTreeNode, String> {
        let token = self
            .current()
            .ok_or_else(|| self.error_at_current("Unexpected end of input in primary expression"))?;

        match token.token_type {
            TokenType::TokenTypeIntegerLiteral => {
                let child = ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalIntegerLiteral,
                    children: Vec::new(),
                    value: token.value.clone(),
                };
                self.consume();
                Ok(ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolNodePrimary,
                    children: vec![child],
                    value: None,
                })
            }

            TokenType::TokenTypeFloatLiteral => {
                let child = ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalFloatLiteral,
                    children: Vec::new(),
                    value: token.value.clone(),
                };
                self.consume();
                Ok(ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolNodePrimary,
                    children: vec![child],
                    value: None,
                })
            }

            TokenType::TokenTypeBooleanLiteral => {
                let child = ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalBooleanLiteral,
                    children: Vec::new(),
                    value: token.value.clone(),
                };
                self.consume();
                Ok(ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolNodePrimary,
                    children: vec![child],
                    value: None,
                })
            }

            TokenType::TokenTypeCharLiteral => {
                let child = ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalCharLiteral,
                    children: Vec::new(),
                    value: token.value.clone(),
                };
                self.consume();
                Ok(ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolNodePrimary,
                    children: vec![child],
                    value: None,
                })
            }

            TokenType::TokenTypeStringLiteral => {
                let child = ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalStringLiteral,
                    children: Vec::new(),
                    value: token.value.clone(),
                };
                self.consume();
                Ok(ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolNodePrimary,
                    children: vec![child],
                    value: None,
                })
            }

            TokenType::TokenTypeIdentifier => {
                let next_type = self.tokens.get(self.token_index + 1).map(|t| t.token_type);
                if next_type == Some(TokenType::TokenTypeLeftParen) {
                    let call_node = self.parse_function_call_expr()?;
                    Ok(ParseTreeNode {
                        symbol: ParseTreeSymbol::ParseTreeSymbolNodePrimary,
                        children: vec![call_node],
                        value: None,
                    })
                } else if next_type == Some(TokenType::TokenTypeLeftBracket) {
                    let ident_token = self.consume();
                    let ident_terminal = ParseTreeNode {
                        symbol: ParseTreeSymbol::ParseTreeSymbolTerminalIdentifier,
                        children: Vec::new(),
                        value: ident_token.value.clone(),
                    };
                    let idx_node = self.parse_array_index_expr(ident_terminal)?;
                    Ok(ParseTreeNode {
                        symbol: ParseTreeSymbol::ParseTreeSymbolNodePrimary,
                        children: vec![idx_node],
                        value: None,
                    })
                } else {
                    let child = ParseTreeNode {
                        symbol: ParseTreeSymbol::ParseTreeSymbolTerminalIdentifier,
                        children: Vec::new(),
                        value: token.value.clone(),
                    };
                    self.consume();
                    Ok(ParseTreeNode {
                        symbol: ParseTreeSymbol::ParseTreeSymbolNodePrimary,
                        children: vec![child],
                        value: None,
                    })
                }
            }

            TokenType::TokenTypeLeftBracket => {
                let array_lit = self.parse_array_literal()?;
                Ok(ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolNodePrimary,
                    children: vec![array_lit],
                    value: None,
                })
            }

            TokenType::TokenTypeLeftParen => {
                let left_paren = ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalLeftParen,
                    children: Vec::new(),
                    value: None,
                };
                self.consume();

                // Recursively parse the expression inside parentheses
                let expr_content = self.parse_equality()?;
                // Wrap it in an Expression node
                let expr = ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolNodeExpression,
                    children: vec![expr_content],
                    value: None,
                };

                let right_paren_token = self
                    .current()
                    .ok_or_else(|| self.error_at_current("Expected ')', found end of input"))?;
                if right_paren_token.token_type != TokenType::TokenTypeRightParen {
                    return Err(self.format_error(right_paren_token, &format!(
                        "Expected ')', found {:?}",
                        right_paren_token.token_type
                    )));
                }
                let right_paren = ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalRightParen,
                    children: Vec::new(),
                    value: None,
                };
                self.consume();

                Ok(ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolNodePrimary,
                    children: vec![left_paren, expr, right_paren],
                    value: None,
                })
            }

            _ => Err(self.format_error(token, &format!(
                "Unexpected token in primary expression: {:?}",
                token.token_type
            ))),
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<ParseTreeNode, String> {
        let type_node = self.parse_type()?;

        let ident_terminal = self.parse_expression()?;

        let equals_token = self
            .current()
            .ok_or_else(|| self.error_at_current("Expected '=', found end of input"))?;
        if equals_token.token_type != TokenType::TokenTypeEquals {
            return Err(self.format_error(equals_token, &format!(
                "Expected '=', found {:?}",
                equals_token.token_type
            )));
        }
        let equals_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalEquals,
            children: vec![],
            value: None,
        };
        self.consume();

        let expr_node = self.parse_expression()?;

        let semi_token = self
            .current()
            .ok_or_else(|| self.error_at_current("Expected semicolon, found end of input"))?;
        if semi_token.token_type != TokenType::TokenTypeSemicolon {
            return Err(self.format_error(semi_token, &format!(
                "Expected semicolon, found {:?}",
                semi_token.token_type
            )));
        }
        let semi_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalSemicolon,
            children: vec![],
            value: None,
        };
        self.consume();

        let var_name = self
            .find_terminal(&ident_terminal)
            .value
            .as_ref()
            .expect("Identifier should have a value")
            .clone();

        let var_type = self.match_type_in_scope(&type_node);
        let var_value = self.build_expr(&expr_node);
        if self.lookup_in_scope(&var_name).is_some() {
            return Err(self.error_at_current(&format!(
                "Duplicate variable name in same scope: {:?}",
                var_name
            )));
        }
        self.insert_in_scope(
            var_name,
            VarEntry {
                var_type,
                var_value,
            },
        );

        Ok(ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeVariableDeclaration,
            children: vec![
                type_node,
                ident_terminal,
                equals_terminal,
                expr_node,
                semi_terminal,
            ],
            value: None,
        })
    }

    fn parse_variable_assignment(&mut self) -> Result<ParseTreeNode, String> {
        let ident_token = self
            .current()
            .ok_or_else(|| self.error_at_current("Expected identifier, found end of input"))?;
        if ident_token.token_type != TokenType::TokenTypeIdentifier {
            return Err(self.format_error(ident_token, &format!(
                "Expected identifier, found {:?}",
                ident_token.token_type
            )));
        }
        let ident_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalIdentifier,
            children: vec![],
            value: ident_token.value.clone(),
        };
        self.consume();

        let equals_token = self
            .current()
            .ok_or_else(|| self.error_at_current("Expected '=', found end of input"))?;
        if equals_token.token_type != TokenType::TokenTypeEquals {
            return Err(self.format_error(equals_token, &format!(
                "Expected '=', found {:?}",
                equals_token.token_type
            )));
        }
        let equals_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalEquals,
            children: vec![],
            value: None,
        };
        self.consume();

        let expr_node = self.parse_expression()?;

        let semi_token = self
            .current()
            .ok_or_else(|| self.error_at_current("Expected semicolon, found end of input"))?;
        if semi_token.token_type != TokenType::TokenTypeSemicolon {
            return Err(self.format_error(semi_token, &format!(
                "Expected semicolon, found {:?}",
                semi_token.token_type
            )));
        }
        let semi_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalSemicolon,
            children: vec![],
            value: None,
        };
        self.consume();

        let var_name = ident_terminal
            .value
            .as_ref()
            .expect("Identifier should have a value")
            .clone();
        let var_value = self.build_expr(&expr_node).clone();
        if self.lookup_in_scope(&var_name).is_none() {
            return Err(self.error_at_current(&format!("Undefined variable {}", var_name)));
        }
        self.update_in_scope(&var_name, var_value)?;

        Ok(ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeVariableAssignment,
            children: vec![ident_terminal, equals_terminal, expr_node, semi_terminal],
            value: None,
        })
    }

    fn parse_type(&mut self) -> Result<ParseTreeNode, String> {
        let mut type_node = self.parse_base_type()?;

        while self.current() != None
            && self.current().unwrap().token_type == TokenType::TokenTypeLeftBracket
        {
            self.consume();
            if self.current() == None || self.current().unwrap().token_type != TokenType::TokenTypeRightBracket {
                return Err(self.error_at_current("Expected ']' after '[' in array type"));
            }
            self.consume();

            let right_bracket_terminal = ParseTreeNode {
                symbol: ParseTreeSymbol::ParseTreeSymbolTerminalRightBracket,
                children: vec![],
                value: None,
            };
            let left_bracket_terminal = ParseTreeNode {
                symbol: ParseTreeSymbol::ParseTreeSymbolTerminalLeftBracket,
                children: vec![],
                value: None,
            };

            type_node = ParseTreeNode {
                symbol: ParseTreeSymbol::ParseTreeSymbolNodeType,
                children: vec![type_node, left_bracket_terminal, right_bracket_terminal],
                value: None,
            };
        }

        Ok(type_node)
    }

    fn parse_base_type(&mut self) -> Result<ParseTreeNode, String> {
        if self.current() != None
            && self.current().unwrap().token_type == TokenType::TokenTypeTypeI32S
        {
            let node = ParseTreeNode {
                symbol: ParseTreeSymbol::ParseTreeSymbolNodeType,
                children: vec![ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalI32S,
                    children: Vec::new(),
                    value: None,
                }],
                value: None,
            };
            self.consume();
            Ok(node)
        } else if self.current() != None
            && self.current().unwrap().token_type == TokenType::TokenTypeTypeF32S
        {
            let node = ParseTreeNode {
                symbol: ParseTreeSymbol::ParseTreeSymbolNodeType,
                children: vec![ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalF32S,
                    children: Vec::new(),
                    value: None,
                }],
                value: None,
            };
            self.consume();
            Ok(node)
        } else if self.current() != None
            && self.current().unwrap().token_type == TokenType::TokenTypeTypeBool
        {
            let node = ParseTreeNode {
                symbol: ParseTreeSymbol::ParseTreeSymbolNodeType,
                children: vec![ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalBool,
                    children: Vec::new(),
                    value: None,
                }],
                value: None,
            };
            self.consume();
            Ok(node)
        } else if self.current() != None
            && self.current().unwrap().token_type == TokenType::TokenTypeTypeVoid
        {
            let node = ParseTreeNode {
                symbol: ParseTreeSymbol::ParseTreeSymbolNodeType,
                children: vec![ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalVoid,
                    children: Vec::new(),
                    value: None,
                }],
                value: None,
            };
            self.consume();
            Ok(node)
        } else if self.current() != None
            && self.current().unwrap().token_type == TokenType::TokenTypeTypeChar
        {
            let node = ParseTreeNode {
                symbol: ParseTreeSymbol::ParseTreeSymbolNodeType,
                children: vec![ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalChar,
                    children: Vec::new(),
                    value: None,
                }],
                value: None,
            };
            self.consume();
            Ok(node)
        } else if self.current() != None
            && self.current().unwrap().token_type == TokenType::TokenTypeTypeString
        {
            let node = ParseTreeNode {
                symbol: ParseTreeSymbol::ParseTreeSymbolNodeType,
                children: vec![ParseTreeNode {
                    symbol: ParseTreeSymbol::ParseTreeSymbolTerminalString,
                    children: Vec::new(),
                    value: None,
                }],
                value: None,
            };
            self.consume();
            Ok(node)
        } else {
            Err(self.error_at_current("expected Type"))
        }
    }

    fn parse_for(&mut self) -> Result<ParseTreeNode, String> {
        if self.current().unwrap().token_type != TokenType::TokenTypeFor {
            return Err(self.error_at_current("Expected 'for'"));
        }
        let terminal_for = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalFor,
            children: vec![],
            value: None,
        };
        self.consume();

        let ident_node = self.parse_expression()?;

        if self.current().unwrap().token_type != TokenType::TokenTypeForIn {
            return Err(self.error_at_current("Expected 'in'"));
        }
        let terminal_for_in = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalForIn,
            children: vec![],
            value: None,
        };
        self.consume();

        let lower_bound_node = self.parse_expression()?;

        if self.current().unwrap().token_type != TokenType::TokenTypeForTo {
            return Err(self.error_at_current("Expected 'to'"));
        }
        let terminal_for_dot = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalForTo,
            children: vec![],
            value: None,
        };
        self.consume();

        let upper_bound_node = self.parse_expression()?;

        if self.current().unwrap().token_type != TokenType::TokenTypeLeftCurlyBrace {
            return Err(self.error_at_current("Expected '{' at start of for-loop block"));
        }

        self.push_scope();

        // push iterator while inside the new scope
        let var_name = self
            .find_terminal(&ident_node)
            .value
            .as_ref()
            .expect("Identifier should have a value")
            .clone();

        let var_type = Type::I32S;
        let var_value = self.build_expr(&lower_bound_node);
        self.insert_in_scope(
            var_name,
            VarEntry {
                var_type,
                var_value,
            },
        );

        let block_node = self.parse_block()?;
        self.pop_scope();

        Ok(ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeFor,
            children: vec![
                terminal_for,
                ident_node,
                terminal_for_in,
                lower_bound_node,
                terminal_for_dot,
                upper_bound_node,
                block_node,
            ],
            value: None,
        })
    }

    fn parse_if(&mut self) -> Result<ParseTreeNode, String> {
        if self.current().unwrap().token_type != TokenType::TokenTypeIf {
            return Err(self.error_at_current("Expected 'if'"));
        }
        let if_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalIf,
            children: vec![],
            value: None,
        };
        self.consume();

        let expr_node = self.parse_expression()?;

        self.push_scope();
        let block_node = self.parse_block()?;
        self.pop_scope();

        let else_node = self.parse_else()?;

        Ok(ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeIf,
            children: vec![if_terminal, expr_node, block_node, else_node],
            value: None,
        })
    }

    fn parse_else(&mut self) -> Result<ParseTreeNode, String> {
        if self.current().unwrap().token_type != TokenType::TokenTypeElse {
            return Ok(ParseTreeNode {
                symbol: ParseTreeSymbol::ParseTreeSymbolNodeElse,
                children: vec![],
                value: None,
            });
        }
        let else_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalElse,
            children: vec![],
            value: None,
        };
        self.consume();

        let child: ParseTreeNode = match self.current().map(|t| t.token_type) {
            Some(TokenType::TokenTypeIf) => self.parse_if()?,
            Some(TokenType::TokenTypeLeftCurlyBrace) => {
                self.push_scope();
                let block = self.parse_block()?;
                self.pop_scope();
                block
            }
            other => {
                return Err(self.error_at_current(&format!("Unexpected token after `else`: {:?}", other)));
            }
        };

        Ok(ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeElse,
            children: vec![else_terminal, child],
            value: None,
        })
    }

    fn parse_block(&mut self) -> Result<ParseTreeNode, String> {
        if self.current().unwrap().token_type != TokenType::TokenTypeLeftCurlyBrace {
            return Err(self.error_at_current("Expected '{' at start of block"));
        }
        let left_bracket_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalLeftCurlyBrace,
            children: vec![],
            value: None,
        };
        self.consume();

        let mut statements = Vec::new();

        while let Some(tok) = self.current() {
            if tok.token_type == TokenType::TokenTypeRightCurlyBrace {
                break; // end of block
            }

            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }

        if self.current().unwrap().token_type != TokenType::TokenTypeRightCurlyBrace {
            return Err(self.error_at_current("Expected '}'"));
        }
        let right_bracket_terminal = ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolTerminalRightCurlyBrace,
            children: vec![],
            value: None,
        };
        self.consume();

        let mut children = Vec::new();
        children.push(left_bracket_terminal);
        children.extend(statements);
        children.push(right_bracket_terminal);

        Ok(ParseTreeNode {
            symbol: ParseTreeSymbol::ParseTreeSymbolNodeBlock,
            children,
            value: None,
        })
    }

    pub fn print_ast(&mut self, node: &AbstractSyntaxTreeNode, indent: usize) {
        for _i in 0..indent {
            print!("  ");
        }
        println!("{:?}", node.symbol);

        for child in &node.children {
            self.print_ast(child, indent + 1);
        }
    }

    fn get_expr_type(&self, expr: &Expr) -> Type {
        match expr {
            Expr::Int(_) => Type::I32S,
            Expr::Float(_) => Type::F32S,
            Expr::Bool(_) => Type::Bool,
            Expr::Char(_) => Type::Char,
            Expr::String(_) => Type::String,
            Expr::Ident(name) => {
                for scope in self.scopes.iter().rev() {
                    if let Some(entry) = scope.get(name) {
                        return entry.var_type.clone();
                    }
                }
                panic!("TypeChecker: Undefined variable '{}'", name);
            }
            Expr::BinaryOp { left, op, right: _ } => {
                // for now, strict typing. left and right must match. and result type depends on op
                // comparisons return bool. arithmetic returns operand type
                let left_type = self.get_expr_type(left);
                match op {
                    BinOpType::Equal
                    | BinOpType::NotEqual
                    | BinOpType::LessThan
                    | BinOpType::LessThanOrEqual
                    | BinOpType::GreaterThan
                    | BinOpType::GreaterThanOrEqual => Type::Bool,
                    _ => left_type, // add, sub, mul, div return same type as operands
                }
            }
            Expr::FunctionCall { name, args: _ } => {
                if let Some((_, return_type)) = self.functions.get(name) {
                     return_type.clone()
                } else {
                    panic!("TypeChecker: Undefined function '{}'", name);
                }
            }
            Expr::ArrayLiteral(elements) => {
                if elements.is_empty() {
                    Type::Array(Box::new(Type::I32S))
                } else {
                    let elem_type = self.get_expr_type(&elements[0]);
                    Type::Array(Box::new(elem_type))
                }
            }
            Expr::ArrayIndex { array, index: _ } => {
                let array_type = self.get_expr_type(array);
                match array_type {
                    Type::Array(inner) => *inner,
                    _ => panic!("TypeChecker: Attempted to index non-array type"),
                }
            }
        }
    }

    pub fn build_ast(&mut self, parse_tree: &ParseTreeNode) -> AbstractSyntaxTreeNode {
        match parse_tree.symbol {
            ParseTreeSymbol::ParseTreeSymbolNodeEntryPoint => {
                let entry_node = AbstractSyntaxTreeNode {
                    symbol: AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolEntry,
                    children: parse_tree
                        .children
                        .iter()
                        .filter_map(|child| match child.symbol {
                            ParseTreeSymbol::ParseTreeSymbolNodeStatement => {
                                Some(self.build_ast(child))
                            }
                            _ => None,
                        })
                        .collect(),
                };
                entry_node
            }

            ParseTreeSymbol::ParseTreeSymbolNodeStatement => {
                if let Some(first_child) = parse_tree.children.first() {
                    self.build_ast(first_child)
                } else {
                    panic!("Statement node has no children");
                }
            }

            ParseTreeSymbol::ParseTreeSymbolNodeExit => {
                // [exit, expression, semicolon]
                if let Some(expr_node) = parse_tree
                    .children
                    .iter()
                    .find(|c| c.symbol == ParseTreeSymbol::ParseTreeSymbolNodeExpression)
                {
                    let value_child_node = self.find_terminal(&expr_node);
                    let expr = match value_child_node.symbol {
                        ParseTreeSymbol::ParseTreeSymbolTerminalIntegerLiteral => {
                            let v = value_child_node
                                .value
                                .as_ref()
                                .unwrap()
                                .parse::<i32>()
                                .unwrap();
                            Expr::Int(v)
                        }
                        ParseTreeSymbol::ParseTreeSymbolTerminalFloatLiteral => {
                            let v = value_child_node
                                .value
                                .as_ref()
                                .unwrap()
                                .parse::<f32>()
                                .unwrap();
                            Expr::Float(v)
                        }
                        ParseTreeSymbol::ParseTreeSymbolTerminalBooleanLiteral => {
                            let v = value_child_node
                                .value
                                .as_ref()
                                .unwrap()
                                .parse::<bool>()
                                .unwrap();
                            Expr::Bool(v)
                        }
                        ParseTreeSymbol::ParseTreeSymbolTerminalIdentifier => {
                            let name = value_child_node.value.as_ref().unwrap().to_string();
                            Expr::Ident(name)
                        }
                        _ => panic!("Invalid expression in exit"),
                    };

                    AbstractSyntaxTreeNode {
                        symbol: AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolExit(expr),
                        children: Vec::new(),
                    }
                } else {
                    panic!("Exit statement has no expression child");
                }
            }

            ParseTreeSymbol::ParseTreeSymbolNodeVariableDeclaration => {
                // Children:
                // [0] = type
                // [1] = identifier
                // [2] = "="
                // [3] = expression
                // [4] = ";"

                let type_node = &parse_tree.children[0];
                let ident_node = &parse_tree.children[1];
                let expr_node = &parse_tree.children[3];

                let name = self
                    .find_terminal(ident_node)
                    .value
                    .as_ref()
                    .unwrap()
                    .clone();

                let value_expr = self.build_expr(expr_node);

                let declared_type = self.match_type_in_scope(type_node);
                let actual_type = self.get_expr_type(&value_expr);

                if declared_type != actual_type {
                     panic!("TypeChecker: Mismatched types in declaration of '{}'. Expected {:?}, found {:?}", name, declared_type, actual_type);
                }

                self.insert_in_scope(name.clone(), VarEntry { var_type: declared_type.clone(), var_value: value_expr.clone() });

                AbstractSyntaxTreeNode {
                    symbol: AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolVariableDeclaration {
                        name,
                        type_: declared_type,
                        value: value_expr,
                    },
                    children: vec![],
                }
            }

            ParseTreeSymbol::ParseTreeSymbolNodeVariableAssignment => {
                let identifier_node = parse_tree
                    .children
                    .iter()
                    .find(|c| c.symbol == ParseTreeSymbol::ParseTreeSymbolTerminalIdentifier)
                    .expect("Variable node has no terminal identifier");
                
                let name = identifier_node.value.as_ref().unwrap().clone();

                let expr_node = parse_tree
                    .children
                    .iter()
                    .find(|c| c.symbol == ParseTreeSymbol::ParseTreeSymbolNodeExpression)
                    .expect("Variable node has no expression");
                
                let value_expr = self.build_expr(expr_node);

                let actual_type = self.get_expr_type(&value_expr);
                
                 let var_type = {
                    let mut found_type = None;
                    for scope in self.scopes.iter().rev() {
                        if let Some(entry) = scope.get(&name) {
                            found_type = Some(entry.var_type.clone());
                            break;
                        }
                    }
                    if found_type.is_none() {
                        panic!("TypeChecker: Undefined variable '{}' in assignment", name);
                    }
                    found_type.unwrap()
                };

                if var_type != actual_type {
                    panic!("TypeChecker: Mismatched types in assignment to '{}'. Expected {:?}, found {:?}", name, var_type, actual_type);
                }

                AbstractSyntaxTreeNode {
                    symbol: AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolVariableAssignment {
                        name,
                        value: value_expr,
                    },
                    children: Vec::new(),
                }
            }

            ParseTreeSymbol::ParseTreeSymbolNodeFor => {
                let mut expr_nodes = parse_tree
                    .children
                    .iter()
                    .filter(|c| c.symbol == ParseTreeSymbol::ParseTreeSymbolNodeExpression);

                let id_expr = expr_nodes.next().expect("Missing iterator expression");
                let begin_expr = expr_nodes.next().expect("Missing begin expression");
                let end_expr = expr_nodes.next().expect("Missing end expression");

                let iterator_name = self.find_terminal(&id_expr).value.as_ref().unwrap().clone();

                let iterator_begin = {
                    let lit = self.find_terminal(&begin_expr);
                    Expr::Int(lit.value.as_ref().unwrap().parse().unwrap())
                };

                let iterator_end = {
                    let lit = self.find_terminal(&end_expr);
                    Expr::Int(lit.value.as_ref().unwrap().parse().unwrap())
                };

                let mut stmt_nodes = Vec::new();
                self.find_statements(parse_tree, &mut stmt_nodes);

                self.push_scope();
                self.insert_in_scope(iterator_name.clone(), VarEntry { var_type: Type::I32S, var_value: Expr::Int(0) });

                let body: Vec<AbstractSyntaxTreeNode> = stmt_nodes
                    .into_iter()
                    .map(|stmt| self.build_ast(stmt))
                    .collect();

                self.pop_scope();

                AbstractSyntaxTreeNode {
                    symbol: AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolFor {
                        iterator_name,
                        iterator_begin,
                        iterator_end,
                        body,
                    },
                    children: vec![],
                }
            }

            ParseTreeSymbol::ParseTreeSymbolNodeIf => {
                // Children:
                // [0] = terminal
                // [1] = expression
                // [2] = block
                // [3] = else

                // Else children
                // empty -> no else
                //
                // if node -> else if
                // block node -> else

                let condition_node = &parse_tree.children[1];
                let condition = self.build_expr(condition_node);

                let mut stmt_nodes = Vec::new();
                self.find_statements(&parse_tree.children[2], &mut stmt_nodes);
                let body: Vec<AbstractSyntaxTreeNode> = stmt_nodes
                    .into_iter()
                    .map(|stmt| self.build_ast(stmt))
                    .collect();

                if parse_tree.children[3].children.is_empty() {
                    // there is no else
                    AbstractSyntaxTreeNode {
                        symbol: AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolIf {
                            condition,
                            body,
                            else_body: None,
                        },
                        children: vec![],
                    }
                } else if parse_tree.children[3].children[1].symbol
                    == ParseTreeSymbol::ParseTreeSymbolNodeIf
                {
                    // there is an else if
                    let else_if = self.build_ast(&parse_tree.children[3].children[1]);
                    AbstractSyntaxTreeNode {
                        symbol: AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolIf {
                            condition,
                            body,
                            else_body: Some(Box::new(else_if)),
                        },
                        children: vec![],
                    }
                } else if parse_tree.children[3].children[1].symbol
                    == ParseTreeSymbol::ParseTreeSymbolNodeBlock
                {
                    // there is an else
                    let mut else_stmts = Vec::new();
                    self.find_statements(&parse_tree.children[3].children[1], &mut else_stmts);
                    let else_body = AbstractSyntaxTreeNode {
                        symbol: AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolBlock {
                            body: else_stmts.into_iter().map(|s| self.build_ast(s)).collect(),
                        },
                        children: vec![],
                    };
                    AbstractSyntaxTreeNode {
                        symbol: AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolIf {
                            condition,
                            body,
                            else_body: Some(Box::new(else_body)),
                        },
                        children: vec![],
                    }
                } else {
                    panic!("Unexpected parse tree node: {:?}", parse_tree.symbol);
                }
            }

            ParseTreeSymbol::ParseTreeSymbolNodeBlock => {
                let mut stmt_nodes = Vec::new();
                self.find_statements(parse_tree, &mut stmt_nodes);

                let body: Vec<AbstractSyntaxTreeNode> = stmt_nodes
                    .into_iter()
                    .map(|stmt| self.build_ast(stmt))
                    .collect();

                AbstractSyntaxTreeNode {
                    symbol: AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolBlock { body },
                    children: vec![],
                }
            }

            ParseTreeSymbol::ParseTreeSymbolNodeFunctionDec => {
                // Children:
                // [0] fn
                // [1] name
                // [2] (
                // [...] params (Type, Ident) [and commas]
                // [n] )
                // [n+1] ->
                // [n+2] Return Type
                // [n+3] Block
                let name = self
                    .find_terminal(&parse_tree.children[1])
                    .value
                    .as_ref()
                    .unwrap()
                    .clone();

                let mut params = Vec::new();
                let mut i = 3;
                while i < parse_tree.children.len() {
                    let child = &parse_tree.children[i];
                    if child.symbol == ParseTreeSymbol::ParseTreeSymbolTerminalRightParen {
                        i += 1;
                        break;
                    }
                    if child.symbol == ParseTreeSymbol::ParseTreeSymbolTerminalComma {
                        i += 1;
                        continue;
                    }

                    let type_node = child;
                    let type_val = self.match_type_in_scope(type_node);
                    i += 1;

                    let ident_child = &parse_tree.children[i];
                    let pname = ident_child.value.as_ref().unwrap().clone();
                    params.push((type_val, pname));
                    i += 1;
                }

                i += 1; // skip ->

                let return_type_node = &parse_tree.children[i];
                let return_type = self.match_type_in_scope(return_type_node);
                i += 1;

                let block_node = &parse_tree.children[i];

                self.functions.insert(name.clone(), (params.iter().map(|(t, _)| t.clone()).collect(), return_type.clone()));
                self.return_type_stack.push(return_type.clone());
                
                self.push_scope();
                 for (ptype, pname) in &params {
                    self.insert_in_scope(
                        pname.clone(),
                        VarEntry {
                            var_type: ptype.clone(),
                            var_value: Expr::Int(0),
                        },
                    );
                }

                let body: Vec<AbstractSyntaxTreeNode> = block_node.children
                    .iter()
                    .filter_map(|stmt| {
                         if stmt.symbol == ParseTreeSymbol::ParseTreeSymbolNodeStatement {
                             Some(self.build_ast(stmt))
                         } else {
                             None
                         }
                    })
                    .collect();

                self.pop_scope();
                self.return_type_stack.pop();

                AbstractSyntaxTreeNode {
                    symbol: AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolFunctionDec {
                        name,
                        params,
                        return_type,
                        body,
                    },
                    children: vec![],
                }
            }

            ParseTreeSymbol::ParseTreeSymbolNodeBlock => {
                self.push_scope();
                let mut stmt_nodes = Vec::new();
                self.find_statements(parse_tree, &mut stmt_nodes);

                let body: Vec<AbstractSyntaxTreeNode> = stmt_nodes
                    .into_iter()
                    .map(|stmt| self.build_ast(stmt))
                    .collect();
                self.pop_scope();

                AbstractSyntaxTreeNode {
                    symbol: AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolBlock { body },
                    children: vec![],
                }
            }

            ParseTreeSymbol::ParseTreeSymbolNodeFunctionCall => {
                let name = self
                    .find_terminal(&parse_tree.children[0])
                    .value
                    .as_ref()
                    .unwrap()
                    .clone();

                let mut args = Vec::new();
                for child in &parse_tree.children {
                    if child.symbol == ParseTreeSymbol::ParseTreeSymbolNodeExpression {
                        args.push(self.build_expr(child));
                    }
                }

                AbstractSyntaxTreeNode {
                    symbol: AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolFunctionCall {
                        name,
                        args,
                    },
                    children: vec![],
                }
            }

            ParseTreeSymbol::ParseTreeSymbolNodeReturn => {
                // [0] return [[1] expr] [last] ;
                // 2 children (ret, semi) or 3 (ret, expr, semi)
                if parse_tree.children.len() == 2 {
                     if let Some(expected_type) = self.return_type_stack.last() {
                         if *expected_type != Type::Void {
                              panic!("TypeChecker: Return type mismatch. Expected {:?}, found Void", expected_type);
                         }
                     } else {
                         panic!("TypeChecker: Return statement outside of function");
                     }
                     
                     AbstractSyntaxTreeNode {
                        symbol: AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolReturn(None),
                        children: vec![],
                    }
                } else {
                    let expr_node = &parse_tree.children[1];
                    let expr = self.build_expr(expr_node);
                    
                    if let Some(expected_type) = self.return_type_stack.last() {
                         let actual_type = self.get_expr_type(&expr);
                         if *expected_type != actual_type {
                              panic!("TypeChecker: Return type mismatch. Expected {:?}, found {:?}", expected_type, actual_type);
                         }
                    } else {
                         panic!("TypeChecker: Return statement outside of function");
                    }

                    AbstractSyntaxTreeNode {
                        symbol: AbstractSyntaxTreeSymbol::AbstractSyntaxTreeSymbolReturn(Some(expr)),
                        children: vec![],
                    }
                }
            }

            _ => {
                panic!("Unexpected parse tree node: {:?}", parse_tree.symbol);
            }
        }
    }

    fn build_primary(&mut self, node: &ParseTreeNode) -> Expr {
        // Parenthesized expression
        if node.children.len() == 3
            && node.children[0].symbol == ParseTreeSymbol::ParseTreeSymbolTerminalLeftParen
            && node.children[2].symbol == ParseTreeSymbol::ParseTreeSymbolTerminalRightParen
        {
            // children: "(", Expr, ")"
            return self.build_expr(&node.children[1]);
        }

        // Simple literal / identifier
        let child = node.children.first().unwrap();
        match child.symbol {
            ParseTreeSymbol::ParseTreeSymbolTerminalIntegerLiteral => {
                let value = child.value.as_ref().unwrap().parse::<i32>().unwrap();
                Expr::Int(value)
            }
            ParseTreeSymbol::ParseTreeSymbolTerminalFloatLiteral => {
                let value = child.value.as_ref().unwrap().parse::<f32>().unwrap();
                Expr::Float(value)
            }
            ParseTreeSymbol::ParseTreeSymbolTerminalBooleanLiteral => {
                let value = child.value.as_ref().unwrap().parse::<bool>().unwrap();
                Expr::Bool(value)
            }
            ParseTreeSymbol::ParseTreeSymbolTerminalIdentifier => {
                let ident = child.value.as_ref().unwrap().clone();
                if self.lookup_in_scope(&ident).is_none() {
                     panic!("Undefined identifier {}", ident);
                }
                Expr::Ident(ident)
            }
            ParseTreeSymbol::ParseTreeSymbolNodeFunctionCall => {
                // child is the FunctionCall node
                // children: Ident, (, Expr..., )
                let name = child.children[0].value.as_ref().unwrap().clone();
                
                let mut args = Vec::new();
                for grandchild in &child.children {
                     if grandchild.symbol == ParseTreeSymbol::ParseTreeSymbolNodeExpression {
                         args.push(self.build_expr(grandchild));
                     }
                }
                Expr::FunctionCall { name, args }
            }
            ParseTreeSymbol::ParseTreeSymbolTerminalCharLiteral => {
                let value = child.value.as_ref().unwrap().chars().next().unwrap();
                Expr::Char(value)
            }
            ParseTreeSymbol::ParseTreeSymbolTerminalStringLiteral => {
                let value = child.value.as_ref().unwrap().clone();
                Expr::String(value)
            }
            ParseTreeSymbol::ParseTreeSymbolNodeArrayLiteral => {
                let mut elements = Vec::new();
                for grandchild in &child.children {
                    if grandchild.symbol == ParseTreeSymbol::ParseTreeSymbolNodeExpression {
                        elements.push(self.build_expr(grandchild));
                    }
                }
                Expr::ArrayLiteral(elements)
            }
            ParseTreeSymbol::ParseTreeSymbolNodeArrayIndex => {
                // Ident, [, Expr, ]
                let array_ident = child.children[0].value.as_ref().unwrap().clone();
                let array_expr = Expr::Ident(array_ident);
                let index_expr = self.build_expr(&child.children[2]);
                Expr::ArrayIndex {
                    array: Box::new(array_expr),
                    index: Box::new(index_expr),
                }
            }
            _ => panic!("Unsupported expression type: {:?}", child.symbol),
        }
    }

    fn build_mul(&mut self, node: &ParseTreeNode) -> Expr {
        let mut expr = self.build_expr(&node.children[0]);

        let mut i = 1;
        while i < node.children.len() {
            let op = &node.children[i].symbol;
            let right = self.build_expr(&node.children[i + 1]);

            expr = match op {
                ParseTreeSymbol::ParseTreeSymbolTerminalStar => Expr::BinaryOp {
                    left: Box::new(expr),
                    op: BinOpType::Multiply,
                    right: Box::new(right),
                },

                ParseTreeSymbol::ParseTreeSymbolTerminalSlash => Expr::BinaryOp {
                    left: Box::new(expr),
                    op: BinOpType::Divide,
                    right: Box::new(right),
                },

                _ => panic!("Unexpected operator in Mul node"),
            };
            i += 2;
        }
        expr
    }

    fn build_add(&mut self, node: &ParseTreeNode) -> Expr {
        let mut expr = self.build_expr(&node.children[0]);

        let mut i = 1;
        while i < node.children.len() {
            let op = &node.children[i].symbol;
            let right = self.build_expr(&node.children[i + 1]);

            expr = match op {
                ParseTreeSymbol::ParseTreeSymbolTerminalPlus => Expr::BinaryOp {
                    left: Box::new(expr),
                    op: BinOpType::Add,
                    right: Box::new(right),
                },

                ParseTreeSymbol::ParseTreeSymbolTerminalMinus => Expr::BinaryOp {
                    left: Box::new(expr),
                    op: BinOpType::Subtract,
                    right: Box::new(right),
                },

                _ => panic!("Unexpected operator in Add node"),
            };
            i += 2;
        }
        expr
    }

    fn build_comparison(&mut self, node: &ParseTreeNode) -> Expr {
        let mut expr = self.build_expr(&node.children[0]);

        let mut i = 1;
        while i < node.children.len() {
            let op = &node.children[i].symbol;
            let right = self.build_expr(&node.children[i + 1]);

            expr = match op {
                ParseTreeSymbol::ParseTreeSymbolTerminalLessThan => Expr::BinaryOp {
                    left: Box::new(expr),
                    op: BinOpType::LessThan,
                    right: Box::new(right),
                },

                ParseTreeSymbol::ParseTreeSymbolTerminalLessThanOrEqual => Expr::BinaryOp {
                    left: Box::new(expr),
                    op: BinOpType::LessThanOrEqual,
                    right: Box::new(right),
                },

                ParseTreeSymbol::ParseTreeSymbolTerminalGreaterThan => Expr::BinaryOp {
                    left: Box::new(expr),
                    op: BinOpType::GreaterThan,
                    right: Box::new(right),
                },

                ParseTreeSymbol::ParseTreeSymbolTerminalGreaterThanOrEqual => Expr::BinaryOp {
                    left: Box::new(expr),
                    op: BinOpType::GreaterThanOrEqual,
                    right: Box::new(right),
                },

                _ => panic!("Unexpected operator in Comparison node"),
            };
            i += 2;
        }
        expr
    }

    fn build_equality(&mut self, node: &ParseTreeNode) -> Expr {
        let mut expr = self.build_expr(&node.children[0]);

        let mut i = 1;
        while i < node.children.len() {
            let op = &node.children[i].symbol;
            let right = self.build_expr(&node.children[i + 1]);

            expr = match op {
                ParseTreeSymbol::ParseTreeSymbolTerminalEqualsEquals => Expr::BinaryOp {
                    left: Box::new(expr),
                    op: BinOpType::Equal,
                    right: Box::new(right),
                },

                ParseTreeSymbol::ParseTreeSymbolTerminalNotEquals => Expr::BinaryOp {
                    left: Box::new(expr),
                    op: BinOpType::NotEqual,
                    right: Box::new(right),
                },

                _ => panic!("Unexpected operator in Equality node"),
            };
            i += 2;
        }
        expr
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn lookup_in_scope(&self, name: &str) -> Option<&VarEntry> {
        for scope in self.scopes.iter().rev() {
            if let Some(v) = scope.get(name) {
                return Some(v);
            }
        }
        None
    }

    fn insert_in_scope(&mut self, name: String, entry: VarEntry) {
        self.scopes.last_mut().unwrap().insert(name, entry);
    }

    fn update_in_scope(&mut self, name: &str, value: Expr) -> Result<(), String> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(var) = scope.get_mut(name) {
                var.var_value = value;
                return Ok(());
            }
        }
        Err(format!("Undefined variable {}", name))
    }

    fn match_type_in_scope(&mut self, node: &ParseTreeNode) -> Type {
        // base types have terminal first child
        if node.children.len() == 1 {
            match node.children.first().unwrap().symbol {
                ParseTreeSymbol::ParseTreeSymbolTerminalI32S => Type::I32S,
                ParseTreeSymbol::ParseTreeSymbolTerminalF32S => Type::F32S,
                ParseTreeSymbol::ParseTreeSymbolTerminalBool => Type::Bool,
                ParseTreeSymbol::ParseTreeSymbolTerminalChar => Type::Char,
                ParseTreeSymbol::ParseTreeSymbolTerminalString => Type::String,
                ParseTreeSymbol::ParseTreeSymbolTerminalVoid => Type::Void,
                _ => panic!("Unsupported type node"),
            }
        } else {
            // array type: [inner_type, [, ]]
            let inner_type = self.match_type_in_scope(&node.children[0]);
            Type::Array(Box::new(inner_type))
        }
    }

    fn build_expr(&mut self, node: &ParseTreeNode) -> Expr {
        let child: &ParseTreeNode;
        if node.symbol == ParseTreeSymbol::ParseTreeSymbolNodeExpression {
            child = node.children.first().unwrap();
        } else {
            child = node;
        }
        match child.symbol {
            ParseTreeSymbol::ParseTreeSymbolNodePrimary => self.build_primary(child),
            ParseTreeSymbol::ParseTreeSymbolNodeMul => self.build_mul(child),
            ParseTreeSymbol::ParseTreeSymbolNodeAdd => self.build_add(child),
            ParseTreeSymbol::ParseTreeSymbolNodeComparison => self.build_comparison(child),
            ParseTreeSymbol::ParseTreeSymbolNodeEquality => self.build_equality(child),
            _ => panic!("Unknown expression node: {:?}", node.symbol),
        }
    }

    fn find_terminal<'a>(&mut self, node: &'a ParseTreeNode) -> &'a ParseTreeNode {
        match node.symbol {
            ParseTreeSymbol::ParseTreeSymbolTerminalIntegerLiteral
            | ParseTreeSymbol::ParseTreeSymbolTerminalFloatLiteral
            | ParseTreeSymbol::ParseTreeSymbolTerminalBooleanLiteral
            | ParseTreeSymbol::ParseTreeSymbolTerminalCharLiteral
            | ParseTreeSymbol::ParseTreeSymbolTerminalIdentifier => return node,

            _ => {}
        }

        for child in &node.children {
            let result = self.find_terminal(child);
            return result;
        }

        panic!("No terminal node found in subtree");
    }

    fn find_statements<'a>(&self, node: &'a ParseTreeNode, out: &mut Vec<&'a ParseTreeNode>) {
        if node.symbol == ParseTreeSymbol::ParseTreeSymbolNodeStatement {
            out.push(node);
        }
        for child in &node.children {
            self.find_statements(child, out);
        }
    }
}