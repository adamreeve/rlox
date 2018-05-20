struct Scanner<'a> {
    /// Source remaining to be scanned
    remaining_source: &'a str,
    /// Offset to next character in bytes
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
        self.skip_whitespace_and_comments();
        // Reset remaining source based on current offset
        self.remaining_source = &self.remaining_source[self.current_offset..];

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

    fn peek(&self) -> Option<char> {
        self.remaining_source[self.current_offset..].chars().next()
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
        let remaining = &self.remaining_source[self.current_offset..];
        // Get index of next character or if there is no next character
        // we're at the end of the input
        self.current_offset += remaining
            .char_indices()
            .map(|ci| ci.0)
            .nth(1)
            .unwrap_or(remaining.len());
    }

    fn is_at_end(&self) -> bool {
        self.current_offset >= self.remaining_source.len()
    }

    fn skip_whitespace_and_comments(&mut self) {
        let mut skip_count = 0;
        let mut possible_comment = false;
        let mut in_comment = false;
        let mut reached_end = true;
        for (index, c) in self.remaining_source[self.current_offset..].char_indices() {
            if possible_comment && c != '/' {
                // skip until start of previous character which was a /
                reached_end = false;
                break;
            }

            if c == '\n' {
                self.line += 1;
                if in_comment {
                    in_comment = false;
                }
            }
            if c.is_whitespace() {
                continue;
            }
            if c == '/' && possible_comment {
                in_comment = true;
                possible_comment = false;
            }
            else if c == '/' {
                possible_comment = true;
                skip_count = index;
            }
            else if !in_comment {
                // Current char is non-whitespace and we're not in a comment
                // so we don't want to skip any further
                reached_end =  false;
                skip_count = index;
                break;
            }
        }
        if reached_end {
            self.current_offset += self.remaining_source.len();
        }
        else {
            self.current_offset += skip_count;
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            source: &self.remaining_source[..self.current_offset],
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
                remaining_source: contents,
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
                remaining_source: contents,
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
                remaining_source: contents,
                current_offset: 0,
                line: 1,
            };
            scanner.skip_whitespace_and_comments();

            assert_eq!(
                scanner.line, expected_line,
                "Expected line to be {} after '{}' but was {}", expected_line, contents, scanner.line);
        }
    }
}
