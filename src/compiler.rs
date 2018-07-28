use chunk::Chunk;
use errors::{InterpretError, InterpretResult};
use instructions::*;
use scanner::{Scanner, Token, TokenType};

pub fn compile(source: &str) -> InterpretResult<Chunk>
{
    let mut scanner = Scanner::new(&source);
    let mut compiler = Compiler::new(&mut scanner);
    compiler.compile()
}

struct Compiler<'a, 's: 'a> {
    scanner: &'a mut Scanner<'s>,
    chunk: Chunk,
    parser: Parser<'s>,
}

struct Parser<'a> {
    current: Option<Token<'a>>,
    previous: Option<Token<'a>>,
    had_error: bool,
    panic_mode: bool,
}

enum ParserToken {
    Current,
    Previous,
}

impl <'a, 's> Compiler<'a, 's> {
    fn new(scanner: &'a mut Scanner<'s>) -> Compiler<'a, 's> {
        Compiler {
            scanner,
            chunk: Chunk::new(),
            parser: Parser {
                current: None,
                previous: None,
                had_error: false,
                panic_mode: false,
            },
        }
    }

    fn compile(mut self) -> InterpretResult<Chunk> {
        self.advance();
        self.expression()?;
        self.consume(TokenType::Eof, "Expected end of expression");
        if self.parser.had_error {
            Err(InterpretError::CompileError("Compilation error occurred".to_string()))
        } else {
            self.end_compiler();
            Ok(self.chunk)
        }
    }

    fn advance(&mut self) {
        self.parser.previous = self.parser.current.take();
        loop {
            let current_token = self.scan_token();
            if current_token.token_type != TokenType::Error {
                break;
            }
            self.error_at_current(current_token.source);
            self.parser.current = Some(current_token);
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        let current_token_type = self.parser.current.as_ref().map(|t| t.token_type);
        if current_token_type == Some(token_type) {
            self.advance();
            return;
        }
        self.error_at_current(message);
    }

    fn end_compiler(&mut self) {
        self.write_instruction(SimpleInstruction::new(OpCode::Return));
    }

    fn error(&mut self, message: &str) {
        self.error_at(ParserToken::Previous, message);
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(ParserToken::Current, message);
    }

    fn error_at(&mut self, token: ParserToken, message: &str) {
        if self.parser.panic_mode {
            // Suppress any further errors being reported
            return;
        }
        self.parser.panic_mode = true;
        let token = match token {
            ParserToken::Current => &self.parser.current,
            ParserToken::Previous => &self.parser.previous,
        };
        if let Some(token) = token {
            eprint!("[line {}] Error", token.line);
            match token.token_type {
                TokenType::Eof => {
                    eprint!(" at end");
                },
                TokenType::Error => {},
                _ => {
                    eprint!(" at '{}'", token.source);
                },
            }
            eprintln!(": {}", message);
        } else {
            eprintln!("Error: {}", message);
        }
        self.parser.had_error = true;
    }

    fn expression(&mut self) -> InterpretResult<()> {
        Ok(())
    }

    fn scan_token(&mut self) -> Token<'s> {
        self.scanner.scan_token()
    }

    fn write_instruction<T: InstructionWrite>(&mut self, instruction: T) {
        let line_no = self.parser.previous.as_ref().map_or(0, |t| t.line);
        self.chunk.write_instruction(instruction, line_no);
    }
}
