/// Embedded VM Opcodes
/// Core opcodes (0x01-0x5F) match desktop VM
/// Embedded-specific opcodes start at 0xE0

// Stack Operations
pub const OP_PUSH: u8 = 0x01;
pub const OP_POP: u8 = 0x02;
pub const OP_SWAP: u8 = 0x03;
pub const OP_DUP: u8 = 0x04;

// Arithmetic
pub const OP_ADD: u8 = 0x10;
pub const OP_SUB: u8 = 0x11;
pub const OP_MUL: u8 = 0x12;
pub const OP_DIV: u8 = 0x13;
pub const OP_MOD: u8 = 0x14;

// Comparison
pub const OP_EQ: u8 = 0x90;
pub const OP_NEQ: u8 = 0x91;
pub const OP_LT: u8 = 0x92;
pub const OP_GT: u8 = 0x93;
pub const OP_LTE: u8 = 0x94;
pub const OP_GTE: u8 = 0x95;

// Control Flow
pub const OP_JMP: u8 = 0x60;
pub const OP_JE: u8 = 0x61;
pub const OP_CALL: u8 = 0x70;
pub const OP_RET: u8 = 0x71;

// I/O
pub const OP_PRINT_VAL: u8 = 0x53;
pub const OP_PRINT_CHAR: u8 = 0x51;

// Locals
pub const OP_GET_LOCAL: u8 = 0x44;
pub const OP_SET_LOCAL: u8 = 0x45;

// Memory
pub const OP_PEEK: u8 = 0x40;
pub const OP_POKE: u8 = 0x41;

// Embedded-Specific Opcodes (0xE0-0xFF)
pub const OP_GPIO_WRITE: u8 = 0xE0;
pub const OP_GPIO_READ: u8 = 0xE1;
pub const OP_GPIO_MODE: u8 = 0xE2;
pub const OP_ANALOG_READ: u8 = 0xE3;
pub const OP_PWM_WRITE: u8 = 0xE4;
pub const OP_I2C_WRITE: u8 = 0xE5;
pub const OP_I2C_READ: u8 = 0xE6;
pub const OP_SPI_TRANSFER: u8 = 0xE7;
pub const OP_UART_WRITE: u8 = 0xE8;
pub const OP_UART_READ: u8 = 0xE9;
pub const OP_DELAY_MS: u8 = 0xEA;
pub const OP_DELAY_US: u8 = 0xEB;
pub const OP_MILLIS: u8 = 0xEC;
pub const OP_MICROS: u8 = 0xED;

pub const OP_EXIT: u8 = 0xFF;
