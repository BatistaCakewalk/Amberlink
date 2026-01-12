#include <iostream>
#include <vector>
#include "avm.hpp"

int main(int argc, char** argv) {
    if (argc < 2) {
        std::cout << "Usage: avm <file.amc>" << std::endl;
        return 1;
    }

    std::vector<uint8_t> bytecode;
    if (Loader::load(argv[1], bytecode)) {
        execute(bytecode);
    } else {
        std::cerr << "Failed to load " << argv[1] << std::endl;
    }

    return 0;
}
