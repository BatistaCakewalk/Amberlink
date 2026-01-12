// amber-vm/include/bytecode.hpp
#pragma once
#include <cstdint>

enum OpCode : uint8_t {
    OP_PUSH  = 0x01, // Followed by 4-byte int
    OP_ADD   = 0x02,
    OP_PRINT = 0x03,
    OP_HALT  = 0xFF
};
