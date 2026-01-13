#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Print,
    Println,
    Input,
    Class,
    Func,
    Var,
    Return,
    New,
    If,
    Else,
    While,
    For,
    Do,
    Asm,
    Spawn, // NEW: Multi-threading
    Lock,  // NEW: Synchronization
    Unlock, // NEW: Synchronization
    Import, // NEW: Standard Library Includes
    Peek,   // NEW: Memory Access
    Poke,   // NEW: Memory Access
    Break,  // NEW: Loop Control
    Continue, // NEW: Loop Control
    Identifier(String),
    String(String),
    Float(f64), // NEW: Float Literal
    Number(i64),
    
    // Type Keywords
    KwInt, KwFloat, KwByte, KwShort, KwLong, KwChar, KwString,
    
    // Boolean
    True, False, Not,
    
    // Vision
    ImgAlloc, ImgFree, ImgDraw, CamCapture, ImgFilter, ImgGet, ImgSet,
    ImgResize, ImgCrop, ImgGrayscale,

    
    // Introspection
    SysPlatform, CamCount,
    
    UpperCase, LowerCase,
    
    LParen,
    RParen,
    LBrace,
    RBrace,
    Slash,
    SlashSlash,
    Star,
    StarStar,
    Percent,
    Eq,
    EqEq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or,
    SemiColon,
    Colon,
    Dot,
    Comma,
    Plus,
    Minus,
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
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn next_token(&mut self) -> (Token, Span) {
        self.skip_whitespace();
        let start_span = Span { line: self.line, col: self.col };
        
        if self.pos >= self.input.len() {
            return (Token::EOF, start_span);
        }
        
        let c = self.input[self.pos];
        
        // Helper to advance and track pos/col
        // But wait, skip_whitespace advances too.
        // We need centralized "advance_char" method to track line/col correctly.
        
        // Let's refactor slightly to just peek here and let specific handlers consume.
        // Current implementation uses self.pos manually.
        // I will stick to existing style but update line/col manually.
        
        match c {
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
            '%' => { self.advance_pos(); (Token::Percent, start_span) },
            '(' => { self.advance_pos(); (Token::LParen, start_span) },
            ')' => { self.advance_pos(); (Token::RParen, start_span) },
            '{' => { self.advance_pos(); (Token::LBrace, start_span) },
            '}' => { self.advance_pos(); (Token::RBrace, start_span) },
            ';' => { self.advance_pos(); (Token::SemiColon, start_span) },
            ':' => { self.advance_pos(); (Token::Colon, start_span) },
            '.' => { self.advance_pos(); (Token::Dot, start_span) },
            ',' => { self.advance_pos(); (Token::Comma, start_span) },
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
                
                // Check if it's a multi-line comment: #* ... *#
                if self.pos < self.input.len() && self.input[self.pos] == '*' {
                    self.advance_pos(); // Skip *
                    
                    // Multi-line comment with nesting support
                    let mut depth = 1;
                    
                    while self.pos < self.input.len() && depth > 0 {
                        if self.input[self.pos] == '#' {
                            self.advance_pos();
                            if self.pos < self.input.len() && self.input[self.pos] == '*' {
                                // Found #* - increase nesting depth
                                depth += 1;
                                self.advance_pos();
                            }
                        } else if self.input[self.pos] == '*' {
                            self.advance_pos();
                            if self.pos < self.input.len() && self.input[self.pos] == '#' {
                                // Found *# - decrease nesting depth
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
            '=' | '!' | '<' | '>' | '&' | '|' => {
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
                          // Unterminated or empty
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
        }
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

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
            self.advance_pos();
        }
    }

    fn lex_number(&mut self, start_span: Span) -> (Token, Span) {
        let mut s = String::new();
        let mut is_float = false;
        while self.pos < self.input.len() {
            let c = self.input[self.pos];
            if c.is_digit(10) {
                s.push(c);
                self.advance_pos();
            } else if c == '.' && !is_float {
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
            "var" => Token::Var,
            "return" => Token::Return,
            "new" => Token::New,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "do" => Token::Do,
            "asm" => Token::Asm,
            "spawn" => Token::Spawn,
            "lock" => Token::Lock,
            "unlock" => Token::Unlock,
            "import" => Token::Import,
            "peek" => Token::Peek,
            "poke" => Token::Poke,
            "break" => Token::Break,
            "continue" => Token::Continue,
            
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
            "img_resize" => Token::ImgResize,
            "img_crop" => Token::ImgCrop,
            "img_grayscale" => Token::ImgGrayscale,
            "UpperCase" => Token::UpperCase,
            "LowerCase" => Token::LowerCase,
            
            "sys_platform" => Token::SysPlatform,
            "cam_count" => Token::CamCount,
            
            "true" => Token::True,
            "false" => Token::False,
            "not" => Token::Not,
            "and" => Token::And,
            "or" => Token::Or,
            
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
        }
        
        match c {
            '=' => (Token::Eq, start_span),
            '<' => (Token::Lt, start_span),
            '>' => (Token::Gt, start_span),
            _ => (Token::Identifier(format!("{}", c)), start_span), // Should not happen often
        }
    }
}
