import sys

with open("src/lexer.rs", "r") as f:
    text = f.read()

old_func = """    fn lex_string(&mut self, start_span: Span) -> (Token, Span) {
        self.advance_pos(); // Skip quote
        let mut s = String::new();
        while self.pos < self.input.len() && self.input[self.pos] != '"' {
             s.push(self.input[self.pos]);
             self.advance_pos();
        }
        self.advance_pos(); // Skip closing quote
        (Token::String(s), start_span)
    }"""

new_func = """    fn lex_string(&mut self, start_span: Span) -> (Token, Span) {
        self.advance_pos(); // Skip quote
        let mut s = String::new();
        while self.pos < self.input.len() && self.input[self.pos] != '"' {
             if self.input[self.pos] == '\\\\' && self.pos + 1 < self.input.len() {
                 self.advance_pos();
                 let c = self.input[self.pos];
                 match c {
                     'n' => s.push('\\n'),
                     'r' => s.push('\\r'),
                     't' => s.push('\\t'),
                     '\\\\' => s.push('\\\\'),
                     '"' => s.push('"'),
                     _ => s.push(c),
                 }
             } else {
                 s.push(self.input[self.pos]);
             }
             self.advance_pos();
        }
        self.advance_pos(); // Skip closing quote
        (Token::String(s), start_span)
    }"""

if old_func in text:
    text = text.replace(old_func, new_func)
    with open("src/lexer.rs", "w") as f:
        f.write(text)
    print("Patched lexer.rs successfully!")
else:
    print("Could not find lex_string!")
