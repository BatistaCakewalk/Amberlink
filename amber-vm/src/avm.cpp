#include "avm.hpp"
#include <iostream>
#include <stack>
#include <stdexcept>
#include <cstring> // For std::memcpy

// A simple macro to reduce boilerplate for binary operations.
// It pops two values, performs the operation, and pushes the result.
#define BINARY_OP(op) \
    do { \
        if (vm_stack.size() < 2) throw std::runtime_error("Stack underflow during binary operation."); \
        int32_t b = vm_stack.back(); vm_stack.pop_back(); \
        int32_t a = vm_stack.back(); vm_stack.pop_back(); \
        vm_stack.push_back(a op b); \
    } while (false)

void execute(const std::vector<uint8_t>& bytecode) {
    if (bytecode.empty()) {
        return; // Nothing to execute
    }

    // Use vector for cache locality (contiguous memory) instead of std::stack/deque
    std::vector<int32_t> vm_stack;
    vm_stack.reserve(1024); // Pre-allocate reasonable stack space

    // Global variables storage (Simple indexed memory)
    std::vector<int32_t> globals;

    const uint8_t* ip = bytecode.data();
    const uint8_t* end = ip + bytecode.size();

    try {
        while (ip < end) {
            uint8_t instruction = *ip++;
            switch (instruction) {
                case 0x01: { // OP_PUSH
                    // Safe unaligned read using memcpy
                    int32_t value;
                    std::memcpy(&value, ip, sizeof(int32_t));
                    vm_stack.push_back(value);
                    ip += sizeof(int32_t);
                    break;
                }
                case 0x02: BINARY_OP(+); break; // OP_ADD
                case 0x03: BINARY_OP(-); break; // OP_SUB
                case 0x04: BINARY_OP(*); break; // OP_MUL
                case 0x05: { // OP_DIV
                    if (vm_stack.size() < 2) throw std::runtime_error("Stack underflow during DIV.");
                    int32_t b = vm_stack.back(); vm_stack.pop_back();
                    if (b == 0) throw std::runtime_error("Division by zero.");
                    int32_t a = vm_stack.back(); vm_stack.pop_back();
                    vm_stack.push_back(a / b);
                    break;
                }
                case 0x06: { // OP_PRINT
                    if (vm_stack.empty()) throw std::runtime_error("Stack underflow during PRINT.");
                    std::cout << "Amber Out: " << vm_stack.back() << std::endl;
                    vm_stack.pop_back();
                    break;
                }
                case 0x07: { // OP_STORE (Global)
                    int32_t index;
                    std::memcpy(&index, ip, sizeof(int32_t));
                    ip += sizeof(int32_t);

                    if (vm_stack.empty()) throw std::runtime_error("Stack underflow during STORE.");
                    int32_t val = vm_stack.back();
                    vm_stack.pop_back();

                    if (index >= globals.size()) globals.resize(index + 1);
                    globals[index] = val;
                    break;
                }
                case 0x08: { // OP_LOAD (Global)
                    int32_t index;
                    std::memcpy(&index, ip, sizeof(int32_t));
                    ip += sizeof(int32_t);

                    if (index < 0 || index >= globals.size()) throw std::runtime_error("Global variable index out of bounds.");
                    vm_stack.push_back(globals[index]);
                    break;
                }
                case 0xFF: { // OP_HALT
                    if (!vm_stack.empty()) {
                        std::cout << "Result: " << vm_stack.back() << std::endl;
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