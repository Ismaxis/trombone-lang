# Makefile for building and testing CMake presets
# Usage:
#   make build PRESET=debug JOBS=8
#   make test PRESET=release JOBS=8

PRESET ?= debug
JOBS ?= 2

.PHONY: build test clean

build:
	cmake --preset $(PRESET)
	cmake --build --preset $(PRESET) --parallel $(JOBS)

test:
	ctest --preset $(PRESET) --output-on-failure

clean:
	cmake --build --preset $(PRESET) --target clean
