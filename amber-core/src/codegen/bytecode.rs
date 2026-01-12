// amber-core/src/codegen/bytecode.rs

#[repr(u8)]
pub enum OpCode {
    // --- Control Flow ---
    Halt = 0x00,
    Jump = 0x01,
    JumpIfFalse = 0x02,

    // --- Constants & Variables ---
    Push = 0x10,
    StoreGlobal = 0x11,
    LoadGlobal = 0x12,
    StoreLocal = 0x13,
    LoadLocal = 0x14,

    // --- Arithmetic & Logic ---
    Add = 0x20,
    Sub = 0x21,
    Mul = 0x22,
    Div = 0x23,
    Less = 0x24,

    // --- Utilities ---
    Call = 0x30,
    Return = 0x31,
    Pop = 0x80,
    Print = 0x81,
}

impl From<OpCode> for u8 {
    fn from(op: OpCode) -> u8 { op as u8 }
}