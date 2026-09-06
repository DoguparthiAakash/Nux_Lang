with open('src/compiler.rs', 'r') as f:
    text = f.read()

start_idx = text.find('fn parse_statement_impl')
dot_idx = text.find('} else if self.current_token == Token::Dot {', start_idx)
end_idx = text.find('Token::While =>', start_idx)

print(text[dot_idx:end_idx][:5000])
