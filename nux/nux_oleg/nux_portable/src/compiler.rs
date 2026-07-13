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
                if parts.len() < 2 { return Err(format!("PUSH missing operand in line {:?}", line)); }
                let val = parts[1].parse::<i64>().map_err(|_| format!("Invalid number for PUSH: '{}' in line {:?}", parts[1], line))?;
                
                if val >= 0 && val <= 15 {
                    ops.push(0xA0 + val as u8); // 1-byte
                } else if val >= -128 && val <= 127 {
                    ops.push(0xB0); // 2-byte
                    ops.push(val as i8 as u8);
                } else if val >= -32768 && val <= 32767 {
                    ops.push(0xB1); // 3-byte
                    ops.extend_from_slice(&(val as i16).to_le_bytes());
                } else if val >= -2147483648 && val <= 2147483647 {
                    ops.push(0xB2); // 5-byte
                    ops.extend_from_slice(&(val as i32).to_le_bytes());
                } else {
                    ops.push(0x01); // OP_PUSH (9-byte)
                    ops.extend_from_slice(&val.to_le_bytes());
                }
            },
            "POP" => ops.push(0x02),
            "SWAP" => ops.push(0x03),
            "ADD" | "OP_ADD" => ops.push(0x10),
            "SUB" | "OP_SUB" => ops.push(0x11),
            "MUL" | "OP_MUL" => ops.push(0x12),
            "DIV" | "OP_DIV" => ops.push(0x13),
            "MOD" | "OP_MOD" => ops.push(0x14),
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
            "OP_DRAW_RECT" => ops.push(0x20),
            "OP_SLEEP" => ops.push(0x30),
            
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
            "OP_IMG_SET" => ops.push(0x3A),
            "OP_IMG_FILL" => ops.push(0x3B),
            
            "OP_TO_UPPER" => ops.push(0x55),
            "OP_TO_LOWER" => ops.push(0x56),
            
            "OP_SYS_PLATFORM" => ops.push(0x58),
            "OP_CAM_COUNT" => ops.push(0x59),
            "OP_IS_KEY_DOWN" => ops.push(0x5A),
            
            "OP_VM_STACK_COPY" | "VM_STACK_COPY" => {
                 ops.push(0x5B);
            },
            "OP_TIME" | "TIME" => ops.push(0x5C),
            "OP_SYSTEM" | "SYSTEM" => ops.push(0x5D),
            "OP_FILE_DELETE" | "FILE_DELETE" => ops.push(0x5E),
            
            // Graphics Opcodes
            "OP_GFX_CLEAR" => ops.push(0x96),
            "OP_DRAW_PIXEL" => ops.push(0x95),
            "OP_DRAW_LINE" => ops.push(0x93),
            "OP_DRAW_CIRCLE" => ops.push(0x94),
            
            "OP_VERIFY" => ops.push(0x81),
            
            "OP_VISION_DETECT" => ops.push(0xB0),
            
            // Embedded Opcodes
            "OP_GPIO_WRITE" => ops.push(0xE0),
            "OP_GPIO_READ" => ops.push(0xE1),
            "OP_GPIO_MODE" => ops.push(0xE2),
            "OP_ANALOG_READ" => ops.push(0xE3),
            "OP_PWM_WRITE" => ops.push(0xE4),
            "OP_I2C_WRITE" => ops.push(0xE5),
            "OP_I2C_READ" => ops.push(0xE6),
            "OP_SPI_TRANSFER" => ops.push(0xE7),
            "OP_UART_WRITE" => ops.push(0xE8),
            "OP_UART_READ" => ops.push(0xE9),
            "OP_DELAY_MS" => ops.push(0xEA),
            "OP_DELAY_US" => ops.push(0xEB),
            "OP_MILLIS" => ops.push(0xEC),
            "OP_MICROS" => ops.push(0xED),
            
            "OP_CHECK_RANGE" => {
                ops.push(0x57);
                // Consume 2 args
                let min_line = lines.next().ok_or("OP_CHECK_RANGE missing min")?.trim();
                let max_line = lines.next().ok_or("OP_CHECK_RANGE missing max")?.trim();
                let min = min_line.parse::<i64>().map_err(|_| "Invalid min for check range")?;
                let max = max_line.parse::<i64>().map_err(|_| "Invalid max for check range")?;
                ops.extend_from_slice(&min.to_le_bytes());
                ops.extend_from_slice(&max.to_le_bytes());
            },
            
            "GET_GLOBAL" => {
                 // Pseudo-Op: PUSH addr; PEEK
                 if parts.len() < 2 { return Err(format!("GET_GLOBAL missing address")); }
                 let addr = parts[1].parse::<i64>().map_err(|_| "Invalid address")?;
                 
                 // PUSH addr
                 ops.push(0x01); // OP_PUSH
                 ops.extend_from_slice(&addr.to_le_bytes());
                 
                 // PEEK
                 ops.push(0x40); // OP_PEEK
            },
            "SET_GLOBAL" => {
                 // Pseudo-Op: PUSH addr; SWAP; POKE
                 if parts.len() < 2 { return Err(format!("SET_GLOBAL missing address")); }
                 let addr = parts[1].parse::<i64>().map_err(|_| "Invalid address")?;
                 
                 // PUSH addr
                 ops.push(0x01);
                 ops.extend_from_slice(&addr.to_le_bytes());
                 
                 // SWAP
                 ops.push(0x03);
                 
                 // POKE
                 ops.push(0x41); 
            },

            "OP_GET_LOCAL" | "GET_LOCAL" => {
                 ops.push(0x44);
                 if parts.len() < 2 { return Err(format!("GET_LOCAL missing offset")); }
                 let val = parts[1].parse::<i64>().map_err(|_| "Invalid number")?;
                 ops.extend_from_slice(&val.to_le_bytes());
            },
            "OP_SET_LOCAL" | "SET_LOCAL" => {
                 ops.push(0x45);
                 if parts.len() < 2 { return Err(format!("SET_LOCAL missing offset")); }
                 let val = parts[1].parse::<i64>().map_err(|_| "Invalid number")?;
                 ops.extend_from_slice(&val.to_le_bytes());
            },
            
            "PEEK" => ops.push(0x40),
            "POKE" => ops.push(0x41),
            
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
            "OP_FSIN" => ops.push(0x48),
            "OP_FCOS" => ops.push(0x49),
            "OP_FSQRT" => ops.push(0x4A),
            "PEEK8" => ops.push(0x42),
            "POKE8" => ops.push(0x43),
            "OP_ALLOC" => ops.push(0x82),
            "OP_FREE" => ops.push(0x83),
            "OP_LIMIT_MEM" => ops.push(0x84),

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
            "RET" => ops.push(0x71),
            "EXIT" => ops.push(0xFF),
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

    // Pass 3: Checksum and Encryption
    if ops.len() > 64 {
        let checksum = adler32(&ops[64..]);
        let checksum_bytes = checksum.to_le_bytes();
        for i in 0..4 {
            ops[4 + i] = checksum_bytes[i];
        }
        
        let key = b"NUX_SECURE_KEY_123";
        xor_cipher(&mut ops[64..], key);
    }

    Ok(ops)
}

pub fn adler32(data: &[u8]) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for &byte in data {
        a = (a + byte as u32) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}

pub fn xor_cipher(data: &mut [u8], key: &[u8]) {
    for (i, byte) in data.iter_mut().enumerate() {
        *byte ^= key[i % key.len()];
    }
}
