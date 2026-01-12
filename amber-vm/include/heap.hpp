#ifndef HEAP_HPP
#define HEAP_HPP

#include <vector>
#include <cstddef>

struct AmberObject {
    bool marked;
};

class Heap {
    std::vector<AmberObject*> objects;
public:
    void* allocate(size_t size);
    void collect();
};

#endif