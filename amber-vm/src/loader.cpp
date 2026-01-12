#include "avm.hpp"
#include <fstream>
#include <iostream>
#include <cstring>

namespace Loader {
    bool load(const char* filename, std::vector<uint8_t>& bytecode, std::vector<std::string>& constants) {
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

        // 3. Read Constant Pool
        uint32_t poolCount;
        file.read(reinterpret_cast<char*>(&poolCount), 4);
        for (uint32_t i = 0; i < poolCount; ++i) {
            uint32_t strLen;
            file.read(reinterpret_cast<char*>(&strLen), 4);
            
            std::string s;
            s.resize(strLen);
            file.read(&s[0], strLen);
            constants.push_back(s);
        }

        // 4. Read Code Length
        uint32_t codeLength;
        file.read(reinterpret_cast<char*>(&codeLength), 4);

        // 5. Read Bytecode
        if (codeLength > 0) {
            bytecode.resize(codeLength);
            file.read(reinterpret_cast<char*>(bytecode.data()), codeLength);
            if (!file) {
                std::cerr << "Error: Unexpected end of file while reading bytecode." << std::endl;
                return false;
            }
        }

        return true;
    }
}