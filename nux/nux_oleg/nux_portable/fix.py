import re

content = open('src/high_level.rs', 'r', encoding='utf-8').read()

pattern = r'let mut sub_out = String::new\(\); let \(_, c\) = self\.parse_expression\(&mut sub_out\)\?; if let Some\(val\) = c \{ match val \{ ConstantValue::Int\(i\) => out\.push_str\(&format!\("PUSH \{\}\\n", i\)\), _ => \{\} \} \} else \{ out\.push_str\(&sub_out\); \}'

new_content = re.sub(pattern, 'self.parse_expression_and_push(out)?;', content)

pattern2 = r'^\s*self\.parse_expression\(out\)\?;\s*$'
new_content2 = []
lines = new_content.split('\n')
for line in lines:
    if re.match(pattern2, line):
        new_content2.append(line.replace('self.parse_expression(out)?', 'self.parse_expression_and_push(out)?'))
    else:
        new_content2.append(line)

new_content = '\n'.join(new_content2)

func_str = '''
    fn parse_expression_and_push(&mut self, out: &mut String) -> Result<Type, CompileError> {
        let mut sub_out = String::new();
        let (typ, const_opt) = self.parse_expression(&mut sub_out)?;
        if let Some(val) = const_opt {
             match val {
                  ConstantValue::Int(i) => out.push_str(&format!("PUSH {}\\n", i)),
                  ConstantValue::Float(f) => out.push_str(&format!("PUSH {}\\n", f.to_bits() as i64)),
                  ConstantValue::Bool(b) => out.push_str(&format!("PUSH {}\\n", if b { 1 } else { 0 })),
                  ConstantValue::String(_) => out.push_str("PUSH 0 ; String Literal Placeholder\\n"),
                  ConstantValue::None => {},
             }
        } else {
             out.push_str(&sub_out);
        }
        Ok(typ)
    }

    fn parse_expression(&mut self, out: &mut String) -> Result<(Type, Option<ConstantValue>), CompileError> {'''

new_content = new_content.replace('    fn parse_expression(&mut self, out: &mut String) -> Result<(Type, Option<ConstantValue>), CompileError> {', func_str)

open('src/high_level.rs', 'w', encoding='utf-8').write(new_content)
