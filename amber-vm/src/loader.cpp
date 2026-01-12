// amber-vm/src/loader.cpp
#include "avm.hpp"
#include <fstream>
#include <iostream>

bool Loader::load(const std::string& path, std::vector<uint8_t>& buffer) {
    std::ifstream file(path, std::ios::binary);
    if (!file) return false;

    char magic[4];
    file.read(magic, 4); // Read "AMBR"
    
    uint16_t version;
    file.read((char*)&version, 2);

    uint32_t entry, len;
    file.read((char*)&entry, 4);
    file.read((char*)&len, 4);

    buffer.resize(len);
    file.read((char*)buffer.data(), len);
    return true;
}
