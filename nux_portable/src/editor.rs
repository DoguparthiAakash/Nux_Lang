use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::process::Command;

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
        
        for (i, line) in self.lines.iter().enumerate() {
            print!("{}\r\n", line); // TODO: Scroll offset support (omitted for brevity)
        }
        
        // Status Bar
        print!("\x1b[H\x1b[999B"); 
        
        let dirty_char = if self.dirty { "[+]" } else { "" };
        let mode_str = match self.mode { Mode::View => "VIEW", Mode::Insert => "INSERT" };

        print!("\n-- {} -- {} Pos: {},{}  {}\r\n", mode_str, dirty_char, self.cx, self.cy, self.msg);
        
        // Cursor
        print!("\x1b[{};{}H", self.cy + 1, self.cx + 1);
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
}
