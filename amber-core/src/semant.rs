// amber-core/src/semant.rs
use std::collections::HashMap;

pub struct FunctionInfo {
    pub name: String,
    pub address: u32, // Where it exists in the bytecode
}

pub struct SymbolTable {
    pub functions: HashMap<String, FunctionInfo>,
    pub variables: HashMap<String, u32>, // Maps "x" -> 0 (Global Index)
    pub next_var_index: u32,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { 
            functions: HashMap::new(),
            variables: HashMap::new(),
            next_var_index: 0,
        }
    }
}
