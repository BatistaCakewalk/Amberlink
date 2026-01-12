// amber-core/src/semant.rs
use std::collections::HashMap;

pub struct FunctionInfo {
    pub name: String,
    pub address: u32, // Where it exists in the bytecode
}

pub struct SymbolTable {
    pub functions: HashMap<String, FunctionInfo>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { functions: HashMap::new() }
    }
}
