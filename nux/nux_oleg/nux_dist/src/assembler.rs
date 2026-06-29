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
            "OP_ADD" => ops.push(0x10), // Alias
            "SWAP" => ops.push(0x17),
            "DUP" => ops.push(0x27),
            "AND" => ops.push(0x18),
            "OR" => ops.push(0x19),
            "NOT" => ops.push(0x1A), // Logical NOT
            "SHL" => ops.push(0x25),
            "SHR" => ops.push(0x26),
            "EQ" => ops.push(0x90),
            "NEQ" => ops.push(0x91),
            "LT" => ops.push(0x92),
            "GT" => ops.push(0x93),
            "LTE" => ops.push(0x94),
            "GTE" => ops.push(0x95),
            
            "XOR" => ops.push(0x22),
            "XAND" => ops.push(0x23),
            "XNOT" => ops.push(0x24),
            "OP_MATCH" => ops.push(0x3B),
            
            "OP_FFI_PYTHON" => ops.push(0xE7),
            "OP_FFI_C" => ops.push(0xE8),
            "OP_Q_ALLOC" => ops.push(0xEA),
            "OP_Q_H" => ops.push(0xEB),
            "OP_Q_X" => ops.push(0xEC),
            "OP_Q_Z" => ops.push(0xED),
            "OP_Q_CX" => ops.push(0xEE),
            "OP_Q_MEASURE" => ops.push(0xEF),
            
            "DRAW_RECT" => ops.push(0x20),
            
            // Vision
            "OP_NET_LISTEN" => ops.push(0xB0),
            "OP_NET_ACCEPT" => ops.push(0xB1),
            "OP_NET_READ" => ops.push(0xB2),
            "OP_NET_WRITE" => ops.push(0xB3),
            "OP_NET_CLOSE" => ops.push(0xB4),
            "OP_NET_LISTEN_TLS" => ops.push(0xB5),
            "OP_FS_READ" => ops.push(0xC0),
            "OP_FS_WRITE" => ops.push(0xC1),
            "OP_FS_EXISTS" => ops.push(0xC2),
            "OP_OS_ENV" => ops.push(0xC5),
            "OP_OS_CWD" => ops.push(0xC6),
            "OP_OS_EXEC" => ops.push(0xC7),
            "OP_TIME_NOW" => ops.push(0xCA),
            "OP_TIME_SLEEP" => ops.push(0xCB),
            "OP_IMG_ALLOC" => ops.push(0x31),
            "OP_IMG_FREE" => ops.push(0x32),
            "OP_IMG_DRAW" => ops.push(0x33),
            "OP_CAM_CAPTURE" => ops.push(0x34),
            "OP_IMG_FILTER" => ops.push(0x35),
            "OP_IMG_GET" => ops.push(0x36),
            "OP_IMG_RESIZE" => ops.push(0x37),
            "OP_IMG_CROP" => ops.push(0x38),
            "OP_IMG_GRAYSCALE" => ops.push(0x39),
            // Video Buffer Extension
            "OP_VBE_SET_MODE" => ops.push(0x3A),
            "OP_VBE_GET_FB" => ops.push(0x3B),
            "OP_VBE_UPDATE" => ops.push(0x3C),
            "OP_VBE_GET_KEY" => ops.push(0x3D),
            "OP_VBE_GET_MOUSE_X" => ops.push(0x3E),
            "OP_VBE_GET_MOUSE_Y" => ops.push(0x3F),
            "OP_VBE_GET_MOUSE_DOWN" => ops.push(0x48),
            
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
            "OP_SLEEP" => ops.push(0x30),
            "PRINT_CHAR" => ops.push(0x51),
            "INPUT" => ops.push(0x52),
            "PRINT_VAL" => ops.push(0x53),
            "PRINT_FLOAT" => ops.push(0x54),
            "PRINT_STR" => ops.push(0x6B),
            "FADD" => ops.push(0xD0),
            "FSUB" => ops.push(0xD1),
            "FMUL" => ops.push(0xD2),
            "FDIV" => ops.push(0xD3),
            "FPOW" => ops.push(0xD4),
            "FSIN" => ops.push(0xD5),
            "FCOS" => ops.push(0xD6),
            "FTAN" => ops.push(0xD7),
            "FSQRT" => ops.push(0xD8),
            "FEQ" => ops.push(0xD9),
            "FNEQ" => ops.push(0xDA),
            "FLT" => ops.push(0xDB),
            "FGT" => ops.push(0xDC),
            "FLTE" => ops.push(0xDD),
            "FGTE" => ops.push(0xDE),
            "ITOF" => ops.push(0x1E),
            "FTOI" => ops.push(0x1F),
            "FFLOORDIV" => ops.push(0x47),
            "PEEK" => ops.push(0x40),
            "POKE" => ops.push(0x41),
            "GFX_TEXT" => ops.push(0x3C),
            "GFX_RECT" => ops.push(0x3D),
            "PEEK8" => ops.push(0x42),
            "POKE8" => ops.push(0x43),
            "OP_PEEK32" => ops.push(0x4C),
            "OP_POKE32" => ops.push(0x49),
            "OP_FSIN" => ops.push(0xD5),
            "OP_FCOS" => ops.push(0xD6),
            "OP_FSQRT" => ops.push(0xD8),
            "OP_FTAN" => ops.push(0xD7),
            "OP_FPOW" => ops.push(0xD4),
            "OP_TO_UPPER" => ops.push(0x69),
            "OP_TO_LOWER" => ops.push(0x6A),
            "OP_STR_LEN" => ops.push(0x6C),
            "OP_STR_CHAR" => ops.push(0x6D),
            "OP_STR_SUB" => ops.push(0x6E),
            
            "OP_TENSOR_NEW" => ops.push(0x82),
            "OP_TENSOR_FREE" => ops.push(0x83),
            "OP_TENSOR_SET" => ops.push(0x84),
            "OP_TENSOR_GET" => ops.push(0x85),
            "OP_TENSOR_COPY" => ops.push(0x86),
            "OP_TENSOR_ADD" => ops.push(0x87),
            "OP_TENSOR_MATMUL" => ops.push(0x88),
            "OP_TENSOR_RELU" => ops.push(0x89),
            "OP_TENSOR_SOFTMAX" => ops.push(0x8A),
            "OP_TENSOR_RMSNORM" => ops.push(0x8B),
            "OP_TENSOR_SCALE" => ops.push(0x8C),
            "OP_TENSOR_EMBEDDING" => ops.push(0x8D),
            
            "OP_GET_LOCAL" => {
                ops.push(0x44);
                if parts.len() < 2 { return Err(format!("OP_GET_LOCAL missing offset")); }
                let val = parts[1].parse::<i64>().map_err(|_| "Invalid number")?;
                ops.extend_from_slice(&val.to_le_bytes());
            },
            "SET_LOCAL" => {
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
            "OP_JOIN" => ops.push(0xE1),
            "SPAWN" => {
                ops.push(0xE0);
                if parts.len() < 2 { return Err(format!("SPAWN missing label")); }
                label_refs.push((ops.len(), String::from(parts[1])));
                ops.extend_from_slice(&[0u8; 8]);
                
                // Num Args argument
                let num_args = if parts.len() >= 3 {
                    parts[2].parse::<i64>().map_err(|_| "Invalid spawn args count")?
                } else {
                    0
                };
                ops.extend_from_slice(&num_args.to_le_bytes());
            },
            "OP_JOIN" => ops.push(0xE1),
            "OP_HW_READ" => ops.push(0xF0),
            "OP_HW_WRITE" => ops.push(0xF1),
            "LOCK" => ops.push(0x73),
            "UNLOCK" => ops.push(0x74),
            
            "RET" => ops.push(0x71),
            "OP_FILE_MKDIR" => ops.push(0x5A),
            "OP_FILE_DELETE" => ops.push(0x5C),
            "OP_DM_GET" => ops.push(0x64),
            "OP_DM_SET" => ops.push(0x65),
            "OP_SEC_LOGIN" => ops.push(0x66),
            "OP_SEC_WHOAMI" => ops.push(0x67),
            "OP_PUSH_STR" => ops.push(0x68),
            
            "OP_PEEK_PTR" => ops.push(0x42),
            "OP_POKE_PTR" => ops.push(0x43),
            "OP_SYSCALL" => ops.push(0x47),
            "OP_UNSAFE_START" => ops.push(0x4A),
            "OP_UNSAFE_END" => ops.push(0x4B),
            
            "OP_ARRAY_ALLOC" => ops.push(0xA0),
            "OP_ARRAY_GET" => ops.push(0xA1),
            "OP_ARRAY_SET" => ops.push(0xA2),
            "OP_ARRAY_NEW" => ops.push(0xA6),
            "OP_ARRAY_LEN" => ops.push(0xA7),
            "OP_ARRAY_FILL" => ops.push(0xA8),
            "OP_THROW" => ops.push(0xA3),
            "OP_CATCH" => {
                ops.push(0xA4);
                if parts.len() < 2 { return Err(format!("OP_CATCH missing label")); }
                label_refs.push((ops.len(), String::from(parts[1])));
                ops.extend_from_slice(&[0u8; 8]);
            },
            "OP_END_TRY" => ops.push(0xA5),
            
            "OP_FFI_LOAD" => ops.push(0x96),
            "OP_FFI_CALL" => ops.push(0x97),
            "OP_CUX_LOAD" => ops.push(0x98),
            "OP_CUX_CALL" => ops.push(0x99),

            // VBE / Graphics handled above
            
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
            return Err(format!("Undefined label: {}", label_name));
        }
    }

    Ok(ops)
}
