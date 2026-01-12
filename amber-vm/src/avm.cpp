#include "avm.hpp"
#include <iostream>
#include <stack>
#include <stdexcept>

// A simple macro to reduce boilerplate for binary operations.
// It pops two values, performs the operation, and pushes the result.
#define BINARY_OP(op) \
    do { \
        if (vm_stack.size() < 2) throw std::runtime_error("Stack underflow during binary operation."); \
        int32_t b = vm_stack.top(); vm_stack.pop(); \
        int32_t a = vm_stack.top(); vm_stack.pop(); \
        vm_stack.push(a op b); \
    } while (false)

void execute(const std::vector<uint8_t>& bytecode) {
    if (bytecode.empty()) {
        return; // Nothing to execute
    }

    std::stack<int32_t> vm_stack;
    const uint8_t* ip = bytecode.data();
    const uint8_t* end = ip + bytecode.size();

    try {
        while (ip < end) {
            uint8_t instruction = *ip++;
            switch (instruction) {
                case 0x01: { // OP_PUSH
                    int32_t value = *reinterpret_cast<const int32_t*>(ip);
                    vm_stack.push(value);
                    ip += sizeof(int32_t);
                    break;
                }
                case 0x02: BINARY_OP(+); break; // OP_ADD
                case 0x03: BINARY_OP(-); break; // OP_SUB
                case 0x04: BINARY_OP(*); break; // OP_MUL
                case 0x05: { // OP_DIV
                    if (vm_stack.size() < 2) throw std::runtime_error("Stack underflow during DIV.");
                    int32_t b = vm_stack.top(); vm_stack.pop();
                    if (b == 0) throw std::runtime_error("Division by zero.");
                    int32_t a = vm_stack.top(); vm_stack.pop();
                    vm_stack.push(a / b);
                    break;
                }
                case 0xFF: { // OP_HALT
                    if (!vm_stack.empty()) {
                        std::cout << "Result: " << vm_stack.top() << std::endl;
                    }
                    return; // End execution
                }
                default: {
                    throw std::runtime_error("Unknown opcode encountered.");
                }
            }
        }
    } catch (const std::runtime_error& e) {
        std::cerr << "AVM Runtime Error: " << e.what() << std::endl;
    }
}