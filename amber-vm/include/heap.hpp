// amber-vm/include/heap.hpp
#pragma once
#include <vector>
#include <cstdint>

struct AmberObject {
    bool marked;
    uint32_t type;
    // Data follows in memory
};

class Heap {
public:
    static Heap& getInstance() {
        static Heap instance;
        return instance;
    }

    void* allocate(size_t size);
    void collect();

private:
    std::vector<AmberObject*> objects;
    Heap() = default;
};
