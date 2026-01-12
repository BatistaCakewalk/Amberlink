#ifndef AVM_HPP
#define AVM_HPP

#include <vector>
#include <cstdint>

// Forward-declare the loader function used in main.cpp
namespace Loader {
    bool load(const char* filename, std::vector<uint8_t>& bytecode);
}

void execute(const std::vector<uint8_t>& bytecode);

#endif // AVM_HPP