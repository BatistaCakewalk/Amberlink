#ifndef HEAP_HPP
#define HEAP_HPP

#include <vector>
#include <cstddef>

enum class ObjType {
    STRING,
    ARRAY,
    INSTANCE
};

struct AmberObject {
    bool marked = false;
    ObjType type;
    virtual ~AmberObject() = default;
};

struct ArrayObject : AmberObject {
    std::vector<int32_t> data;
    ArrayObject(size_t size) {
        type = ObjType::ARRAY;
        data.resize(size, 0);
    }
};

class Heap {
public:
    std::vector<AmberObject*> objects; // Public for direct access by VM
    std::vector<size_t> free_slots;    // Indices of freed objects (holes)
    ~Heap();
    int32_t register_object(AmberObject* obj);
    void mark(AmberObject* obj, size_t constant_pool_size);
    void collect(const std::vector<int32_t>& stack, const std::vector<int32_t>& globals, size_t constant_pool_size);
    void sweep();
};

#endif