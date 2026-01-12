// amber-core/src/codegen/emitter.rs
use std::fs::File;
use std::io::{Write, BufWriter};
use crate::ast::{Expr, Op};
use crate::ast::Stmt;
use crate::semant::SymbolTable;

pub struct Emitter {
    pub code: Vec<u8>,
}

impl Emitter {
    pub fn new() -> Self { Self { code: Vec::new() } }

    pub fn emit_byte(&mut self, b: u8) { self.code.push(b); }
    pub fn emit_int(&mut self, val: i32) {
        self.code.extend_from_slice(&val.to_le_bytes());
    }

    pub fn emit_expr(&mut self, expr: &Expr, symbols: &mut SymbolTable) {
        match expr {
            Expr::Integer(val) => {
                self.emit_byte(0x01); // OP_PUSH
                self.emit_int(*val);
            }
            Expr::Variable(name) => {
                let index = symbols.variables.get(name)
                    .expect(&format!("Undefined variable: {}", name));
                self.emit_byte(0x08); // OP_LOAD
                self.emit_int(*index as i32);
            }
            Expr::Binary(left, op, right) => {
                self.emit_expr(left, symbols);
                self.emit_expr(right, symbols);
                match op {
                    Op::Add => self.emit_byte(0x02),
                    Op::Sub => self.emit_byte(0x03),
                    Op::Mul => self.emit_byte(0x04),
                    Op::Div => self.emit_byte(0x05),
                }
            }
        }
    }

    pub fn emit_stmt(&mut self, stmt: &Stmt, symbols: &mut SymbolTable) {
        match stmt {
            Stmt::VarDecl(name, expr) => {
                self.emit_expr(expr, symbols); // Push value
                
                // Assign index
                let index = symbols.next_var_index;
                symbols.variables.insert(name.clone(), index);
                symbols.next_var_index += 1;

                self.emit_byte(0x07); // OP_STORE
                self.emit_int(index as i32);
            }
            Stmt::Expression(expr) => {
                self.emit_expr(expr, symbols);
                // Note: In a full language, we might pop here to keep stack clean
            }
            _ => {} // Skip functions for now
        }
    }

    pub fn write_file(&self, path: &str) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(b"AMBR")?; // Magic
        writer.write_all(&1u16.to_le_bytes())?; // Version
        writer.write_all(&0u32.to_le_bytes())?; // Entry point placeholder
        writer.write_all(&(self.code.len() as u32).to_le_bytes())?;
        writer.write_all(&self.code)?;
        Ok(())
    }
}
