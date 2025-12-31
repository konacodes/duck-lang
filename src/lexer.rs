// Lexer - tokenization for Duck language

/// Represents the different kinds of tokens in Duck-Lang
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Special keyword
    Quack,

    // Brackets and parentheses
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    EqualEqual,  // ==
    NotEqual,    // !=
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Arrow,       // ->
    Comma,
    Dot,

    // Keywords
    Let,
    Be,
    Becomes,
    Define,
    Taking,
    As,
    If,
    Then,
    Otherwise,
    Match,
    With,
    When,
    Repeat,
    Times,
    While,
    Do,
    For,
    Each,
    In,
    Struct,
    Return,
    And,
    Or,
    Not,
    List,
    Push,
    At,
    Length,
    Print,
    Break,
    Continue,
    Honk,
    Attempt,
    Rescue,
    Migrate,

    // Boolean and null literals
    True,
    False,
    Nil,

    // Special tokens
    Underscore,  // _

    // Literals and identifiers
    Identifier,
    Number,
    StringLiteral,

    // String interpolation parts
    StringStart,
    StringMiddle,
    StringEnd,
    InterpolationStart,
    InterpolationEnd,

    // End of file
    Eof,
}

/// A token with its kind, lexeme, and source location
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: usize, column: usize) -> Self {
        Token { kind, lexeme, line, column }
    }
}

/// The lexer struct that maintains state during tokenization
pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
    start_column: usize,
}

impl Lexer {
    /// Create a new lexer for the given source code
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
            start_column: 1,
        }
    }

    /// Tokenize the source and return the list of tokens
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        while !self.is_at_end() {
            self.start = self.current;
            self.start_column = self.column;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(TokenKind::Eof, String::new(), self.line, self.column));
        Ok(self.tokens.clone())
    }

    /// Check if we've reached the end of the source
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Advance to the next character and return the current one
    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        self.column += 1;
        c
    }

    /// Peek at the current character without advancing
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    /// Peek at the next character (one ahead of current)
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    /// Match the current character and advance if it matches
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            self.column += 1;
            true
        }
    }

    /// Get the current lexeme
    fn current_lexeme(&self) -> String {
        self.source[self.start..self.current].iter().collect()
    }

    /// Add a token to the list
    fn add_token(&mut self, kind: TokenKind) {
        let lexeme = self.current_lexeme();
        self.tokens.push(Token::new(kind, lexeme, self.line, self.start_column));
    }

    /// Add a token with a specific lexeme
    fn add_token_with_lexeme(&mut self, kind: TokenKind, lexeme: String) {
        self.tokens.push(Token::new(kind, lexeme, self.line, self.start_column));
    }

    /// Scan a single token
    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();

        match c {
            // Whitespace
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
                self.column = 1;
            }

            // Single-character tokens
            '[' => self.add_token(TokenKind::LeftBracket),
            ']' => self.add_token(TokenKind::RightBracket),
            '(' => self.add_token(TokenKind::LeftParen),
            ')' => self.add_token(TokenKind::RightParen),
            '{' => self.add_token(TokenKind::LeftBrace),
            '}' => self.add_token(TokenKind::RightBrace),
            '+' => self.add_token(TokenKind::Plus),
            '*' => self.add_token(TokenKind::Star),
            '/' => self.add_token(TokenKind::Slash),
            '%' => self.add_token(TokenKind::Percent),
            ',' => self.add_token(TokenKind::Comma),
            '.' => self.add_token(TokenKind::Dot),
            '_' => {
                // Could be underscore or start of identifier
                if self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
                    self.identifier();
                } else {
                    self.add_token(TokenKind::Underscore);
                }
            }

            // Two-character tokens or single
            '-' => {
                if self.match_char('-') {
                    // Comment: skip until end of line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_char('>') {
                    self.add_token(TokenKind::Arrow);
                } else {
                    self.add_token(TokenKind::Minus);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::EqualEqual);
                } else {
                    return Err(format!("Unexpected character '=' at line {}. Did you mean '==' or 'becomes'?", self.line));
                }
            }
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::NotEqual);
                } else {
                    return Err(format!("Unexpected character '!' at line {}. Did you mean '!=' or 'not'?", self.line));
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::LessEqual);
                } else {
                    self.add_token(TokenKind::Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::GreaterEqual);
                } else {
                    self.add_token(TokenKind::Greater);
                }
            }

            // String literals
            '"' => self.string()?,

            // Numbers
            c if c.is_ascii_digit() => self.number()?,

            // Identifiers and keywords
            c if c.is_ascii_alphabetic() => self.identifier(),

            _ => {
                return Err(format!("Unexpected character '{}' at line {}", c, self.line));
            }
        }

        Ok(())
    }

    /// Scan a regular string literal (no interpolation - braces are literal)
    fn string(&mut self) -> Result<(), String> {
        let start_line = self.line;
        let mut value = String::new();

        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }

            if self.peek() == '\\' {
                // Handle escape sequences
                self.advance(); // consume backslash
                if self.is_at_end() {
                    return Err(format!("Unterminated string starting at line {}", start_line));
                }
                let escaped = self.advance();
                match escaped {
                    '"' => value.push('"'),
                    '\\' => value.push('\\'),
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    _ => {
                        return Err(format!(
                            "Invalid escape sequence '\\{}' at line {}",
                            escaped, self.line
                        ));
                    }
                }
            } else {
                // Regular character (including { and } which are literal in regular strings)
                value.push(self.advance());
            }
        }

        if self.is_at_end() {
            return Err(format!("Unterminated string starting at line {}", start_line));
        }

        self.advance(); // consume closing '"'
        self.add_token_with_lexeme(TokenKind::StringLiteral, value);

        Ok(())
    }

    /// Scan an f-string literal with interpolation: f"Hello {name}!"
    fn fstring(&mut self) -> Result<(), String> {
        let start_line = self.line;
        let mut value = String::new();
        let mut has_interpolation = false;
        let mut is_first_part = true;

        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }

            if self.peek() == '\\' {
                // Handle escape sequences
                self.advance(); // consume backslash
                if self.is_at_end() {
                    return Err(format!("Unterminated f-string starting at line {}", start_line));
                }
                let escaped = self.advance();
                match escaped {
                    '"' => value.push('"'),
                    '\\' => value.push('\\'),
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    '{' => value.push('{'),  // escaped brace, not interpolation
                    '}' => value.push('}'),
                    _ => {
                        return Err(format!(
                            "Invalid escape sequence '\\{}' at line {}",
                            escaped, self.line
                        ));
                    }
                }
            } else if self.peek() == '{' {
                // String interpolation starts
                has_interpolation = true;

                // Emit the string part before the interpolation
                if is_first_part {
                    self.add_token_with_lexeme(TokenKind::StringStart, value.clone());
                    is_first_part = false;
                } else {
                    self.add_token_with_lexeme(TokenKind::StringMiddle, value.clone());
                }
                value.clear();

                self.advance(); // consume '{'
                self.add_token_with_lexeme(TokenKind::InterpolationStart, "{".to_string());

                // Tokenize the interpolation expression until we hit '}'
                self.scan_interpolation(start_line)?;

                self.add_token_with_lexeme(TokenKind::InterpolationEnd, "}".to_string());
            } else {
                value.push(self.advance());
            }
        }

        if self.is_at_end() {
            return Err(format!("Unterminated f-string starting at line {}", start_line));
        }

        self.advance(); // consume closing '"'

        if has_interpolation {
            // This is the end part of an interpolated string
            self.add_token_with_lexeme(TokenKind::StringEnd, value);
        } else {
            // F-string without interpolation is just a regular string
            self.add_token_with_lexeme(TokenKind::StringLiteral, value);
        }

        Ok(())
    }

    /// Scan the inside of a string interpolation `{...}`
    fn scan_interpolation(&mut self, string_start_line: usize) -> Result<(), String> {
        let mut brace_depth = 1;

        while brace_depth > 0 && !self.is_at_end() {
            // Skip whitespace inside interpolation
            while !self.is_at_end() && (self.peek() == ' ' || self.peek() == '\t') {
                self.advance();
            }

            if self.is_at_end() {
                return Err(format!(
                    "Unterminated string interpolation in string starting at line {}",
                    string_start_line
                ));
            }

            if self.peek() == '}' {
                brace_depth -= 1;
                if brace_depth == 0 {
                    self.advance(); // consume the closing '}'
                    break;
                }
            }

            if self.peek() == '{' {
                brace_depth += 1;
            }

            self.start = self.current;
            self.start_column = self.column;
            self.scan_token()?;
        }

        Ok(())
    }

    /// Scan a number literal (integer or float)
    fn number(&mut self) -> Result<(), String> {
        // Consume all digits
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a decimal part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // consume the '.'

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let text = self.current_lexeme();
        // Validate the number
        if text.parse::<f64>().is_err() {
            return Err(format!("Invalid number '{}' at line {}", text, self.line));
        }

        self.add_token(TokenKind::Number);
        Ok(())
    }

    /// Scan an identifier or keyword
    fn identifier(&mut self) {
        // Check for f-string: f"..."
        let first_char = self.source.get(self.start).copied().unwrap_or(' ');
        if first_char == 'f' && self.current == self.start + 1 && self.peek() == '"' {
            self.advance(); // consume the opening quote
            self.fstring().ok(); // process as f-string (interpolated)
            return;
        }

        // Identifiers can contain letters, digits, underscores, and hyphens
        // but must start with a letter (already consumed) or underscore
        while !self.is_at_end() {
            let c = self.peek();
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                // Hyphens are allowed, but not at the end or followed by non-alphanumeric
                if c == '-' {
                    // Peek ahead to see if this hyphen is part of the identifier
                    if self.peek_next().is_ascii_alphanumeric() {
                        self.advance();
                    } else {
                        // Hyphen at end or followed by non-alphanumeric, treat as operator
                        break;
                    }
                } else {
                    self.advance();
                }
            } else {
                break;
            }
        }

        let text = self.current_lexeme();
        let kind = self.keyword_or_identifier(&text);
        self.add_token(kind);
    }

    /// Check if the identifier is a keyword, return appropriate token kind
    fn keyword_or_identifier(&self, text: &str) -> TokenKind {
        match text {
            "quack" => TokenKind::Quack,
            "let" => TokenKind::Let,
            "be" => TokenKind::Be,
            "becomes" => TokenKind::Becomes,
            "define" => TokenKind::Define,
            "taking" => TokenKind::Taking,
            "as" => TokenKind::As,
            "if" => TokenKind::If,
            "then" => TokenKind::Then,
            "otherwise" => TokenKind::Otherwise,
            "match" => TokenKind::Match,
            "with" => TokenKind::With,
            "when" => TokenKind::When,
            "repeat" => TokenKind::Repeat,
            "times" => TokenKind::Times,
            "while" => TokenKind::While,
            "do" => TokenKind::Do,
            "for" => TokenKind::For,
            "each" => TokenKind::Each,
            "in" => TokenKind::In,
            "struct" => TokenKind::Struct,
            "return" => TokenKind::Return,
            "and" => TokenKind::And,
            "or" => TokenKind::Or,
            "not" => TokenKind::Not,
            "list" => TokenKind::List,
            "push" => TokenKind::Push,
            "at" => TokenKind::At,
            "length" => TokenKind::Length,
            "print" => TokenKind::Print,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "honk" => TokenKind::Honk,
            "attempt" => TokenKind::Attempt,
            "rescue" => TokenKind::Rescue,
            "migrate" => TokenKind::Migrate,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "nil" => TokenKind::Nil,
            _ => TokenKind::Identifier,
        }
    }
}

/// Main entry point: tokenize source code into a vector of tokens
pub fn lex(source: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_source() {
        let tokens = lex("").unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Eof);
    }

    #[test]
    fn test_quack_keyword() {
        let tokens = lex("quack").unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::Quack);
        assert_eq!(tokens[1].kind, TokenKind::Eof);
    }

    #[test]
    fn test_brackets_and_parens() {
        let tokens = lex("[](){}").unwrap();
        assert_eq!(tokens.len(), 7);
        assert_eq!(tokens[0].kind, TokenKind::LeftBracket);
        assert_eq!(tokens[1].kind, TokenKind::RightBracket);
        assert_eq!(tokens[2].kind, TokenKind::LeftParen);
        assert_eq!(tokens[3].kind, TokenKind::RightParen);
        assert_eq!(tokens[4].kind, TokenKind::LeftBrace);
        assert_eq!(tokens[5].kind, TokenKind::RightBrace);
    }

    #[test]
    fn test_operators() {
        let tokens = lex("+ - * / % == != < > <= >= -> , .").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Plus);
        assert_eq!(tokens[1].kind, TokenKind::Minus);
        assert_eq!(tokens[2].kind, TokenKind::Star);
        assert_eq!(tokens[3].kind, TokenKind::Slash);
        assert_eq!(tokens[4].kind, TokenKind::Percent);
        assert_eq!(tokens[5].kind, TokenKind::EqualEqual);
        assert_eq!(tokens[6].kind, TokenKind::NotEqual);
        assert_eq!(tokens[7].kind, TokenKind::Less);
        assert_eq!(tokens[8].kind, TokenKind::Greater);
        assert_eq!(tokens[9].kind, TokenKind::LessEqual);
        assert_eq!(tokens[10].kind, TokenKind::GreaterEqual);
        assert_eq!(tokens[11].kind, TokenKind::Arrow);
        assert_eq!(tokens[12].kind, TokenKind::Comma);
        assert_eq!(tokens[13].kind, TokenKind::Dot);
    }

    #[test]
    fn test_numbers() {
        let tokens = lex("42 3.14 0 100.0").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Number);
        assert_eq!(tokens[0].lexeme, "42");
        assert_eq!(tokens[1].kind, TokenKind::Number);
        assert_eq!(tokens[1].lexeme, "3.14");
        assert_eq!(tokens[2].kind, TokenKind::Number);
        assert_eq!(tokens[3].kind, TokenKind::Number);
    }

    #[test]
    fn test_string_literal() {
        let tokens = lex(r#""hello world""#).unwrap();
        assert_eq!(tokens[0].kind, TokenKind::StringLiteral);
        assert_eq!(tokens[0].lexeme, "hello world");
    }

    #[test]
    fn test_string_escape_sequences() {
        let tokens = lex(r#""line1\nline2\ttab\"quote\\backslash""#).unwrap();
        assert_eq!(tokens[0].kind, TokenKind::StringLiteral);
        assert_eq!(tokens[0].lexeme, "line1\nline2\ttab\"quote\\backslash");
    }

    #[test]
    fn test_string_interpolation() {
        // F-strings use f"..." prefix for interpolation
        let tokens = lex(r#"f"hello {name}!""#).unwrap();
        assert_eq!(tokens[0].kind, TokenKind::StringStart);
        assert_eq!(tokens[0].lexeme, "hello ");
        assert_eq!(tokens[1].kind, TokenKind::InterpolationStart);
        assert_eq!(tokens[2].kind, TokenKind::Identifier);
        assert_eq!(tokens[2].lexeme, "name");
        assert_eq!(tokens[3].kind, TokenKind::InterpolationEnd);
        assert_eq!(tokens[4].kind, TokenKind::StringEnd);
        assert_eq!(tokens[4].lexeme, "!");
    }

    #[test]
    fn test_regular_string_no_interpolation() {
        // Regular strings treat braces as literal characters
        let tokens = lex(r#""{\"key\": \"value\"}""#).unwrap();
        assert_eq!(tokens[0].kind, TokenKind::StringLiteral);
        assert_eq!(tokens[0].lexeme, "{\"key\": \"value\"}");
    }

    #[test]
    fn test_identifier_with_hyphen() {
        let tokens = lex("my-variable another-one").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Identifier);
        assert_eq!(tokens[0].lexeme, "my-variable");
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[1].lexeme, "another-one");
    }

    #[test]
    fn test_keywords() {
        let tokens = lex("let be becomes define taking as if then otherwise").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[1].kind, TokenKind::Be);
        assert_eq!(tokens[2].kind, TokenKind::Becomes);
        assert_eq!(tokens[3].kind, TokenKind::Define);
        assert_eq!(tokens[4].kind, TokenKind::Taking);
        assert_eq!(tokens[5].kind, TokenKind::As);
        assert_eq!(tokens[6].kind, TokenKind::If);
        assert_eq!(tokens[7].kind, TokenKind::Then);
        assert_eq!(tokens[8].kind, TokenKind::Otherwise);
    }

    #[test]
    fn test_more_keywords() {
        let tokens = lex("match with when repeat times while do for each in").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Match);
        assert_eq!(tokens[1].kind, TokenKind::With);
        assert_eq!(tokens[2].kind, TokenKind::When);
        assert_eq!(tokens[3].kind, TokenKind::Repeat);
        assert_eq!(tokens[4].kind, TokenKind::Times);
        assert_eq!(tokens[5].kind, TokenKind::While);
        assert_eq!(tokens[6].kind, TokenKind::Do);
        assert_eq!(tokens[7].kind, TokenKind::For);
        assert_eq!(tokens[8].kind, TokenKind::Each);
        assert_eq!(tokens[9].kind, TokenKind::In);
    }

    #[test]
    fn test_remaining_keywords() {
        let tokens = lex("struct return and or not list push at length print").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Struct);
        assert_eq!(tokens[1].kind, TokenKind::Return);
        assert_eq!(tokens[2].kind, TokenKind::And);
        assert_eq!(tokens[3].kind, TokenKind::Or);
        assert_eq!(tokens[4].kind, TokenKind::Not);
        assert_eq!(tokens[5].kind, TokenKind::List);
        assert_eq!(tokens[6].kind, TokenKind::Push);
        assert_eq!(tokens[7].kind, TokenKind::At);
        assert_eq!(tokens[8].kind, TokenKind::Length);
        assert_eq!(tokens[9].kind, TokenKind::Print);
    }

    #[test]
    fn test_boolean_and_nil() {
        let tokens = lex("true false nil").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::True);
        assert_eq!(tokens[1].kind, TokenKind::False);
        assert_eq!(tokens[2].kind, TokenKind::Nil);
    }

    #[test]
    fn test_underscore() {
        let tokens = lex("_ _foo").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Underscore);
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[1].lexeme, "_foo");
    }

    #[test]
    fn test_comments() {
        let tokens = lex("quack -- this is a comment\nquack").unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind, TokenKind::Quack);
        assert_eq!(tokens[1].kind, TokenKind::Quack);
        assert_eq!(tokens[2].kind, TokenKind::Eof);
    }

    #[test]
    fn test_arrow_lambda() {
        let tokens = lex("x -> x + 1").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Identifier);
        assert_eq!(tokens[1].kind, TokenKind::Arrow);
        assert_eq!(tokens[2].kind, TokenKind::Identifier);
        assert_eq!(tokens[3].kind, TokenKind::Plus);
        assert_eq!(tokens[4].kind, TokenKind::Number);
    }

    #[test]
    fn test_line_numbers() {
        let tokens = lex("quack\nquack\nquack").unwrap();
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[1].line, 2);
        assert_eq!(tokens[2].line, 3);
    }

    #[test]
    fn test_complex_expression() {
        let tokens = lex("let my-var be 42 + 3.14").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[1].lexeme, "my-var");
        assert_eq!(tokens[2].kind, TokenKind::Be);
        assert_eq!(tokens[3].kind, TokenKind::Number);
        assert_eq!(tokens[4].kind, TokenKind::Plus);
        assert_eq!(tokens[5].kind, TokenKind::Number);
    }

    #[test]
    fn test_function_definition() {
        let tokens = lex("define add taking a, b as a + b").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Define);
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[2].kind, TokenKind::Taking);
        assert_eq!(tokens[3].kind, TokenKind::Identifier);
        assert_eq!(tokens[4].kind, TokenKind::Comma);
        assert_eq!(tokens[5].kind, TokenKind::Identifier);
        assert_eq!(tokens[6].kind, TokenKind::As);
        assert_eq!(tokens[7].kind, TokenKind::Identifier);
        assert_eq!(tokens[8].kind, TokenKind::Plus);
        assert_eq!(tokens[9].kind, TokenKind::Identifier);
    }

    #[test]
    fn test_error_unterminated_string() {
        let result = lex(r#""hello"#);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unterminated string"));
    }

    #[test]
    fn test_error_invalid_escape() {
        let result = lex(r#""hello\x""#);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid escape sequence"));
    }

    #[test]
    fn test_multiple_interpolations() {
        // F-strings support multiple interpolations
        let tokens = lex(r#"f"Hello {first} {last}!""#).unwrap();
        // "Hello " + {first} + " " + {last} + "!"
        assert_eq!(tokens[0].kind, TokenKind::StringStart);
        assert_eq!(tokens[0].lexeme, "Hello ");
        assert_eq!(tokens[1].kind, TokenKind::InterpolationStart);
        assert_eq!(tokens[2].kind, TokenKind::Identifier);
        assert_eq!(tokens[2].lexeme, "first");
        assert_eq!(tokens[3].kind, TokenKind::InterpolationEnd);
        assert_eq!(tokens[4].kind, TokenKind::StringMiddle);
        assert_eq!(tokens[4].lexeme, " ");
        assert_eq!(tokens[5].kind, TokenKind::InterpolationStart);
        assert_eq!(tokens[6].kind, TokenKind::Identifier);
        assert_eq!(tokens[6].lexeme, "last");
        assert_eq!(tokens[7].kind, TokenKind::InterpolationEnd);
        assert_eq!(tokens[8].kind, TokenKind::StringEnd);
        assert_eq!(tokens[8].lexeme, "!");
    }

    #[test]
    fn test_list_operations() {
        let tokens = lex("list push at length [1, 2, 3]").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::List);
        assert_eq!(tokens[1].kind, TokenKind::Push);
        assert_eq!(tokens[2].kind, TokenKind::At);
        assert_eq!(tokens[3].kind, TokenKind::Length);
        assert_eq!(tokens[4].kind, TokenKind::LeftBracket);
        assert_eq!(tokens[5].kind, TokenKind::Number);
        assert_eq!(tokens[6].kind, TokenKind::Comma);
        assert_eq!(tokens[7].kind, TokenKind::Number);
        assert_eq!(tokens[8].kind, TokenKind::Comma);
        assert_eq!(tokens[9].kind, TokenKind::Number);
        assert_eq!(tokens[10].kind, TokenKind::RightBracket);
    }
}
