// Parser - AST generation for Duck language
// Implements a recursive descent parser with quack authorization tracking

use crate::ast::{
    AssignTarget, BinaryOp, Block, Expr, Literal, MatchArm, Pattern, Statement, StringPart,
    UnaryOp,
};
use crate::lexer::{Token, TokenKind};

/// Parser for Duck language
/// Tracks quack count - when you see N quacks, the next N blocks are "authorized"
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    quack_count: usize, // pending quacks
    errors: Vec<String>,
}

impl Parser {
    /// Create a new parser from a token stream
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            pos: 0,
            quack_count: 0,
            errors: Vec::new(),
        }
    }

    /// Parse the entire token stream into a list of blocks
    pub fn parse(&mut self) -> Result<Vec<Block>, Vec<String>> {
        let mut blocks = Vec::new();

        while !self.is_at_end() {
            // Count consecutive quacks
            while self.check(TokenKind::Quack) {
                self.advance();
                self.quack_count += 1;
            }

            if self.is_at_end() {
                break;
            }

            // Parse a block if we see one
            if self.check(TokenKind::LeftBracket) {
                match self.parse_block() {
                    Ok(block) => blocks.push(block),
                    Err(e) => {
                        self.errors.push(e);
                        self.synchronize();
                    }
                }
            } else if !self.is_at_end() {
                // Unexpected token - skip it
                let token = self.advance();
                self.errors.push(format!(
                    "Unexpected token {:?} at line {}",
                    token.kind, token.line
                ));
            }
        }

        if self.errors.is_empty() {
            Ok(blocks)
        } else {
            Err(self.errors.clone())
        }
    }

    /// Parse a single block [...]
    fn parse_block(&mut self) -> Result<Block, String> {
        let line = self.current_line();
        self.expect(TokenKind::LeftBracket)?;

        // Determine if this block is authorized (was preceded by quack)
        let was_quacked = if self.quack_count > 0 {
            self.quack_count -= 1;
            true
        } else {
            false
        };

        // Parse the statement inside the block
        let statement = self.parse_statement()?;

        self.expect(TokenKind::RightBracket)?;

        Ok(Block {
            statement,
            was_quacked,
            line,
        })
    }

    /// Parse a statement inside a block
    fn parse_statement(&mut self) -> Result<Statement, String> {
        // Check for keywords to determine statement type
        if self.check(TokenKind::Let) {
            self.parse_let_statement()
        } else if self.check(TokenKind::Define) {
            self.parse_function_definition()
        } else if self.check(TokenKind::If) {
            self.parse_if_statement()
        } else if self.check(TokenKind::Match) {
            self.parse_match_statement()
        } else if self.check(TokenKind::Repeat) {
            self.parse_repeat_statement()
        } else if self.check(TokenKind::While) {
            self.parse_while_statement()
        } else if self.check(TokenKind::For) {
            self.parse_for_statement()
        } else if self.check(TokenKind::Return) {
            self.parse_return_statement()
        } else if self.check(TokenKind::Print) {
            self.parse_print_statement()
        } else if self.check(TokenKind::Struct) {
            self.parse_struct_definition()
        } else if self.check(TokenKind::Break) {
            self.advance();
            Ok(Statement::Break)
        } else if self.check(TokenKind::Continue) {
            self.advance();
            Ok(Statement::Continue)
        } else if self.check(TokenKind::Identifier) {
            self.parse_identifier_statement()
        } else {
            // Try to parse as expression
            let expr = self.parse_expression()?;
            Ok(Statement::Expression(expr))
        }
    }

    /// Parse: [let x be <expr>]
    fn parse_let_statement(&mut self) -> Result<Statement, String> {
        self.expect(TokenKind::Let)?;

        let name = self.expect_identifier()?;

        self.expect(TokenKind::Be)?;

        let value = self.parse_expression()?;

        Ok(Statement::Let { name, value })
    }

    /// Parse: [define name taking [params] as ...]
    fn parse_function_definition(&mut self) -> Result<Statement, String> {
        self.expect(TokenKind::Define)?;

        let name = self.expect_identifier()?;

        self.expect(TokenKind::Taking)?;

        // Parse parameter list [param1, param2, ...]
        self.expect(TokenKind::LeftBracket)?;
        let params = self.parse_parameter_list()?;
        self.expect(TokenKind::RightBracket)?;

        self.expect(TokenKind::As)?;

        // Parse function body - collect statements from nested blocks
        let body = self.parse_statement_body()?;

        Ok(Statement::FunctionDef { name, params, body })
    }

    /// Parse a list of identifiers separated by commas
    fn parse_parameter_list(&mut self) -> Result<Vec<String>, String> {
        let mut params = Vec::new();

        if self.check(TokenKind::RightBracket) {
            return Ok(params);
        }

        params.push(self.expect_identifier()?);

        while self.check(TokenKind::Comma) {
            self.advance();
            params.push(self.expect_identifier()?);
        }

        Ok(params)
    }

    /// Parse a body consisting of quacks and nested blocks
    fn parse_statement_body(&mut self) -> Result<Vec<Statement>, String> {
        let mut body = Vec::new();

        while !self.check(TokenKind::RightBracket) && !self.is_at_end() {
            // Count quacks
            while self.check(TokenKind::Quack) {
                self.advance();
                self.quack_count += 1;
            }

            if self.check(TokenKind::LeftBracket) {
                let block = self.parse_block()?;
                body.push(block.statement);
            } else if self.check(TokenKind::RightBracket) || self.is_at_end() {
                break;
            } else if self.check(TokenKind::Otherwise) {
                // End of then-branch
                break;
            } else {
                // Skip unexpected tokens in body
                self.advance();
            }
        }

        Ok(body)
    }

    /// Parse: [if <cond> then quack [...] otherwise quack [...]]
    fn parse_if_statement(&mut self) -> Result<Statement, String> {
        self.expect(TokenKind::If)?;

        let condition = self.parse_expression()?;

        self.expect(TokenKind::Then)?;

        let then_block = self.parse_statement_body()?;

        let otherwise_block = if self.check(TokenKind::Otherwise) {
            self.advance();
            Some(self.parse_statement_body()?)
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_block,
            otherwise_block,
        })
    }

    /// Parse: [match value with [when pattern then ...] ...]
    fn parse_match_statement(&mut self) -> Result<Statement, String> {
        self.expect(TokenKind::Match)?;

        let value = self.parse_expression()?;

        self.expect(TokenKind::With)?;

        let mut arms = Vec::new();

        // Parse match arms
        while self.check(TokenKind::LeftBracket) && !self.is_at_end() {
            arms.push(self.parse_match_arm()?);
        }

        Ok(Statement::Match { value, arms })
    }

    /// Parse a single match arm: [when pattern then quack [...]]
    fn parse_match_arm(&mut self) -> Result<MatchArm, String> {
        self.expect(TokenKind::LeftBracket)?;

        self.expect(TokenKind::When)?;

        let pattern = self.parse_pattern()?;

        self.expect(TokenKind::Then)?;

        let body = self.parse_statement_body()?;

        self.expect(TokenKind::RightBracket)?;

        Ok(MatchArm {
            pattern,
            expression: None,
            body: Some(body),
        })
    }

    /// Parse a pattern (for match arms)
    fn parse_pattern(&mut self) -> Result<Pattern, String> {
        if self.check(TokenKind::Underscore) {
            self.advance();
            Ok(Pattern::Wildcard)
        } else if self.check(TokenKind::Number) {
            let token = self.advance();
            let value: f64 = token.lexeme.parse().map_err(|_| "Invalid number")?;
            if value.fract() == 0.0 {
                Ok(Pattern::Literal(Literal::Int(value as i64)))
            } else {
                Ok(Pattern::Literal(Literal::Float(value)))
            }
        } else if self.check(TokenKind::StringLiteral) {
            let token = self.advance();
            Ok(Pattern::Literal(Literal::String(token.lexeme)))
        } else if self.check(TokenKind::True) {
            self.advance();
            Ok(Pattern::Literal(Literal::Bool(true)))
        } else if self.check(TokenKind::False) {
            self.advance();
            Ok(Pattern::Literal(Literal::Bool(false)))
        } else if self.check(TokenKind::Nil) {
            self.advance();
            Ok(Pattern::Literal(Literal::Nil))
        } else if self.check(TokenKind::Identifier) {
            let name = self.expect_identifier()?;
            Ok(Pattern::Variable(name))
        } else {
            Err(format!(
                "Expected pattern at line {}",
                self.current_line()
            ))
        }
    }

    /// Parse: [repeat N times quack [...]]
    fn parse_repeat_statement(&mut self) -> Result<Statement, String> {
        self.expect(TokenKind::Repeat)?;

        let count = self.parse_expression()?;

        self.expect(TokenKind::Times)?;

        let body = self.parse_statement_body()?;

        Ok(Statement::Repeat { count, body })
    }

    /// Parse: [while <cond> do quack [...]]
    fn parse_while_statement(&mut self) -> Result<Statement, String> {
        self.expect(TokenKind::While)?;

        let condition = self.parse_expression()?;

        self.expect(TokenKind::Do)?;

        let body = self.parse_statement_body()?;

        Ok(Statement::While { condition, body })
    }

    /// Parse: [for each [item] in collection do quack [...]]
    fn parse_for_statement(&mut self) -> Result<Statement, String> {
        self.expect(TokenKind::For)?;

        self.expect(TokenKind::Each)?;

        // Parse variable binding [item]
        self.expect(TokenKind::LeftBracket)?;
        let variable = self.expect_identifier()?;
        self.expect(TokenKind::RightBracket)?;

        self.expect(TokenKind::In)?;

        let iterable = self.parse_expression()?;

        self.expect(TokenKind::Do)?;

        let body = self.parse_statement_body()?;

        Ok(Statement::ForEach {
            variable,
            iterable,
            body,
        })
    }

    /// Parse: [return <expr>]
    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        self.expect(TokenKind::Return)?;

        // Check if there's an expression to return
        if self.check(TokenKind::RightBracket) {
            Ok(Statement::Return(None))
        } else {
            let value = self.parse_expression()?;
            Ok(Statement::Return(Some(value)))
        }
    }

    /// Parse: [print <expr>]
    fn parse_print_statement(&mut self) -> Result<Statement, String> {
        self.expect(TokenKind::Print)?;

        let value = self.parse_expression()?;

        Ok(Statement::Print(value))
    }

    /// Parse: [struct name with [field1, field2, ...]]
    fn parse_struct_definition(&mut self) -> Result<Statement, String> {
        self.expect(TokenKind::Struct)?;

        let name = self.expect_identifier()?;

        self.expect(TokenKind::With)?;

        // Parse field list [field1, field2, ...]
        self.expect(TokenKind::LeftBracket)?;
        let fields = self.parse_field_list()?;
        self.expect(TokenKind::RightBracket)?;

        Ok(Statement::StructDef { name, fields })
    }

    /// Parse struct field list
    fn parse_field_list(&mut self) -> Result<Vec<String>, String> {
        let mut fields = Vec::new();

        if self.check(TokenKind::RightBracket) {
            return Ok(fields);
        }

        fields.push(self.expect_identifier()?);

        while self.check(TokenKind::Comma) {
            self.advance();
            fields.push(self.expect_identifier()?);
        }

        Ok(fields)
    }

    /// Parse statements that start with an identifier
    fn parse_identifier_statement(&mut self) -> Result<Statement, String> {
        let name = self.expect_identifier()?;

        // Check what follows the identifier
        if self.check(TokenKind::Becomes) {
            // Assignment: [x becomes <expr>]
            self.advance();
            let value = self.parse_expression()?;
            Ok(Statement::Assign {
                target: AssignTarget::Variable(name),
                value,
            })
        } else if self.check(TokenKind::Dot) {
            // Field access/assignment: [obj.field becomes <expr>]
            self.advance();
            let field = self.expect_identifier()?;

            if self.check(TokenKind::Becomes) {
                self.advance();
                let value = self.parse_expression()?;
                Ok(Statement::Assign {
                    target: AssignTarget::Field {
                        object: Box::new(Expr::Identifier(name)),
                        field,
                    },
                    value,
                })
            } else {
                // Field access as expression
                let expr = Expr::FieldAccess {
                    object: Box::new(Expr::Identifier(name)),
                    field,
                };
                Ok(Statement::Expression(expr))
            }
        } else if self.check(TokenKind::Push) {
            // List push: [list push <value>]
            self.advance();
            let value = self.parse_expression()?;
            Ok(Statement::Push {
                list: Expr::Identifier(name),
                value,
            })
        } else if self.check(TokenKind::At) {
            // List index access or assignment
            self.advance();
            let index = self.parse_primary_expression()?;

            if self.check(TokenKind::Becomes) {
                self.advance();
                let value = self.parse_expression()?;
                Ok(Statement::Assign {
                    target: AssignTarget::Index {
                        object: Box::new(Expr::Identifier(name)),
                        index: Box::new(index),
                    },
                    value,
                })
            } else {
                let expr = Expr::Index {
                    object: Box::new(Expr::Identifier(name)),
                    index: Box::new(index),
                };
                Ok(Statement::Expression(expr))
            }
        } else {
            // Function call: [name arg1 arg2...] or just identifier
            let args = self.parse_call_arguments()?;
            if args.is_empty() {
                Ok(Statement::Expression(Expr::Identifier(name)))
            } else {
                Ok(Statement::Expression(Expr::Call {
                    callee: Box::new(Expr::Identifier(name)),
                    arguments: args,
                }))
            }
        }
    }

    /// Parse function call arguments (expressions until ])
    fn parse_call_arguments(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = Vec::new();

        while !self.check(TokenKind::RightBracket) && !self.is_at_end() {
            // Don't consume expressions that are part of control flow
            if self.is_end_of_call_args() {
                break;
            }
            args.push(self.parse_expression()?);
        }

        Ok(args)
    }

    /// Check if we've reached the end of call arguments
    fn is_end_of_call_args(&self) -> bool {
        self.check(TokenKind::RightBracket)
            || self.check(TokenKind::Then)
            || self.check(TokenKind::Otherwise)
            || self.check(TokenKind::Do)
            || self.check(TokenKind::Times)
            || self.check(TokenKind::With)
            || self.check(TokenKind::As)
    }

    // =============================================
    // Expression Parsing (Precedence Climbing)
    // =============================================

    /// Parse an expression with proper precedence
    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_or_expression()
    }

    /// Parse logical OR (lowest precedence)
    fn parse_or_expression(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and_expression()?;

        while self.check(TokenKind::Or) {
            self.advance();
            let right = self.parse_and_expression()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse logical AND
    fn parse_and_expression(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_equality_expression()?;

        while self.check(TokenKind::And) {
            self.advance();
            let right = self.parse_equality_expression()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator: BinaryOp::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse equality (==, !=)
    fn parse_equality_expression(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_comparison_expression()?;

        while self.check(TokenKind::EqualEqual) || self.check(TokenKind::NotEqual) {
            let op = if self.check(TokenKind::EqualEqual) {
                BinaryOp::Eq
            } else {
                BinaryOp::NotEq
            };
            self.advance();
            let right = self.parse_comparison_expression()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse comparison (<, >, <=, >=)
    fn parse_comparison_expression(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_additive_expression()?;

        while self.check(TokenKind::Less)
            || self.check(TokenKind::Greater)
            || self.check(TokenKind::LessEqual)
            || self.check(TokenKind::GreaterEqual)
        {
            let op = if self.check(TokenKind::Less) {
                BinaryOp::Lt
            } else if self.check(TokenKind::Greater) {
                BinaryOp::Gt
            } else if self.check(TokenKind::LessEqual) {
                BinaryOp::LtEq
            } else {
                BinaryOp::GtEq
            };
            self.advance();
            let right = self.parse_additive_expression()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse addition/subtraction
    fn parse_additive_expression(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplicative_expression()?;

        while self.check(TokenKind::Plus) || self.check(TokenKind::Minus) {
            let op = if self.check(TokenKind::Plus) {
                BinaryOp::Add
            } else {
                BinaryOp::Sub
            };
            self.advance();
            let right = self.parse_multiplicative_expression()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse multiplication/division/modulo
    fn parse_multiplicative_expression(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary_expression()?;

        while self.check(TokenKind::Star)
            || self.check(TokenKind::Slash)
            || self.check(TokenKind::Percent)
        {
            let op = if self.check(TokenKind::Star) {
                BinaryOp::Mul
            } else if self.check(TokenKind::Slash) {
                BinaryOp::Div
            } else {
                BinaryOp::Mod
            };
            self.advance();
            let right = self.parse_unary_expression()?;
            left = Expr::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse unary operators (not, -)
    fn parse_unary_expression(&mut self) -> Result<Expr, String> {
        if self.check(TokenKind::Not) {
            self.advance();
            let operand = self.parse_unary_expression()?;
            return Ok(Expr::Unary {
                operator: UnaryOp::Not,
                operand: Box::new(operand),
            });
        }

        if self.check(TokenKind::Minus) {
            self.advance();
            let operand = self.parse_unary_expression()?;
            return Ok(Expr::Unary {
                operator: UnaryOp::Neg,
                operand: Box::new(operand),
            });
        }

        self.parse_postfix_expression()
    }

    /// Parse postfix expressions (field access, list access, method calls)
    fn parse_postfix_expression(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary_expression()?;

        loop {
            if self.check(TokenKind::Dot) {
                self.advance();
                let field = self.expect_identifier()?;
                expr = Expr::FieldAccess {
                    object: Box::new(expr),
                    field,
                };
            } else if self.check(TokenKind::At) {
                self.advance();
                let index = self.parse_primary_expression()?;
                expr = Expr::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else if self.check(TokenKind::LeftParen) {
                // Function call with parentheses
                self.advance();
                let mut args = Vec::new();
                if !self.check(TokenKind::RightParen) {
                    args.push(self.parse_expression()?);
                    while self.check(TokenKind::Comma) {
                        self.advance();
                        args.push(self.parse_expression()?);
                    }
                }
                self.expect(TokenKind::RightParen)?;
                expr = Expr::Call {
                    callee: Box::new(expr),
                    arguments: args,
                };
            } else if self.check(TokenKind::Arrow) {
                // Lambda: [x] -> expr  or  x -> expr
                self.advance();

                // expr should be the parameter(s)
                let params = self.extract_lambda_params(expr)?;
                let body = self.parse_expression()?;

                expr = Expr::Lambda {
                    params,
                    body: Box::new(body),
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    /// Extract lambda parameters from an expression
    fn extract_lambda_params(&self, expr: Expr) -> Result<Vec<String>, String> {
        match expr {
            Expr::Identifier(name) => Ok(vec![name]),
            Expr::List(items) => {
                let mut params = Vec::new();
                for item in items {
                    if let Expr::Identifier(name) = item {
                        params.push(name);
                    } else {
                        return Err("Lambda parameters must be identifiers".to_string());
                    }
                }
                Ok(params)
            }
            _ => Err("Invalid lambda parameter syntax".to_string()),
        }
    }

    /// Parse primary expressions (literals, identifiers, parenthesized, etc.)
    fn parse_primary_expression(&mut self) -> Result<Expr, String> {
        // Number literal
        if self.check(TokenKind::Number) {
            let token = self.advance();
            let value: f64 = token
                .lexeme
                .parse()
                .map_err(|_| format!("Invalid number: {}", token.lexeme))?;
            if value.fract() == 0.0 && value.abs() < i64::MAX as f64 {
                return Ok(Expr::Literal(Literal::Int(value as i64)));
            } else {
                return Ok(Expr::Literal(Literal::Float(value)));
            }
        }

        // String literal
        if self.check(TokenKind::StringLiteral) {
            let token = self.advance();
            return Ok(Expr::Literal(Literal::String(token.lexeme)));
        }

        // String interpolation
        if self.check(TokenKind::StringStart) {
            return self.parse_interpolated_string();
        }

        // Boolean literals
        if self.check(TokenKind::True) {
            self.advance();
            return Ok(Expr::Literal(Literal::Bool(true)));
        }

        if self.check(TokenKind::False) {
            self.advance();
            return Ok(Expr::Literal(Literal::Bool(false)));
        }

        // Nil literal
        if self.check(TokenKind::Nil) {
            self.advance();
            return Ok(Expr::Literal(Literal::Nil));
        }

        // List constructor: list(1, 2, 3)
        if self.check(TokenKind::List) {
            return self.parse_list_constructor();
        }

        // Parenthesized expression
        if self.check(TokenKind::LeftParen) {
            self.advance();
            let expr = self.parse_expression()?;
            self.expect(TokenKind::RightParen)?;
            return Ok(expr);
        }

        // Identifier (variable, function name, etc.)
        if self.check(TokenKind::Identifier) {
            let name = self.expect_identifier()?;

            // Check if it's a struct constructor: name(args)
            if self.check(TokenKind::LeftParen) {
                return self.parse_struct_or_call(name);
            }

            return Ok(Expr::Identifier(name));
        }

        Err(format!(
            "Unexpected token in expression: {:?} at line {}",
            self.peek().map(|t| &t.kind),
            self.current_line()
        ))
    }

    /// Parse interpolated string: "hello {name}!"
    fn parse_interpolated_string(&mut self) -> Result<Expr, String> {
        let mut parts = Vec::new();

        // Get the start part
        let start_token = self.expect(TokenKind::StringStart)?;
        if !start_token.lexeme.is_empty() {
            parts.push(StringPart::Literal(start_token.lexeme));
        }

        loop {
            // Expect interpolation start
            if !self.check(TokenKind::InterpolationStart) {
                break;
            }
            self.advance();

            // Parse the expression inside
            let expr = self.parse_expression()?;
            parts.push(StringPart::Expr(expr));

            // Expect interpolation end
            self.expect(TokenKind::InterpolationEnd)?;

            // Check for more string content
            if self.check(TokenKind::StringMiddle) {
                let middle_token = self.advance();
                if !middle_token.lexeme.is_empty() {
                    parts.push(StringPart::Literal(middle_token.lexeme));
                }
            } else if self.check(TokenKind::StringEnd) {
                let end_token = self.advance();
                if !end_token.lexeme.is_empty() {
                    parts.push(StringPart::Literal(end_token.lexeme));
                }
                break;
            } else {
                break;
            }
        }

        Ok(Expr::StringInterpolation(parts))
    }

    /// Parse list constructor: list(1, 2, 3)
    fn parse_list_constructor(&mut self) -> Result<Expr, String> {
        self.expect(TokenKind::List)?;
        self.expect(TokenKind::LeftParen)?;

        let mut elements = Vec::new();

        if !self.check(TokenKind::RightParen) {
            elements.push(self.parse_expression()?);

            while self.check(TokenKind::Comma) {
                self.advance();
                elements.push(self.parse_expression()?);
            }
        }

        self.expect(TokenKind::RightParen)?;

        Ok(Expr::List(elements))
    }

    /// Parse struct constructor or function call: name(arg1, arg2, ...)
    fn parse_struct_or_call(&mut self, name: String) -> Result<Expr, String> {
        self.expect(TokenKind::LeftParen)?;

        let mut args = Vec::new();

        if !self.check(TokenKind::RightParen) {
            args.push(self.parse_expression()?);

            while self.check(TokenKind::Comma) {
                self.advance();
                args.push(self.parse_expression()?);
            }
        }

        self.expect(TokenKind::RightParen)?;

        // This could be a function call or struct instantiation
        // For now, we treat it as a function call - struct instantiation can use
        // a different syntax: StructName { field: value }
        Ok(Expr::Call {
            callee: Box::new(Expr::Identifier(name)),
            arguments: args,
        })
    }

    // =============================================
    // Helper Methods
    // =============================================

    /// Check if at end of token stream
    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
            || self.peek().map(|t| t.kind == TokenKind::Eof).unwrap_or(true)
    }

    /// Peek at current token
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    /// Get current line number
    fn current_line(&self) -> usize {
        self.peek().map(|t| t.line).unwrap_or(0)
    }

    /// Advance and return current token
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.pos += 1;
        }
        self.tokens.get(self.pos - 1).cloned().unwrap_or(Token {
            kind: TokenKind::Eof,
            lexeme: String::new(),
            line: 0,
            column: 0,
        })
    }

    /// Check if current token matches expected kind
    fn check(&self, kind: TokenKind) -> bool {
        self.peek().map(|t| t.kind == kind).unwrap_or(false)
    }

    /// Expect a specific token kind
    fn expect(&mut self, kind: TokenKind) -> Result<Token, String> {
        if self.check(kind.clone()) {
            Ok(self.advance())
        } else {
            Err(format!(
                "Expected {:?}, found {:?} at line {}",
                kind,
                self.peek().map(|t| &t.kind),
                self.current_line()
            ))
        }
    }

    /// Expect and return an identifier
    fn expect_identifier(&mut self) -> Result<String, String> {
        if self.check(TokenKind::Identifier) {
            Ok(self.advance().lexeme)
        } else {
            Err(format!(
                "Expected identifier, found {:?} at line {}",
                self.peek().map(|t| &t.kind),
                self.current_line()
            ))
        }
    }

    /// Synchronize after an error by skipping to next statement boundary
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            // Stop at block boundaries
            if self.check(TokenKind::RightBracket) {
                return;
            }

            // Stop at quack (new statement)
            if self.check(TokenKind::Quack) {
                return;
            }

            // Stop at left bracket (new block)
            if self.check(TokenKind::LeftBracket) {
                return;
            }

            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;

    fn parse_source(source: &str) -> Result<Vec<Block>, Vec<String>> {
        let tokens = lex(source).unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    #[test]
    fn test_simple_block_without_quack() {
        let result = parse_source("[let x be 10]").unwrap();
        assert_eq!(result.len(), 1);
        assert!(!result[0].was_quacked);
    }

    #[test]
    fn test_block_with_quack() {
        let result = parse_source("quack [let x be 10]").unwrap();
        assert_eq!(result.len(), 1);
        assert!(result[0].was_quacked);
    }

    #[test]
    fn test_multi_quack_pattern() {
        let result = parse_source("quack quack quack [print 1] [print 2] [print 3]").unwrap();
        assert_eq!(result.len(), 3);
        assert!(result[0].was_quacked);
        assert!(result[1].was_quacked);
        assert!(result[2].was_quacked);
    }

    #[test]
    fn test_mixed_quacked_and_unquacked() {
        let result = parse_source("quack [print 1] [print 2]").unwrap();
        assert_eq!(result.len(), 2);
        assert!(result[0].was_quacked);
        assert!(!result[1].was_quacked);
    }
}
