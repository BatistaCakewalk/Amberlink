// amber-vm/src/interpreter.cpp
void AVM::Interpreter::run(const std::vector<uint8_t>& bytecode) {
    size_t ip = 0; // Instruction Pointer
    std::vector<int32_t> stack; // Our working memory

    while (ip < bytecode.size()) {
        uint8_t opcode = bytecode[ip++];

        switch (static_cast<OpCode>(opcode)) {
            case OpCode::OP_PUSH: {
                // Read next 4 bytes as an integer operand
                int32_t val;
                memcpy(&val, &bytecode[ip], 4);
                stack.push_back(val);
                ip += 4;
                break;
            }
            case OpCode::OP_ADD: {
                int32_t b = stack.back(); stack.pop_back();
                int32_t a = stack.back(); stack.pop_back();
                stack.push_back(a + b);
                break;
            }
            case OpCode::OP_PRINT: {
                std::cout << ">> " << stack.back() << std::endl;
                stack.pop_back();
                break;
            }
            case OpCode::OP_HALT:
                return;
        }
    }
}
