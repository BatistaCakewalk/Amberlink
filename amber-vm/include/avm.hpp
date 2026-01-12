#ifndef AVM_HPP
#define AVM_HPP

#include <vector>
#include <cstdint>
#include <string>

// Forward-declare the loader function used in main.cpp
namespace Loader {
    bool load(const char* filename, std::vector<uint8_t>& bytecode, std::vector<std::string>& constants);
}

void execute(const std::vector<uint8_t>& bytecode, std::vector<std::string>& constants);

#endif // AVM_HPP