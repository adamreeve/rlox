struct Scanner<'a> {
    /// Source remaining to be scanned
    remaining_source: &'a str,
    /// Offset to next character in terms of number of unicode characters
    current_offset: usize,
    /// Current line
    line: u32,
}

struct Token<'a> {
    token_type: TokenType,
    source: &'a str,
    line: u32,
}

enum TokenType {
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
            remaining_source: source,
            current_offset: 0,
            line: 1
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        self.remaining_source = &self.remaining_source[self.current_offset_in_bytes()..];
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

        return self.error_token("Unexpected character");
    }

    fn advance(&mut self) -> char {
        let next_char = self.remaining_source
            .chars().nth(self.current_offset)
            .expect("Unexpected EOF");
        self.current_offset += 1;
        next_char
    }

    fn match_next(&mut self, test_char: char) -> bool {
        let next_char = self.remaining_source
            .chars().nth(self.current_offset);
        match next_char {
            Some(c) => {
                if c == test_char {
                    self.current_offset += 1;
                    true
                } else {
                    false
                }
            }
            None => false
        }
    }

    fn is_at_end(&self) -> bool {
        self.current_offset >= self.remaining_source.len()
    }

    fn skip_whitespace(&mut self) {
        // TODO: Increment line count, skip comments too
        let whitespace_count = self.remaining_source
            .char_indices()
            .take_while(|ci| ci.1.is_whitespace())
            .map(|ci| ci.0)
            .last().unwrap_or(0);
        self.remaining_source = &self.remaining_source[whitespace_count..];
    }

    fn current_offset_in_bytes(&self) -> usize {
        match self.remaining_source.char_indices().nth(self.current_offset + 1) {
            Some(char_index) => char_index.0,
            None => self.remaining_source.len()
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            source: &self.remaining_source[..self.current_offset_in_bytes()],
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
}
