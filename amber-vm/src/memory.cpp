// amber-vm/src/memory.cpp
#include "heap.hpp"
#include <cstdlib>

void* Heap::allocate(size_t size) {
    if (objects.size() > 1000) collect(); // Simple threshold trigger

    void* raw = std::malloc(sizeof(AmberObject) + size);
    AmberObject* obj = (AmberObject*)raw;
    obj->marked = false;
    objects.push_back(obj);
    
    return (void*)(obj + 1); // Return pointer to data after header
}

void Heap::collect() {
    // 1. Mark Phase (Simplified: in a real VM, you'd scan the stack)
    // 2. Sweep Phase
    for (size_t i = 0; i < objects.size(); ) {
        if (!objects[i]->marked) {
            std::free(objects[i]);
            // Optimization: Swap with last element and pop to avoid O(N) shift
            objects[i] = objects.back();
            objects.pop_back();
        } else {
            objects[i]->marked = false; // Reset for next cycle
            i++;
        }
    }
}
