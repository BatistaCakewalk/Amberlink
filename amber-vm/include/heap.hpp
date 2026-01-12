// amber-vm/include/heap.hpp
#pragma once
#include <cstdint>

namespace AVM {
    enum class ObjType {
        INSTANCE, // A class object
        STRING,
        ARRAY,
        FUNC_PTR  // For those standalone functions!
    };

    struct AmberObject {
        // --- The Header (8 bytes) ---
        uint32_t type_tag;  // What kind of object is this?
        bool marked;        // Used by the Garbage Collector
        uint8_t flags;      // Special properties (e.g., is it immutable?)
        
        // --- The Payload ---
        // Data follows here in memory...
    };
}
