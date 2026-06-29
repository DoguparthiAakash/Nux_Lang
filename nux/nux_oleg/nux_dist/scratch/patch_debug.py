import re

with open("src/compiler.rs", "r", encoding="utf-8") as f:
    content = f.read()

content = content.replace(
    "fn parse_statement_impl(&mut self, out: &mut String, expect_semi: bool) -> Result<(), CompileError> {",
    "fn parse_statement_impl(&mut self, out: &mut String, expect_semi: bool) -> Result<(), CompileError> {\n        println!(\"DEBUG: parse_statement_impl, token: {:?}, expect_semi: {}\", self.current_token, expect_semi);"
)

content = content.replace(
    "fn parse_func(&mut self, out: &mut String, class_prefix: &str) -> Result<String, CompileError> {",
    "fn parse_func(&mut self, out: &mut String, class_prefix: &str) -> Result<String, CompileError> {\n        println!(\"DEBUG: enter parse_func\");"
)

content = content.replace(
    "fn parse_class(&mut self, out: &mut String) -> Result<(), CompileError> {",
    "fn parse_class(&mut self, out: &mut String) -> Result<(), CompileError> {\n        println!(\"DEBUG: enter parse_class\");"
)

with open("src/compiler.rs", "w", encoding="utf-8") as f:
    f.write(content)
