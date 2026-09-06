// Integrated Debugger - GDB-like debugger for Nux
// Provides breakpoints, step execution, variable inspection, and stack traces

use std::collections::HashMap;
use crate::nvm::vm::NuxVM;
use crate::nvm::bytecode::{BytecodeChunk, Value};

/// Integrated debugger for Nux
pub struct NuxDebugger {
    vm: NuxVM,
    breakpoints: HashMap<usize, Breakpoint>,
    watchpoints: HashMap<String, Watchpoint>,
    call_stack: Vec<StackFrame>,
    current_line: usize,
    state: DebuggerState,
}

/// Debugger state
#[derive(Debug, Clone, PartialEq)]
pub enum DebuggerState {
    Running,
    Paused,
    Stopped,
    SteppingOver,
    SteppingInto,
    SteppingOut,
}

/// Breakpoint
#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub id: usize,
    pub location: BreakpointLocation,
    pub condition: Option<String>,
    pub hit_count: usize,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub enum BreakpointLocation {
    Line(usize),
    Function(String),
    Address(usize),
}

/// Watchpoint (data breakpoint)
#[derive(Debug, Clone)]
pub struct Watchpoint {
    pub variable: String,
    pub old_value: Option<Value>,
    pub watch_type: WatchType,
}

#[derive(Debug, Clone)]
pub enum WatchType {
    Read,
    Write,
    ReadWrite,
}

/// Stack frame
#[derive(Debug, Clone)]
pub struct StackFrame {
    pub function: String,
    pub line: usize,
    pub locals: HashMap<String, Value>,
}

impl NuxDebugger {
    pub fn new(vm: NuxVM) -> Self {
        NuxDebugger {
            vm,
            breakpoints: HashMap::new(),
            watchpoints: HashMap::new(),
            call_stack: Vec::new(),
            current_line: 0,
            state: DebuggerState::Stopped,
        }
    }

    /// Start debugging session
    pub fn start(&mut self) -> Result<(), DebuggerError> {
        println!("[DEBUGGER] Starting debugging session");
        self.state = DebuggerState::Paused;
        self.print_current_line();
        Ok(())
    }

    /// Set breakpoint
    pub fn set_breakpoint(&mut self, location: BreakpointLocation) -> usize {
        let id = self.breakpoints.len();
        let breakpoint = Breakpoint {
            id,
            location: location.clone(),
            condition: None,
            hit_count: 0,
            enabled: true,
        };

        self.breakpoints.insert(id, breakpoint);
        
        match location {
            BreakpointLocation::Line(line) => {
                println!("[DEBUGGER] Breakpoint {} set at line {}", id, line);
            }
            BreakpointLocation::Function(ref func) => {
                println!("[DEBUGGER] Breakpoint {} set at function {}", id, func);
            }
            BreakpointLocation::Address(addr) => {
                println!("[DEBUGGER] Breakpoint {} set at address 0x{:x}", id, addr);
            }
        }

        id
    }

    /// Remove breakpoint
    pub fn remove_breakpoint(&mut self, id: usize) -> Result<(), DebuggerError> {
        if self.breakpoints.remove(&id).is_some() {
            println!("[DEBUGGER] Breakpoint {} removed", id);
            Ok(())
        } else {
            Err(DebuggerError::BreakpointNotFound(id))
        }
    }

    /// List all breakpoints
    pub fn list_breakpoints(&self) {
        println!("[DEBUGGER] Breakpoints:");
        for (id, bp) in &self.breakpoints {
            let status = if bp.enabled { "enabled" } else { "disabled" };
            match &bp.location {
                BreakpointLocation::Line(line) => {
                    println!("  {} at line {} ({}, hit {} times)", id, line, status, bp.hit_count);
                }
                BreakpointLocation::Function(func) => {
                    println!("  {} at function {} ({}, hit {} times)", id, func, status, bp.hit_count);
                }
                BreakpointLocation::Address(addr) => {
                    println!("  {} at 0x{:x} ({}, hit {} times)", id, addr, status, bp.hit_count);
                }
            }
        }
    }

    /// Set watchpoint
    pub fn set_watchpoint(&mut self, variable: String, watch_type: WatchType) {
        let watchpoint = Watchpoint {
            variable: variable.clone(),
            old_value: None,
            watch_type,
        };

        self.watchpoints.insert(variable.clone(), watchpoint);
        println!("[DEBUGGER] Watchpoint set on variable: {}", variable);
    }

    /// Continue execution
    pub fn continue_execution(&mut self) -> Result<(), DebuggerError> {
        println!("[DEBUGGER] Continuing execution");
        self.state = DebuggerState::Running;

        // Run until breakpoint or completion
        while self.state == DebuggerState::Running {
            if self.check_breakpoint() {
                self.state = DebuggerState::Paused;
                println!("[DEBUGGER] Breakpoint hit");
                self.print_current_line();
                break;
            }

            // Execute next instruction (simplified)
            self.current_line += 1;
        }

        Ok(())
    }

    /// Step over (execute current line, don't enter functions)
    pub fn step_over(&mut self) -> Result<(), DebuggerError> {
        println!("[DEBUGGER] Stepping over");
        self.state = DebuggerState::SteppingOver;
        self.current_line += 1;
        self.state = DebuggerState::Paused;
        self.print_current_line();
        Ok(())
    }

    /// Step into (enter function calls)
    pub fn step_into(&mut self) -> Result<(), DebuggerError> {
        println!("[DEBUGGER] Stepping into");
        self.state = DebuggerState::SteppingInto;
        self.current_line += 1;
        self.state = DebuggerState::Paused;
        self.print_current_line();
        Ok(())
    }

    /// Step out (execute until current function returns)
    pub fn step_out(&mut self) -> Result<(), DebuggerError> {
        println!("[DEBUGGER] Stepping out");
        self.state = DebuggerState::SteppingOut;
        
        let initial_depth = self.call_stack.len();
        while self.call_stack.len() >= initial_depth {
            self.current_line += 1;
        }

        self.state = DebuggerState::Paused;
        self.print_current_line();
        Ok(())
    }

    /// Print backtrace (call stack)
    pub fn backtrace(&self) {
        println!("[DEBUGGER] Call stack:");
        for (i, frame) in self.call_stack.iter().enumerate().rev() {
            println!("  #{} {} at line {}", i, frame.function, frame.line);
        }
    }

    /// Inspect variable
    pub fn inspect_variable(&self, name: &str) -> Result<Value, DebuggerError> {
        // Check current frame
        if let Some(frame) = self.call_stack.last() {
            if let Some(value) = frame.locals.get(name) {
                println!("[DEBUGGER] {} = {:?}", name, value);
                return Ok(value.clone());
            }
        }

        Err(DebuggerError::VariableNotFound(name.to_string()))
    }

    /// List local variables
    pub fn list_locals(&self) {
        println!("[DEBUGGER] Local variables:");
        if let Some(frame) = self.call_stack.last() {
            for (name, value) in &frame.locals {
                println!("  {} = {:?}", name, value);
            }
        } else {
            println!("  (no local variables)");
        }
    }

    /// Evaluate expression
    pub fn evaluate(&self, expression: &str) -> Result<Value, DebuggerError> {
        // Simplified expression evaluation
        println!("[DEBUGGER] Evaluating: {}", expression);
        
        // Try to parse as variable name
        if let Ok(value) = self.inspect_variable(expression) {
            return Ok(value);
        }

        // Try to parse as literal
        if let Ok(num) = expression.parse::<i64>() {
            return Ok(Value::Int(num));
        }

        Err(DebuggerError::EvaluationFailed(expression.to_string()))
    }

    /// Print current source line
    fn print_current_line(&self) {
        println!("[DEBUGGER] Current line: {}", self.current_line);
        println!("  => <source code would be displayed here>");
    }

    /// Check if breakpoint is hit
    fn check_breakpoint(&mut self) -> bool {
        for (_, bp) in self.breakpoints.iter_mut() {
            if !bp.enabled {
                continue;
            }

            match &bp.location {
                BreakpointLocation::Line(line) => {
                    if self.current_line == *line {
                        bp.hit_count += 1;
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    /// Get debugger state
    pub fn get_state(&self) -> &DebuggerState {
        &self.state
    }
}

/// Debugger errors
#[derive(Debug)]
pub enum DebuggerError {
    BreakpointNotFound(usize),
    VariableNotFound(String),
    EvaluationFailed(String),
    InvalidCommand(String),
}

impl std::fmt::Display for DebuggerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DebuggerError::BreakpointNotFound(id) => write!(f, "Breakpoint not found: {}", id),
            DebuggerError::VariableNotFound(name) => write!(f, "Variable not found: {}", name),
            DebuggerError::EvaluationFailed(expr) => write!(f, "Evaluation failed: {}", expr),
            DebuggerError::InvalidCommand(cmd) => write!(f, "Invalid command: {}", cmd),
        }
    }
}

impl std::error::Error for DebuggerError {}

/// Debug command-line interface
pub struct DebuggerCLI {
    debugger: NuxDebugger,
}

impl DebuggerCLI {
    pub fn new(vm: NuxVM) -> Self {
        DebuggerCLI {
            debugger: NuxDebugger::new(vm),
        }
    }

    /// Run interactive debugger
    pub fn run(&mut self) -> Result<(), DebuggerError> {
        println!("Nux Debugger v1.0");
        println!("Type 'help' for available commands");

        self.debugger.start()?;

        // In production, this would have an interactive REPL
        // For now, just demonstrate commands
        self.print_help();

        Ok(())
    }

    fn print_help(&self) {
        println!("\nAvailable commands:");
        println!("  break <line>     - Set breakpoint at line");
        println!("  continue         - Continue execution");
        println!("  step             - Step over");
        println!("  stepi            - Step into");
        println!("  finish           - Step out");
        println!("  backtrace        - Print call stack");
        println!("  print <var>      - Print variable value");
        println!("  locals           - List local variables");
        println!("  eval <expr>      - Evaluate expression");
        println!("  breakpoints      - List breakpoints");
        println!("  help             - Show this help");
        println!("  quit             - Exit debugger");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debugger_creation() {
        let vm = NuxVM::new();
        let debugger = NuxDebugger::new(vm);
        assert_eq!(debugger.state, DebuggerState::Stopped);
    }

    #[test]
    fn test_breakpoint_setting() {
        let vm = NuxVM::new();
        let mut debugger = NuxDebugger::new(vm);
        
        let id = debugger.set_breakpoint(BreakpointLocation::Line(10));
        assert_eq!(id, 0);
        assert_eq!(debugger.breakpoints.len(), 1);
    }

    #[test]
    fn test_breakpoint_removal() {
        let vm = NuxVM::new();
        let mut debugger = NuxDebugger::new(vm);
        
        let id = debugger.set_breakpoint(BreakpointLocation::Line(10));
        let result = debugger.remove_breakpoint(id);
        assert!(result.is_ok());
        assert_eq!(debugger.breakpoints.len(), 0);
    }
}
