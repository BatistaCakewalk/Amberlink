// amber-vm/include/bytecode.hpp
#pragma once
#include <cstdint>

enum OpCode : uint8_t {
    // Control Flow
    OP_HALT  = 0x00,

    // Constants
    OP_PUSH  = 0x01, // Followed by 4-byte int

    // Arithmetic
    OP_ADD   = 0x02,
    OP_SUB   = 0x03,
    OP_MUL   = 0x04,
    OP_DIV   = 0x05,

    // Debugging
    OP_PRINT = 0xFD,
};
