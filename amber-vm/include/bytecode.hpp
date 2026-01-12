// amber-vm/include/bytecode.hpp
#pragma once
#include <cstdint>

enum OpCode : uint8_t {
    // --- Control Flow ---
    OP_HALT           = 0x00, // Stop execution
    OP_JUMP           = 0x01, // Unconditional jump by a 4-byte signed offset
    OP_JUMP_IF_FALSE  = 0x02, // Pop a value; jump if it's 0

    // --- Constants & Variables ---
    OP_PUSH           = 0x10, // Push a 4-byte constant onto the stack
    OP_STORE_GLOBAL   = 0x11, // Pop a value and store it in a global variable slot (by 4-byte index)
    OP_LOAD_GLOBAL    = 0x12, // Load a global variable (by 4-byte index) onto the stack

    // --- Arithmetic & Logic ---
    OP_ADD            = 0x20,
    OP_SUB            = 0x21,
    OP_MUL            = 0x22,
    OP_DIV            = 0x23,
    // Future: OP_EQUAL, OP_GREATER, OP_LESS

    // --- Functions & Calls ---
    // Future: OP_CALL, OP_RETURN

    // --- Utilities ---
    OP_POP            = 0x80, // Pop the top value from the stack and discard it
    OP_PRINT          = 0x81, // Pop the top value and print it to the console
};
