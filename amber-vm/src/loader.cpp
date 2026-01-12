#include "avm.hpp"
#include <fstream>
#include <iostream>
#include <cstring>

namespace Loader {
    bool load(const char* filename, std::vector<uint8_t>& bytecode) {
        std::ifstream file(filename, std::ios::binary);
        if (!file) {
            std::cerr << "Error: Could not open file " << filename << std::endl;
            return false;
        }

        // 1. Verify Magic Header "AMBR"
        char magic[4];
        file.read(magic, 4);
        if (std::strncmp(magic, "AMBR", 4) != 0) {
            std::cerr << "Error: Invalid file format. Expected 'AMBR' header." << std::endl;
            return false;
        }

        // 2. Skip Version (2 bytes) and Entry Point (4 bytes)
        file.ignore(6);

        // 3. Read Code Length
        uint32_t codeLength;
        file.read(reinterpret_cast<char*>(&codeLength), 4);

        // 4. Read Bytecode
        if (codeLength > 0) {
            bytecode.resize(codeLength);
            file.read(reinterpret_cast<char*>(bytecode.data()), codeLength);
        }

        return true;
    }
}