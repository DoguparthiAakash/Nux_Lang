import re

with open('src/compiler.rs', 'r') as f:
    text = f.read()

start_idx = text.find('fn parse_statement_impl')
end_idx = text.find('fn parse_expression', start_idx)
impl_text = text[start_idx:end_idx]

id_start = impl_text.find('Token::Identifier(name) => {')
print(impl_text[id_start:id_start+15000])
