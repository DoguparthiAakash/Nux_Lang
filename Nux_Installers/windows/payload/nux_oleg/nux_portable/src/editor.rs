use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::process::Command;

// ANSI Color codes for syntax highlighting
const COLOR_KEYWORD: &str = "\x1b[34m";    // Blue
const COLOR_STRING: &str = "\x1b[32m";     // Green  
const COLOR_NUMBER: &str = "\x1b[33m";     // Yellow
const COLOR_COMMENT: &str = "\x1b[90m";    // Gray
const COLOR_OPERATOR: &str = "\x1b[35m";   // Magenta
const COLOR_RESET: &str = "\x1b[0m";

pub fn run(filename: &str) {
    // Try system editor first? User seems to want THIS specific behavior now.
    // But keeping the fallback logic is good practice. 
    // However, if the user explicitly asked for THESE bindings, they might be testing Nux's editor.
    // I will comment out system delegation for testing, or keep it but assume user calls `nux edit` when they want nux.
    // Actually, widespread behavior is `nux edit` -> `vim` is good. 
    // I'll keep delegation but maybe user didn't have vim? 
    // Wait, previous logs showed they had `vi`.
    // The user explicitly tested `nux edit sample.nux`.
    // If I delegate, they get `vi` which definitely doesn't use `Ctrl+I` for insert.
    // **Correction**: The user WANTS these bindings. If I launch `vi`, I ignore their request.
    // I should FORCE Nux Editor if they use `nux edit`? 
    // Or maybe add a flag?
    // Current instruction: "nope. fix it fully... this makes easy to develop".
    // This implies they want the NUX editor to behave this way.
    // I will REMOVE the delegation to `vim` so they get the Nux editor with these bindings.
    
    // println!("System editor not found. Launching Nux-Vim..."); 
    // (We remove the check and always use built-in for now to satisfy the user request).
    
    let mut editor = MiniEditor::new(filename);
    editor.run();
}

// --- Editor Implementation ---

#[derive(Debug, PartialEq, Clone, Copy)]
enum Key {
    Char(char),
    Ctrl(char),
    Esc,
    Enter,
    Backspace,
    Delete,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    Unknown(u8),
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Mode {
    View,
    Insert,
}

#[derive(Clone)]
struct HistoryState {
    lines: Vec<String>,
    cx: usize,
    cy: usize,
}

struct MiniEditor {
    filename: String,
    lines: Vec<String>,
    cx: usize,
    cy: usize,
    mode: Mode,
    msg: String,
    quit: bool,
    dirty: bool,
    escape_state: u8,
    
    // Undo/Redo
    history: Vec<HistoryState>,
    history_idx: usize, // Current position in history
}

impl MiniEditor {
    fn new(filename: &str) -> Self {
        let content = fs::read_to_string(filename).unwrap_or_default();
        let lines: Vec<String> = if content.is_empty() {
            vec![String::new()]
        } else {
            content.lines().map(|s| s.to_string()).collect()
        };

        let initial_state = HistoryState {
            lines: lines.clone(),
            cx: 0,
            cy: 0,
        };

        Self {
            filename: filename.to_string(),
            lines,
            cx: 0,
            cy: 0,
            mode: Mode::View,
            msg: String::from("Ctrl+I: Insert | Ctrl+S: Save | Ctrl+A: Save&Exit | Ctrl+B: Exit"),
            quit: false,
            dirty: false,
            escape_state: 0,
            history: vec![initial_state],
            history_idx: 0,
        }
    }

    fn run(&mut self) {
        self.enable_raw_mode();
        loop {
            self.refresh_screen();
            if self.quit { break; }
            self.process_keypress();
        }
        self.disable_raw_mode();
        println!("Bye!");
    }

    fn enable_raw_mode(&self) {
        let _ = Command::new("stty").arg("raw").arg("-echo").status();
    }

    fn disable_raw_mode(&self) {
        let _ = Command::new("stty").arg("-raw").arg("echo").status();
    }

    fn refresh_screen(&self) {
        print!("\x1b[2J\x1b[H"); 
        
        // Render lines with line numbers and syntax highlighting
        for (i, line) in self.lines.iter().enumerate() {
            let line_num = format!("{:4} ", i + 1);
            let line_num_color = if i == self.cy { "\x1b[36;1m" } else { "\x1b[90m" };
            print!("{}{}{}", line_num_color, line_num, COLOR_RESET);
            
            // Apply syntax highlighting
            let highlighted = self.highlight_line(line);
            print!("{}\r\n", highlighted);
        }
        
        // Status Bar
        print!("\x1b[H\x1b[999B"); 
        
        let dirty_char = if self.dirty { "[+]" } else { "" };
        let mode_str = match self.mode { Mode::View => "VIEW", Mode::Insert => "INSERT" };

        print!("\n-- {} -- {} Line: {}/{}  {}\r\n", mode_str, dirty_char, self.cy + 1, self.lines.len(), self.msg);
        
        // Cursor (adjust for line number gutter)
        print!("\x1b[{};{}H", self.cy + 1, self.cx + 6);
        io::stdout().flush().unwrap();
    }

    fn read_key(&mut self) -> Key {
        let mut buf = [0; 1];
        if io::stdin().read_exact(&mut buf).is_err() { return Key::Unknown(0); }
        let b = buf[0];

        match self.escape_state {
            0 => match b {
                13 => Key::Enter,
                127 | 8 => Key::Backspace,
                27 => { // Esc
                    self.escape_state = 1;
                    let mut next = [0; 1];
                    if io::stdin().read_exact(&mut next).is_ok() {
                        if next[0] == b'[' || next[0] == b'O' {
                            self.escape_state = 2; 
                            return self.read_escape_sequence();
                        }
                    }
                    Key::Esc
                },
                1..=26 => Key::Ctrl((b + 64) as char), // Ctrl+A..Z
                _ => Key::Char(b as char),
            },
            _ => { self.escape_state = 0; Key::Unknown(b) }
        }
    }
    
    fn read_escape_sequence(&mut self) -> Key {
        let mut buf = [0; 1];
        if io::stdin().read_exact(&mut buf).is_err() { 
            self.escape_state = 0; return Key::Unknown(0); 
        }
        let b = buf[0];
        self.escape_state = 0;
        
        match b {
            b'A' => Key::Up,
            b'B' => Key::Down,
            b'C' => Key::Right,
            b'D' => Key::Left,
            b'H' | b'1' => Key::Home,
            b'F' | b'4' => Key::End,
            b'3' => Key::Delete,
            b'5' => Key::PageUp,
            b'6' => Key::PageDown,
            _ => Key::Unknown(b),
        }
    }

    fn process_keypress(&mut self) {
        let key = self.read_key();
        
        // Check for command mode
        if self.mode == Mode::View && matches!(key, Key::Char(':')) {
            self.handle_command();
            return;
        }
        
        // Ensure bounds
        if !self.lines.is_empty() {
             if self.cy >= self.lines.len() { self.cy = self.lines.len() - 1; }
             if self.cx > self.lines[self.cy].len() { self.cx = self.lines[self.cy].len(); }
        }

        match key {
            // GLOBAL COMMANDS
            Key::Ctrl('I') => { self.mode = Mode::Insert; self.msg = "INSERT MODE".to_string(); },
            Key::Esc => { self.mode = Mode::View; self.msg = "VIEW MODE".to_string(); },
            Key::Ctrl('S') => self.save_file(),
            Key::Ctrl('A') => { self.save_file(); self.quit = true; },
            Key::Ctrl('B') => { self.quit = true; }, // Discard and Exit
            Key::Ctrl('Z') => self.undo(),
            Key::Ctrl('Y') => self.redo(),
            
            // NAVIGATION (Always active)
            Key::Up => if self.cy > 0 { self.cy -= 1 },
            Key::Down => if self.cy < self.lines.len().saturating_sub(1) { self.cy += 1 },
            Key::Left => if self.cx > 0 { self.cx -= 1 },
            Key::Right => {
                let len = if self.cy < self.lines.len() { self.lines[self.cy].len() } else { 0 };
                if self.cx < len { self.cx += 1 }
            },
            
            // EDITING (Only in Insert Mode)
            Key::Char(c) if self.mode == Mode::Insert => self.insert_char(c),
            Key::Enter if self.mode == Mode::Insert => self.insert_newline(),
            Key::Backspace if self.mode == Mode::Insert => self.backspace(),
            Key::Delete if self.mode == Mode::Insert => self.delete_char(),
            
            _ => {},
        }
    }
    
    // --- Actions ---

    fn snapshot(&mut self) {
        // Remove redo history if we fork
        if self.history_idx < self.history.len() - 1 {
            self.history.truncate(self.history_idx + 1);
        }
        
        self.history.push(HistoryState {
            lines: self.lines.clone(),
            cx: self.cx,
            cy: self.cy,
        });
        self.history_idx += 1;
        self.dirty = true;
    }

    fn undo(&mut self) {
        if self.history_idx > 0 {
            self.history_idx -= 1;
            let state = &self.history[self.history_idx];
            self.lines = state.lines.clone();
            self.cx = state.cx;
            self.cy = state.cy;
            self.msg = "Undid change".to_string();
            // Don't set dirty=true for undo itself, but technically we are modified from 'now'. 
            // Just leaving dirty as is or setting it? 
            // If we undo to initial state, dirty could be false? complex.
        } else {
            self.msg = "Already at oldest state".to_string();
        }
    }

    fn redo(&mut self) {
        if self.history_idx < self.history.len() - 1 {
            self.history_idx += 1;
            let state = &self.history[self.history_idx];
            self.lines = state.lines.clone();
            self.cx = state.cx;
            self.cy = state.cy;
            self.msg = "Redid change".to_string();
        } else {
            self.msg = "Already at newest state".to_string();
        }
    }
    
    fn insert_char(&mut self, c: char) {
        self.snapshot();
        if self.cy >= self.lines.len() { self.lines.push(String::new()); }
        let line = &mut self.lines[self.cy];
        if self.cx >= line.len() { line.push(c); } else { line.insert(self.cx, c); }
        self.cx += 1;
    }

    fn insert_newline(&mut self) {
        self.snapshot();
        if self.cy >= self.lines.len() { self.lines.push(String::new()); return; }
        let current = &mut self.lines[self.cy];
        let rest = if self.cx < current.len() { current.split_off(self.cx) } else { String::new() };
        self.lines.insert(self.cy + 1, rest);
        self.cy += 1;
        self.cx = 0;
    }
    
    fn backspace(&mut self) {
        self.snapshot();
        if self.cx > 0 {
             let line = &mut self.lines[self.cy];
             if self.cx <= line.len() { line.remove(self.cx - 1); self.cx -= 1; }
        } else if self.cy > 0 {
             let curr = self.lines.remove(self.cy);
             self.cy -= 1;
             self.cx = self.lines[self.cy].len();
             self.lines[self.cy].push_str(&curr);
        }
    }
    
    fn delete_char(&mut self) {
        self.snapshot();
        if self.cy < self.lines.len() {
            let line = &mut self.lines[self.cy];
            if self.cx < line.len() { line.remove(self.cx); }
        }
    }

    fn save_file(&mut self) {
        let content = self.lines.join("\n");
        if let Err(e) = fs::write(&self.filename, content) {
            self.msg = format!("Error saving: {}", e);
        } else {
            self.msg = "File saved.".to_string();
            self.dirty = false;
        }
    }
    
    fn highlight_line(&self, line: &str) -> String {
        let keywords = [
            // Function declarations
            "func", "fun", "fn", "def",
            // Control flow
            "if", "else", "while", "for", "do", "loop", "return", "break", "continue",
            // Variables
            "var", "const", "mut", "let",
            // Types
            "int", "float", "byte", "short", "long", "char", "string",
            // OOP
            "class", "struct", "impl", "new",
            // Access modifiers
            "pub", "public", "private", "protected",
            // Safety & Concurrency
            "safe", "verify", "spawn", "lock", "unlock",
            // Memory
            "alloc", "free", "peek", "poke", "limit_mem",
            // Other
            "import", "use", "mod", "asm",
            // Booleans
            "true", "false", "not", "and", "or",
        ];
        let mut result = String::new();
        let mut chars = line.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '#' {
                result.push_str(COLOR_COMMENT);
                result.push(ch);
                result.push_str(&chars.collect::<String>());
                result.push_str(COLOR_RESET);
                break;
            }
            
            if ch == '"' {
                result.push_str(COLOR_STRING);
                result.push(ch);
                while let Some(c) = chars.next() {
                    result.push(c);
                    if c == '"' { break; }
                }
                result.push_str(COLOR_RESET);
                continue;
            }
            
            if ch.is_numeric() {
                result.push_str(COLOR_NUMBER);
                result.push(ch);
                while let Some(&c) = chars.peek() {
                    if c.is_numeric() || c == '.' {
                        result.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                result.push_str(COLOR_RESET);
                continue;
            }
            
            if ch.is_alphabetic() || ch == '_' {
                let mut word = String::new();
                word.push(ch);
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        word.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                
                if keywords.contains(&word.as_str()) {
                    result.push_str(COLOR_KEYWORD);
                    result.push_str(&word);
                    result.push_str(COLOR_RESET);
                } else {
                    result.push_str(&word);
                }
                continue;
            }
            
            if "+-*/=<>!&|".contains(ch) {
                result.push_str(COLOR_OPERATOR);
                result.push(ch);
                result.push_str(COLOR_RESET);
                continue;
            }
            
            result.push(ch);
        }
        
        result
    }
    
    fn get_indent_level(&self, line_idx: usize) -> usize {
        if line_idx >= self.lines.len() {
            return 0;
        }
        let line = &self.lines[line_idx];
        line.chars().take_while(|c| *c == ' ').count()
    }
    
    fn handle_command(&mut self) {
        // Read command
        print!("\x1b[H\x1b[999B\r\n:");
        io::stdout().flush().unwrap();
        
        let mut cmd = String::new();
        loop {
            let key = self.read_key();
            match key {
                Key::Enter => break,
                Key::Char(c) => {
                    cmd.push(c);
                    print!("{}", c);
                    io::stdout().flush().unwrap();
                }
                Key::Backspace => {
                    if !cmd.is_empty() {
                        cmd.pop();
                        print!("\x08 \x08");
                        io::stdout().flush().unwrap();
                    }
                }
                _ => {}
            }
        }
        
        // Execute command
        match cmd.trim() {
            "compile" | "c" => {
                self.save_file();
                self.msg = format!("Compiling {}...", self.filename);
                self.refresh_screen();
                
                let output = Command::new("nux")
                    .arg("build")
                    .arg(&self.filename)
                    .output();
                
                match output {
                    Ok(result) => {
                        if result.status.success() {
                            self.msg = "✅ Compilation successful!".to_string();
                        } else {
                            let err = String::from_utf8_lossy(&result.stderr);
                            self.msg = format!("❌ Compilation failed: {}", err.lines().next().unwrap_or("Unknown error"));
                        }
                    }
                    Err(e) => {
                        self.msg = format!("Error running compiler: {}", e);
                    }
                }
            }
            "run" | "r" => {
                self.save_file();
                self.msg = "Running...".to_string();
                self.refresh_screen();
                
                self.disable_raw_mode();
                let _ = Command::new("nux")
                    .arg("run")
                    .arg(&self.filename)
                    .status();
                self.enable_raw_mode();
                
                self.msg = "Program finished. Press any key...".to_string();
                self.refresh_screen();
                self.read_key();
            }
            "q" | "quit" => {
                self.quit = true;
            }
            "w" | "write" => {
                self.save_file();
            }
            "wq" => {
                self.save_file();
                self.quit = true;
            }
            _ => {
                self.msg = format!("Unknown command: {}", cmd);
            }
        }
    }
}
