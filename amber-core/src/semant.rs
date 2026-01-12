// amber-core/src/semant.rs
use std::collections::HashMap;

pub struct SymbolTable {
    // Maps function names to their return types and parameters
    functions: HashMap<String, FunctionInfo>,
}

pub struct FunctionInfo {
    pub params: Vec<String>, // e.g., ["Int", "Int"]
    pub return_type: String, // e.g., "Int"
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { functions: HashMap::new() }
    }

    pub fn define_func(&mut self, name: String, info: FunctionInfo) {
        self.functions.insert(name, info);
    }
}
