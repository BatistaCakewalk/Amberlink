#include "heap.hpp"
#include <iostream>
#include <algorithm>

Heap::~Heap() {
    for (AmberObject* obj : objects) {
        if (obj) delete obj;
    }
    objects.clear();
}

int32_t Heap::register_object(AmberObject* obj) {
    if (!free_slots.empty()) {
        size_t idx = free_slots.back();
        free_slots.pop_back();
        objects[idx] = obj;
        std::cout << "[GC] Reusing slot " << idx << std::endl;
        return static_cast<int32_t>(idx);
    } else {
        objects.push_back(obj);
        std::cout << "[GC] Allocating new slot " << (objects.size() - 1) << std::endl;
        return static_cast<int32_t>(objects.size() - 1);
    }
}

void Heap::mark(AmberObject* obj, size_t constant_pool_size) {
    if (obj == nullptr || obj->marked) return;
    
    obj->marked = true;
    
    if (obj->type == ObjType::ARRAY) {
        ArrayObject* arr = static_cast<ArrayObject*>(obj);
        for (int32_t val : arr->data) {
            // Check if val is a heap handle
            // Handle logic: val = -(HEAP_HANDLE_OFFSET + heap_idx)
            // Therefore: -val = HEAP_HANDLE_OFFSET + heap_idx
            if (val <= -HEAP_HANDLE_OFFSET) {
                size_t heap_idx = -val - HEAP_HANDLE_OFFSET;
                if (heap_idx < objects.size()) {
                    mark(objects[heap_idx], constant_pool_size);
                }
            }
        }
    } else if (obj->type == ObjType::INSTANCE) {
        InstanceObject* inst = static_cast<InstanceObject*>(obj);
        for (int32_t val : inst->fields) {
            if (val <= -HEAP_HANDLE_OFFSET) {
                size_t heap_idx = -val - HEAP_HANDLE_OFFSET;
                if (heap_idx < objects.size()) {
                    mark(objects[heap_idx], constant_pool_size);
                }
            }
        }
    }
}

void Heap::collect(const std::vector<int32_t>& stack, const std::vector<int32_t>& globals, size_t constant_pool_size) {
    // 1. Unmark all objects (Reset)
    for (AmberObject* obj : objects) {
        if (obj) obj->marked = false;
    }

    // 2. Mark Roots (Stack)
    for (int32_t val : stack) {
        if (val <= -HEAP_HANDLE_OFFSET) {
            size_t heap_idx = -val - HEAP_HANDLE_OFFSET;
            if (heap_idx < objects.size()) {
                mark(objects[heap_idx], constant_pool_size);
            }
        }
    }

    // 3. Mark Roots (Globals)
    for (int32_t val : globals) {
        if (val <= -HEAP_HANDLE_OFFSET) {
            size_t heap_idx = -val - HEAP_HANDLE_OFFSET;
            if (heap_idx < objects.size()) {
                mark(objects[heap_idx], constant_pool_size);
            }
        }
    }

    // 4. Sweep
    sweep();
}

void Heap::sweep() {
    for (size_t i = 0; i < objects.size(); ++i) {
        AmberObject* obj = objects[i];
        if (obj) {
            if (!obj->marked) {
                delete obj;
                objects[i] = nullptr;
                free_slots.push_back(i);
            }
        }
    }
}