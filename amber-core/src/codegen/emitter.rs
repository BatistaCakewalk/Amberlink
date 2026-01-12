// amber-core/src/codegen/emitter.rs
use std::fs::File;
use std::io::{Write, BufWriter};

pub struct Emitter {
    bytecode: Vec<u8>,
}

impl Emitter {
    pub fn new() -> Self {
        Self { bytecode: Vec::new() }
    }

    // Add an OpCode byte
    pub fn emit_byte(&mut self, byte: u8) {
        self.bytecode.push(byte);
    }

    // Add a 4-byte integer (Little Endian)
    pub fn emit_int(&mut self, value: i32) {
        self.bytecode.extend_from_slice(&value.to_le_bytes());
    }

    pub fn save_to_file(&self, filename: &str, entry_point: u32) -> std::io::Result<()> {
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);

        // 1. Magic Number: "AMBR"
        writer.write_all(b"AMBR")?;

        // 2. Metadata (Version 1, Entry Point)
        writer.write_all(&1u16.to_le_bytes())?;
        writer.write_all(&entry_point.to_le_bytes())?;

        // 3. Code Length and Bytecode
        writer.write_all(&(self.bytecode.len() as u32).to_le_bytes())?;
        writer.write_all(&self.bytecode)?;

        writer.flush()?;
        Ok(())
    }
}
