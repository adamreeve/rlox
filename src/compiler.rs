use std::error::Error;
use num_traits::FromPrimitive;
use num_traits::ToPrimitive;

use chunk::Chunk;
use debug;
use errors::{InterpretError, InterpretResult};
use instructions::*;
use scanner::{Scanner, Token, TokenType};
use value::Value;

pub fn compile(source: &str) -> InterpretResult<Chunk>
{
    let mut scanner = Scanner::new(&source);
    let compiler = Compiler::new(&mut scanner);
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

struct ParseRule<'a, 's:'a> {
    prefix: Option<fn(&mut Compiler<'a, 's>)>,
    infix: Option<fn(&mut Compiler<'a, 's>)>,
    precedence: Precedence,
}

#[derive(Debug,Copy,Clone,Primitive)]
enum Precedence {
    None = 0,
    Assignment = 1,  // =
    Or = 2,          // or
    And = 3,         // and
    Equality = 4,    // == !=
    Comparison = 5,  // < > <= >=
    Term = 6,        // + -
    Factor = 7,      // * /
    Unary = 8,       // ! - +
    Call = 9,        // . () []
    Primary = 10
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
        self.expression();
        self.consume(TokenType::Eof, "Expected end of expression");
        if self.parser.had_error {
            #[cfg(feature="debug-print-code")]
            {
                debug::disassemble_chunk(&self.chunk, "code");
            }
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
                self.parser.current = Some(current_token);
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
        self.write_op_code(OpCode::Return);
        #[cfg(feature="debug-print-code")]
        {
            if !self.parser.had_error {
                debug::disassemble_chunk(&self.chunk, "code");
            }
        }
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

    fn current_token_type(&self) -> TokenType {
        self.parser.current.as_ref().unwrap().token_type
    }

    fn previous_token_type(&self) -> TokenType {
        self.parser.previous.as_ref().unwrap().token_type
    }

    fn scan_token(&mut self) -> Token<'s> {
        self.scanner.scan_token()
    }

    fn write_instruction<T: InstructionWrite>(&mut self, instruction: T) {
        let line_no = self.parser.previous.as_ref().map_or(0, |t| t.line);
        self.chunk.write_instruction(instruction, line_no);
    }

    fn write_op_code(&mut self, op_code: OpCode) {
        let line_no = self.parser.previous.as_ref().map_or(0, |t| t.line);
        let instruction = SimpleInstruction::new(op_code);
        self.chunk.write_instruction(instruction, line_no);
    }

    fn write_op_codes(&mut self, op_codes: &[OpCode]) {
        let line_no = self.parser.previous.as_ref().map_or(0, |t| t.line);
        for op_code in op_codes {
            let instruction = SimpleInstruction::new(op_code.clone());
            self.chunk.write_instruction(instruction, line_no);
        }
    }

    fn emit_constant(&mut self, value: Value) {
        let line_no = self.parser.previous.as_ref().map_or(0, |t| t.line);
        if let Err(err) = self.chunk.write_constant(value, line_no) {
            self.error(err.description());
        }
    }

    // Expression parsing
    // ------------------

    fn binary(&mut self) {
        let operator_type = self.previous_token_type();

        // Compile the right operand
        // Use one higher precedence due to operators being left-associative
        let rule = get_rule(operator_type);
        let precedence: Precedence = FromPrimitive::from_usize(rule.precedence.to_usize().unwrap() + 1).unwrap();
        self.parse_precedence(precedence);

        // Emit the operator instruction
        match operator_type {
            TokenType::BangEqual => self.write_op_codes(&[OpCode::Equal, OpCode::Not]),
            TokenType::EqualEqual => self.write_op_code(OpCode::Equal),
            TokenType::Greater => self.write_op_code(OpCode::Greater),
            TokenType::GreaterEqual => self.write_op_codes(&[OpCode::Less, OpCode::Not]),
            TokenType::Less => self.write_op_code(OpCode::Less),
            TokenType::LessEqual => self.write_op_codes(&[OpCode::Greater, OpCode::Not]),
            TokenType::Plus => self.write_op_code(OpCode::Add),
            TokenType::Minus => self.write_op_code(OpCode::Subtract),
            TokenType::Star => self.write_op_code(OpCode::Multiply),
            TokenType::Slash => self.write_op_code(OpCode::Divide),
            _ => self.error("Invalid binary operator"),
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expected ')' after expression");
    }

    fn literal(&mut self) {
        let op = self.parser.previous.as_ref().unwrap().token_type;
        match op {
            TokenType::True => self.write_instruction(SimpleInstruction::new(OpCode::True)),
            TokenType::False => self.write_instruction(SimpleInstruction::new(OpCode::False)),
            TokenType::Nil => self.write_instruction(SimpleInstruction::new(OpCode::Nil)),
            _ => panic!("Invalid literal opcode"),
        }
    }

    fn number(&mut self) {
        let number: Result<f64,_> = self.parser.previous.as_ref().unwrap().source.parse();
        let value = Value::number(number.unwrap());
        self.emit_constant(value);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let parse_rule = get_rule(self.previous_token_type());
        match parse_rule.prefix {
            Some(prefix_parse) => {prefix_parse(self)},
            None => self.error("Expected expression."),
        }

        while precedence.to_usize() <= get_rule(self.current_token_type()).precedence.to_usize() {
            self.advance();
            let parse_rule = get_rule(self.previous_token_type());
            match parse_rule.infix {
                Some(infix_parse) => {infix_parse(self)},
                None => self.error("No infix parsing method set."),
            }
        }
    }

    fn unary(&mut self) {
        let operator_type = self.previous_token_type();

        // Compile the operand
        self.parse_precedence(Precedence::Unary);

        // Emit the operator instruction
        match operator_type {
            TokenType::Bang => {
                self.write_op_code(OpCode::Not);
            },
            TokenType::Minus => {
                self.write_op_code(OpCode::Negate);
            },
            _ => {
                self.error("Invalid unary operator");
            }
        }
    }
}

impl <'a, 's> ParseRule<'a, 's> {
    fn new(prefix: Option<fn(&mut Compiler<'a, 's>)>, infix: Option<fn(&mut Compiler<'a, 's>)>, precedence: Precedence) -> ParseRule<'a, 's> {
        ParseRule { prefix, infix, precedence }
    }
}

fn get_rule<'a, 's>(token: TokenType) -> ParseRule<'a, 's> {
    match token {
        TokenType::LeftParen    => ParseRule::new(Some(Compiler::grouping), None,                   Precedence::Call),
        TokenType::RightParen   => ParseRule::new(None,                     None,                   Precedence::None),
        TokenType::LeftBrace    => ParseRule::new(None,                     None,                   Precedence::None),
        TokenType::RightBrace   => ParseRule::new(None,                     None,                   Precedence::None),
        TokenType::Comma        => ParseRule::new(None,                     None,                   Precedence::None),
        TokenType::Dot          => ParseRule::new(None,                     None,                   Precedence::Call),
        TokenType::Minus        => ParseRule::new(Some(Compiler::unary),    Some(Compiler::binary), Precedence::Term),
        TokenType::Plus         => ParseRule::new(None,                     Some(Compiler::binary), Precedence::Term),
        TokenType::Semicolon    => ParseRule::new(None,                     None,                   Precedence::None),
        TokenType::Slash        => ParseRule::new(None,                     Some(Compiler::binary), Precedence::Factor),
        TokenType::Star         => ParseRule::new(None,                     Some(Compiler::binary), Precedence::Factor),
        TokenType::Bang         => ParseRule::new(Some(Compiler::unary),    None,                   Precedence::None),
        TokenType::BangEqual    => ParseRule::new(None,                     Some(Compiler::binary), Precedence::Equality),
        TokenType::Equal        => ParseRule::new(None,                     None,                   Precedence::None),
        TokenType::EqualEqual   => ParseRule::new(None,                     Some(Compiler::binary), Precedence::Equality),
        TokenType::Greater      => ParseRule::new(None,                     Some(Compiler::binary), Precedence::Comparison),
        TokenType::GreaterEqual => ParseRule::new(None,                     Some(Compiler::binary), Precedence::Comparison),
        TokenType::Less         => ParseRule::new(None,                     Some(Compiler::binary), Precedence::Comparison),
        TokenType::LessEqual    => ParseRule::new(None,                     Some(Compiler::binary), Precedence::Comparison),
        TokenType::Identifier   => ParseRule::new(None,                     None,                   Precedence::None),
        TokenType::String       => ParseRule::new(None,                     None,                   Precedence::None),
        TokenType::Number       => ParseRule::new(Some(Compiler::number),   None,                   Precedence::None),
        TokenType::And          => ParseRule::new(None,                     None,                   Precedence::And),
        TokenType::Class        => ParseRule::new(None,                     None,                   Precedence::None),
        TokenType::Else         => ParseRule::new(None,                     None,                   Precedence::None),
        TokenType::False        => ParseRule::new(Some(Compiler::literal),  None,                   Precedence::None),
        TokenType::Fun          => ParseRule::new(None,                     None,                   Precedence::None),
        TokenType::For          => ParseRule::new(None,                     None,                   Precedence::None),
        TokenType::If           => ParseRule::new(None,                     None,                   Precedence::None),
        TokenType::Nil          => ParseRule::new(Some(Compiler::literal),  None,                   Precedence::None),
        TokenType::Or           => ParseRule::new(None,                     None,                   Precedence::Or),
        TokenType::Print        => ParseRule::new(None,                     None,                   Precedence::None),
        TokenType::Return       => ParseRule::new(None,                     None,                   Precedence::None),
        TokenType::Super        => ParseRule::new(None,                     None,                   Precedence::None),
        TokenType::This         => ParseRule::new(None,                     None,                   Precedence::None),
        TokenType::True         => ParseRule::new(Some(Compiler::literal),  None,                   Precedence::None),
        TokenType::Var          => ParseRule::new(None,                     None,                   Precedence::None),
        TokenType::While        => ParseRule::new(None,                     None,                   Precedence::None),
        TokenType::Error        => ParseRule::new(None,                     None,                   Precedence::None),
        TokenType::Eof          => ParseRule::new(None,                     None,                   Precedence::None),
    }
}
