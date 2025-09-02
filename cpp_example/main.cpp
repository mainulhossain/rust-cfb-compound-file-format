#include <iostream>
#include <fstream>
#include <vector>
#include <string>

#include "../include/cfb.h"

int main() {
    const char* cfb_path = "test.cfb";
    const char* stream_path = "/hello.txt";
    const std::string content = "Hello, World!";

    // Create a new compound file.
    if (cfb_create_compound_file(cfb_path) != 0) {
        std::cerr << "Failed to create compound file." << std::endl;
        return 1;
    }

    // Open the compound file.
    OpaqueCompoundFile* cf = cfb_open_compound_file(cfb_path);
    if (cf == nullptr) {
        std::cerr << "Failed to open compound file." << std::endl;
        return 1;
    }

    // Create a stream and write data to it.
    if (cfb_create_stream(cf, stream_path, reinterpret_cast<const uint8_t*>(content.c_str()), content.length()) != 0) {
        std::cerr << "Failed to create stream." << std::endl;
        cfb_close_compound_file(cf);
        return 1;
    }

    // Close the compound file.
    cfb_close_compound_file(cf);

    std::cout << "Successfully created " << cfb_path << " with stream " << stream_path << std::endl;

    return 0;
}
