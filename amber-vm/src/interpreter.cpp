// amber-vm/src/interpreter.cpp
#include "bytecode.hpp"
#include <vector>
#include <iostream>
#include <cstring>

void execute(const std::vector<uint8_t>& code) {
    std::vector<int32_t> stack;
    size_t ip = 0;

    while (ip < code.size()) {
        uint8_t op = code[ip++];
        switch (op) {
            case OP_PUSH: {
                int32_t val;
                std::memcpy(&val, &code[ip], 4);
                stack.push_back(val);
                ip += 4;
                break;
            }
            case OP_ADD: {
                int32_t b = stack.back(); stack.pop_back();
                int32_t a = stack.back(); stack.pop_back();
                stack.push_back(a + b);
                break;
            }
            case OP_PRINT:
                std::cout << "Amber Out: " << stack.back() << std::endl;
                stack.pop_back();
                break;
            case OP_HALT:
                return;
        }
    }
}
