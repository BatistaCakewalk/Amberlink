// amber-vm/include/bytecode.hpp
#pragma once
#include <cstdint>

namespace Amber {
    enum class OpCode : uint8_t {
        OP_PUSH = 0x01,  // Push value to stack
        OP_ADD  = 0x02,  // Add top two stack values
        OP_PRINT = 0x03, // Print top value
        OP_HALT = 0xFF   // Stop execution
    };

    struct Instruction {
        OpCode code;
        int32_t operand;
    };
}
