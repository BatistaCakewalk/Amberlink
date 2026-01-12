// amber-vm/src/memory.cpp
#include "heap.hpp"
#include "collector.hpp"
#include <cstdlib>

namespace AVM {
    void* Heap::allocate(size_t size) {
        // 1. Calculate total size needed (Header + Data)
        size_t total_size = sizeof(AmberObject) + size;

        // 2. Threshold Check: If we exceed 80% of Nursery, collect first
        if (this->used_memory + total_size > this->nursery_threshold) {
            GC::minor_collect(); 
        }

        // 3. The Actual Allocation
        void* raw_mem = std::malloc(total_size);
        if (!raw_mem) return nullptr; // Out of Memory!

        // 4. Initialize the Header
        AmberObject* obj = static_cast<AmberObject*>(raw_mem);
        obj->type_tag = 0;      // Set by the caller
        obj->marked = false;    // Default to unmarked
        obj->flags = 0;

        this->used_memory += total_size;
        
        // Return the address right after the header
        return static_cast<void*>(obj + 1);
    }
}
