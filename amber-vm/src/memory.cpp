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
    for (auto it = objects.begin(); it != objects.end();) {
        if (!(*it)->marked) {
            std::free(*it);
            it = objects.erase(it);
        } else {
            (*it)->marked = false; // Reset for next cycle
            ++it;
        }
    }
}
