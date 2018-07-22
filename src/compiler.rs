use chunk::Chunk;
use errors::InterpretResult;
use scanner::{Scanner, Token, TokenType};

pub fn compile(source: &str) -> InterpretResult<Chunk>
{
    let mut scanner = Scanner::new(&source);
    let mut compiler = Compiler::new(&mut scanner);
    compiler.compile()
}

struct Compiler<'a> {
    scanner: &'a Scanner<'a>,
    chunk: Chunk,
    parser: Parser<'a>,
}

struct Parser<'a> {
    current: Option<Token<'a>>,
    previous: Option<Token<'a>>,
    had_error: bool,
}

impl <'a> Compiler<'a> {
    fn new(scanner: &'a Scanner) -> Compiler<'a> {
        Compiler {
            scanner,
            chunk: Chunk::new(),
            parser: Parser { current: None, previous: None, had_error: false },
        }
    }

    fn compile(mut self) -> InterpretResult<Chunk> {
        self.advance()?;
        self.expression()?;
        self.consume(TokenType::Eof)?;
        Ok(self.chunk)
    }

    fn advance(&mut self) -> InterpretResult<()> {
        self.parser.previous = self.parser.current;
        loop {
            let current_token = self.scan_token();
            if current_token.token_type != TokenType::Error {
                break;
            }
            self.error_at_current(current_token.source);
            self.parser.current = Some(current_token);
        }
        Ok(())
    }

    fn consume(&mut self, tokenType: TokenType) -> InterpretResult<()> {
        Ok(())
    }

    fn error(&mut self, message: &str) {
        match self.parser.previous {
            Some(token) => self.error_at(&token, message),
            None => {},
        }
    }

    fn error_at_current(&mut self, message: &str) {
        match self.parser.current {
            Some(token) => self.error_at(&token, message),
            None => {},
        }
    }

    fn error_at(&mut self, token: &Token, message: &str) {
        eprint!("[line {}]");
        self.parser.had_error = true;
    }

    fn expression(&mut self) -> InterpretResult<()> {
        Ok(())
    }

    fn scan_token(&mut self) -> Token<'a> {
    }
}
