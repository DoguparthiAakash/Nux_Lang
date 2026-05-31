use std::vec::Vec;
use std::string::String;
use std::format;
// use std::string::ToString;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Print,
    Println,
    Input,
    Class,
    Enum,
    Trait,
    Func,
    Fn,
    Var,
    Let,
    Const,
    Return,
    New,
    This,
    If,
    Else,
    While,
    For,
    Do,
    Defer,
    Match,
    Asm,
    Unsafe,
    Spawn,
    Join,
    Parallel,
    Async,
    Lock,
    Unlock,
    Import,
    Peek,
    Peek32,
    Poke,
    Poke32,
    Break,
    Continue,
    Try,
    Catch,
    Throw,
    Identifier(String),
    String(String),
    Float(f64),
    Number(i64),
    
    // Type Keywords
    KwInt, KwFloat, KwByte, KwShort, KwLong, KwChar, KwString,
    
    // Boolean
    True, False, Not,
    
    // Vision
    ImgAlloc, ImgFree, ImgDraw, CamCapture, ImgFilter, ImgGet, ImgSet, ImgFill,
    ImgResize, ImgCrop, ImgGrayscale,

    // Math Intrinsics
    Sin, Cos, Sqrt,
    
    // Introspection
    SysPlatform, CamCount, IsKeyDown,
    
    UpperCase, LowerCase,
    
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Slash,
    SlashSlash,
    Star,
    StarStar,
    Percent,
    Eq,
    EqEq,
    FatArrow,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or,
    Xor,
    Xand,
    Xnot,
    
    SemiColon,
    Colon,
    ColonColon,
    ColonEq,
    Dot,
    Comma,
    Plus,
    Minus,
    At,
    LShift,
    RShift,
    Caret,
    Ampersand,
    Pipe,
    Tilde,
    EOF,
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub line: usize,
    pub col: usize,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
    last_token: Option<Token>,
    pending_semi: bool,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
            last_token: None,
            pending_semi: false,
        }
    }

    pub fn next_token(&mut self) -> (Token, Span) {
        if self.pending_semi {
            self.pending_semi = false;
            let span = Span { line: self.line, col: self.col };
            self.last_token = Some(Token::SemiColon);
            return (Token::SemiColon, span);
        }
        
        let start_span = Span { line: self.line, col: self.col };
        if self.skip_whitespace_and_check_asi() {
            self.pending_semi = false;
            self.last_token = Some(Token::SemiColon);
            return (Token::SemiColon, start_span);
        }
        
        let start_span = Span { line: self.line, col: self.col };
        
        if self.pos >= self.input.len() {
            return (Token::EOF, start_span);
        }
        
        let c = self.input[self.pos];
        
        let tok = match c {
            '+' => { self.advance_pos(); (Token::Plus, start_span) },
            '-' => { self.advance_pos(); (Token::Minus, start_span) },
            '*' => {
                self.advance_pos();
                if self.pos < self.input.len() && self.input[self.pos] == '*' {
                    self.advance_pos();
                    (Token::StarStar, start_span)
                } else {
                    (Token::Star, start_span)
                }
            },
            '=' => {
                self.advance_pos();
                if self.pos < self.input.len() && self.input[self.pos] == '=' {
                    self.advance_pos();
                    (Token::EqEq, start_span)
                } else if self.pos < self.input.len() && self.input[self.pos] == '>' {
                    self.advance_pos();
                    (Token::FatArrow, start_span)
                } else {
                    (Token::Eq, start_span)
                }
            },
            '%' => { self.advance_pos(); (Token::Percent, start_span) },
            '(' => { self.advance_pos(); (Token::LParen, start_span) },
            ')' => { self.advance_pos(); (Token::RParen, start_span) },
            '[' => { self.advance_pos(); (Token::LBracket, start_span) },
            ']' => { self.advance_pos(); (Token::RBracket, start_span) },
            '{' => { self.advance_pos(); (Token::LBrace, start_span) },
            '}' => { self.advance_pos(); (Token::RBrace, start_span) },
            ';' => { self.advance_pos(); (Token::SemiColon, start_span) },
            ':' => {
                self.advance_pos();
                if self.pos < self.input.len() && self.input[self.pos] == '=' {
                    self.advance_pos();
                    (Token::ColonEq, start_span)
                } else if self.pos < self.input.len() && self.input[self.pos] == ':' {
                    self.advance_pos();
                    (Token::ColonColon, start_span)
                } else {
                    (Token::Colon, start_span)
                }
            },
            '.' => { self.advance_pos(); (Token::Dot, start_span) },
            ',' => { self.advance_pos(); (Token::Comma, start_span) },
            '@' => { self.advance_pos(); (Token::At, start_span) },
            '/' => {
                self.advance_pos();
                if self.pos < self.input.len() && self.input[self.pos] == '/' {
                    // Floor division //
                    self.advance_pos();
                    (Token::SlashSlash, start_span)
                } else {
                    (Token::Slash, start_span)
                }
            },
            '#' => {
                self.advance_pos(); // Skip #
                
                // Multi-line comment: #* ... *#
                if self.pos < self.input.len() && self.input[self.pos] == '*' {
                    self.advance_pos(); // Skip *
                    
                    let mut depth = 1;
                    
                    while self.pos < self.input.len() && depth > 0 {
                        if self.input[self.pos] == '#' {
                            self.advance_pos();
                            if self.pos < self.input.len() && self.input[self.pos] == '*' {
                                depth += 1;
                                self.advance_pos();
                            }
                        } else if self.input[self.pos] == '*' {
                            self.advance_pos();
                            if self.pos < self.input.len() && self.input[self.pos] == '#' {
                                depth -= 1;
                                self.advance_pos();
                            }
                        } else {
                            self.advance_pos();
                        }
                    }
                    self.next_token()
                } else {
                    // Single-line comment: skip to end of line
                    while self.pos < self.input.len() && self.input[self.pos] != '\n' {
                        self.advance_pos();
                    }
                    self.next_token()
                }
            },
            '=' | '!' | '<' | '>' | '&' | '|' | '^' | '~' => {
                 self.lex_operator(start_span)
            },
            '\'' => {
                 self.advance_pos(); // Skip open quote
                 if self.pos < self.input.len() {
                     let c = self.input[self.pos];
                     self.advance_pos();
                     if self.pos < self.input.len() && self.input[self.pos] == '\'' {
                         self.advance_pos(); // Skip closing quote
                         (Token::Number(c as i64), start_span)
                     } else {
                          (Token::Identifier(format!("Invalid char literal")), start_span)
                     }
                 } else {
                     (Token::Identifier(format!("Unexpected EOF in char")), start_span)
                 }
            },
            '"' => self.lex_string(start_span),
            _ if c.is_digit(10) => self.lex_number(start_span),
            _ if c.is_alphabetic() || c == '_' => self.lex_identifier(start_span),
            _ => {
                self.advance_pos(); // Skip unknown
                (Token::Identifier(format!("UNKNOWN_CHAR_{}", c)), start_span)
            }
        };
        self.last_token = Some(tok.0.clone());
        tok
    }
    
    fn advance_pos(&mut self) {
        if self.pos < self.input.len() {
            let c = self.input[self.pos];
            self.pos += 1;
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
    }

    fn skip_whitespace_and_check_asi(&mut self) -> bool {
        let mut hit_newline = false;
        while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
            if self.input[self.pos] == '\n' {
                hit_newline = true;
            }
            self.advance_pos();
        }
        
        // If we hit a newline and the last token can end a statement, infer semicolon
        if hit_newline {
            if let Some(tok) = &self.last_token {
                match tok {
                    Token::Identifier(_) | Token::Number(_) | Token::Float(_) | Token::String(_) |
                    Token::RBrace | Token::RParen | Token::Return | Token::Break | Token::Continue |
                    Token::True | Token::False => {
                        return true;
                    }
                    _ => {}
                }
            }
        }
        false
    }

    fn lex_number(&mut self, start_span: Span) -> (Token, Span) {
        let mut s = String::new();
        let mut is_float = false;
        
        // Consume the first digit already at current pos
        let first = self.input[self.pos];
        s.push(first);
        self.advance_pos();
        
        // Check for hex (0x) or binary (0b) prefix
        if first == '0' && self.pos < self.input.len() {
            let next = self.input[self.pos];
            if next == 'x' || next == 'X' {
                // Hex literal
                self.advance_pos(); // skip 'x'
                let mut hex = String::new();
                while self.pos < self.input.len() && (self.input[self.pos].is_ascii_hexdigit() || self.input[self.pos] == '_') {
                    if self.input[self.pos] != '_' { hex.push(self.input[self.pos]); }
                    self.advance_pos();
                }
                let val = i64::from_str_radix(&hex, 16).unwrap_or(0);
                return (Token::Number(val), start_span);
            } else if next == 'b' || next == 'B' {
                // Binary literal
                self.advance_pos(); // skip 'b'
                let mut bin = String::new();
                while self.pos < self.input.len() && (self.input[self.pos] == '0' || self.input[self.pos] == '1' || self.input[self.pos] == '_') {
                    if self.input[self.pos] != '_' { bin.push(self.input[self.pos]); }
                    self.advance_pos();
                }
                let val = i64::from_str_radix(&bin, 2).unwrap_or(0);
                return (Token::Number(val), start_span);
            }
        }
        
        while self.pos < self.input.len() {
            let c = self.input[self.pos];
            if c.is_digit(10) || c == '_' {
                if c != '_' { s.push(c); }
                self.advance_pos();
            } else if c == '.' && !is_float && self.pos + 1 < self.input.len() && self.input[self.pos + 1].is_digit(10) {
                is_float = true;
                s.push(c);
                self.advance_pos();
            } else {
                break;
            }
        }
        
        if is_float {
            (Token::Float(s.parse().unwrap_or(0.0)), start_span)
        } else {
            (Token::Number(s.parse().unwrap_or(0)), start_span)
        }
    }

    fn lex_identifier(&mut self, start_span: Span) -> (Token, Span) {
        let mut text = String::new();
        while self.pos < self.input.len() && (self.input[self.pos].is_alphanumeric() || self.input[self.pos] == '_') {
            text.push(self.input[self.pos]);
            self.advance_pos();
        }

        let token = match text.as_str() {
            "print" => Token::Print,
            "println" => Token::Println,
            "input" => Token::Input,
            "func" => Token::Func,
            "fn" => Token::Fn,
            "var" => Token::Var,
            "let" => Token::Let,
            "const" => Token::Const,
            "return" => Token::Return,
            "new" => Token::New,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "do" => Token::Do,
            "defer" => Token::Defer,
            "match" => Token::Match,
            "asm" => Token::Asm,
            "unsafe" => Token::Unsafe,
            "spawn" => Token::Spawn,
            "join" => Token::Join,
            "parallel" => Token::Parallel,
            "async" => Token::Async,
            "try" => Token::Try,
            "catch" => Token::Catch,
            "throw" => Token::Throw,
            "lock" => Token::Lock,
            "unlock" => Token::Unlock,
            "import" => Token::Import,
            "peek" => Token::Peek,
            "peek32" => Token::Peek32,
            "poke" => Token::Poke,
            "poke32" => Token::Poke32,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "class" => Token::Class,
            "enum" => Token::Enum,
            "trait" => Token::Trait,
            "this" => Token::This,
            
            // Types
            "int" => Token::KwInt,
            "float" => Token::KwFloat,
            "byte" => Token::KwByte,
            "short" => Token::KwShort,
            "long" => Token::KwLong,
            "char" => Token::KwChar,
            "string" => Token::KwString,
            
            // Vision Intrinsics
            "img_alloc" => Token::ImgAlloc,
            "img_free" => Token::ImgFree,
            "img_draw" => Token::ImgDraw,
            "cam_capture" => Token::CamCapture,
            "img_filter" => Token::ImgFilter,
            "img_get" => Token::ImgGet,
            "img_set" => Token::ImgSet,
            "img_fill" => Token::ImgFill,
            "img_resize" => Token::ImgResize,
            "img_crop" => Token::ImgCrop,
            "img_grayscale" => Token::ImgGrayscale,

            "sin" => Token::Sin,
            "cos" => Token::Cos,
            "sqrt" => Token::Sqrt,

            "UpperCase" => Token::UpperCase,
            "LowerCase" => Token::LowerCase,
            
            "sys_platform" => Token::SysPlatform,
            "cam_count" => Token::CamCount,
            "is_key_down" => Token::IsKeyDown,
            
            "true" => Token::True,
            "false" => Token::False,
            "not" => Token::Not,
            "and" => Token::And,
            "or" => Token::Or,
            "xor" => Token::Xor,
            "xand" => Token::Xand,
            "xnot" => Token::Xnot,
            
            _ => Token::Identifier(text),
        };
        (token, start_span)
    }

    fn lex_string(&mut self, start_span: Span) -> (Token, Span) {
        self.advance_pos(); // Skip quote
        let mut s = String::new();
        while self.pos < self.input.len() && self.input[self.pos] != '"' {
             s.push(self.input[self.pos]);
             self.advance_pos();
        }
        self.advance_pos(); // Skip closing quote
        (Token::String(s), start_span)
    }
    
    fn lex_operator(&mut self, start_span: Span) -> (Token, Span) {
        let c = self.input[self.pos];
        self.advance_pos();
        
        if self.pos < self.input.len() {
            let next = self.input[self.pos];
            if c == '=' && next == '=' { self.advance_pos(); return (Token::EqEq, start_span); }
            if c == '!' && next == '=' { self.advance_pos(); return (Token::NotEq, start_span); }
            if c == '<' && next == '=' { self.advance_pos(); return (Token::LtEq, start_span); }
            if c == '>' && next == '=' { self.advance_pos(); return (Token::GtEq, start_span); }
            if c == '&' && next == '&' { self.advance_pos(); return (Token::And, start_span); }
            if c == '|' && next == '|' { self.advance_pos(); return (Token::Or, start_span); }
            if c == '<' && next == '<' { self.advance_pos(); return (Token::LShift, start_span); }
            if c == '>' && next == '>' { self.advance_pos(); return (Token::RShift, start_span); }
        }
        
        match c {
            '=' => (Token::Eq, start_span),
            '<' => (Token::Lt, start_span),
            '>' => (Token::Gt, start_span),
            '&' => (Token::Ampersand, start_span),
            '|' => (Token::Pipe, start_span),
            '^' => (Token::Caret, start_span),
            '~' => (Token::Tilde, start_span),
            _ => (Token::Identifier(format!("{}", c)), start_span),
        }
    }
}
