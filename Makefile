# Makefile for building the C++ example with the Rust static library

# Variables
CARGO := cargo
CXX := g++
RUST_LIB_NAME := cfb
RUST_STATIC_LIB := target/release/lib$(RUST_LIB_NAME).a
CPP_EXAMPLE_DIR := cpp_example
CPP_EXAMPLE_SRC := $(CPP_EXAMPLE_DIR)/main.cpp
CPP_EXAMPLE_OBJ := $(CPP_EXAMPLE_DIR)/main.o
CPP_EXAMPLE_BIN := $(CPP_EXAMPLE_DIR)/main
HEADER_DIR := include
HEADER_FILE := $(HEADER_DIR)/cfb.h
CXXFLAGS := -I$(HEADER_DIR) -std=c++11
LDFLAGS := -Ltarget/release -l$(RUST_LIB_NAME) -ldl -lpthread

.PHONY: all clean

all: $(CPP_EXAMPLE_BIN)

# Build the C++ example
$(CPP_EXAMPLE_BIN): $(CPP_EXAMPLE_OBJ) $(RUST_STATIC_LIB)
	$(CXX) $(CXXFLAGS) $(CPP_EXAMPLE_OBJ) -o $@ $(LDFLAGS)

$(CPP_EXAMPLE_OBJ): $(CPP_EXAMPLE_SRC) $(HEADER_FILE)
	$(CXX) $(CXXFLAGS) -c $(CPP_EXAMPLE_SRC) -o $@

# Build the Rust static library
$(RUST_STATIC_LIB):
	$(CARGO) build --release

# Generate the C header file
$(HEADER_FILE): src/ffi.rs cbindgen.toml
	~/.cargo/bin/cbindgen --config cbindgen.toml --crate cfb --output $(HEADER_FILE)

# Clean up build artifacts
clean:
	rm -f $(CPP_EXAMPLE_OBJ) $(CPP_EXAMPLE_BIN)
	$(CARGO) clean
	rm -rf $(HEADER_DIR)
