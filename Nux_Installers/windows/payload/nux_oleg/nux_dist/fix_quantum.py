import os

path_compiler = r"E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\compiler.rs"

with open(path_compiler, "r", encoding="utf-8") as f:
    content = f.read()

# Fix expect_semi and Ok(()) -> Ok(Type::Int)
content = content.replace('if expect_semi && self.current_token == Token::SemiColon { self.advance(); }', '')
content = content.replace('else if self.current_token == Token::SemiColon { self.advance(); }', '')
content = content.replace('return Ok(());', 'return Ok(Type::Int);')

with open(path_compiler, "w", encoding="utf-8") as f:
    f.write(content)

path_vm = r"E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\vm.rs"

with open(path_vm, "r", encoding="utf-8") as f:
    content = f.read()

# Fix thread_rng (must import Rng)
# wait, the error is `cannot find function thread_rng in crate rand`.
# It's `rand::thread_rng()`. `rand` is imported. But maybe the `rand` version in Cargo.toml doesn't have `thread_rng`?
# Or we need `use rand::Rng;` and it was unused earlier because it wasn't used until now. But it IS there.
# Let's check `rand` version in Cargo.toml or use `rand::random::<f64>()`.

content = content.replace('let mut rng = rand::thread_rng();\\n                    let random_val: f64 = rng.gen();', 'let random_val: f64 = rand::random();')
content = content.replace('let mut rng = rand::thread_rng();\n                    let random_val: f64 = rng.gen();', 'let random_val: f64 = rand::random();')

with open(path_vm, "w", encoding="utf-8") as f:
    f.write(content)

print("Fixed compile errors!")
