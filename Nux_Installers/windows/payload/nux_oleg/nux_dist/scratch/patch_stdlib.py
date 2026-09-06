import sys

def patch_file(filepath, replacements):
    with open(filepath, "r", encoding="utf-8") as f:
        content = f.read()
    
    for old, new in replacements:
        if old in content:
            content = content.replace(old, new)
        else:
            print(f"Warning: Could not find snippet in {filepath}:\n{old[:100]}...")
            
    with open(filepath, "w", encoding="utf-8") as f:
        f.write(content)


# 1. Update Lexer
lexer_replacements = [
    (
        "// Vision Intrinsics",
        "// Stdlib Intrinsics\n            \"fs_read\" => Token::FsRead,\n            \"fs_write\" => Token::FsWrite,\n            \"fs_exists\" => Token::FsExists,\n            \"os_env\" => Token::OsEnv,\n            \"os_cwd\" => Token::OsCwd,\n            \"os_exec\" => Token::OsExec,\n            \"time_now\" => Token::TimeNow,\n            \"time_sleep\" => Token::TimeSleep,\n            // Vision Intrinsics"
    ),
    (
        "// Vision",
        "// Stdlib\n    FsRead, FsWrite, FsExists, OsEnv, OsCwd, OsExec, TimeNow, TimeSleep,\n\n    // Vision"
    )
]

patch_file("src/lexer.rs", lexer_replacements)


# 2. Update Compiler
compiler_replacements = [
    (
        "Token::NetListenTls => 0xB5,",
        "Token::NetListenTls => 0xB5,\n            Token::FsRead => 0xC0,\n            Token::FsWrite => 0xC1,\n            Token::FsExists => 0xC2,\n            Token::OsEnv => 0xC5,\n            Token::OsCwd => 0xC6,\n            Token::OsExec => 0xC7,\n            Token::TimeNow => 0xCA,\n            Token::TimeSleep => 0xCB,"
    )
]

patch_file("src/compiler.rs", compiler_replacements)


# 3. Update VM
vm_replacements = [
    (
        "use std::collections::{HashMap, BTreeMap};",
        "use std::collections::{HashMap, BTreeMap};\nuse std::process::Command;\nuse std::time::{SystemTime, UNIX_EPOCH};"
    ),
    (
        "0xB5 => { // OP_NET_LISTEN_TLS",
        """0xC0 => { // OP_FS_READ
                        let path_id = self.stack.pop().unwrap_or(0) as usize;
                        let mut path = String::new();
                        {
                            let heap_strings = self.shared.heap_strings.read().unwrap();
                            if path_id < heap_strings.len() { path = heap_strings[path_id].clone(); }
                        }
                        let content = std::fs::read_to_string(path).unwrap_or_default();
                        let mut heap_strings = self.shared.heap_strings.write().unwrap();
                        let id = heap_strings.len();
                        heap_strings.push(content);
                        self.stack.push(id as i64);
                    }
                    0xC1 => { // OP_FS_WRITE
                        let data_id = self.stack.pop().unwrap_or(0) as usize;
                        let path_id = self.stack.pop().unwrap_or(0) as usize;
                        let mut path = String::new();
                        let mut data = String::new();
                        {
                            let heap_strings = self.shared.heap_strings.read().unwrap();
                            if path_id < heap_strings.len() { path = heap_strings[path_id].clone(); }
                            if data_id < heap_strings.len() { data = heap_strings[data_id].clone(); }
                        }
                        let res = std::fs::write(path, data).is_ok();
                        self.stack.push(if res { 1 } else { 0 });
                    }
                    0xC2 => { // OP_FS_EXISTS
                        let path_id = self.stack.pop().unwrap_or(0) as usize;
                        let mut path = String::new();
                        {
                            let heap_strings = self.shared.heap_strings.read().unwrap();
                            if path_id < heap_strings.len() { path = heap_strings[path_id].clone(); }
                        }
                        let res = std::path::Path::new(&path).exists();
                        self.stack.push(if res { 1 } else { 0 });
                    }
                    0xC5 => { // OP_OS_ENV
                        let key_id = self.stack.pop().unwrap_or(0) as usize;
                        let mut key = String::new();
                        {
                            let heap_strings = self.shared.heap_strings.read().unwrap();
                            if key_id < heap_strings.len() { key = heap_strings[key_id].clone(); }
                        }
                        let val = std::env::var(key).unwrap_or_default();
                        let mut heap_strings = self.shared.heap_strings.write().unwrap();
                        let id = heap_strings.len();
                        heap_strings.push(val);
                        self.stack.push(id as i64);
                    }
                    0xC6 => { // OP_OS_CWD
                        let val = std::env::current_dir().unwrap_or_default().to_string_lossy().to_string();
                        let mut heap_strings = self.shared.heap_strings.write().unwrap();
                        let id = heap_strings.len();
                        heap_strings.push(val);
                        self.stack.push(id as i64);
                    }
                    0xC7 => { // OP_OS_EXEC
                        let cmd_id = self.stack.pop().unwrap_or(0) as usize;
                        let mut cmd = String::new();
                        {
                            let heap_strings = self.shared.heap_strings.read().unwrap();
                            if cmd_id < heap_strings.len() { cmd = heap_strings[cmd_id].clone(); }
                        }
                        let output = Command::new("cmd")
                            .args(&["/C", &cmd])
                            .output();
                        let val = if let Ok(o) = output { String::from_utf8_lossy(&o.stdout).to_string() } else { String::new() };
                        let mut heap_strings = self.shared.heap_strings.write().unwrap();
                        let id = heap_strings.len();
                        heap_strings.push(val);
                        self.stack.push(id as i64);
                    }
                    0xCA => { // OP_TIME_NOW
                        let start = SystemTime::now();
                        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
                        self.stack.push(since_the_epoch.as_millis() as i64);
                    }
                    0xCB => { // OP_TIME_SLEEP
                        let ms = self.stack.pop().unwrap_or(0) as u64;
                        std::thread::sleep(std::time::Duration::from_millis(ms));
                    }
                    0xB5 => { // OP_NET_LISTEN_TLS"""
    )
]

patch_file("src/vm.rs", vm_replacements)

print("Patch applied.")
