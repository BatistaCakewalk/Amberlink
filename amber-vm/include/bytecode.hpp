// amber-vm/include/bytecode.hpp
#pragma once
#include <vector>

namespace AVM {
    enum OpCode {
        PUSH_CONST,  // Push a value to the stack
        CALL_FUNC,   // Call a standalone function
        INVOKE_METH, // Call a class method
        STORE_VAR,   // Save to memory
        HALT
    };

    struct Bytecode {
        OpCode op;
        int arg;
    };
}
