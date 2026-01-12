// amber-vm/src/interpreter.cpp
#include "avm.hpp"
#include "bytecode.hpp"
#include <vector>
#include <iostream>

void run_vm(const std::vector<Amber::Instruction>& program) {
    std::vector<int32_t> stack;
    
    for (const auto& inst : program) {
        switch (inst.code) {
            case Amber::OpCode::OP_PUSH:
                stack.push_back(inst.operand);
                break;
            case Amber::OpCode::OP_ADD: {
                int32_t b = stack.back(); stack.pop_back();
                int32_t a = stack.back(); stack.pop_back();
                stack.push_back(a + b);
                break;
            }
            case Amber::OpCode::OP_PRINT:
                std::cout << stack.back() << std::endl;
                break;
            case Amber::OpCode::OP_HALT:
                return;
        }
    }
}
