#include "avm.hpp"
#include "bytecode.hpp"
#include <iostream>
#include <vector>
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
                // --- Control Flow ---
                case OP_HALT: {
                    return; // End execution
                }
                case OP_JUMP: {
                    int32_t offset;
                    std::memcpy(&offset, ip, sizeof(int32_t));
                    ip += 4;      // Consume the 4-byte offset from the instruction stream
                    ip += offset; // Apply the relative jump
                    break;
                }
                case OP_JUMP_IF_FALSE: {
                    int32_t offset;
                    std::memcpy(&offset, ip, sizeof(int32_t));
                    ip += 4; // Advance past the offset bytes

                    if (vm_stack.empty()) throw std::runtime_error("Stack underflow during JUMP_IF_FALSE.");
                    int32_t condition = vm_stack.back(); vm_stack.pop_back();
                    
                    if (condition == 0) { // 0 is False
                        ip += offset;
                    }
                    break;
                }

                // --- Constants & Variables ---
                case OP_PUSH: {
                    int32_t value;
                    std::memcpy(&value, ip, sizeof(int32_t));
                    vm_stack.push_back(value);
                    ip += sizeof(int32_t);
                    break;
                }
                case OP_STORE_GLOBAL: {
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
                case OP_LOAD_GLOBAL: {
                    int32_t index;
                    std::memcpy(&index, ip, sizeof(int32_t));
                    ip += sizeof(int32_t);

                    if (index < 0 || index >= globals.size()) throw std::runtime_error("Global variable index out of bounds.");
                    vm_stack.push_back(globals[index]);
                    break;
                }

                // --- Arithmetic & Logic ---
                case OP_ADD: BINARY_OP(+); break;
                case OP_SUB: BINARY_OP(-); break;
                case OP_MUL: BINARY_OP(*); break;
                case OP_DIV: {
                    if (vm_stack.size() < 2) throw std::runtime_error("Stack underflow during DIV.");
                    int32_t b = vm_stack.back(); vm_stack.pop_back();
                    if (b == 0) throw std::runtime_error("Division by zero.");
                    int32_t a = vm_stack.back(); vm_stack.pop_back();
                    vm_stack.push_back(a / b);
                    break;
                }

                // --- Utilities ---
                case OP_POP: vm_stack.pop_back(); break;
                case OP_PRINT: std::cout << "Amber Out: " << vm_stack.back() << std::endl; vm_stack.pop_back(); break;

                default: {
                    throw std::runtime_error("Unknown opcode encountered.");
                }
            }
        }
    } catch (const std::runtime_error& e) {
        std::cerr << "AVM Runtime Error: " << e.what() << std::endl;
    }
}