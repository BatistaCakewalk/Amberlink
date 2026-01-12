// amber-vm/src/loader.cpp
#include "avm.hpp"
#include <fstream>
#include <vector>
#include <iostream>

bool AVM::Loader::load_file(const std::string& filename, std::vector<uint8_t>& code_buffer) {
    std::ifstream file(filename, std::ios::binary);
    if (!file.is_open()) return false;

    // 1. Verify Magic Number
    uint32_t magic;
    file.read(reinterpret_cast<char*>(&magic), 4);
    if (magic != 0x414D4252) { // "AMBR"
        std::cerr << "Error: Not a valid Amberlink file!" << std::endl;
        return false;
    }

    // 2. Read Metadata
    uint16_t version;
    file.read(reinterpret_cast<char*>(&version), 2);

    uint32_t entry_point;
    file.read(reinterpret_cast<char*>(&entry_point), 4);

    // 3. Load the Bytecode
    uint32_t code_len;
    file.read(reinterpret_cast<char*>(&code_len), 4);
    
    code_buffer.resize(code_len);
    file.read(reinterpret_cast<char*>(code_buffer.data()), code_len);

    file.close();
    return true;
}
