mod lexer;
mod parser;
mod semant;
mod codegen;

use lexer::Lexer;
use parser::Parser;
use semant::SymbolTable;
use codegen::emitter::Emitter;

fn main() {
    let source = "func main() { val x = 10 }".to_string();
    
    // 1. Tokenize
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    // 2. Parse & Semantic Analysis
    let mut symbols = SymbolTable::new();
    let mut parser = Parser::new(tokens);
    parser.parse(&mut symbols);

    // 3. Emit (Simple test: push 10, push 20, add, halt)
    let mut emitter = Emitter::new();
    emitter.emit_byte(0x01); // OP_PUSH
    emitter.emit_int(10);
    emitter.emit_byte(0x01); // OP_PUSH
    emitter.emit_int(20);
    emitter.emit_byte(0x02); // OP_ADD
    emitter.emit_byte(0xFF); // OP_HALT

    emitter.write_file("output.amc").expect("Failed to write file");
    println!("Amberlink: Compiled to output.amc");
}
