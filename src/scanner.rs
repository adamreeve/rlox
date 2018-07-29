pub struct Scanner<'a> {
    /// Source to be scanned
    source: &'a str,
    /// Offset to first character of current token in bytes
    start_offset: usize,
    /// Offset to next character in bytes
    current_offset: usize,
    /// Current line
    line: usize,
}

pub struct Token<'a> {
    pub token_type: TokenType,
    pub source: &'a str,
    pub line: usize,
}

#[derive(Debug,PartialEq,Eq,Copy,Clone)]
pub enum TokenType {
  LeftParen, RightParen,
  LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus,
  Semicolon, Slash, Star,

  // One or two character tokens.
  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual,

  // Literals.
  Identifier, String, Number,

  // Keywords.
  And, Class, Else, False,
  Fun, For, If, Nil, Or,
  Print, Return, Super, This,
  True, Var, While,

  Error,
  Eof
}

impl <'a> Scanner<'a> {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source: source,
            start_offset: 0,
            current_offset: 0,
            line: 1
        }
    }

    pub fn scan_token(&mut self) -> Token<'a> {
        self.skip_whitespace_and_comments();
        self.start_offset = self.current_offset;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        // Single character tokens
        let c = self.advance();
        let token_type = match c {
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            ';' => Some(TokenType::Semicolon),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Dot),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            '/' => Some(TokenType::Slash),
            '*' => Some(TokenType::Star),
            '!' => Some(if self.match_next('=') { TokenType::BangEqual } else { TokenType::Bang }),
            '=' => Some(if self.match_next('=') { TokenType::EqualEqual } else { TokenType::Equal }),
            '<' => Some(if self.match_next('=') { TokenType::LessEqual } else { TokenType::Less }),
            '>' => Some(if self.match_next('=') { TokenType::GreaterEqual } else { TokenType::Greater }),
            _ => None
        };

        if let Some(token_type) = token_type  {
            return self.make_token(token_type)
        }

        if c == '"' {
            return self.string_token();
        }

        if is_digit(c) {
            return self.number_token();
        }

        if is_alpha(c) {
            return self.identifier_token();
        }

        return self.error_token("Unexpected character");
    }

    fn peek(&self) -> Option<char> {
        self.remaining_source().chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        self.remaining_source().chars().nth(1)
    }

    fn remaining_source(&self) -> &'a str {
        &self.source[self.current_offset..]
    }

    fn current_token_source(&self) -> &'a str {
        &self.source[self.start_offset..self.current_offset]
    }

    fn advance(&mut self) -> char {
        let next_char = self.peek().expect("Unexpected EOF");
        self.increment_offset();
        next_char
    }

    fn match_next(&mut self, test_char: char) -> bool {
        match self.peek() {
            Some(c) if c == test_char => {
                self.increment_offset();
                true
            }
            _ => false
        }
    }

    fn increment_offset(&mut self) {
        let remaining = self.remaining_source();
        // Get index of next character or if there is no next character
        // we're at the end of the input
        self.current_offset += remaining
            .char_indices()
            .map(|ci| ci.0)
            .nth(1)
            .unwrap_or(remaining.len());
    }

    fn is_at_end(&self) -> bool {
        self.current_offset >= self.source.len()
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            if let Some(c) = self.peek() {
                if c == ' ' || c == '\r' || c == '\t' {
                    self.advance();
                    continue;
                }
                if c == '\n' {
                    self.line += 1;
                    self.advance();
                    continue;
                }
                if c == '/' {
                    if self.peek_next() == Some('/') {
                        while self.peek() != Some('\n') && !self.is_at_end() {
                            self.advance();
                        }
                    }
                    else {
                        return;
                    }
                    continue;
                }
                return;
            }
            else {
                return;
            }
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token<'a> {
        Token {
            token_type,
            source: self.current_token_source(),
            line: self.line
        }
    }

    fn error_token(&self, message: &'static str) -> Token<'static> {
        Token {
            token_type: TokenType::Error,
            source: message,
            line: self.line
        }
    }

    fn string_token(&mut self) -> Token<'a> {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return self.error_token("Unterminated string");
        }
        // Skip closing "
        self.advance();
        self.make_token(TokenType::String)
    }

    fn number_token(&mut self) -> Token<'a> {
        while is_some_where(self.peek(), &is_digit) {
            self.advance();
        }
        if self.peek() == Some('.') && is_some_where(self.peek_next(), &is_digit) {
            // Consume '.'
            self.advance();
            while is_some_where(self.peek(), &is_digit) {
                self.advance();
            }
        }
        self.make_token(TokenType::Number)
    }

    fn identifier_token(&mut self) -> Token<'a> {
        while is_some_where(self.peek(), &|c| {is_alpha(c) || is_digit(c)}) {
            self.advance();
        }
        return self.make_token(self.identifier_type());
    }

    // Once we've scanned an identifier work out the actual token type
    fn identifier_type(&self) -> TokenType {
        match self.current_token_source().chars().next() {
            Some('a') => self.check_keyword(1, "nd", TokenType::And),
            Some('c') => self.check_keyword(1, "lass", TokenType::Class),
            Some('e') => self.check_keyword(1, "lse", TokenType::Else),
            Some('f') => match self.current_token_source().chars().nth(1) {
                Some('a') => self.check_keyword(2, "lse", TokenType::False),
                Some('o') => self.check_keyword(2, "r", TokenType::For),
                Some('u') => self.check_keyword(2, "n", TokenType::Fun),
                _ => TokenType::Identifier
            }
            Some('i') => self.check_keyword(1, "f", TokenType::If),
            Some('n') => self.check_keyword(1, "il", TokenType::Nil),
            Some('o') => self.check_keyword(1, "r", TokenType::Or),
            Some('p') => self.check_keyword(1, "rint", TokenType::Print),
            Some('r') => self.check_keyword(1, "eturn", TokenType::Return),
            Some('s') => self.check_keyword(1, "uper", TokenType::Super),
            Some('t') => match self.current_token_source().chars().nth(1) {
                Some('h') => self.check_keyword(2, "is", TokenType::This),
                Some('r') => self.check_keyword(2, "ue", TokenType::True),
                _ => TokenType::Identifier
            }
            Some('v') => self.check_keyword(1, "ar", TokenType::Var),
            Some('w') => self.check_keyword(1, "hile", TokenType::While),
            _ => TokenType::Identifier
        }
    }

    fn check_keyword(&self, start: usize, rest: &str, token_type: TokenType) -> TokenType {
        let length_matches = self.current_offset == start + rest.len();
        let all_chars_match = length_matches && self.current_token_source()
            .chars()
            .skip(start)
            .take(rest.len())
            .zip(rest.chars())
            .all(|(a, b)| a == b);
        if all_chars_match { token_type } else { TokenType::Identifier }

    }
}

fn is_digit(c: char) -> bool {
    c >= '0' &&  c <= '9'
}

fn is_some_where<T: Copy>(x: Option<T>, predicate: &Fn(T) -> bool) -> bool {
    match x {
        Some(x) if predicate(x) => true,
        _ => false,
    }
}

fn is_alpha(c: char) -> bool {
    c >= 'a' &&  c <= 'z'
        || c >= 'A' && c <= 'Z'
        || c == '_'
}

fn is_alphanumeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skipping_whitespace_and_comments_before_more_tokens() {
        let test_cases = vec![
            ("a", 0),
            ("  a", 2),
            (" \t  a", 4),
            (" \t  +", 4),
            ("// Test a comment\nabc", 18),
            (" // Test a comment\n+", 19),
        ];
        for (contents, expected_skip_count) in test_cases {
            let mut scanner = Scanner {
                source: contents,
                start_offset: 0,
                current_offset: 0,
                line: 1,
            };
            scanner.skip_whitespace_and_comments();

            assert_eq!(
                scanner.current_offset, expected_skip_count,
                "Expected to skip {} bytes for '{}' but was {}", expected_skip_count, contents, scanner.current_offset);
        }
    }

    #[test]
    fn test_skipping_whitespace_and_comments_before_eof() {
        let test_cases = vec![
            ("", 0),
            ("  ", 2),
            (" \t  ", 4),
            ("// Test a comment\n", 18),
            (" // Test a comment", 18),
        ];
        for (contents, expected_skip_count) in test_cases {
            let mut scanner = Scanner {
                source: contents,
                start_offset: 0,
                current_offset: 0,
                line: 1,
            };
            scanner.skip_whitespace_and_comments();

            assert_eq!(
                scanner.current_offset, expected_skip_count,
                "Expected to skip {} bytes for '{}' but was {}", expected_skip_count, contents, scanner.current_offset);
        }
    }

    #[test]
    fn test_incrementing_line_count() {
        let test_cases = vec![
            ("", 1),
            ("  ", 1),
            (" \n  ", 2),
            (" \n  a", 2),
            (" \n\n  ", 3),
            ("// Test a comment\n", 2),
            ("// Test a comment\na", 2),
            (" // Test a comment", 1),
        ];
        for (contents, expected_line) in test_cases {
            let mut scanner = Scanner {
                source: contents,
                start_offset: 0,
                current_offset: 0,
                line: 1,
            };
            scanner.skip_whitespace_and_comments();

            assert_eq!(
                scanner.line, expected_line,
                "Expected line to be {} after '{}' but was {}", expected_line, contents, scanner.line);
        }
    }

    #[test]
    fn test_parse_single_characters() {
        test_parse("(", TokenType::LeftParen, 1);
        test_parse(")", TokenType::RightParen, 1);
        test_parse("{", TokenType::LeftBrace, 1);
        test_parse("}", TokenType::RightBrace, 1);
        test_parse(";", TokenType::Semicolon, 1);
        test_parse(",", TokenType::Comma, 1);
        test_parse(".", TokenType::Dot, 1);
        test_parse("-", TokenType::Minus, 1);
        test_parse("+", TokenType::Plus, 1);
        test_parse("/", TokenType::Slash, 1);
        test_parse("*", TokenType::Star, 1);
    }

    #[test]
    fn test_parse_multiple_characters() {
        test_parse("<", TokenType::Less, 1);
        test_parse(">", TokenType::Greater, 1);
        test_parse("<1", TokenType::Less, 1);
        test_parse(">2", TokenType::Greater, 1);
        test_parse("<=", TokenType::LessEqual, 2);
        test_parse(">=", TokenType::GreaterEqual, 2);
        test_parse("! ", TokenType::Bang, 1);
        test_parse("!=", TokenType::BangEqual, 2);
        test_parse("=", TokenType::Equal, 1);
        test_parse("=1", TokenType::Equal, 1);
        test_parse("==", TokenType::EqualEqual, 2);
    }

    #[test]
    fn test_parse_keywords() {
        test_parse("and", TokenType::And, 3);
        test_parse("class", TokenType::Class, 5);
        test_parse("else", TokenType::Else, 4);
        test_parse("false", TokenType::False, 5);
        test_parse("for", TokenType::For, 3);
        test_parse("fun", TokenType::Fun, 3);
        test_parse("if", TokenType::If, 2);
        test_parse("nil", TokenType::Nil, 3);
        test_parse("or", TokenType::Or, 2);
        test_parse("print", TokenType::Print, 5);
        test_parse("return", TokenType::Return, 6);
        test_parse("super", TokenType::Super, 5);
        test_parse("this", TokenType::This, 4);
        test_parse("true", TokenType::True, 4);
        test_parse("var", TokenType::Var, 3);
        test_parse("while", TokenType::While, 5);
    }

    #[test]
    fn test_parse_identifiers() {
        let test_cases = vec!["and1", "true_", "my_var", "my_var_2"];
        for test_case in test_cases {
            test_parse(test_case, TokenType::Identifier, test_case.len());
        }
    }

    #[test]
    fn test_parsing_expression() {
        let source = "1 + 2";
        let mut scanner = Scanner {
            source: source,
            start_offset: 0,
            current_offset: 0,
            line: 1
        };
        {
            let token = scanner.scan_token();
            assert_eq!(token.source, "1", "Expected first token to be 1");
            assert_eq!(token.token_type, TokenType::Number, "Expected first token to be a number");
        }
        {
            let token = scanner.scan_token();
            assert_eq!(token.source, "+", "Expected second token to be +");
            assert_eq!(token.token_type, TokenType::Plus, "Expected second token to be plus");
        }
        {
            let token = scanner.scan_token();
            assert_eq!(token.source, "2", "Expected third token to be 2");
            assert_eq!(token.token_type, TokenType::Number, "Expected third token to be a number");
        }
        {
            let token = scanner.scan_token();
            assert_eq!(token.token_type, TokenType::Eof, "Expected Eof token");
        }
    }

    fn test_parse(source: &'static str, token_type: TokenType, end: usize) {
        let mut scanner = Scanner {
            source: source,
            start_offset: 0,
            current_offset: 0,
            line: 1
        };
        {
            let token = scanner.scan_token();
            assert_eq!(
                token.token_type, token_type,
                "Expected to get type {:?} but was {:?} when parsing '{}'", token_type, token.token_type, source);
        }
        let new_offset = scanner.current_offset;
        assert_eq!(
            new_offset, end,
            "Expected offset to be {} but was {} when parsing '{}'", end, new_offset, source);
    }
}
