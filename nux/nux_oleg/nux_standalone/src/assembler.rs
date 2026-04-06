use std::vec::Vec;
use std::string::String;
use std::collections::BTreeMap;
use std::format;

// Import OpCodes from vm.rs logic (hardcoded here for now or shared)
// We should ideally share them, but for modularity I'll define map here.

pub fn compile(source: &str) -> Result<Vec<u8>, String> {
    let mut ops = Vec::new();
    let mut labels = BTreeMap::new(); // Label Name -> ByteOffset
    let mut label_refs = Vec::new(); // (ByteOffsetToPatch, LabelName)

    // Pass 1: Parse and Generate Code (with placeholder for labels)
    // 64 Byte Header
    ops.extend_from_slice(b"ANUX");
    ops.extend_from_slice(&[0u8; 60]); // Padding
    
    // We parse line by line
    let mut lines = source.lines();
    while let Some(line) = lines.next() {
        let line = line.trim();
        // Remove comments
        let line = if let Some(idx) = line.find(';') {
            &line[..idx]
        } else {
            line
        }.trim();

        if line.is_empty() { continue; }

        // Check Label
        if line.ends_with(':') {
            let label_name = &line[..line.len()-1];
            labels.insert(String::from(label_name), ops.len());
            continue;
        }

        // Parse Instruction
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mnemonic = parts[0].to_ascii_uppercase();

        match mnemonic.as_str() {
            "PUSH" => {
                ops.push(0x01); // OP_PUSH
                if parts.len() < 2 { return Err(format!("PUSH missing operand")); }
                let val = parts[1].parse::<i64>().map_err(|_| "Invalid number")?;
                ops.extend_from_slice(&val.to_le_bytes());
            },
            "POP" => ops.push(0x02),
            "ADD" => ops.push(0x10),
            "SUB" => ops.push(0x11),
            "MUL" => ops.push(0x12),
            "DIV" => ops.push(0x13),
            "MOD" => ops.push(0x14),
            "POW" => ops.push(0x15),
            "FLOORDIV" => ops.push(0x16),
            "AND" => ops.push(0x18),
            "OR" => ops.push(0x19),
            "EQ" => ops.push(0x90),
            "NEQ" => ops.push(0x91),
            "LT" => ops.push(0x92),
            "GT" => ops.push(0x93),
            "LTE" => ops.push(0x94),
            "GTE" => ops.push(0x95),
            
            "XOR" => ops.push(0x22),
            "XAND" => ops.push(0x23),
            "XNOT" => ops.push(0x24),
            
            "DRAW_RECT" => ops.push(0x20),
            
            // Vision
            "OP_IMG_ALLOC" => ops.push(0x31),
            "OP_IMG_FREE" => ops.push(0x32),
            "OP_IMG_DRAW" => ops.push(0x33),
            "OP_CAM_CAPTURE" => ops.push(0x34),
            "OP_IMG_FILTER" => ops.push(0x35),
            "OP_IMG_GET" => ops.push(0x36),
            "OP_IMG_RESIZE" => ops.push(0x37),
            "OP_IMG_CROP" => ops.push(0x38),
            "OP_IMG_GRAYSCALE" => ops.push(0x39),
            
            "OP_IMG_GRAYSCALE" => ops.push(0x39),
            
            // File I/O
            "FILE_OPEN" => ops.push(0x55),
            "FILE_CLOSE" => ops.push(0x56),
            "FILE_READ" => ops.push(0x57),
            "FILE_WRITE" => ops.push(0x58),
            "FILE_EXISTS" => ops.push(0x59),
            "FILE_MKDIR" => ops.push(0x5A),
            "FILE_DELETE" => ops.push(0x5C),
            
            "VM_STACK_COPY" => ops.push(0x5B),
            
            "SYSTEM" => ops.push(0x81),

            "SLEEP" => ops.push(0x30),
            "PRINT_CHAR" => ops.push(0x51),
            "INPUT" => ops.push(0x52),
            "PRINT_VAL" => ops.push(0x53),
            "PRINT_FLOAT" => ops.push(0x54),
            "FADD" => ops.push(0x1A),
            "FSUB" => ops.push(0x1B),
            "FMUL" => ops.push(0x1C),
            "FDIV" => ops.push(0x1D),
            "ITOF" => ops.push(0x1E),
            "FTOI" => ops.push(0x1F),
            "FPOW" => ops.push(0x46),
            "FFLOORDIV" => ops.push(0x47),
            "PEEK" => ops.push(0x40),
            "POKE" => ops.push(0x41),
            "GFX_TEXT" => ops.push(0x3C),
            "GFX_RECT" => ops.push(0x3D),
            "PEEK8" => ops.push(0x42),
            "POKE8" => ops.push(0x43),
            "GET_LOCAL" | "OP_GET_LOCAL" => {
                ops.push(0x44);
                if parts.len() < 2 { return Err(format!("GET_LOCAL missing offset")); }
                let val = parts[1].parse::<i64>().map_err(|_| "Invalid number")?;
                ops.extend_from_slice(&val.to_le_bytes());
            },
            "SET_LOCAL" | "OP_SET_LOCAL" => {
                ops.push(0x45);
                if parts.len() < 2 { return Err(format!("SET_LOCAL missing offset")); }
                let val = parts[1].parse::<i64>().map_err(|_| "Invalid number")?;
                ops.extend_from_slice(&val.to_le_bytes());
            },
            "DEBUG" => ops.push(0x50), // DEBUG_PRINT
            "JMP" => {
                ops.push(0x60);
                if parts.len() < 2 { return Err(format!("JMP missing label")); }
                label_refs.push((ops.len(), String::from(parts[1])));
                ops.extend_from_slice(&[0u8; 8]); // Placeholder
            },
            "JE" => {
                ops.push(0x61);
                if parts.len() < 2 { return Err(format!("JE missing label")); }
                label_refs.push((ops.len(), String::from(parts[1])));
                ops.extend_from_slice(&[0u8; 8]);
            },
            "CALL" => {
                ops.push(0x70);
                if parts.len() < 2 { return Err(format!("CALL missing label")); }
                label_refs.push((ops.len(), String::from(parts[1])));
                ops.extend_from_slice(&[0u8; 8]);
                
                // Num Args argument
                let num_args = if parts.len() >= 3 {
                    parts[2].parse::<i64>().map_err(|_| "Invalid call args count")?
                } else {
                    0
                };
                ops.extend_from_slice(&num_args.to_le_bytes());
            },
            "SPAWN" => {
                 ops.push(0x72); // Note: kernel vm uses 0x72 for spawn? Check vm.rs.
                 // In vm.rs: const OP_SPAWN: u8 = 0x72;
                 // It expects a popped function pointer, not an argument?
                 // Wait, portable used 0x3A. Kernel uses 0x72.
                 // Lexer says `spawn func_name`.
                 // High level parser emits `PUSH func_name \n SPAWN`.
                 // So Compiler just sees SPAWN.
            },
            "LOCK" => ops.push(0x73),
            "UNLOCK" => ops.push(0x74),
            
            "TIME" => ops.push(0x75),
            "RANDOM" => ops.push(0x76),
            
            "RET" => ops.push(0x71),
            "OP_FILE_MKDIR" => ops.push(0x5A),
            "OP_FILE_DELETE" => ops.push(0x5C),
            "OP_DM_GET" => ops.push(0x64),
            "OP_DM_SET" => ops.push(0x65),
            "OP_SEC_LOGIN" => ops.push(0x66),
            "OP_SEC_WHOAMI" => ops.push(0x67),
            "OP_PUSH_STR" => ops.push(0x68),
            "EXIT" => ops.push(0xFF),
            "BYTE" => {
                // Emit a single byte (for string data)
                if parts.len() < 2 { return Err(format!("BYTE missing operand")); }
                let val = parts[1].parse::<u8>().map_err(|_| "Invalid byte value")?;
                ops.push(val);
            },
            _ => return Err(format!("Unknown instruction: {}", mnemonic)),
        }
    }

    // Pass 2: Patch Labels
    for (offset, label_name) in label_refs {
        if let Some(&target_addr) = labels.get(&label_name) {
             let bytes = (target_addr as i64).to_le_bytes();
             for i in 0..8 {
                 ops[offset + i] = bytes[i];
             }
        } else {
            // DEBUG
            if label_name.contains("while_end") {
                eprintln!("DEBUG: Labels map keys:");
                for k in labels.keys() {
                    if k.contains("while_end") {
                        eprintln!("  Found: '{}'", k);
                    }
                }
            }
            return Err(format!("Undefined label: '{}' (Len: {})", label_name, label_name.len()));
        }
    }

    Ok(ops)
}
