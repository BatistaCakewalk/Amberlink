// amber-core/src/codegen/emitter.rs
use std::fs::File;
use std::io::{Write, BufWriter};
use super::bytecode::OpCode;
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
                self.emit_byte(OpCode::Push.into());
                self.emit_int(*val);
            }
            Expr::Variable(name) => {
                let index = symbols.variables.get(name)
                    .expect(&format!("Undefined variable: {}", name));
                self.emit_byte(OpCode::LoadGlobal.into());
                self.emit_int(*index as i32);
            }
            Expr::Binary(left, op, right) => {
                self.emit_expr(left, symbols);
                self.emit_expr(right, symbols);
                match op {
                    Op::Add => self.emit_byte(OpCode::Add.into()),
                    Op::Sub => self.emit_byte(OpCode::Sub.into()),
                    Op::Mul => self.emit_byte(OpCode::Mul.into()),
                    Op::Div => self.emit_byte(OpCode::Div.into()),
                }
            }
        }
    }

    // Emits a jump instruction with a placeholder offset. Returns the index of the placeholder.
    fn emit_jump(&mut self, instruction: u8) -> usize {
        self.emit_byte(instruction);
        self.emit_byte(0xFF); // Placeholder (4 bytes)
        self.emit_byte(0xFF);
        self.emit_byte(0xFF);
        self.emit_byte(0xFF);
        self.code.len() - 4
    }

    fn patch_jump(&mut self, offset_index: usize) {
        let jump_dist = (self.code.len() - offset_index - 4) as i32;
        let bytes = jump_dist.to_le_bytes();
        for i in 0..4 {
            self.code[offset_index + i] = bytes[i];
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

                self.emit_byte(OpCode::StoreGlobal.into());
                self.emit_int(index as i32);
            }
            Stmt::Block(stmts) => {
                for s in stmts {
                    self.emit_stmt(s, symbols);
                }
            }
            Stmt::If(cond, then_branch, else_branch) => {
                self.emit_expr(cond, symbols);
                
                // Jump to Else if false
                let then_jump = self.emit_jump(OpCode::JumpIfFalse.into());
                
                self.emit_stmt(then_branch, symbols);
                
                let else_jump = self.emit_jump(OpCode::Jump.into());
                
                self.patch_jump(then_jump);
                
                if let Some(else_stmt) = else_branch {
                    self.emit_stmt(else_stmt, symbols);
                }
                
                self.patch_jump(else_jump);
            }
            Stmt::While(cond, body) => {
                let loop_start = self.code.len();
                
                self.emit_expr(cond, symbols);
                let exit_jump = self.emit_jump(OpCode::JumpIfFalse.into());
                
                self.emit_stmt(body, symbols);
                self.emit_byte(OpCode::Jump.into());
                
                let offset = (loop_start as i32) - (self.code.len() as i32) - 4;
                self.emit_int(offset);
                
                self.patch_jump(exit_jump);
            }
            Stmt::Expression(expr) => {
                self.emit_expr(expr, symbols);
                // An expression used as a statement should have its result popped.
                self.emit_byte(OpCode::Pop.into());
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
