import re
with open(src/compiler.rs, r) as f:
    content = f.read()
content = re.sub(
    rif part1 == vbe_set_mode \{\s*if self\.current_token != Token::LParen \{ return self\.error\(Expected \("\.to_string\(\)\); \}\s*self\.advance\(\);\s*self\.parse_expression\(out\)\?;\s*if self\.current_token != Token::Comma \{ return self\.error\(Expected ,\.to_string\(\)\); \}\s*self\.advance\(\);\s*self\.parse_expression\(out\)\?;\s*if self\.current_token != Token::RParen \{ return self\.error\(Expected \)"\.to_string\(\)\); \},
    rif part1 == vbe_set_mode { if self.current_token != Token::LParen { return self.error(Expected (.to_string()); } self.advance(); self.parse_expression(out)?; if self.current_token != Token::Comma { return self.error(Expected ,.to_string()); } self.advance(); self.parse_expression(out)?; if self.current_token != Token::Comma { return self.error(Expected ,.to_string()); } self.advance(); self.parse_expression(out)?; if self.current_token != Token::RParen { return self.error(Expected ).to_string()); },
    content
)
with open(src/compiler.rs, w) as f:
    f.write(content)
